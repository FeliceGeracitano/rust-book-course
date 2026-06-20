# Chapter 20 — Solutions

Reference implementations for the advanced-features exercises.

```rust
use std::ops::Add;

// Exercise 1 — Unsafe Rust: a safe wrapper around raw pointers
pub fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // SAFETY: `mid <= len`, so both ranges stay within the allocation, and the
    // two resulting slices cover disjoint regions, so the `&mut` references
    // never alias.
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Exercise 2 — Advanced Traits: operator overloading with `Add`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Exercise 3 — Advanced Types: the newtype pattern
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meters(f64);

impl Meters {
    pub fn new(value: f64) -> Meters {
        Meters(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Add for Meters {
    type Output = Meters;

    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}

// Exercise 4 — Macros: a declarative `macro_rules!`
#[macro_export]
macro_rules! string_vec {
    ( $( $x:expr ),* $(,)? ) => {
        {
            let mut temp = Vec::new();
            $(
                temp.push($x.to_string());
            )*
            temp
        }
    };
}
```
