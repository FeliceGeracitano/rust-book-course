//! Chapter 20 — Advanced Features
//!
//! These exercises drill four of the chapter's "sharp tools": wrapping `unsafe`
//! raw-pointer code inside a safe API, operator overloading with the `Add`
//! trait, the newtype pattern for type-safe wrappers, and writing a small
//! declarative `macro_rules!`.
//!
//! Complete each `todo!()` in the items below, then run:
//!
//! ```text
//! cargo test -p ch20_advanced_features
//! ```

use std::ops::Add;

// ---------------------------------------------------------------------------
// Exercise 1 — Unsafe Rust: a safe wrapper around raw pointers
// ---------------------------------------------------------------------------

/// Splits a mutable slice into two non-overlapping mutable halves at `mid`.
///
/// You cannot write this with safe Rust alone: the borrow checker refuses two
/// `&mut` references into the same slice, even though the halves never overlap.
/// The trick from the Book is to drop to raw pointers inside a small `unsafe`
/// block, then hand back ordinary safe references. The function itself is safe
/// to call — the caller can never trigger undefined behaviour as long as we
/// uphold the invariant that the two ranges are disjoint.
///
/// # Panics
///
/// Panics if `mid` is greater than the slice length.
///
/// # Examples
///
/// ```
/// use ch20_advanced_features::split_at_mut;
///
/// let mut data = [1, 2, 3, 4, 5, 6];
/// let (left, right) = split_at_mut(&mut data, 3);
/// assert_eq!(left, &mut [1, 2, 3]);
/// assert_eq!(right, &mut [4, 5, 6]);
/// ```
pub fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // TODO: read `slice.len()` and `slice.as_mut_ptr()`, then `assert!(mid <= len)`.
    // In an `unsafe` block, return a tuple of two slices built with
    // `std::slice::from_raw_parts_mut`: the first covers `ptr` for `mid`
    // elements, the second covers `ptr.add(mid)` for `len - mid` elements.
    todo!("split the slice into two disjoint mutable halves using raw pointers")
}

// ---------------------------------------------------------------------------
// Exercise 2 — Advanced Traits: operator overloading with `Add`
// ---------------------------------------------------------------------------

/// A 2-D point that supports the `+` operator via the [`Add`] trait.
///
/// Implementing `Add` is operator overloading: it teaches `+` how to combine
/// two `Point` values by adding their fields component-wise.
///
/// # Examples
///
/// ```
/// use ch20_advanced_features::Point;
///
/// let sum = Point { x: 1, y: 2 } + Point { x: 3, y: 4 };
/// assert_eq!(sum, Point { x: 4, y: 6 });
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// The horizontal coordinate.
    pub x: i32,
    /// The vertical coordinate.
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    /// Adds two points component-wise.
    fn add(self, other: Point) -> Point {
        // TODO: return a `Point` whose `x` is `self.x + other.x` and whose
        // `y` is `self.y + other.y`.
        todo!("add the two points component-wise")
    }
}

// ---------------------------------------------------------------------------
// Exercise 3 — Advanced Types: the newtype pattern
// ---------------------------------------------------------------------------

/// A distance in metres, wrapping a plain `f64` in a *newtype*.
///
/// The newtype pattern gives a primitive a distinct type so the compiler can
/// stop you mixing, say, metres with seconds. It is a zero-cost tuple struct:
/// at runtime it is just the inner `f64`, but at compile time it is its own
/// type with its own methods and trait impls.
///
/// # Examples
///
/// ```
/// use ch20_advanced_features::Meters;
///
/// let total = Meters::new(2.0) + Meters::new(3.0);
/// assert_eq!(total.value(), 5.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meters(f64);

impl Meters {
    /// Wraps a raw `f64` count of metres.
    pub fn new(value: f64) -> Meters {
        // TODO: construct a `Meters` holding `value` in its single field.
        todo!("wrap `value` in a Meters")
    }

    /// Returns the underlying number of metres.
    pub fn value(&self) -> f64 {
        // TODO: return the inner `f64` (`self.0`).
        todo!("return the wrapped number of metres")
    }
}

impl Add for Meters {
    type Output = Meters;

    /// Adds two distances, keeping the `Meters` unit.
    fn add(self, other: Meters) -> Meters {
        // TODO: return a `Meters` wrapping `self.0 + other.0`.
        todo!("add the two distances and keep the Meters unit")
    }
}

// ---------------------------------------------------------------------------
// Exercise 4 — Macros: a declarative `macro_rules!`
// ---------------------------------------------------------------------------

/// Builds a `Vec<String>` from a comma-separated list of string-like values.
///
/// This is a declarative macro: it matches a *repetition* of expressions
/// (`$( $x:expr ),*`) and expands to code that pushes each one onto a fresh
/// vector after converting it with `.to_string()`. A trailing comma is allowed.
///
/// # Examples
///
/// ```
/// use ch20_advanced_features::string_vec;
///
/// let v = string_vec!["a", "b", "c"];
/// assert_eq!(v, vec![String::from("a"), String::from("b"), String::from("c")]);
/// ```
#[macro_export]
macro_rules! string_vec {
    // TODO: match a comma-separated repetition of expressions, allowing a
    // trailing comma: `( $( $x:expr ),* $(,)? )`. In the expansion, create a
    // `let mut temp = Vec::new();`, then repeat `temp.push($x.to_string());`
    // once per matched expression with `$( ... )*`, and finish with `temp`.
    //
    // The arm below keeps the crate compiling but always returns an empty
    // vector, so the tests fail until you build the real expansion.
    ( $( $x:expr ),* $(,)? ) => {
        {
            let temp: Vec<String> = Vec::new();
            $(
                let _ = &$x; // touch each input so type inference still works
            )*
            temp
        }
    };
}
