use ch05_structs::{area, build_rectangle, with_email, Rectangle, User};

// --- 5.1 Defining and Instantiating Structs ---

#[test]
fn build_rectangle_uses_field_init_shorthand() {
    let r = build_rectangle(30, 50);
    assert_eq!(r.width, 30);
    assert_eq!(r.height, 50);
    assert_eq!(r, Rectangle { width: 30, height: 50 });
}

#[test]
fn with_email_uses_struct_update_syntax() {
    let base = User {
        username: String::from("ferris"),
        email: String::from("old@example.com"),
        active: true,
        sign_in_count: 3,
    };
    let updated = with_email(base, String::from("new@example.com"));

    assert_eq!(updated.email, "new@example.com");
    // Every other field is reused from `base`.
    assert_eq!(updated.username, "ferris");
    assert!(updated.active);
    assert_eq!(updated.sign_in_count, 3);
}

// --- 5.2 An Example Program Using Structs ---

#[test]
fn area_function_multiplies_dimensions() {
    let r = build_rectangle(30, 50);
    assert_eq!(area(&r), 1500);

    let zero = build_rectangle(0, 7);
    assert_eq!(area(&zero), 0);
}

// --- 5.3 Method Syntax ---

#[test]
fn area_method_matches_free_function() {
    let r = build_rectangle(30, 50);
    assert_eq!(r.area(), area(&r));
    assert_eq!(r.area(), 1500);
}

#[test]
fn can_hold_compares_both_dimensions() {
    let big = build_rectangle(30, 50);
    let small = build_rectangle(10, 40);
    let too_wide = build_rectangle(40, 40);

    assert!(big.can_hold(&small));
    assert!(!small.can_hold(&big));
    // Equal size: fits (>=), but a wider-only rectangle does not fit.
    assert!(big.can_hold(&big));
    assert!(!big.can_hold(&too_wide));
}

#[test]
fn square_associated_function_builds_equal_sides() {
    let sq = Rectangle::square(20);
    assert_eq!(sq.width, 20);
    assert_eq!(sq.height, 20);
    assert_eq!(sq.area(), 400);
}
