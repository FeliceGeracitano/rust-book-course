# Rust Book Course 🦀

An interactive, self-hosted course for learning Rust, structured after
[**The Rust Programming Language**](https://doc.rust-lang.org/book/) (2024 edition).

Read each lesson in the browser, write **real Rust** in an in-page editor (or your own),
and hit **Check** to run that chapter's tests. Interactive visualizations make the hard
ideas — ownership, smart pointers, concurrency, async — click.

> `Rust 2024` · `React + Vite` · `rust-analyzer` · `Docker` — one command to run.

<img width="1866" height="1072" alt="Rust Book Course running locally" src="https://github.com/user-attachments/assets/366e6540-c27b-40fb-a488-b29caad2335e" />

---

## Quick start

```bash
docker compose up        # → http://localhost:8080
```

Only **Docker** is required (the container carries the Rust + Node toolchains). A local
Rust install is optional and only nice-to-have for editing in your own terminal.

Then: pick a chapter → read the lesson → make its tests pass → **Check** → 🎉.

---

## How it works

```
┌── Chapters ──┐   ┌──────── Lesson ────────┐   ┌──────── Editor ────────┐
│ 1 Getting…  ✓│   │  4.1 What is Ownership │   │  fn main() { … }        │
│ 4 Ownership ●│   │  [ownership viz ▸]     │   │  ⚡ rust-analyzer        │
│ …            │   │  prose + examples      │   │  [Check][Clippy][Hints] │
└──────────────┘   └────────────────────────┘   └──── pass / fail ───────┘
```

Each chapter is a real Cargo crate. Exercises ship **failing on purpose** (`todo!()`s) —
your job is to make them green. **Check** runs `cargo test` for that chapter; **Clippy**
runs the linter; **Hints** and **Reveal solution** are a tab away when you're stuck.

## Two ways to work

**In the browser** — edit in the built-in Monaco editor (rust-analyzer completions, hover
types, live error squiggles), then Check. Your edits autosave.

**In your terminal** — edit the files directly and run tests yourself:

```bash
cd chapters
cargo test -p ch01_getting_started      # one chapter
```

Both edit the same files, so you can mix and match.

---

## Features

- 🦀 **All 21 Book chapters + appendix** — real exercises, tests, solutions, and prose.
- ✍️ **In-browser Monaco editor** — resizable, with autocomplete and a **↺ Reset** to
  restore the pristine exercise.
- 🧠 **rust-analyzer** — semantic completions, hover types, and live error squiggles via
  a sidecar (falls back to a curated list if unavailable).
- ✅ **Check / Clippy** — run tests and idiomatic-Rust lints; output is color-highlighted
  in a resizable, tabbed panel (Output · Hints · Solution).
- 🎨 **Interactive visualizations** — ownership, collections, smart pointers, concurrency,
  and async, animated with step controls.
- 🧩 **Quality-of-life** — collapsible sidebar, progress ticks, confetti on pass, and an
  "open in your editor" deep link.

---

## Project layout

| Path | What |
|------|------|
| `chapters/` | One Cargo crate per chapter — the Rust you edit. `.exercise.rs` is the pristine original; `src/lib.rs` is your (git-ignored) working copy. |
| `content/`  | Lesson prose (`content/<crate>/*.md`) + `course.json` (the table of contents). |
| `server/`   | Rust `tiny_http` server — serves the UI, lessons, and runs your tests. |
| `client/`   | React + Vite UI (chapter tree, lessons, editor, visualizations). |
| `lsp/`      | rust-analyzer ↔ browser WebSocket bridge (LSP). |

## Tech stack

**Server** Rust · `tiny_http` · `serde_json`
**Client** React · Vite · TypeScript · Tailwind v4 · Shiki · Framer Motion · Monaco
**Language intelligence** rust-analyzer over a Node WebSocket bridge
**Run** Docker Compose (app + lsp)

## Develop the UI

```bash
cd client && npm install && npm run dev   # Vite on :5173, proxies /api → :8080
```

> Editor deep link opens VS Code by default; for Cursor: `EDITOR_SCHEME=cursor docker compose up`.

---

## Credits & attribution

This course is **derived from [The Rust Programming Language](https://doc.rust-lang.org/book/)**
("the Book") — its chapter structure, topic order, and learning progression all come from
there. The Book is written and maintained by the Rust team and contributors.

This repo is an **unofficial** companion: it reorganizes the Book's curriculum into
hands-on, test-driven exercises plus a small local UI. It is **not affiliated with or
endorsed by** the Rust project. For the authoritative text, always read the Book itself:
<https://doc.rust-lang.org/book/>.

The Book is licensed under **MIT OR Apache-2.0**; any prose adapted from it here is used
under those terms. See the Book's repository: <https://github.com/rust-lang/book>.
