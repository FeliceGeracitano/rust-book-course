# 11.3 Test Organization

Rust recognizes two kinds of tests. **Unit tests** live next to the code they
check, inside the same file, in a module marked `#[cfg(test)]`. That attribute
means the module is compiled only during `cargo test`, so it never bloats your
release build. Because the module is a child of your crate, it can reach
*private* functions — letting you test internals directly.

```rust
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(internal_adder(2, 2), 4);
    }
}
```

**Integration tests** live in a separate `tests/` directory, each file compiled
as its own crate. They `use` your library exactly as an outside user would, so
they only touch the *public* API — a good check that your public surface is
usable. The tests for this chapter live in `tests/exercises.rs` for that reason.

Note: only library crates can be integration-tested. That's a common reason to
split a project's logic into a `lib.rs` with a thin `main.rs` on top.

```quiz
{
  "id": "discovery",
  "question": "Which API can tests in the tests/ directory normally access?",
  "options": [
    "Every private function in every module",
    "Only binary main functions",
    "The library’s public API"
  ],
  "answer": 2,
  "explain": "Integration tests are separate crates and exercise the library as an external consumer. Unit tests inside the source can test private implementation details."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Complete the public items in `chapters/ch11_testing/src/lib.rs` so the
integration tests in `tests/exercises.rs` pass:

```bash
cargo test -p ch11_testing
```
