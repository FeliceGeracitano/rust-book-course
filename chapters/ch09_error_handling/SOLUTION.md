# Chapter 9 — Solutions

```rust
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum MathError {
    DivideByZero,
    Overflow,
}

pub fn safe_divide(numerator: i64, denominator: i64) -> Result<i64, MathError> {
    if denominator == 0 {
        return Err(MathError::DivideByZero);
    }
    numerator
        .checked_div(denominator)
        .ok_or(MathError::Overflow)
}

pub fn parse_and_double(text: &str) -> Result<i32, ParseIntError> {
    let value: i32 = text.parse()?;
    Ok(value * 2)
}

pub fn sum_parsed(inputs: &[&str]) -> Result<i32, ParseIntError> {
    let mut total = 0;
    for input in inputs {
        total += input.parse::<i32>()?;
    }
    Ok(total)
}
```

## Why it works

- **`safe_divide`** returns a `Result` with a custom `MathError` enum instead of
  panicking. We check `denominator == 0` up front, then use `checked_div` to turn
  the one overflow case (`i64::MIN / -1`) into `MathError::Overflow` via `ok_or`.
- **`parse_and_double`** uses the `?` operator: `text.parse()?` returns early with
  the `ParseIntError` if the string isn't a number, otherwise unwraps the value.
- **`sum_parsed`** loops and applies `?` to each entry, so the first bad entry
  short-circuits the whole function with its error.
