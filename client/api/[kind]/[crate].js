// Vercel Function: run a chapter's `cargo test` / `cargo clippy` in a Vercel
// Sandbox and return { pass, stdout, stderr } — the only piece of the old Rust
// server that genuinely needs a backend.
//
// Route: POST /api/<kind>/<crate>  where kind ∈ {check, clippy}.
// Body: raw text = the learner's src/lib.rs.
//
// Uses an explicit nested dynamic route ([kind]/[crate]) rather than a single
// catch-all ([...path]) — on Vercel the catch-all only matched one path segment,
// so /api/check/<crate> (two segments) 404'd.
//
// The chapter crates inherit edition/version from the `chapters/` Cargo
// workspace, so they are NOT standalone — we materialise the whole workspace in
// the sandbox (git source) and run `cargo <cmd> -p <crate>`, exactly like local.
// No chapter has external dependencies, so there are no crates.io fetches.
//
// Written against @vercel/sandbox 2.x: Sandbox.create({ source }) clones the repo
// (or restores a snapshot); runCommand returns a CommandFinished with .exitCode
// and async .stdout()/.stderr(); writeFiles accepts string content. The cloned
// repo lands at the sandbox working dir, so paths below are relative to it.

import { Sandbox } from '@vercel/sandbox'

// @vercel/sandbox's API client calls the deprecated url.parse() internally
// (DEP0169). It's harmless and not our code to fix — silence it so it doesn't
// clutter the function logs.
process.noDeprecation = true

// Hobby caps function duration at 300s; a cold toolchain install + compile can
// approach a minute. Bake a snapshot (see DEPLOY-VERCEL.md) to make it fast/cheap.
export const config = { maxDuration: 300 }

const CRATE_RE = /^[a-z0-9_]+$/
const TIMEOUT_MS = 290_000

// Clone URL of THIS repo + the ref to test against (Vercel project env).
const REPO_URL = process.env.COURSE_REPO_URL
const REPO_REF = process.env.COURSE_REPO_REF || 'main'
// Optional creds for a private repo (e.g. a GitHub PAT as the password).
const REPO_USER = process.env.COURSE_REPO_USERNAME
const REPO_PASS = process.env.COURSE_REPO_TOKEN
// Optional pre-baked snapshot (toolchain + cloned workspace + warm target) that
// skips the cold install/clone on every request.
const SNAPSHOT_ID = process.env.SANDBOX_SNAPSHOT_ID

export default async function handler(req, res) {
  if (req.method !== 'POST') {
    return res.status(405).json({ pass: false, stdout: '', stderr: 'method not allowed' })
  }
  const { kind, crate } = req.query
  if ((kind !== 'check' && kind !== 'clippy') || !crate || !CRATE_RE.test(crate)) {
    return res.status(404).json({ pass: false, stdout: '', stderr: 'not found' })
  }
  if (!SNAPSHOT_ID && !REPO_URL) {
    return res.status(500).json({
      pass: false,
      stdout: '',
      stderr: 'server misconfigured: set COURSE_REPO_URL (or SANDBOX_SNAPSHOT_ID)',
    })
  }

  const code = await readBody(req)
  try {
    return res.status(200).json(await runInSandbox(kind, crate, code))
  } catch (e) {
    // Surface sandbox/infra failures in the output panel rather than a bare 500.
    return res.status(200).json({ pass: false, stdout: '', stderr: `sandbox error: ${e?.message ?? e}` })
  }
}

function createParams() {
  if (SNAPSHOT_ID) {
    return { source: { type: 'snapshot', snapshotId: SNAPSHOT_ID }, timeout: TIMEOUT_MS }
  }
  const source = { type: 'git', url: REPO_URL, revision: REPO_REF, depth: 1 }
  if (REPO_USER && REPO_PASS) {
    source.username = REPO_USER
    source.password = REPO_PASS
  }
  return { source, timeout: TIMEOUT_MS }
}

// Keep the toolchain inside the sandbox-writable workdir (HOME is /root, which the
// unprivileged sandbox user can't write) so install + cargo both run without sudo.
const RUST_ENV =
  'export CARGO_HOME=/vercel/sandbox/.cargo RUSTUP_HOME=/vercel/sandbox/.rustup ' +
  'PATH=/vercel/sandbox/.cargo/bin:$PATH;'

async function runInSandbox(kind, crate, code) {
  const sandbox = await Sandbox.create(createParams())
  try {
    // 1. Toolchain. A snapshot already has it; otherwise install it cold.
    if (!SNAPSHOT_ID) {
      // Rust links test binaries with a C linker (`cc`); the base image ships none.
      const gcc = await sh(sandbox, 'sudo dnf install -y gcc')
      if (gcc.exitCode !== 0) {
        return { pass: false, stdout: gcc.stdout, stderr: `installing gcc failed:\n${gcc.stderr}` }
      }
      // Install the latest stable via rustup — edition 2024 needs rustc >= 1.85,
      // newer than the distro's packaged rust. --no-modify-path since PATH is set.
      const install = await sh(
        sandbox,
        RUST_ENV +
          "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs " +
          '| sh -s -- -y --profile minimal --component clippy --no-modify-path',
      )
      if (install.exitCode !== 0) {
        return { pass: false, stdout: install.stdout, stderr: `toolchain install failed:\n${install.stderr}` }
      }
    }

    // 2. Overwrite the chapter's working file with the learner's code. src/lib.rs
    //    is git-ignored, so it won't exist in the clone — create the dir first.
    const libDir = `chapters/${crate}/src`
    await sh(sandbox, `mkdir -p ${libDir}`)
    await sandbox.writeFiles([{ path: `${libDir}/lib.rs`, content: code }])

    // 3. Compile + test (or lint). -D warnings makes "clippy clean" meaningful.
    const cargo =
      kind === 'check'
        ? `cargo test -p ${crate}`
        : `cargo clippy -p ${crate} --tests -- -D warnings`
    const run = await sh(sandbox, `${RUST_ENV} cd chapters && ${cargo}`)

    return { pass: run.exitCode === 0, stdout: run.stdout, stderr: run.stderr }
  } finally {
    try {
      await sandbox.stop()
    } catch {
      /* best effort */
    }
  }
}

// Run one shell line; resolve once it exits. Returns {exitCode, stdout, stderr}.
async function sh(sandbox, line) {
  const cmd = await sandbox.runCommand({ cmd: 'bash', args: ['-lc', line] })
  const [stdout, stderr] = await Promise.all([cmd.stdout(), cmd.stderr()])
  return { exitCode: cmd.exitCode ?? 0, stdout: String(stdout), stderr: String(stderr) }
}

function readBody(req) {
  if (typeof req.body === 'string') return Promise.resolve(req.body)
  if (req.body && typeof req.body === 'object' && typeof req.body.code === 'string') {
    return Promise.resolve(req.body.code)
  }
  return new Promise((resolve) => {
    let data = ''
    req.on('data', (c) => (data += c))
    req.on('end', () => resolve(data))
    req.on('error', () => resolve(''))
  })
}
