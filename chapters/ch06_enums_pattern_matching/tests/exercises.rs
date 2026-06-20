use ch06_enums_pattern_matching::{
    apply_command, describe_quarter, plus_one, username_or_guest, value_in_cents, Coin, Command,
    UsState,
};

#[test]
fn coins_have_expected_values() {
    assert_eq!(value_in_cents(Coin::Penny), 1);
    assert_eq!(value_in_cents(Coin::Nickel), 5);
    assert_eq!(value_in_cents(Coin::Dime), 10);
}

#[test]
fn quarter_is_worth_25_regardless_of_state() {
    assert_eq!(value_in_cents(Coin::Quarter(UsState::Alabama)), 25);
    assert_eq!(value_in_cents(Coin::Quarter(UsState::Alaska)), 25);
    assert_eq!(value_in_cents(Coin::Quarter(UsState::California)), 25);
}

#[test]
fn plus_one_adds_to_some_and_keeps_none() {
    assert_eq!(plus_one(Some(5)), Some(6));
    assert_eq!(plus_one(Some(-1)), Some(0));
    assert_eq!(plus_one(None), None);
}

#[test]
fn describe_quarter_only_matches_quarters() {
    assert_eq!(
        describe_quarter(Coin::Quarter(UsState::California)),
        Some(String::from("Quarter from California"))
    );
    assert_eq!(
        describe_quarter(Coin::Quarter(UsState::Alabama)),
        Some(String::from("Quarter from Alabama"))
    );
    assert_eq!(describe_quarter(Coin::Penny), None);
    assert_eq!(describe_quarter(Coin::Nickel), None);
    assert_eq!(describe_quarter(Coin::Dime), None);
}

#[test]
fn apply_command_moves_and_homes() {
    assert_eq!(apply_command((0, 0), Command::Move { dx: 3, dy: 4 }), (3, 4));
    assert_eq!(
        apply_command((1, 1), Command::Move { dx: 2, dy: -3 }),
        (3, -2)
    );
    assert_eq!(apply_command((4, 9), Command::Home), (0, 0));
    assert_eq!(apply_command((-5, 2), Command::Home), (0, 0));
}

#[test]
fn username_lookup_falls_back_to_guest() {
    let users = [(1, "alice"), (2, "bob")];
    assert_eq!(username_or_guest(&users, 1), "alice");
    assert_eq!(username_or_guest(&users, 2), "bob");
    assert_eq!(username_or_guest(&users, 99), "guest");
    assert_eq!(username_or_guest(&[], 1), "guest");
}
