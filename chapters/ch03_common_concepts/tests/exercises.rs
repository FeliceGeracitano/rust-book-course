use ch03_common_concepts::{
    classify, fahrenheit_to_celsius, fib, min_and_max, parse_and_square, SECONDS_PER_HOUR,
};

#[test]
fn const_seconds_per_hour() {
    assert_eq!(SECONDS_PER_HOUR, 3600);
}

#[test]
fn parse_and_square_handles_valid_input() {
    assert_eq!(parse_and_square("9"), Some(81));
    assert_eq!(parse_and_square("  12  "), Some(144));
    assert_eq!(parse_and_square("-4"), Some(16));
    assert_eq!(parse_and_square("0"), Some(0));
}

#[test]
fn parse_and_square_rejects_invalid_input() {
    assert_eq!(parse_and_square("abc"), None);
    assert_eq!(parse_and_square(""), None);
    assert_eq!(parse_and_square("3.5"), None);
}

#[test]
fn fahrenheit_to_celsius_known_points() {
    assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
    assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    assert_eq!(fahrenheit_to_celsius(98.6), 37.0);
}

#[test]
fn fahrenheit_to_celsius_below_freezing() {
    assert_eq!(fahrenheit_to_celsius(-40.0), -40.0);
}

#[test]
fn min_and_max_finds_extremes() {
    assert_eq!(min_and_max([3, 7, 1, 9, 4]), (1, 9));
    assert_eq!(min_and_max([5, 5, 5, 5, 5]), (5, 5));
    assert_eq!(min_and_max([-2, -8, -1, -5, -3]), (-8, -1));
}

#[test]
fn fib_sequence() {
    let expected = [0u64, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    for (n, &value) in expected.iter().enumerate() {
        assert_eq!(fib(n as u32), value, "fib({n}) should be {value}");
    }
}

#[test]
fn classify_all_branches() {
    assert_eq!(classify(-7), "negative");
    assert_eq!(classify(0), "zero");
    assert_eq!(classify(1), "small");
    assert_eq!(classify(9), "small");
    assert_eq!(classify(10), "large");
    assert_eq!(classify(1000), "large");
}
