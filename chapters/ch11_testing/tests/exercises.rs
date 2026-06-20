//! Integration tests for Chapter 11.
//!
//! These mirror the style from the Book: `assert!` for boolean logic,
//! `assert_eq!` for exact values, custom messages, and `#[should_panic]` for
//! the failure path. They assert real expected values, so making them pass
//! means your implementations are correct.

use ch11_testing::{add_two, greeting, Guess, Rectangle};

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(larger.can_hold(&smaller));
}

#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 5,
        height: 1,
    };

    assert!(!smaller.can_hold(&larger));
}

#[test]
fn equal_rectangles_hold_each_other() {
    let a = Rectangle {
        width: 4,
        height: 4,
    };
    let b = Rectangle {
        width: 4,
        height: 4,
    };

    assert!(a.can_hold(&b));
    assert!(b.can_hold(&a));
}

#[test]
fn taller_but_narrower_cannot_hold() {
    let tall = Rectangle {
        width: 2,
        height: 9,
    };
    let wide = Rectangle {
        width: 9,
        height: 2,
    };

    assert!(!tall.can_hold(&wide));
    assert!(!wide.can_hold(&tall));
}

#[test]
fn adds_two() {
    assert_eq!(add_two(2), 4);
    assert_eq!(add_two(0), 2);
    assert_eq!(add_two(-2), 0);
}

#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "greeting did not contain name, value was `{result}`"
    );
}

#[test]
fn guess_in_range_keeps_value() {
    assert_eq!(Guess::new(1).value(), 1);
    assert_eq!(Guess::new(50).value(), 50);
    assert_eq!(Guess::new(100).value(), 100);
}

#[test]
#[should_panic(expected = "greater than or equal to 1")]
fn guess_below_range_panics() {
    Guess::new(0);
}

#[test]
#[should_panic(expected = "less than or equal to 100")]
fn guess_above_range_panics() {
    Guess::new(200);
}
