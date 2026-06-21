//! Chapter 10 — Generic Types, Traits, and Lifetimes
//!
//! Three focused exercises:
//!   1. Generics — `largest` and `Pair<T>`        (10.1)
//!   2. Traits   — the `Summary` trait + impls     (10.2)
//!   3. Lifetimes — `longest` and `Excerpt<'a>`    (10.3)
//!
//! Complete each `todo!()`, then run `cargo test -p ch10_generics_traits_lifetimes`.

// ---------------------------------------------------------------------------
// 10.1 Generic Data Types
// ---------------------------------------------------------------------------

/// Return the largest value in a slice.
///
/// This is generic over any `T` that can be compared (`PartialOrd`) and copied
/// out of the slice (`Copy`). Returns `None` for an empty slice so the caller
/// never has to guess at a sentinel value.
///
/// ```
/// use ch10_generics_traits_lifetimes::largest;
/// assert_eq!(largest(&[3, 7, 2, 9, 4]), Some(9));
/// assert_eq!(largest::<i32>(&[]), None);
/// ```
pub fn largest<T: PartialOrd + Copy>(items: &[T]) -> Option<T> {
    // TODO: return None if `items` is empty, otherwise walk the slice and
    // keep track of the largest value seen so far, then return Some(best).
    todo!("find the largest element, or None when the slice is empty")
}

/// A pair of two values of the same generic type `T`.
///
/// Used to practice generic structs and methods that work for any `T`.
pub struct Pair<T> {
    /// The first element.
    pub first: T,
    /// The second element.
    pub second: T,
}

impl<T: PartialOrd + Copy> Pair<T> {
    /// Build a new pair from two values.
    pub fn new(first: T, second: T) -> Self {
        // TODO: construct a Pair from the two arguments.
        todo!("build a Pair from `first` and `second`")
    }

    /// Return the larger of the two elements.
    ///
    /// ```
    /// use ch10_generics_traits_lifetimes::Pair;
    /// assert_eq!(Pair::new(3, 8).larger(), 8);
    /// assert_eq!(Pair::new(8, 3).larger(), 8);
    /// ```
    pub fn larger(&self) -> T {
        // TODO: compare self.first and self.second; return whichever is larger.
        todo!("return the larger of the two fields")
    }
}

// ---------------------------------------------------------------------------
// 10.2 Defining Shared Behavior with Traits
// ---------------------------------------------------------------------------

/// Shared behavior for things that can be summarized into a short line.
///
/// `summarize_author` is required, but `summarize` has a default implementation
/// built on top of it — implementors get `summarize` for free.
pub trait Summary {
    /// Return a label for whoever authored this item, e.g. `"@alice"`.
    fn summarize_author(&self) -> String;

    /// Return a one-line summary. Defaults to `"(Read more from <author>...)"`.
    fn summarize(&self) -> String {
        // TODO: provide a DEFAULT body, e.g. "(Read more from <author>...)"
        // using self.summarize_author().
        todo!("default summary built from summarize_author()")
    }
}

/// A short social-media style post.
pub struct Tweet {
    /// The handle of the user who posted, without the leading `@`.
    pub username: String,
    /// The body text of the tweet.
    pub content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        // TODO: return the username prefixed with '@'.
        todo!("return \"@<username>\"")
    }

    /// Override the default to show the actual content.
    fn summarize(&self) -> String {
        // TODO: return "<author>: <content>" (override the trait default).
        todo!("return \"<author>: <content>\"")
    }
}

/// A news article. Relies on the *default* `summarize` implementation.
pub struct Article {
    /// The article headline.
    pub headline: String,
    /// The byline / author name.
    pub author: String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        // TODO: return the author's name. Do NOT implement `summarize` here —
        // Article should inherit the trait's default `summarize`.
        todo!("return the author's name")
    }
}

/// Take anything that implements [`Summary`] and notify about it.
///
/// Demonstrates a trait bound used as a function parameter (`impl Trait`).
///
/// ```
/// use ch10_generics_traits_lifetimes::{notify, Article};
/// let a = Article { headline: "Rust 2024".into(), author: "Jo".into() };
/// assert_eq!(notify(&a), "Breaking! (Read more from Jo...)");
/// ```
pub fn notify(item: &impl Summary) -> String {
    // TODO: return "Breaking! <item.summarize()>".
    todo!("prefix the item's summary with \"Breaking! \"")
}

// ---------------------------------------------------------------------------
// 10.3 Validating References with Lifetimes
// ---------------------------------------------------------------------------

/// Return whichever of the two string slices is longer.
///
/// The lifetime `'a` ties the returned reference to *both* inputs, so the
/// compiler can prove the result never outlives the data it points to.
///
/// ```
/// use ch10_generics_traits_lifetimes::longest;
/// assert_eq!(longest("apple", "fig"), "apple");
/// assert_eq!(longest("hi", "world"), "world");
/// ```
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // TODO: return `x` when it is at least as long as `y`, otherwise `y`.
    todo!("return the longer of the two slices (prefer x on a tie)")
}

/// Holds a borrowed slice of some larger string — it cannot outlive that string.
pub struct Excerpt<'a> {
    /// The borrowed text fragment.
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    /// Build an excerpt from the first sentence of `text`.
    ///
    /// A "sentence" is everything up to (and excluding) the first `'.'`, or the
    /// whole string if there is no period.
    ///
    /// ```
    /// use ch10_generics_traits_lifetimes::Excerpt;
    /// let novel = String::from("Call me Ishmael. Some years ago...");
    /// let e = Excerpt::first_sentence(&novel);
    /// assert_eq!(e.part, "Call me Ishmael");
    /// ```
    pub fn first_sentence(text: &'a str) -> Excerpt<'a> {
        // TODO: find the first '.'; take everything before it (or all of `text`
        // when there is no '.'), and store that slice in an Excerpt.
        todo!("build an Excerpt from the text up to the first period")
    }

    /// Return how many bytes long the excerpt is.
    pub fn len(&self) -> usize {
        // TODO: return the byte length of self.part.
        todo!("return the length of the stored slice")
    }
}
