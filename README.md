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
| `server/`   | Rust `tiny_http` server (added in Plan #2). |
| `client/`   | React + Vite UI (added in Plan #2). |

## Learn in the terminal (works today)

```bash
cd chapters
# pick a chapter, open its src/, complete the `todo!()`s, then:
cargo test -p ch01_getting_started
```

Exercises ship **failing on purpose** — your job is to make them green.

## Run the full app

Added in Plan #2:

```bash
docker compose up   # -> http://localhost:8080
```

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
