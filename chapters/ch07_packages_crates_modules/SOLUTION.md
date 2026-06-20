# Chapter 7 — Solutions

A single library crate that models a small restaurant with a module tree.
The bodies below are the reference implementations for the `todo!()`s in
`src/lib.rs`.

```rust
/// The "front of house": everything customers interact with.
pub mod front_of_house {
    /// Seating and waitlist management.
    pub mod hosting {
        /// Add a party to the waitlist and return its 1-based position.
        pub fn add_to_waitlist(current_len: usize) -> usize {
            current_len + 1
        }

        /// Seat the party at the front, returning the number still waiting.
        pub fn seat_at_table(waiting: usize) -> usize {
            waiting.saturating_sub(1)
        }
    }
}

/// The "back of house": the kitchen, hidden from customers.
pub mod back_of_house {
    /// A breakfast order: public `toast`, private `seasonal_fruit`.
    #[derive(Debug, PartialEq, Eq)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        /// Build a summer breakfast; the seasonal fruit is fixed to "peaches".
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        /// Read the private seasonal fruit through a public accessor.
        pub fn fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }

    /// A `pub` enum: every variant is public, no per-variant annotation needed.
    #[derive(Debug, PartialEq, Eq)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Appetizer {
        /// Whether this appetizer is served hot. Soup is hot; salad is not.
        pub fn is_hot(&self) -> bool {
            matches!(self, Appetizer::Soup)
        }
    }
}

// Bring the parent module into scope; call `hosting::add_to_waitlist`.
use crate::front_of_house::hosting;

// `pub use` re-exports the enum at the crate root.
pub use crate::back_of_house::Appetizer;

/// Add one party to the waitlist via both an absolute and a relative path.
pub fn eat_at_restaurant(current_len: usize) -> usize {
    let after_absolute = crate::front_of_house::hosting::add_to_waitlist(current_len);
    let after_relative = hosting::add_to_waitlist(after_absolute - 1);
    debug_assert_eq!(after_absolute, after_relative);
    after_relative
}

/// Order a summer breakfast, then change the public `toast` field.
pub fn order_breakfast(toast: &str, new_toast: &str) -> back_of_house::Breakfast {
    let mut meal = back_of_house::Breakfast::summer(toast);
    meal.toast = String::from(new_toast);
    meal
}

/// Describe an appetizer using the re-exported `Appetizer` type.
pub fn describe(appetizer: &Appetizer) -> String {
    let temperature = if appetizer.is_hot() { "hot" } else { "cold" };
    let name = match appetizer {
        Appetizer::Soup => "soup",
        Appetizer::Salad => "salad",
    };
    format!("The {name} is served {temperature}.")
}
```
