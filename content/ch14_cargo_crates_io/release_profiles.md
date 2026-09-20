# 14.1 Customizing Builds with Release Profiles

A **profile** is a named set of compiler options. Cargo ships two main ones:
`dev` (used by `cargo build`) optimizes for fast compiles and good debugging,
while `release` (used by `cargo build --release`) optimizes the resulting
binary for speed. The headline knob is `opt-level`, which ranges from `0` (no
optimization, fast to compile) to `3` (aggressive optimization). By default
`dev` and `test` use `opt-level = 0`; `release` and `bench` use `opt-level = 3`.

You override any default by adding a `[profile.*]` section to `Cargo.toml`. For
example, to optimize your dev builds a little without paying the full release
cost:

```toml
[profile.dev]
opt-level = 1

[profile.release]
opt-level = 3
```

Cargo merges your settings over its built-in defaults, so you only specify what
you want to change. Profiles let you trade compile time against runtime
performance per build kind, which is why a fast `dev` loop and a fast `release`
binary can coexist.

```quiz
{
  "id": "discovery",
  "question": "Which command uses the optimized release build profile?",
  "options": [
    "cargo build --release",
    "cargo build",
    "cargo new --release"
  ],
  "answer": 0,
  "explain": "cargo build uses the dev profile by default. --release selects the release profile, whose defaults favor optimized output over fast compilation."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Implement `opt_level` in `chapters/ch14_cargo_crates_io/src/lib.rs` so it maps
each profile name to its default optimization level, then run:

```bash
cargo test -p ch14_cargo_crates_io
```
