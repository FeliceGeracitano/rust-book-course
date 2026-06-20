//! Chapter 7 — Packages, Crates, and Modules
//!
//! This crate is a single library that models a small restaurant, using a
//! *module tree* to control scope and privacy. Work through the exercises by
//! filling in each `todo!()` so that the tests pass:
//!
//! ```bash
//! cargo test -p ch07_packages_crates_modules
//! ```
//!
//! The pieces you will build:
//! - `front_of_house::hosting` — a nested module reached by absolute/relative paths.
//! - `back_of_house::Breakfast` — a struct mixing `pub` and private fields.
//! - `back_of_house::Appetizer` — a `pub` enum (every variant is public).
//! - `eat_at_restaurant` / `order_breakfast` / `describe` — callers that reach the
//!   items above through paths and `use`.

/// The "front of house": everything customers interact with.
///
/// A module groups related items and, by default, keeps them *private* to their
/// parent. Marking the module and its items `pub` is what lets code outside the
/// module (including the tests) reach them through a path.
pub mod front_of_house {
    /// Seating and waitlist management.
    pub mod hosting {
        /// Add a party of `party_size` to the waitlist and return its
        /// 1-based position. The first party added is at position `1`.
        ///
        /// (The real Book example reads/writes shared state; here we keep the
        /// logic pure so the behavior is deterministic and easy to test.)
        pub fn add_to_waitlist(current_len: usize) -> usize {
            // TODO: the new party joins at the end, so return the length plus one.
            todo!("return current_len + 1")
        }

        /// Seat the party at the front of the waitlist, returning the number
        /// of parties still waiting afterward. Seating an empty waitlist
        /// changes nothing and returns `0`.
        pub fn seat_at_table(waiting: usize) -> usize {
            // TODO: one party leaves the waitlist, but never go below zero
            // (hint: `usize::saturating_sub`).
            todo!("return waiting minus one, saturating at 0")
        }
    }
}

/// The "back of house": the kitchen, hidden from customers.
pub mod back_of_house {
    /// A breakfast order. The customer may pick the `toast`, but the
    /// `seasonal_fruit` is decided by the chef, so that field stays private:
    /// it can only be set through an associated function like [`Breakfast::summer`].
    #[derive(Debug, PartialEq, Eq)]
    pub struct Breakfast {
        /// Toast choice (e.g. `"Wheat"`), freely chosen by the customer.
        pub toast: String,
        // Private: callers outside this module cannot read or set it directly.
        seasonal_fruit: String,
    }

    impl Breakfast {
        /// Build a summer breakfast with the given `toast`. The seasonal fruit
        /// is fixed to `"peaches"`, demonstrating that a constructor can set a
        /// private field that callers cannot touch.
        pub fn summer(toast: &str) -> Breakfast {
            // TODO: construct a Breakfast whose `toast` comes from the argument
            // and whose private `seasonal_fruit` is "peaches".
            todo!("build a Breakfast with toast = `toast` and seasonal_fruit = \"peaches\"")
        }

        /// Read the (private) seasonal fruit through a public accessor.
        pub fn fruit(&self) -> &str {
            // TODO: return a reference to the private `seasonal_fruit` field.
            todo!("return &self.seasonal_fruit")
        }
    }

    /// An appetizer. Because the enum is `pub`, *all* of its variants are public
    /// too — unlike struct fields, you do not annotate each variant.
    #[derive(Debug, PartialEq, Eq)]
    pub enum Appetizer {
        /// Soup of the day.
        Soup,
        /// A simple salad.
        Salad,
    }

    impl Appetizer {
        /// Whether this appetizer is served hot. Soup is hot; salad is not.
        pub fn is_hot(&self) -> bool {
            // TODO: return true only for `Appetizer::Soup`
            // (hint: the `matches!` macro is handy).
            todo!("return whether self is Appetizer::Soup")
        }
    }
}

// Bring a deeply-nested path into scope so we can call `hosting::...` instead of
// repeating the full path. Idiomatically, for functions we bring the *parent*
// module into scope and call `hosting::add_to_waitlist`.
use crate::front_of_house::hosting;

// `pub use` re-exports a path: external code (and the tests) can now reach
// `back_of_house::Appetizer` as `ch07_packages_crates_modules::Appetizer`.
pub use crate::back_of_house::Appetizer;

/// Seat one party, then add it to the waitlist, returning the new waitlist
/// length. This calls `add_to_waitlist` two ways to show that absolute and
/// relative paths name the same item.
pub fn eat_at_restaurant(current_len: usize) -> usize {
    // TODO: call `add_to_waitlist` once via the absolute path
    // (`crate::front_of_house::hosting::add_to_waitlist`) and once via the
    // relative `hosting` alias brought into scope above, then return the new
    // waitlist length.
    let _ = hosting::add_to_waitlist; // keep the `use` import exercised
    todo!("grow the waitlist by one using both an absolute and a relative path")
}

/// Order a summer breakfast on the given `toast`, then change the toast to
/// `new_toast`. Returns the finished [`Breakfast`]. Demonstrates that the
/// public `toast` field is mutable from outside the module while the private
/// `seasonal_fruit` is not reachable here.
pub fn order_breakfast(toast: &str, new_toast: &str) -> back_of_house::Breakfast {
    // TODO: build a `Breakfast::summer(toast)`, reassign its public `toast`
    // field to `new_toast`, and return it.
    todo!("make a summer breakfast, then overwrite its public toast field")
}

/// Describe an appetizer in one short sentence, using the re-exported
/// [`Appetizer`] type. Soup is described as hot, salad as cold.
pub fn describe(appetizer: &Appetizer) -> String {
    // TODO: build a sentence like "The soup is served hot." — use `is_hot`
    // for the temperature and `match` to name the appetizer.
    todo!("format a one-sentence description of the appetizer")
}
