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

### Exercise

Complete the public items in `chapters/ch11_testing/src/lib.rs` so the
integration tests in `tests/exercises.rs` pass:

```bash
cargo test -p ch11_testing
```
