# Chapter 9 — Error Handling

> Book: https://doc.rust-lang.org/book/ch09-00-error-handling.html

## Summary
Rust's two error kinds — unrecoverable (`panic!`) and recoverable (`Result`) —
and when to use each.

## You will learn
- `panic!` and when it fires
- `Result<T, E>` and the `?` operator
- Propagating errors
- Deciding between `panic!` and `Result`

## Subchapters
- 9.1 Unrecoverable Errors with `panic!`
- 9.2 Recoverable Errors with `Result`
- 9.3 To `panic!` or Not to `panic!`

## Exercises
Open `src/lib.rs`, complete each `todo!()`, then make the tests pass:

```bash
cargo test -p ch09_error_handling
```

Stuck? See `SOLUTION.md`.
