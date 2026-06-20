# 14.2 Publishing a Crate to Crates.io

[crates.io](https://crates.io) is the community package registry. Before
publishing, add metadata to `Cargo.toml` (`description`, `license`, etc.) and
document your public API with `///` doc comments, which `cargo doc` renders into
HTML. You log in with `cargo login`, then run `cargo publish`.

The crucial idea is **versioning**. Every release carries a
[Semantic Version](https://semver.org/) `MAJOR.MINOR.PATCH`. You bump `patch`
for bug fixes, `minor` for backwards-compatible features, and `major` for
breaking changes. Published versions are permanent — you can never overwrite or
delete a version, only `cargo yank` it to stop *new* projects from selecting it.

Other crates depend on yours using these numbers. A requirement like
`my_crate = "1.2.0"` means "compatible with 1.2.0" — Cargo will accept `1.9.0`
but not `2.0.0`, because a major bump signals a breaking change.

```rust
/// Adds one to the given number.
///
/// # Examples
/// ```
/// assert_eq!(my_crate::add_one(41), 42);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

### Exercise
Implement `SemVer::parse`, its ordering, and `is_compatible_with` in
`chapters/ch14_cargo_crates_io/src/lib.rs`, then run:

```bash
cargo test -p ch14_cargo_crates_io
```
