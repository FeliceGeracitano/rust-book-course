//! Chapter 4 — Understanding Ownership
//!
//! Three focused exercises covering the chapter's core ideas:
//!
//! 1. `combine` — ownership & moves: take owned `String`s and return a new one.
//! 2. `Counter` + `count_up` — references & borrowing: read with `&`, mutate
//!    in place with `&mut`.
//! 3. `first_word` & `window` — the slice type: borrow part of data
//!    without copying it.
//!
//! Complete every `todo!()`, then run:
//!
//! ```text
//! cargo test -p ch04_ownership
//! ```

/// Ownership & moves: consume two owned `String`s and return a brand-new
/// `String` containing `a`, a space, then `b`.
///
/// Because the parameters are owned (not references), the caller's values are
/// *moved* into this function and can no longer be used afterwards. The newly
/// built `String` is moved back out to the caller.
///
/// ```
/// # use ch04_ownership::combine;
/// assert_eq!(combine(String::from("hello"), String::from("world")), "hello world");
/// ```
pub fn combine(a: String, b: String) -> String {
    // TODO: build and return a new owned String: `a`, then a space, then `b`.
    // Hint: take ownership of `a` into a `mut` binding, then `push`/`push_str`.
    todo!("join the two owned Strings with a space and return the result")
}

/// A tiny mutable tally used to practice shared (`&`) and mutable (`&mut`)
/// borrows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counter {
    // TODO: store the current count here (a `u32`). Keep it private.
    value: u32,
}

impl Counter {
    /// Create a counter starting at `start`.
    pub fn new(start: u32) -> Self {
        // TODO: construct a `Counter` whose value is `start`.
        todo!("build a Counter starting at `start`")
    }

    /// Read the current value through a *shared* reference (`&self`).
    ///
    /// A shared borrow lets many readers look at the data at once, and never
    /// lets them change it.
    pub fn value(&self) -> u32 {
        // TODO: return the stored value (no mutation allowed through `&self`).
        todo!("return the current value")
    }
}

/// Mutate a `Counter` in place through a *mutable* reference, adding `amount`
/// to its value.
///
/// Taking `&mut Counter` borrows the counter exclusively: while this borrow is
/// live, no other reference to the same counter may exist. The caller keeps
/// ownership and sees the change after the call returns.
///
/// ```
/// # use ch04_ownership::{Counter, count_up};
/// let mut c = Counter::new(5);
/// count_up(&mut c, 3);
/// assert_eq!(c.value(), 8);
/// ```
pub fn count_up(counter: &mut Counter, amount: u32) {
    // TODO: add `amount` to the counter's value through the `&mut` reference.
    todo!("increase the counter's value by `amount` in place")
}

/// The slice type: return the first whitespace-delimited word of `s` as a
/// string slice (`&str`) that borrows from the input.
///
/// No new `String` is allocated — the returned slice points into `s`. If `s`
/// has no spaces, the whole string is returned. An empty string yields an empty
/// slice.
///
/// ```
/// # use ch04_ownership::first_word;
/// assert_eq!(first_word("hello world"), "hello");
/// assert_eq!(first_word("solo"), "solo");
/// ```
pub fn first_word(s: &str) -> &str {
    // TODO: scan the bytes of `s` for the first space and return `&s[..i]`.
    // Hint: `s.as_bytes()` plus `.iter().enumerate()`. If no space is found,
    // return the whole `s`.
    todo!("return the first word as a &str slice borrowing from `s`")
}

/// The slice type over collections: return a *sub-slice* of `numbers` of length
/// `len`, starting at index `start`.
///
/// The returned `&[i32]` borrows from `numbers` without copying any elements.
/// If the requested range runs past the end of the slice, the slice is clamped
/// to the available elements instead of panicking.
///
/// ```
/// # use ch04_ownership::window;
/// let data = [10, 20, 30, 40, 50];
/// assert_eq!(window(&data, 1, 3), &[20, 30, 40]);
/// assert_eq!(window(&data, 4, 10), &[50]);
/// assert_eq!(window(&data, 9, 2), &[] as &[i32]);
/// ```
pub fn window(numbers: &[i32], start: usize, len: usize) -> &[i32] {
    // TODO: return `&numbers[start..end]` where `end` is clamped to the length.
    // Hint: if `start >= numbers.len()` return an empty slice; otherwise use
    // `start.saturating_add(len).min(numbers.len())` as the end index.
    todo!("return the clamped sub-slice borrowing from `numbers`")
}
