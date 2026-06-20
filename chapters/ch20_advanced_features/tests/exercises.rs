use ch20_advanced_features::{Meters, Point, split_at_mut, string_vec};

// --- Exercise 1: Unsafe Rust — safe wrapper around raw pointers -------------

#[test]
fn split_at_mut_splits_into_two_halves() {
    let mut data = [1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut data, 3);
    assert_eq!(left, &mut [1, 2, 3]);
    assert_eq!(right, &mut [4, 5, 6]);
}

#[test]
fn split_at_mut_allows_independent_mutation() {
    let mut data = [10, 20, 30, 40];
    let (left, right) = split_at_mut(&mut data, 2);
    left[0] += 1;
    right[1] += 5;
    assert_eq!(data, [11, 20, 30, 45]);
}

#[test]
fn split_at_mut_handles_edge_positions() {
    let mut data = [1, 2, 3];

    let (left, right) = split_at_mut(&mut data, 0);
    assert_eq!(left, &mut []);
    assert_eq!(right, &mut [1, 2, 3]);

    let (left, right) = split_at_mut(&mut data, 3);
    assert_eq!(left, &mut [1, 2, 3]);
    assert_eq!(right, &mut []);
}

#[test]
#[should_panic]
fn split_at_mut_panics_past_end() {
    let mut data = [1, 2, 3];
    let _ = split_at_mut(&mut data, 4);
}

// --- Exercise 2: Operator overloading with Add ------------------------------

#[test]
fn points_add_componentwise() {
    let sum = Point { x: 1, y: 2 } + Point { x: 3, y: 4 };
    assert_eq!(sum, Point { x: 4, y: 6 });
}

#[test]
fn adding_zero_point_is_identity() {
    let p = Point { x: 7, y: -3 };
    let zero = Point { x: 0, y: 0 };
    assert_eq!(p + zero, p);
}

// --- Exercise 3: Newtype pattern --------------------------------------------

#[test]
fn meters_expose_inner_value() {
    let d = Meters::new(42.5);
    assert_eq!(d.value(), 42.5);
}

#[test]
fn meters_add_while_keeping_their_unit() {
    let total = Meters::new(2.0) + Meters::new(3.0);
    assert_eq!(total, Meters::new(5.0));
    assert_eq!(total.value(), 5.0);
}

// --- Exercise 4: Declarative macro ------------------------------------------

#[test]
fn string_vec_builds_a_vec_of_strings() {
    let v = string_vec!["a", "b", "c"];
    assert_eq!(
        v,
        vec![String::from("a"), String::from("b"), String::from("c")]
    );
}

#[test]
fn string_vec_accepts_trailing_comma_and_is_empty_ready() {
    let v = string_vec!["only",];
    assert_eq!(v, vec![String::from("only")]);

    let empty: Vec<String> = string_vec![];
    assert!(empty.is_empty());
}

#[test]
fn string_vec_converts_non_str_expressions() {
    // Any value with `.to_string()` works, e.g. integers.
    let v = string_vec![1, 2, 3];
    assert_eq!(
        v,
        vec![String::from("1"), String::from("2"), String::from("3")]
    );
}
