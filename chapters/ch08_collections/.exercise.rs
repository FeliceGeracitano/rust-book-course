//! Chapter 8 — Common Collections
//!
//! Three exercises, one per subchapter, covering the everyday heap-backed
//! collections from the standard library: [`Vec`], [`String`], and
//! [`std::collections::HashMap`]. Complete each `todo!()`, then run:
//!
//! ```bash
//! cargo test -p ch08_collections
//! ```

use std::collections::HashMap;

/// 8.1 Vectors — build a new vector of *running totals*.
///
/// Given a slice of integers, return a `Vec<i64>` whose element at index `i`
/// is the sum of all input elements from `0` through `i` (inclusive). This is
/// also called a prefix sum.
///
/// An empty input yields an empty vector.
///
/// # Examples
///
/// ```
/// use ch08_collections::running_totals;
///
/// assert_eq!(running_totals(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
/// assert_eq!(running_totals(&[]), Vec::<i64>::new());
/// ```
pub fn running_totals(numbers: &[i64]) -> Vec<i64> {
    // TODO: create a new `Vec<i64>`, keep a running `sum`, and for each input
    // value push the updated sum onto the vector. Return the vector.
    todo!("build a vector of prefix sums")
}

/// 8.2 Strings — capitalize the first letter of every whitespace-separated word.
///
/// Words are separated by runs of whitespace. For each word, the first
/// character is upper-cased and the rest are left untouched. Words are then
/// rejoined with a single ASCII space, regardless of the original spacing.
///
/// Because Rust strings are UTF-8, iterate over `char`s (not bytes) so that
/// multi-byte characters are handled correctly.
///
/// # Examples
///
/// ```
/// use ch08_collections::capitalize_words;
///
/// assert_eq!(capitalize_words("hello world"), "Hello World");
/// assert_eq!(capitalize_words("  rust   is fun "), "Rust Is Fun");
/// assert_eq!(capitalize_words(""), "");
/// ```
pub fn capitalize_words(text: &str) -> String {
    // TODO: split `text` with `split_whitespace`. For each word, take its first
    // `char`, upper-case it (`char::to_uppercase` returns an iterator), then
    // append the rest of the word unchanged. Join words with a single ' '.
    todo!("capitalize the first char of each word")
}

/// 8.3 Hash Maps — count how many times each word appears.
///
/// Split the input on whitespace and return a map from each word to the number
/// of times it occurs. Use the entry API so a missing key starts at `0` before
/// being incremented.
///
/// # Examples
///
/// ```
/// use ch08_collections::word_count;
///
/// let counts = word_count("the cat the dog the");
/// assert_eq!(counts.get("the"), Some(&3));
/// assert_eq!(counts.get("cat"), Some(&1));
/// assert_eq!(counts.get("fish"), None);
/// ```
pub fn word_count(text: &str) -> HashMap<String, u32> {
    // TODO: create a `HashMap<String, u32>`. For each whitespace-separated word,
    // use the entry API (`entry(...).or_insert(0)`) to get a mutable reference
    // to its count and increment it. Return the map.
    todo!("count occurrences of each word")
}
