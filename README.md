# Rust Book Course 🦀

An interactive, self-hosted course for learning Rust, structured after
[The Rust Programming Language](https://doc.rust-lang.org/book/) (2024 edition).

You read each lesson in the browser, write real Rust in your terminal, and click
**Check** to run that chapter's tests. Progress and visualizations live in the UI;
the actual learning happens in `chapters/`.

## Layout

| Path | What |
|------|------|
| `chapters/` | One Cargo crate per chapter — the Rust you edit. Make the tests pass. |
| `content/`  | Lesson prose (`content/<crate>/*.md`) + `course.json` (the table of contents). |
| `server/`   | Rust `tiny_http` server: serves the UI, the lessons, and runs your tests. |
| `client/`   | React + Vite UI (chapter tree, lessons, **Check** button). |

## Learn in the terminal (works today)

```bash
cd chapters
# pick a chapter, open its src/, complete the `todo!()`s, then:
cargo test -p ch01_getting_started
```

Exercises ship **failing on purpose** — your job is to make them green.

## Run the full app

```bash
docker compose up   # -> http://localhost:8080
```

Open the page, pick a chapter, edit its crate in `chapters/`, and hit **Check** —
the server runs `cargo test` for that chapter and shows pass/fail. `chapters/` and
`content/` are mounted, so your edits are live without a rebuild. Only Docker is
required; a local Rust install is optional.

### Develop the UI

```bash
cd client && npm install && npm run dev   # Vite on :5173, proxies /api to :8080
```

## Roadmap / TODO

Tackle one at a time:

- [x] **Resizable editor pane** — drag the divider between the lesson and the editor.
- [x] **Rust autocomplete in Monaco** — curated keywords/types/macros/snippets.
- [x] **Smarter autocomplete** — also suggests symbols defined in the current file (fn/struct/enum/trait/type/const/let), non-AI.
- [x] **Resizable / draggable Check + Clippy output** — drag to grow the results panel.
- [x] **Tabbed results panel** — Output / Hints / Solution tabs; Check/Clippy focus Output.
- [x] **Chapter reset** — "↺ Reset" restores the pristine exercise (`.exercise.rs`,
      never edited) over the working `src/lib.rs`. Two-step confirm.
- [x] **Collapsible left sidebar** — ☰ toggles the chapter tree (persists).
- [x] **Highlight Clippy output** — clippy frame lines (locations, carets, notes) color-coded.
- [x] **Pointer cursor on buttons** — hand cursor on clickable buttons/links (Tailwind v4 default).
- [x] **Appendix: completed** — A–G reference pages authored, each linking the
      official Book; appendix is reference-only (no exercises).

### Bigger / later

- [x] **rust-analyzer integration** — semantic autocomplete, hover types, and live
      error squiggles via a rust-analyzer sidecar (`lsp/`) bridged over WebSocket;
      a thin in-editor LSP client (`lspClient.ts`) drives Monaco. Falls back to the
      curated list when no LSP server is configured. Runs via `docker compose up`.
- [ ] **Code-split the client bundle** — lazy-load visualizations and heavy deps
      so the initial download is smaller and first paint is faster.

## Credits & attribution

This course is **derived from [The Rust Programming Language](https://doc.rust-lang.org/book/)**
("the Book") — its chapter structure, topic order, and learning progression all come
from there. The Book is written and maintained by the Rust team and contributors.

This repo is an **unofficial** companion: it reorganizes the Book's curriculum into
hands-on, test-driven exercises plus a small local UI. It is not affiliated with or
endorsed by the Rust project. For the authoritative text, always read the Book itself:
<https://doc.rust-lang.org/book/>.

The Book is licensed under **MIT OR Apache-2.0**; any prose adapted from it here is used
under those terms. See the Book's repository: <https://github.com/rust-lang/book>.
