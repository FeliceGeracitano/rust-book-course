# Deploying to Vercel (Sandbox build)

This branch (`vercel-sandbox-deploy`) is a **slimmed, all-Vercel** version of the
course: a static React client on Vercel + a single Function that runs each
chapter's `cargo test` / `cargo clippy` in a **Vercel Sandbox**.

**Dropped vs the local Docker build** (on purpose, to fit a $0 host):
- no rust-analyzer LSP (editor uses the curated completion list)
- no server-side file persistence — your edits + progress live in the browser
  (`localStorage`)
- no "open in your editor" deep link

## What runs where

| Piece | Where |
|---|---|
| React UI, lessons, solutions, exercises | static assets on Vercel (`/course/*`, baked at build by `scripts/prepare-content.mjs`) |
| Editor working copy + progress | browser `localStorage` |
| `Check` / `Clippy` | `client/api/[...path].js` → Vercel Sandbox runs `cargo` |

## One-time setup

1. **Push this branch** to GitHub. The Sandbox **git-clones the repo** to get the
   `chapters/` workspace, so the repo must be reachable (public is simplest; a
   private repo needs a token — see `COURSE_REPO_URL` below).

2. **Import to Vercel:** New Project → import this repo → set **Root Directory =
   `client`**. Vercel auto-detects Vite (build `vite build`, output `dist`).

3. **Environment variables** (Project → Settings → Environment Variables):
   - `COURSE_REPO_URL` = `https://github.com/<you>/rust-book-course.git`
   - `COURSE_REPO_REF` = `vercel-sandbox-deploy` (or `main` once merged)
   - `COURSE_REPO_USERNAME` + `COURSE_REPO_TOKEN` = *(only for a private repo — e.g.
     your GitHub username and a PAT; public repos need neither)*
   - `SANDBOX_SNAPSHOT_ID` = *(optional, set after baking a snapshot — see below)*

4. **Deploy.** Push to the branch → Vercel builds and gives you a URL.

Vercel Sandbox uses the project's OIDC token automatically — no extra auth.

## Make it fast & cheap (recommended): bake a snapshot

Without a snapshot, **every** check cold-installs the Rust toolchain (~30–60s) and
re-clones the repo — slow, and it burns the Hobby free Sandbox budget
(5 Active-CPU-hrs/mo) in ~100 checks. A snapshot bakes the toolchain + cloned
workspace (and, if you pre-build, a warm `target/`) so each check is just an
incremental compile (~5–15s).

Bake one once (locally, after `vercel link` + `vercel env pull`), then set
`SANDBOX_SNAPSHOT_ID` to the printed id and redeploy. Sketch (@vercel/sandbox 2.x):

```js
import { Sandbox } from '@vercel/sandbox'
const ENV =
  'export CARGO_HOME=/vercel/sandbox/.cargo RUSTUP_HOME=/vercel/sandbox/.rustup ' +
  'PATH=/vercel/sandbox/.cargo/bin:$PATH;'
// Clone the workspace via the git source, same as the function does.
const s = await Sandbox.create({
  source: { type: 'git', url: process.env.COURSE_REPO_URL, revision: '<ref>', depth: 1 },
  timeout: 600_000,
})
const sh = (line) => s.runCommand({ cmd: 'bash', args: ['-lc', line] })
await sh('sudo dnf install -y gcc')  // C linker (cc) — base image has none
await sh(ENV + "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --component clippy --no-modify-path")
// Warm target/: materialise every chapter's lib.rs from .exercise.rs and build once.
await sh(ENV + 'cd chapters && for d in */; do c=${d%/}; mkdir -p $c/src; cp $c/.exercise.rs $c/src/lib.rs; done; cargo test --workspace --no-run || true')
const snap = await s.snapshot()
console.log('SANDBOX_SNAPSHOT_ID=', snap.snapshotId)
await s.stop()
```

## Limits to know

- **Hobby Sandbox free tier:** ~5 Active-CPU-hrs + 420 GB-hrs RAM + 5,000
  creations / month. With a snapshot, ~450 checks/mo; cold, far fewer.
- **Region:** Sandbox runs in `iad1` (us-east) only.
- **Function duration:** 300s max on Hobby (set via `maxDuration` in the function).
- **Concurrency:** Hobby allows 10 concurrent sandboxes — fine for personal/demo.
