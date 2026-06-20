# Chapter 11 — Solutions

Reference implementations for `src/lib.rs`. Try the exercises first; peek only
when stuck. Then run `cargo test -p ch11_testing`.

```rust
/// A rectangle measured in whole units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Returns `true` if `self` can completely contain `other`.
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

/// Adds two to `value`.
pub fn add_two(value: i32) -> i32 {
    value + 2
}

/// Builds a personalized greeting containing `name`.
pub fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

/// A guessed number guaranteed to be between 1 and 100 (inclusive).
#[derive(Debug, PartialEq)]
pub struct Guess {
    value: i32,
}

impl Guess {
    /// Creates a `Guess`, panicking if `value` is outside `1..=100`.
    ///
    /// # Panics
    ///
    /// Panics when `value < 1` or `value > 100`.
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }

        Guess { value }
    }

    /// Returns the validated inner value.
    pub fn value(&self) -> i32 {
        self.value
    }
}
```
