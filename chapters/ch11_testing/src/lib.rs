//! Chapter 11 — Writing Automated Tests
//!
//! These exercises give you small, pure functions and types to *test*. Complete
//! each implementation in this file, then write/inspect the tests and run
//! `cargo test -p ch11_testing`.
//!
//! Everything here is std-only and deterministic — no I/O, no randomness — so the
//! tests assert exact, repeatable values.

/// A rectangle measured in whole units.
///
/// Used to practice the boolean-returning logic that `assert!` checks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Returns `true` if `self` can completely contain `other`.
    ///
    /// A rectangle can hold another when it is at least as wide *and* at least
    /// as tall. Equal dimensions still count as holding.
    ///
    /// ```
    /// use ch11_testing::Rectangle;
    /// let big = Rectangle { width: 8, height: 7 };
    /// let small = Rectangle { width: 5, height: 1 };
    /// assert!(big.can_hold(&small));
    /// assert!(!small.can_hold(&big));
    /// ```
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        // TODO: return true only when self is at least as wide AND as tall as other
        todo!("compare width and height of self against other")
    }
}

/// Adds two to `value`.
///
/// A deliberately tiny function: the point is to practice `assert_eq!` with an
/// exact expected result.
///
/// ```
/// use ch11_testing::add_two;
/// assert_eq!(add_two(2), 4);
/// ```
pub fn add_two(value: i32) -> i32 {
    // TODO: return value plus 2
    todo!("add 2 to value")
}

/// Builds a personalized greeting containing `name`.
///
/// The exact wording can change, but the result must always *contain* the name.
/// This is the classic case for `assert!` with a custom failure message.
///
/// ```
/// use ch11_testing::greeting;
/// assert!(greeting("Carol").contains("Carol"));
/// ```
pub fn greeting(name: &str) -> String {
    // TODO: build a String that contains `name` (e.g. "Hello, {name}!")
    todo!("format a greeting that contains name")
}

/// A guessed number that is guaranteed to be between 1 and 100 (inclusive).
///
/// Constructing one with an out-of-range value is a programmer error, so
/// [`Guess::new`] panics. That panic is what `#[should_panic]` tests verify.
#[derive(Debug, PartialEq)]
pub struct Guess {
    value: i32,
}

impl Guess {
    /// Creates a `Guess`, panicking if `value` is outside `1..=100`.
    ///
    /// The panic message names which bound was violated so tests can match on
    /// it with `#[should_panic(expected = "...")]`.
    ///
    /// # Panics
    ///
    /// Panics when `value < 1` or `value > 100`.
    pub fn new(value: i32) -> Guess {
        // TODO: panic with a message containing "greater than or equal to 1"
        //       when value < 1, and "less than or equal to 100" when value > 100;
        //       otherwise construct and return Guess { value }.
        todo!("validate the 1..=100 range, panicking with a descriptive message")
    }

    /// Returns the validated inner value.
    pub fn value(&self) -> i32 {
        // TODO: return the stored value
        todo!("return self.value")
    }
}
