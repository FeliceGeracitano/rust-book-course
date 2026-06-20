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

### Exercise
Implement `opt_level` in `chapters/ch14_cargo_crates_io/src/lib.rs` so it maps
each profile name to its default optimization level, then run:

```bash
cargo test -p ch14_cargo_crates_io
```
