# Chapter 4 — Solutions

```rust
/// Ownership & moves: consume two owned `String`s and return a brand-new
/// `String` containing `a`, a space, then `b`.
pub fn combine(a: String, b: String) -> String {
    let mut out = a;
    out.push(' ');
    out.push_str(&b);
    out
}

/// A tiny mutable tally used to practice shared (`&`) and mutable (`&mut`) borrows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counter {
    value: u32,
}

impl Counter {
    /// Create a counter starting at `start`.
    pub fn new(start: u32) -> Self {
        Counter { value: start }
    }

    /// Read the current value through a *shared* reference (`&self`).
    pub fn value(&self) -> u32 {
        self.value
    }
}

/// Mutate a `Counter` in place through a *mutable* reference, adding `amount`.
pub fn count_up(counter: &mut Counter, amount: u32) {
    counter.value += amount;
}

/// The slice type: return the first whitespace-delimited word of `s` as a
/// string slice (`&str`) that borrows from the input.
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

/// The slice type over collections: return a *sub-slice* of `numbers` of length
/// `len`, starting at index `start`, clamped to the available elements.
pub fn window(numbers: &[i32], start: usize, len: usize) -> &[i32] {
    if start >= numbers.len() {
        return &numbers[numbers.len()..];
    }
    let end = start.saturating_add(len).min(numbers.len());
    &numbers[start..end]
}
```
