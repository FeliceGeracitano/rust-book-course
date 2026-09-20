# Rust Book Course 🦀

Discover Rust through **87 interactive lessons** following
[The Rust Programming Language](https://doc.rust-lang.org/book/).
Read a short explanation, make a prediction, and explore why the answer works.
No Rust installation, Docker, backend, or account is needed to take the course.

## Run locally

Use Node.js 22.22.2+, 24.15+, or 26+:

```bash
cd client
npm ci
npm run dev
```

Open the URL printed by Vite. `npm run build` type-checks and produces `client/dist`;
`npm run preview` serves that production build locally.

## Learn by discovery

- All 21 Book chapters plus the appendix, with a question and explanatory feedback
  in every lesson. Answers can be retried.
- Five guided traces covering moves, borrowing, error propagation, iterators, and
  reference counting. These illustrate authored examples; they do not execute code.
- Interactive chapter visualizations for ownership, collections, smart pointers,
  concurrency, and async.
- Previous/next navigation, shareable lesson links, and a mobile chapter menu.
- Answers, attempts, lesson completion, and your last lesson saved in this browser.
  Completion is up to you; a quiz score does not prevent moving ahead. If browser
  storage is unavailable, progress lasts for the current page session.

The earlier browser editor and Rust runner have been removed. The course now uses
static assets, inspired by `go-tour-course`. See the
[refactor plan and tradeoffs](docs/refactor-plan.md).

## Optional terminal practice

The original Cargo exercises and solutions remain in `chapters/`. Install Rust
separately if you want to write and test real code. From the repository root:

```bash
node scripts/prepare-exercises.mjs
cd chapters
cargo test -p ch01_getting_started
```

The preparation script copies each committed `.exercise.rs` to `src/lib.rs` **only
when that working file is missing**. Your existing edits are preserved. Exercises
start with `todo!()` and are expected to fail until completed. Each chapter’s
`SOLUTION.md` offers help. These tests do not change browser progress.

## Verify changes

```bash
cd client
npm test
npm run build
```

Tests validate all lessons and discovery blocks, exercise navigation and interactions,
and cover progress recovery and the optional exercise initializer. CI runs these
same checks. No Rust compiler is required for building or testing the site.

## Deploy with AWS Amplify Hosting

Connect this repository and the desired branch to a static Amplify Hosting app.
Use the repository root as the build root so both `client/` and `content/` are available.
The committed [`amplify.yml`](amplify.yml) selects Node 22, runs `npm ci`, runs the tests
and type-checked build, and publishes `client/dist`. No backend environment variables
or AWS credentials are needed by the client. AWS account setup and connecting the
repository happen in the Amplify console; this PR does not create or deploy an app.

Lesson URLs use fragments, for example `/#ch04_ownership/what_is_ownership`, so direct
links and refreshes need no SPA rewrite. Follow the
[AWS buildspec reference](https://docs.aws.amazon.com/amplify/latest/userguide/yml-specification-syntax.html)
when adjusting the hosting build. Automatic Vercel deployments remain disabled.

## Layout

- `content/course.json` — chapter and lesson order.
- `content/**/*.md` — lesson prose and JSON `quiz` / `trace` fences.
- `client/` — React, TypeScript, Vite, Tailwind, Shiki, and visualizations.
- `chapters/` — optional Rust exercises and solutions.
- `scripts/prepare-exercises.mjs` — safe initializer for local exercise files.

See [content authoring](content/README.md) to add or revise a lesson.

## Credits & attribution

This unofficial companion follows the chapter structure and learning progression of
[The Rust Programming Language](https://doc.rust-lang.org/book/), written and maintained
by the Rust team and contributors. It is not affiliated with or endorsed by the Rust
project. Read the Book for the authoritative text.

The Book is licensed under MIT OR Apache-2.0; prose adapted here is used under those
terms. See the [Book repository](https://github.com/rust-lang/book).
