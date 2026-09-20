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

```trace
{
  "title": "Follow the early return through ?",
  "code": "fn parse_next(text: &str) -> Result<u32, std::num::ParseIntError> {\n    let value = text.parse::<u32>()?;\n    Ok(value + 1)\n}\nfn main() {\n    let result = parse_next(\"oops\");\n    println!(\"{}\", result.is_err());\n}",
  "steps": [
    {
      "line": 6,
      "note": "The caller passes text that cannot be parsed as a u32.",
      "state": {
        "text": "\"oops\""
      },
      "output": ""
    },
    {
      "line": 2,
      "note": "Parsing returns Err. The ? operator returns that error immediately, skipping Ok(value + 1).",
      "state": {
        "parse result": "Err(ParseIntError)",
        "value": "never initialized"
      },
      "output": ""
    },
    {
      "line": 6,
      "note": "The caller receives Err; there was no panic.",
      "state": {
        "result": "Err(ParseIntError)"
      },
      "output": ""
    },
    {
      "line": 7,
      "note": "is_err observes the error and returns true.",
      "state": {
        "result.is_err()": "true"
      },
      "output": "true\n"
    }
  ]
}
```

```quiz
{
  "id": "discovery",
  "question": "What does ? do when parse returns Err here?",
  "options": [
    "Return the error from read_number",
    "Turn the error into 0",
    "Retry parsing automatically"
  ],
  "answer": 0,
  "explain": "? unwraps Ok for continued work and propagates Err to the caller, converting the error when a suitable conversion exists.",
  "code": "fn read_number(s: &str) -> Result<u32, std::num::ParseIntError> {\n    let n = s.parse::<u32>()?;\n    Ok(n + 1)\n}"
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

In `chapters/ch09_error_handling/src/lib.rs`, implement `safe_divide` (return a
custom `MathError`), `parse_and_double`, and `sum_parsed` — all using `Result`
and `?` instead of panicking. Then run:

```bash
cargo test -p ch09_error_handling
```
