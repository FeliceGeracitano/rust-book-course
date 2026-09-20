# Static discovery course

## Problem and decision

The previous UI needed a Rust HTTP server just to load its chapters and lessons.
Checking exercises additionally depended on the local compiler, editable files,
a Monaco editor, and a rust-analyzer WebSocket sidecar. The baseline local Vite
session returned HTTP 502 for both `/api/chapters` and `/api/check/ch01_getting_started`
when that backend was absent. This reproduces the infrastructure dependency, not
a diagnosis of the reported hosted runner outage.

Follow `go-tour-course`'s learning model: read an explanation, predict what happens,
inspect feedback, and explore an illustrated execution. All learning interactions
run in the browser against authored content. The site does not execute arbitrary Rust.

## Implementation sequence

1. Bundle the existing manifest and all 87 Markdown lessons using Vite. Use stable
   chapter/lesson fragment links, previous/next navigation, and a responsive sidebar.
2. Replace runner exercises in the primary learning flow with authored questions
   in every lesson and five step-through traces. Preserve the five chapter
   visualizations. Keep terminal exercises as optional practice.
3. Store answers, attempts, completion, and the last lesson in versioned localStorage.
   Completion is learner-controlled and separate from answering a question correctly.
4. Delete the API client, editor, LSP, Rust server, and Docker runtime. Supply an
   explicit non-overwriting initializer for optional terminal exercises.
5. Validate manifest coverage, quiz answers, trace lines, navigation, state isolation,
   storage failure behavior, and production output. Add CI and an Amplify buildspec.

## Acceptance criteria

- The complete course builds and runs without Rust, Docker, a backend, or credentials.
- Every listed lesson loads and includes a topic-specific discovery question.
- Feedback, retries, walkthrough controls, saved progress, and lesson links work.
- Invalid links and failed lesson loads offer a recovery path.
- A fresh clone can initialize optional exercises without overwriting existing work.
- Amplify installs and verifies the client, then publishes `client/dist`.

## Scope and tradeoffs

Keep the Book's 21 chapters plus appendix and existing Rust source exercises.
Do not create another execution backend or automatically deploy an AWS application.
Authored traces illustrate one specific example; they are not a Rust interpreter.
Progress stays on one browser/device and old server-only chapter ticks cannot be
recovered. Fragment navigation makes deep links work on plain static hosting without
an SPA rewrite. Vercel automatic deployment remains disabled.
