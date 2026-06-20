//! Chapter 19 — Patterns and Matching
//!
//! Complete each exercise below by replacing the `todo!()` bodies, then run
//! `cargo test -p ch19_patterns_matching`.
//!
//! These exercises drill the heart of Rust's pattern syntax — **match guards**,
//! **ranges**, **destructuring** (structs, tuples, enums), and **`@` bindings**.
//! Everything is pure: no I/O, no randomness, just patterns over plain data.

/// Classify a single character into a small set of buckets, using `match`
/// **ranges** plus an `_` catch-all.
///
/// Ranges in patterns (`'a'..='z'`) match any value in the inclusive interval,
/// which is far tidier than chaining `|` over every element. Order matters:
/// arms are tried top to bottom, and `match` must stay exhaustive, so the final
/// `_` arm covers "everything else".
///
/// Buckets:
/// - `'0'..='9'` -> `"digit"`
/// - `'a'..='z'` or `'A'..='Z'` -> `"letter"`
/// - `' '`, `'\t'`, `'\n'` -> `"space"`
/// - anything else -> `"other"`
///
/// ```
/// use ch19_patterns_matching::classify_char;
/// assert_eq!(classify_char('7'), "digit");
/// assert_eq!(classify_char('Q'), "letter");
/// assert_eq!(classify_char('\n'), "space");
/// assert_eq!(classify_char('%'), "other");
/// ```
pub fn classify_char(c: char) -> &'static str {
    // TODO: `match` on `c` using inclusive ranges. Map `'0'..='9'` to "digit",
    // `'a'..='z' | 'A'..='Z'` to "letter", the space/tab/newline chars to
    // "space", and a final `_` arm to "other".
    todo!("classify the char with range patterns")
}

/// A point in 2D space, used to practice **struct destructuring**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Describe where a [`Point`] sits relative to the axes, by **destructuring the
/// struct** in `match` arms.
///
/// `Point { x, y }` pulls the fields into bindings; `Point { x: 0, y: 0 }` uses
/// literals in the pattern to match an exact value. Later arms can match "on one
/// axis" by fixing one field to `0` and binding the other.
///
/// - `(0, 0)` -> `"origin"`
/// - on the x-axis (`y == 0`, `x != 0`) -> `"on the x axis"`
/// - on the y-axis (`x == 0`, `y != 0`) -> `"on the y axis"`
/// - anywhere else -> `"off both axes"`
///
/// ```
/// use ch19_patterns_matching::{Point, describe_point};
/// assert_eq!(describe_point(Point { x: 0, y: 0 }), "origin");
/// assert_eq!(describe_point(Point { x: 4, y: 0 }), "on the x axis");
/// assert_eq!(describe_point(Point { x: 0, y: -2 }), "on the y axis");
/// assert_eq!(describe_point(Point { x: 3, y: 5 }), "off both axes");
/// ```
pub fn describe_point(p: Point) -> &'static str {
    // TODO: `match` on `p`, destructuring the struct. Use literal `0`s to pin a
    // field: `Point { x: 0, y: 0 }` -> "origin", `Point { x: _, y: 0 }` ->
    // "on the x axis", `Point { x: 0, y: _ }` -> "on the y axis", and a final
    // `Point { .. }` -> "off both axes".
    todo!("describe the point by destructuring its fields")
}

/// A message an app might process, used to practice **enum + tuple
/// destructuring** together with **match guards**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// Quit the app.
    Quit,
    /// Move the cursor to `(x, y)`.
    Move { x: i32, y: i32 },
    /// Write some text.
    Write(String),
    /// Change the color to an `(r, g, b)` triple.
    ChangeColor(i32, i32, i32),
}

/// Summarize a [`Message`] into a human-readable string, combining
/// **destructuring** with **match guards** (the `if` after a pattern).
///
/// A guard is an extra boolean condition that must also hold for the arm to fire;
/// it lets one pattern split into cases the pattern alone cannot express. Note
/// that two arms here share the `Move` pattern, separated only by their guards —
/// the first matching arm (top to bottom) wins.
///
/// - `Quit` -> `"quit"`
/// - `Move { x, y }` where `x == y` -> `"move diagonally to {x}"`
/// - `Move { x, y }` (otherwise) -> `"move to ({x}, {y})"`
/// - `Write(text)` -> `"write: {text}"`
/// - `ChangeColor(r, g, b)` where every channel is in `0..=255` -> `"color #{r:02x}{g:02x}{b:02x}"`
/// - `ChangeColor(..)` with any out-of-range channel -> `"invalid color"`
///
/// ```
/// use ch19_patterns_matching::{Message, summarize};
/// assert_eq!(summarize(Message::Quit), "quit");
/// assert_eq!(summarize(Message::Move { x: 3, y: 3 }), "move diagonally to 3");
/// assert_eq!(summarize(Message::Move { x: 1, y: 4 }), "move to (1, 4)");
/// assert_eq!(summarize(Message::Write(String::from("hi"))), "write: hi");
/// assert_eq!(summarize(Message::ChangeColor(255, 0, 128)), "color #ff0080");
/// assert_eq!(summarize(Message::ChangeColor(300, 0, 0)), "invalid color");
/// ```
pub fn summarize(msg: Message) -> String {
    // TODO: `match` on `msg`. Handle `Quit` and `Write(text)` directly. For
    // `Move { x, y }`, add a guard `if x == y` for the diagonal case before the
    // general arm. For `ChangeColor(r, g, b)`, add a guard checking every channel
    // is in `0..=255` (use `(0..=255).contains(&r)` etc.) and format the hex
    // string with `{r:02x}`; a trailing `ChangeColor(..)` arm returns
    // "invalid color". Remember: top-to-bottom, the first matching arm wins.
    todo!("summarize the message using destructuring and match guards")
}

/// Validate a player's score and, when valid, report it *and keep the value*
/// using an **`@` binding**.
///
/// The `@` operator lets one pattern both **test** a value against a range and
/// **bind** it to a name in the same breath: `n @ 1..=100` matches any score in
/// `1..=100` while making the matched number available as `n`. Without `@` you
/// could test the range or bind the value, but not both at once.
///
/// - a score `n` in `90..=100` -> `Some("grade A (n)")`
/// - a score `n` in `1..=89` -> `Some("grade B (n)")`
/// - anything else (`<= 0` or `> 100`) -> `None`
///
/// ```
/// use ch19_patterns_matching::grade_score;
/// assert_eq!(grade_score(95), Some(String::from("grade A (95)")));
/// assert_eq!(grade_score(42), Some(String::from("grade B (42)")));
/// assert_eq!(grade_score(0), None);
/// assert_eq!(grade_score(200), None);
/// ```
pub fn grade_score(score: i32) -> Option<String> {
    // TODO: `match` on `score` using `@` bindings to test a range AND capture the
    // value: `n @ 90..=100` -> `Some(format!("grade A ({n})"))`, `n @ 1..=89` ->
    // `Some(format!("grade B ({n})"))`, and a final `_` -> `None`.
    todo!("grade the score using `@` range bindings")
}
