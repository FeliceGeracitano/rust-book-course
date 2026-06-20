# Chapter 3 — Solutions

```rust
pub const SECONDS_PER_HOUR: u32 = 60 * 60;

pub fn parse_and_square(input: &str) -> Option<i64> {
    let input = input.trim();
    let input: i64 = input.parse().ok()?;
    let input = input * input;
    Some(input)
}

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn min_and_max(values: [i32; 5]) -> (i32, i32) {
    let mut min = values[0];
    let mut max = values[0];
    for &value in &values {
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }
    (min, max)
}

pub fn fib(n: u32) -> u64 {
    let mut previous: u64 = 0;
    let mut current: u64 = 1;
    let mut count = 0;
    while count < n {
        let next = previous + current;
        previous = current;
        current = next;
        count += 1;
    }
    previous
}

pub fn classify(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else if n <= 9 {
        "small"
    } else {
        "large"
    }
}
```
