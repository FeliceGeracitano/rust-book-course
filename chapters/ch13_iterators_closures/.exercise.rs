//! Chapter 13 — Functional Language Features: Iterators and Closures
//!
//! Complete each exercise below by replacing the `todo!()` bodies, then run
//! `cargo test -p ch13_iterators_closures`.
//!
//! The exercises build from closures (13.1) up through iterator adaptors and
//! consumers like `map`, `filter`, `collect`, and `fold` (13.2). Exercise 4
//! mirrors the pure searching logic from the I/O project (13.3) using
//! iterators instead of an explicit loop.

/// A simple inventory of shirts in two colors.
///
/// This mirrors the Book's shirt-giveaway example: when a customer wins a
/// shirt, the store gives away whichever color it has the most of. We use a
/// *closure* as the default so the (possibly expensive) "most stocked" lookup
/// only runs when the customer has no preference.
#[derive(Debug, Clone)]
pub struct Inventory {
    /// Number of red shirts in stock.
    pub red: u32,
    /// Number of blue shirts in stock.
    pub blue: u32,
}

/// A shirt color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShirtColor {
    /// A red shirt.
    Red,
    /// A blue shirt.
    Blue,
}

impl Inventory {
    /// Decide which shirt to give a customer.
    ///
    /// If `user_preference` is `Some(color)`, give that color. Otherwise call
    /// `most_stocked()` to pick the color the store has the most of.
    ///
    /// `unwrap_or_else` takes a *closure* that is only invoked when the
    /// `Option` is `None`, so the fallback lookup is lazy.
    ///
    /// # Examples
    ///
    /// ```
    /// use ch13_iterators_closures::{Inventory, ShirtColor};
    /// let store = Inventory { red: 3, blue: 5 };
    /// assert_eq!(store.giveaway(Some(ShirtColor::Red)), ShirtColor::Red);
    /// assert_eq!(store.giveaway(None), ShirtColor::Blue);
    /// ```
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // TODO: return `user_preference` if it is `Some`, otherwise fall back
        // to `self.most_stocked()`. Use `Option::unwrap_or_else` with a closure
        // so `most_stocked()` is only called when there is no preference.
        todo!("use unwrap_or_else with a closure that calls self.most_stocked()")
    }

    /// Return whichever color this inventory has the most of.
    ///
    /// Ties favor `Red`.
    pub fn most_stocked(&self) -> ShirtColor {
        // TODO: compare `self.red` and `self.blue` and return the larger
        // color. On a tie, return `ShirtColor::Red`.
        todo!("return the color with the higher count (ties favor Red)")
    }
}

/// Apply a function to each element, returning a new `Vec` of results.
///
/// This is a hand-rolled version of [`Iterator::map`] + [`Iterator::collect`].
/// The closure `f` is a generic `FnMut(&T) -> U`, so callers can pass any
/// closure or function that turns a `&T` into a `U`.
///
/// # Examples
///
/// ```
/// use ch13_iterators_closures::map_collect;
/// let doubled = map_collect(&[1, 2, 3], |n| n * 2);
/// assert_eq!(doubled, vec![2, 4, 6]);
/// ```
pub fn map_collect<T, U, F>(items: &[T], mut f: F) -> Vec<U>
where
    F: FnMut(&T) -> U,
{
    // TODO: iterate over `items`, apply `f` to each element with `.map(...)`,
    // and `.collect()` the results into a `Vec<U>`.
    todo!("use items.iter().map(...).collect()")
}

/// Keep only the elements for which `predicate` returns `true`, cloning them
/// into a new `Vec`.
///
/// This combines [`Iterator::filter`], [`Iterator::cloned`], and
/// [`Iterator::collect`]. The `predicate` is an `FnMut(&T) -> bool`.
///
/// # Examples
///
/// ```
/// use ch13_iterators_closures::filter_collect;
/// let evens = filter_collect(&[1, 2, 3, 4], |n| n % 2 == 0);
/// assert_eq!(evens, vec![2, 4]);
/// ```
pub fn filter_collect<T, F>(items: &[T], mut predicate: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T) -> bool,
{
    // TODO: iterate over `items`, keep the ones where `predicate` is true with
    // `.filter(...)`, `.cloned()` them into owned values, then `.collect()`.
    todo!("use items.iter().filter(...).cloned().collect()")
}

/// Sum the lengths of every string in `words` using [`Iterator::fold`].
///
/// `fold` carries an accumulator (starting at `0`) across the iterator,
/// combining it with each item via the closure. Here each step adds the
/// current word's length to the running total.
///
/// # Examples
///
/// ```
/// use ch13_iterators_closures::total_length;
/// assert_eq!(total_length(&["ab", "cde"]), 5);
/// assert_eq!(total_length(&[]), 0);
/// ```
pub fn total_length(words: &[&str]) -> usize {
    // TODO: use `.fold(0, ...)` to accumulate the sum of each word's `.len()`.
    todo!("use words.iter().fold(0, |acc, word| ...)")
}

/// Case-insensitively find every line in `contents` that contains `query`.
///
/// This is the pure search logic from the Chapter 12 minigrep project,
/// rewritten in the iterator style introduced in Chapter 13: filter the lines,
/// then collect the matches. No file or terminal I/O is performed, which keeps
/// it easy to test.
///
/// # Examples
///
/// ```
/// use ch13_iterators_closures::search_insensitive;
/// let text = "Rust\nsafe, fast\nTrust me\nPick three.";
/// assert_eq!(search_insensitive("rUsT", text), vec!["Rust", "Trust me"]);
/// ```
pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // TODO: lowercase the query once, then iterate `contents.lines()`,
    // `.filter(...)` the lines whose lowercased form `.contains(&query)`, and
    // `.collect()` the matches into a `Vec<&str>`.
    todo!("filter contents.lines() by case-insensitive containment, then collect")
}
