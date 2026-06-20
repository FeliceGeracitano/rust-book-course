# content/

- `course.json` — the table of contents (single source of truth). The server serves
  it verbatim at `GET /api/chapters`; the client renders the chapter tree from it.
- `<crate>/<subchapter_id>.md` — lesson prose for one subchapter, e.g.
  `ch01_getting_started/installation.md`. Served at `GET /api/lesson/<crate>/<subchapter_id>`.

Lesson markdown is plain CommonMark. Code fences use ` ```rust ` for Shiki
highlighting. Helpful terminal commands go in a ` ```bash ` block so the client can
render a copy button.
