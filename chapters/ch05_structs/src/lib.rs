//! Chapter 5 — Using Structs to Structure Related Data
//!
//! Complete each item below by replacing the `todo!()` bodies with real
//! implementations, then run `cargo test -p ch05_structs`.
//!
//! The exercises build up from defining and instantiating structs (5.1), to a
//! small program that uses them (5.2), to adding behavior with methods and
//! associated functions (5.3).

/// 5.1 — A rectangle described by its `width` and `height` (in pixels).
///
/// `Clone`, `Copy`, and `PartialEq` are derived so the type is easy to use in
/// tests and so struct update syntax can copy fields out of an existing value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

/// 5.1 — Build a [`Rectangle`] using *field init shorthand*.
///
/// When a function parameter has the same name as a struct field, you can
/// write just the field name instead of `field: value`.
///
/// ```
/// use ch05_structs::build_rectangle;
/// let r = build_rectangle(30, 50);
/// assert_eq!(r.width, 30);
/// assert_eq!(r.height, 50);
/// ```
pub fn build_rectangle(width: u32, height: u32) -> Rectangle {
    // TODO: build a `Rectangle` from `width` and `height` using field init
    // shorthand (write the field name once, not `width: width`).
    todo!("construct a Rectangle using field init shorthand")
}

/// 5.1 — A registered user of some service.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
    pub active: bool,
    pub sign_in_count: u64,
}

/// 5.1 — Produce a new [`User`] with a different `email`, reusing every other
/// field from `base` via *struct update syntax* (`..base`).
///
/// ```
/// use ch05_structs::{User, with_email};
/// let base = User {
///     username: String::from("ferris"),
///     email: String::from("old@example.com"),
///     active: true,
///     sign_in_count: 3,
/// };
/// let updated = with_email(base, String::from("new@example.com"));
/// assert_eq!(updated.email, "new@example.com");
/// assert_eq!(updated.username, "ferris");
/// assert_eq!(updated.sign_in_count, 3);
/// ```
pub fn with_email(base: User, email: String) -> User {
    // TODO: return a new User that uses the given `email` and copies every
    // other field from `base` with struct update syntax (`..base`).
    todo!("build a User using struct update syntax")
}

/// 5.2 — Compute the area of a rectangle, taking it by reference so the caller
/// keeps ownership.
///
/// ```
/// use ch05_structs::{build_rectangle, area};
/// let r = build_rectangle(30, 50);
/// assert_eq!(area(&r), 1500);
/// ```
pub fn area(rectangle: &Rectangle) -> u32 {
    // TODO: return the rectangle's area (width times height).
    todo!("multiply width by height")
}

impl Rectangle {
    /// 5.3 — The same area calculation as a *method*. `&self` borrows the
    /// rectangle the method is called on.
    ///
    /// ```
    /// use ch05_structs::build_rectangle;
    /// let r = build_rectangle(30, 50);
    /// assert_eq!(r.area(), 1500);
    /// ```
    pub fn area(&self) -> u32 {
        // TODO: return `self.width * self.height`.
        todo!("multiply self.width by self.height")
    }

    /// 5.3 — Return `true` when `self` can completely contain `other`: it must
    /// be at least as wide *and* at least as tall.
    ///
    /// ```
    /// use ch05_structs::build_rectangle;
    /// let big = build_rectangle(30, 50);
    /// let small = build_rectangle(10, 40);
    /// assert!(big.can_hold(&small));
    /// assert!(!small.can_hold(&big));
    /// ```
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        // TODO: true only when self is at least as wide AND at least as tall
        // as `other`.
        todo!("compare both width and height")
    }

    /// 5.3 — An *associated function* (no `self`) that builds a square
    /// [`Rectangle`]. Call it with `Rectangle::square(size)`.
    ///
    /// ```
    /// use ch05_structs::Rectangle;
    /// let sq = Rectangle::square(20);
    /// assert_eq!(sq.width, sq.height);
    /// assert_eq!(sq.area(), 400);
    /// ```
    pub fn square(size: u32) -> Rectangle {
        // TODO: build a Rectangle whose width and height both equal `size`.
        todo!("construct a square Rectangle")
    }
}
