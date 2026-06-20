//! Chapter 9 — Error Handling
//!
//! These exercises practice Rust's *recoverable* error story: the
//! [`Result<T, E>`] type, custom error enums, and the `?` operator for
//! propagating errors instead of panicking.
//!
//! Complete each `todo!()`, then run `cargo test -p ch09_error_handling`.
//!
//! Note: the tested paths must never panic. Every fallible function returns a
//! `Result` and the caller decides what to do with the error.

use std::num::ParseIntError;

/// Errors that arithmetic in this module can produce.
///
/// Deriving `Debug` and `PartialEq` lets tests compare error values directly.
#[derive(Debug, PartialEq, Eq)]
pub enum MathError {
    /// The divisor was zero.
    DivideByZero,
    /// The operation overflowed the `i64` range.
    Overflow,
}

/// Divide `numerator` by `denominator` without ever panicking.
///
/// Returns:
/// - `Err(MathError::DivideByZero)` when `denominator` is `0`,
/// - `Err(MathError::Overflow)` when the division overflows
///   (only `i64::MIN / -1`),
/// - `Ok(quotient)` otherwise.
///
/// # Examples
///
/// ```
/// use ch09_error_handling::{safe_divide, MathError};
///
/// assert_eq!(safe_divide(10, 2), Ok(5));
/// assert_eq!(safe_divide(7, 0), Err(MathError::DivideByZero));
/// ```
pub fn safe_divide(numerator: i64, denominator: i64) -> Result<i64, MathError> {
    // TODO: return Err(MathError::DivideByZero) when denominator is 0, otherwise
    // use i64::checked_div and turn its `None` (overflow) into
    // Err(MathError::Overflow), e.g. with `.ok_or(...)`.
    todo!("divide without panicking, returning a MathError on bad input")
}

/// Parse `text` as an `i32` and double it, propagating any parse error.
///
/// Use the `?` operator so that an unparsable string short-circuits with the
/// underlying [`ParseIntError`] instead of panicking.
///
/// # Examples
///
/// ```
/// use ch09_error_handling::parse_and_double;
///
/// assert_eq!(parse_and_double("21").unwrap(), 42);
/// assert!(parse_and_double("oops").is_err());
/// ```
pub fn parse_and_double(text: &str) -> Result<i32, ParseIntError> {
    // TODO: parse `text` into an i32 using `?` to propagate the error, then
    // return Ok of the value multiplied by 2.
    todo!("parse with `?` and double the result")
}

/// Parse every entry in `inputs` as an `i32` and return their sum.
///
/// Iterate the slice and use `?` to propagate the first parse failure: as soon
/// as one entry fails to parse, return that error and stop. If every entry
/// parses, return `Ok` with the total.
///
/// # Examples
///
/// ```
/// use ch09_error_handling::sum_parsed;
///
/// assert_eq!(sum_parsed(&["1", "2", "3"]).unwrap(), 6);
/// assert!(sum_parsed(&["1", "nope", "3"]).is_err());
/// ```
pub fn sum_parsed(inputs: &[&str]) -> Result<i32, ParseIntError> {
    // TODO: loop over `inputs`, parse each `&str` into an i32 with `?`, add it
    // to a running total, then return Ok(total). An empty slice sums to 0.
    todo!("sum the parsed entries, propagating the first error with `?`")
}
