# Chapter 8 — Solutions

```rust
use std::collections::HashMap;

/// 8.1 Vectors — build a new vector of running totals (prefix sums).
pub fn running_totals(numbers: &[i64]) -> Vec<i64> {
    let mut totals = Vec::with_capacity(numbers.len());
    let mut sum = 0;
    for &n in numbers {
        sum += n;
        totals.push(sum);
    }
    totals
}

/// 8.2 Strings — capitalize the first letter of every whitespace-separated word.
pub fn capitalize_words(text: &str) -> String {
    let mut result = String::new();
    for word in text.split_whitespace() {
        if !result.is_empty() {
            result.push(' ');
        }
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            // `to_uppercase` yields a (possibly multi-char) iterator.
            for upper in first.to_uppercase() {
                result.push(upper);
            }
            result.push_str(chars.as_str());
        }
    }
    result
}

/// 8.3 Hash Maps — count how many times each word appears.
pub fn word_count(text: &str) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}
```
