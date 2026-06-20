# Chapter 2 — Solutions

```rust
use std::cmp::Ordering;

pub fn parse_guess(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();
    let number: u32 = trimmed
        .parse()
        .map_err(|_| format!("`{trimmed}` is not a number"))?;
    if (1..=100).contains(&number) {
        Ok(number)
    } else {
        Err(format!("{number} is out of the range 1..=100"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    TooSmall,
    TooBig,
    Correct,
}

pub fn check_guess(guess: u32, secret: u32) -> Outcome {
    match guess.cmp(&secret) {
        Ordering::Less => Outcome::TooSmall,
        Ordering::Greater => Outcome::TooBig,
        Ordering::Equal => Outcome::Correct,
    }
}

pub fn play_round(secret: u32, inputs: &[&str]) -> Option<usize> {
    for (index, line) in inputs.iter().enumerate() {
        let guess = match parse_guess(line) {
            Ok(value) => value,
            Err(_) => continue,
        };
        if check_guess(guess, secret) == Outcome::Correct {
            return Some(index + 1);
        }
    }
    None
}
```
