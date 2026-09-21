# Authoring lessons

`course.json` determines order: `parts` → `chapters` → `subchapters`. Each chapter/lesson
pair maps to `content/<chapter.id>/<subchapter.id>.md` (the part is not in the path).
Keep these IDs stable: links and saved progress use them. Lesson text is bundled by
Vite; there is no content API.

Chapter `number` and lesson `number` are optional; Book chapters have them, pattern and
toolchain chapters do not. A lesson may carry `problem` (one line: what problem it
solves, shown under the eyebrow) and `ref` (the authoritative source URL, shown after
the article).

Use ordinary Markdown, with `rust`, `bash`, `toml`, or `json` fences for highlighted
examples. Every lesson needs at least one `quiz` fence. Its body is JSON:

```quiz
{
  "id": "discovery",
  "question": "What does let mut allow?",
  "options": ["Reassigning the binding", "Skipping type checking"],
  "answer": 0,
  "explain": "mut permits reassignment while the binding's type stays fixed."
}
```

`answer` is a zero-based option index. `code` optionally provides a Rust snippet
shown before the choices. The quiz `id` must be unique within the lesson and remain
stable across prose edits. Change the ID if you change option ordering or meaning,
so old saved choices are not applied to a different question.

A `trace` fence contains `title`, `code`, and a non-empty `steps` array. Each step
has a one-based `line`, explanatory `note`, and a `state` object mapping names to
strings. Optional `output` is the complete output so far, not an incremental delta.
Every step describes the full state to display. Traces are authored walkthroughs,
not an interpreter. Use runnable, deterministic examples and verify their behavior.

Keep terminal exercises under `### Optional terminal practice`, after discovery
content. They must not require completing an exercise to navigate or finish a lesson.
The shared setup command is `node scripts/prepare-exercises.mjs` from the repo root.

From `client/`, run `npm test` and `npm run build` before submitting. Content tests
check full manifest coverage, unique IDs, schemas, answer indices, and trace line
bounds. Review explanations for language accuracy as well as structural validity.

## Rust snippets in Parts 2 and 3

`npm run check:rust` (from `client/`, needs Rust 1.85+) verifies every ```rust fence in
the *Patterns & use cases* and *Toolchain* parts. Book lessons are not checked. The
first line of a fence decides how it is treated:

| Fence | Treatment |
|---|---|
| contains `fn main(` | Compiled and run in `examples/snippets`. End with `// Output:` and `// ` lines to assert stdout. |
| contains `#[test]` / `#[tokio::test]`, no `main` | Compiled and run with `cargo test` in `examples/snippets`. |
| first line `// compile-fail: E0499` | Must fail to compile with that error code (dependency-free, `rustc` only). |
| first line `// Source: examples/large-service/<path>` | Must appear verbatim in that file (trailing whitespace ignored). |
| first line `// no-check` | Skipped on purpose. Use sparingly. |
| anything else | A fragment; not checked. |

Snippets may use only the dependencies pinned in `examples/snippets/Cargo.toml`; add a
crate there first if a lesson needs it. The checker also runs `cargo test`, `clippy`,
and `fmt --check` on `examples/large-service`.
