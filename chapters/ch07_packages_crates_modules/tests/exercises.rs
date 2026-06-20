//! Integration tests for Chapter 7 — Packages, Crates, and Modules.
//!
//! These exercise the module tree from outside the crate, which is exactly how
//! `pub`, paths, and `use` are meant to be observed.

use ch07_packages_crates_modules::back_of_house::Breakfast;
use ch07_packages_crates_modules::front_of_house::hosting;
use ch07_packages_crates_modules::{
    describe, eat_at_restaurant, order_breakfast, Appetizer,
};

#[test]
fn waitlist_position_is_one_based() {
    // Adding to an empty waitlist puts you at position 1.
    assert_eq!(hosting::add_to_waitlist(0), 1);
    assert_eq!(hosting::add_to_waitlist(3), 4);
}

#[test]
fn seating_shrinks_the_waitlist_but_not_below_zero() {
    assert_eq!(hosting::seat_at_table(2), 1);
    assert_eq!(hosting::seat_at_table(0), 0);
}

#[test]
fn eat_at_restaurant_grows_the_waitlist_by_one() {
    assert_eq!(eat_at_restaurant(0), 1);
    assert_eq!(eat_at_restaurant(5), 6);
}

#[test]
fn breakfast_has_public_toast_and_private_summer_fruit() {
    let meal = Breakfast::summer("Rye");
    // Public field is readable from outside the module.
    assert_eq!(meal.toast, "Rye");
    // Private field is reachable only through the accessor.
    assert_eq!(meal.fruit(), "peaches");
}

#[test]
fn order_breakfast_can_change_the_public_toast() {
    let meal = order_breakfast("Wheat", "Sourdough");
    assert_eq!(meal.toast, "Sourdough");
    assert_eq!(meal.fruit(), "peaches");
}

#[test]
fn appetizer_temperature() {
    assert!(Appetizer::Soup.is_hot());
    assert!(!Appetizer::Salad.is_hot());
}

#[test]
fn describe_uses_the_reexported_enum() {
    assert_eq!(describe(&Appetizer::Soup), "The soup is served hot.");
    assert_eq!(describe(&Appetizer::Salad), "The salad is served cold.");
}
