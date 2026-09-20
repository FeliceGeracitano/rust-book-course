# 1.3 Hello, Cargo!

**Cargo** is Rust's build system and package manager. You'll use it for everything
from here on.

```bash
cargo new hello_cargo   # create a project
cd hello_cargo          # enter the project
cargo build             # compile
cargo run               # compile + run
cargo check             # type-check without producing a binary
cargo test              # run tests
```

```quiz
{
  "id": "discovery",
  "question": "Which command checks types without producing a runnable binary?",
  "options": [
    "cargo new",
    "cargo check",
    "cargo run"
  ],
  "answer": 1,
  "explain": "cargo check skips the final executable generation, making it useful for quick feedback. cargo run builds and executes; cargo new creates a project."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Make `build_tool()` return `"cargo"`:

```bash
cargo test -p ch01_getting_started
```

The tests give feedback on this optional terminal exercise.
