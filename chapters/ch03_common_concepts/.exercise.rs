//! Chapter 3 — Common Programming Concepts
//!
//! These exercises drill the universal building blocks of Rust: variables and
//! shadowing, scalar and compound types, functions with return values, and
//! control flow with `if`, `loop`, `while`, and `for`.
//!
//! Complete each `todo!()` in this file, then run:
//!
//! ```text
//! cargo test -p ch03_common_concepts
//! ```

/// The number of seconds in one hour, expressed as a `const`.
///
/// `const` values are compile-time constants: their type is required and they
/// can be used anywhere, including at item scope. Unlike `let`, a `const` can
/// never be made mutable.
pub const SECONDS_PER_HOUR: u32 = 60 * 60;

/// Demonstrates **shadowing**: reusing a name with a new `let` binding.
///
/// Given a trimmed-looking string slice such as `"  42  "`, parse the numeric
/// content and return its value squared. Implement this by *shadowing*:
///
/// 1. bind `input` to its trimmed `&str`,
/// 2. shadow `input` with the parsed `i64` value,
/// 3. shadow it once more with the squared result, then return it.
///
/// Shadowing lets the same name change type (`&str` -> `i64`) without `mut`.
///
/// Returns `None` if the trimmed text is not a valid integer.
///
/// # Examples
///
/// ```
/// use ch03_common_concepts::parse_and_square;
/// assert_eq!(parse_and_square("  9 "), Some(81));
/// assert_eq!(parse_and_square("abc"), None);
/// ```
pub fn parse_and_square(input: &str) -> Option<i64> {
    // TODO: trim `input`, then shadow it with the parsed `i64` (use `.parse()`
    // and `.ok()?` to return None on failure), then shadow again with the
    // squared value and return it wrapped in `Some`.
    todo!("shadow `input` from &str to i64 to its square")
}

/// Converts a temperature in Fahrenheit to Celsius using floating-point math.
///
/// Uses the formula `C = (F - 32) * 5 / 9`. The result is a `f64`, exercising
/// scalar float types and a pure arithmetic function with an explicit return
/// type.
///
/// # Examples
///
/// ```
/// use ch03_common_concepts::fahrenheit_to_celsius;
/// assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
/// assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
/// ```
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    // TODO: apply the formula C = (F - 32) * 5 / 9 using f64 literals.
    todo!("convert Fahrenheit to Celsius")
}

/// Returns the `(min, max)` of an array of five integers as a **tuple**.
///
/// This exercises a fixed-size compound type (`[i32; 5]`), tuple return values,
/// and a `for` loop over the array's elements. The array always has five
/// elements, so a result always exists.
///
/// # Examples
///
/// ```
/// use ch03_common_concepts::min_and_max;
/// assert_eq!(min_and_max([3, 7, 1, 9, 4]), (1, 9));
/// ```
pub fn min_and_max(values: [i32; 5]) -> (i32, i32) {
    // TODO: seed `min` and `max` from `values[0]` (use `let mut`), loop over the
    // array with `for`, update each as needed, then return the `(min, max)` tuple.
    todo!("find the min and max and return them as a tuple")
}

/// Computes the `n`-th Fibonacci number using a `while` loop (no recursion).
///
/// `fib(0) == 0`, `fib(1) == 1`, and each later value is the sum of the two
/// before it. This exercises `let mut`, reassignment, a `while` loop, and a
/// `u64` accumulator.
///
/// # Examples
///
/// ```
/// use ch03_common_concepts::fib;
/// assert_eq!(fib(0), 0);
/// assert_eq!(fib(10), 55);
/// ```
pub fn fib(n: u32) -> u64 {
    // TODO: keep two `u64` accumulators (start at 0 and 1). Use a `while` loop
    // that runs `n` times, advancing the pair each iteration, then return the
    // value that holds fib(n).
    todo!("compute the n-th Fibonacci number with a while loop")
}

/// Classifies a number using `if`/`else if`/`else` returning a `&'static str`.
///
/// Returns:
/// - `"negative"` when `n < 0`,
/// - `"zero"` when `n == 0`,
/// - `"small"` when `1 <= n <= 9`,
/// - `"large"` otherwise.
///
/// This exercises `if` as an expression and multi-armed branching.
///
/// # Examples
///
/// ```
/// use ch03_common_concepts::classify;
/// assert_eq!(classify(0), "zero");
/// assert_eq!(classify(5), "small");
/// assert_eq!(classify(42), "large");
/// ```
pub fn classify(n: i32) -> &'static str {
    // TODO: use an if / else if / else chain to return one of "negative",
    // "zero", "small" (1..=9), or "large".
    todo!("classify `n` into one of the four categories")
}
