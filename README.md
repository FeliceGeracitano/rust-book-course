# Rust Book Course 🦀

An interactive, static website for learning Rust: read, predict, and explore.
Follows [The Rust Programming Language](https://doc.rust-lang.org/book/) chapter by
chapter. No Rust installation, backend, or account needed.

## Run locally

Node.js 22.22+, 24.15+, or 26+.

```bash
cd client
npm ci
npm run dev      # → http://localhost:5173
npm test         # unit tests + content validation
npm run build    # type-check + production build in client/dist
npm run check:rust   # compile lesson snippets, verify excerpts, test the example (needs Rust 1.85+)
```

## What's inside

Three parts, like [go-tour-course](https://github.com/FeliceGeracitano/go-tour-course):
**Lessons** (the Book), **Patterns & use cases** (problem-first idiomatic Rust, including
how to structure a large codebase), and **Toolchain** (the `cargo` command).

- 87 Book lessons across all 21 chapters plus the appendix, each with a discovery
  question and explanatory feedback.
- Pattern lessons backed by compile-checked snippets and a runnable modular-monolith
  example, `examples/large-service/`, a ten-crate Cargo workspace with tests.
- Five step-through traces (moves, borrowing, `?`, iterators, `Rc`) and interactive
  visualizations for ownership, collections, smart pointers, concurrency, and async.
  Traces illustrate authored examples; nothing is executed.
- Landing page, collapsible chapter sidebar with progress rings, previous/next
  navigation, and shareable lesson links such as `/#ch04_ownership/what_is_ownership`.
- Progress (answers, completion, last lesson) lives in the browser's `localStorage`;
  nothing is sent anywhere.

## Optional terminal practice

The Cargo exercises and solutions from the original course remain in `chapters/`:

```bash
node scripts/prepare-exercises.mjs   # copies .exercise.rs → src/lib.rs, never overwrites
cd chapters && cargo test -p ch01_getting_started
```

Exercises start with `todo!()` and fail until completed; each chapter has a `SOLUTION.md`.

## Layout

- `content/course.json` — course manifest (chapters → lessons)
- `content/**/*.md` — lessons; interactive blocks are fenced JSON `quiz` and `trace`
  blocks ([authoring guide](content/README.md))
- `client/` — Vite + React app
- `chapters/` — optional Rust exercises
- `examples/large-service/` — runnable architecture example for the large-codebase chapter
- `examples/snippets/` — pinned crate that `check:rust` compiles lesson snippets in
- `docs/refactor-plan.md` — why the runner was replaced by a static site


## Credits

Unofficial companion to [The Rust Programming Language](https://doc.rust-lang.org/book/)
(MIT OR Apache-2.0), not affiliated with the Rust project. Read the Book for the
authoritative text.
