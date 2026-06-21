//! Chapter 6 — Enums and Pattern Matching
//!
//! Complete each exercise below by replacing the `todo!()` bodies, then run
//! `cargo test -p ch06_enums_pattern_matching`.
//!
//! The exercises build on the Book's coin/`Option` examples but stay pure: no
//! I/O, no randomness, just enums, `match`, `if let`, and `let...else`.

/// A US state, used to tag a quarter with the state it was minted for.
///
/// This is a *fieldless* enum: each variant is just a name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsState {
    Alabama,
    Alaska,
    California,
}

/// A coin. The `Quarter` variant carries data: the [`UsState`] it came from.
///
/// Enums let a single type be "one of several" shapes, and a variant may hold
/// values of different types (here, `Quarter` holds a `UsState`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/// Return a coin's value in cents using a `match` expression.
///
/// Every variant must be handled — `match` is exhaustive. The `Quarter` arm
/// *binds* the inner `UsState` to a name (we ignore it here, but it is in
/// scope), showing how `match` destructures data out of a variant.
///
/// ```
/// use ch06_enums_pattern_matching::{Coin, UsState, value_in_cents};
/// assert_eq!(value_in_cents(Coin::Dime), 10);
/// assert_eq!(value_in_cents(Coin::Quarter(UsState::Alaska)), 25);
/// ```
pub fn value_in_cents(coin: Coin) -> u32 {
    // TODO: `match` on `coin` and return 1, 5, 10, or 25. Handle every variant;
    // the `Quarter` arm can ignore the inner state with `Coin::Quarter(_)`.
    todo!("match each Coin variant to its value in cents")
}

/// Add one to an `Option<i32>`, propagating absence.
///
/// `Some(n)` maps to `Some(n + 1)`; `None` stays `None`. This is the Book's
/// classic example of matching on `Option<T>` instead of using null.
///
/// ```
/// use ch06_enums_pattern_matching::plus_one;
/// assert_eq!(plus_one(Some(5)), Some(6));
/// assert_eq!(plus_one(None), None);
/// ```
pub fn plus_one(x: Option<i32>) -> Option<i32> {
    // TODO: `match` on `x`: map `Some(n)` to `Some(n + 1)` and `None` to `None`.
    todo!("add one to the inner value when present, else return None")
}

/// Describe a coin only when it is a state quarter, using `if let`.
///
/// `if let` is concise sugar for a `match` that cares about a single pattern.
/// Return `Some("Quarter from {state:?}")` for a `Quarter`, and `None` for any
/// other coin — the `else` branch handles "everything else" without listing it.
///
/// ```
/// use ch06_enums_pattern_matching::{Coin, UsState, describe_quarter};
/// assert_eq!(
///     describe_quarter(Coin::Quarter(UsState::California)),
///     Some(String::from("Quarter from California")),
/// );
/// assert_eq!(describe_quarter(Coin::Penny), None);
/// ```
pub fn describe_quarter(coin: Coin) -> Option<String> {
    // TODO: use `if let Coin::Quarter(state) = coin { ... } else { ... }`.
    // In the match branch return `Some(format!("Quarter from {state:?}"))`,
    // otherwise return `None`.
    todo!("return Some(description) only for a Quarter, else None")
}

/// A command to drive a cursor on a grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    /// Move by `(dx, dy)`.
    Move { dx: i32, dy: i32 },
    /// Jump to the origin `(0, 0)`.
    Home,
}

/// Apply a `Command` to a position, returning the new `(x, y)`.
///
/// Demonstrates matching on a struct-like variant and binding its named fields.
///
/// ```
/// use ch06_enums_pattern_matching::{Command, apply_command};
/// assert_eq!(apply_command((1, 1), Command::Move { dx: 2, dy: -3 }), (3, -2));
/// assert_eq!(apply_command((4, 9), Command::Home), (0, 0));
/// ```
pub fn apply_command(position: (i32, i32), command: Command) -> (i32, i32) {
    // TODO: `match` on `command`. For `Command::Move { dx, dy }` add the deltas
    // to `position.0` and `position.1`; for `Command::Home` return `(0, 0)`.
    todo!("compute the new position from the command")
}

/// Look up a username by id, falling back to `"guest"` when absent.
///
/// Uses `let...else`: bind the happy path in the `let`, and on `None` run the
/// `else` block, which **must diverge** (here, `return`). After the `let` the
/// `name` binding is available in the rest of the function — no rightward drift.
///
/// `users` is a slice of `(id, name)` pairs standing in for a lookup table.
///
/// ```
/// use ch06_enums_pattern_matching::username_or_guest;
/// let users = [(1, "alice"), (2, "bob")];
/// assert_eq!(username_or_guest(&users, 2), "bob");
/// assert_eq!(username_or_guest(&users, 99), "guest");
/// ```
pub fn username_or_guest<'a>(users: &[(u32, &'a str)], id: u32) -> &'a str {
    // TODO: use `let...else`. Bind the matching name from
    // `users.iter().find(...)` and `return "guest"` in the `else` block when no
    // entry has the given `id`. Then return the bound `name`.
    todo!("look up the name for `id`, falling back to \"guest\"")
}
