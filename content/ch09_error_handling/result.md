# 9.2 Recoverable Errors with Result

Most failures aren't bugs — a file might be missing, a string might not be a
number. For these, Rust uses the `Result<T, E>` enum, which forces you to
acknowledge the failure path:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

You handle a `Result` by matching, or with helpers like `map`, `unwrap_or`, and
`ok_or`. But hand-matching at every call gets noisy, so Rust gives you the `?`
operator. Placed after an expression that returns a `Result`, `?` unwraps the
`Ok` value or *returns early* with the `Err` — propagating it to the caller:

```rust
use std::num::ParseIntError;

fn double(text: &str) -> Result<i32, ParseIntError> {
    let n: i32 = text.parse()?; // returns the Err if parsing fails
    Ok(n * 2)
}
```

You can define your own error type — often an `enum` of the failure modes — and
return `Result<T, MyError>`. Deriving `Debug` and `PartialEq` makes such errors
easy to test. The `?` operator works in any function whose return type can hold
the error, so propagating failures upward stays terse and explicit.

### Exercise

In `chapters/ch09_error_handling/src/lib.rs`, implement `safe_divide` (return a
custom `MathError`), `parse_and_double`, and `sum_parsed` — all using `Result`
and `?` instead of panicking. Then run:

```bash
cargo test -p ch09_error_handling
```
