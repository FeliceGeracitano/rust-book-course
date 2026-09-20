# 9.3 To panic! or Not to panic!

So when should code panic, and when should it return a `Result`? The guiding
question is: *could a reasonable caller recover?* If yes, return `Result` and let
them decide. If the failure means a bug or a broken assumption that makes
continuing meaningless, panicking is appropriate.

Returning `Result` is the right default for library code and any operation that
depends on input you don't control — parsing, arithmetic that can overflow,
lookups that can miss. It keeps the failure visible in the type and hands control
to the caller:

```rust
#[derive(Debug, PartialEq)]
enum MathError { DivideByZero }

fn safe_divide(a: i64, b: i64) -> Result<i64, MathError> {
    if b == 0 {
        return Err(MathError::DivideByZero);
    }
    Ok(a / b)
}
```

Panicking is reasonable in examples, prototypes, and tests, or when a failed
invariant means the program is in a state it should never reach. `unwrap` and
`expect` are fine when you can *prove* a value is present; `expect` even lets you
document why. The aim is code that fails loudly on real bugs but stays recoverable
on ordinary, expected errors.

```quiz
{
  "id": "discovery",
  "question": "A configuration file supplied by a user is missing. What is usually the best API behavior?",
  "options": [
    "Always panic immediately",
    "Silently replace every setting with zero",
    "Return a Result so the caller can decide how to recover"
  ],
  "answer": 2,
  "explain": "Missing external input is an anticipated failure. A Result lets callers report it, choose a fallback, or ask for a different file."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Finish the `Result`-returning functions in
`chapters/ch09_error_handling/src/lib.rs` — none of the tested paths should ever
panic. Then run:

```bash
cargo test -p ch09_error_handling
```
