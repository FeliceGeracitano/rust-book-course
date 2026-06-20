use ch09_error_handling::{parse_and_double, safe_divide, sum_parsed, MathError};

// --- safe_divide: Result with a custom error enum ---

#[test]
fn divides_evenly() {
    assert_eq!(safe_divide(10, 2), Ok(5));
}

#[test]
fn truncates_toward_zero() {
    assert_eq!(safe_divide(7, 2), Ok(3));
    assert_eq!(safe_divide(-7, 2), Ok(-3));
}

#[test]
fn divide_by_zero_is_an_error_not_a_panic() {
    assert_eq!(safe_divide(1, 0), Err(MathError::DivideByZero));
}

#[test]
fn detects_overflow() {
    assert_eq!(safe_divide(i64::MIN, -1), Err(MathError::Overflow));
}

// --- parse_and_double: the ? operator propagating ParseIntError ---

#[test]
fn parses_and_doubles() {
    assert_eq!(parse_and_double("21"), Ok(42));
    assert_eq!(parse_and_double("-5"), Ok(-10));
}

#[test]
fn parse_failure_propagates() {
    assert!(parse_and_double("oops").is_err());
    assert!(parse_and_double("").is_err());
}

// --- sum_parsed: propagating the first error across a collection ---

#[test]
fn sums_when_all_parse() {
    assert_eq!(sum_parsed(&["1", "2", "3"]), Ok(6));
    assert_eq!(sum_parsed(&[]), Ok(0));
}

#[test]
fn stops_at_first_bad_entry() {
    let result = sum_parsed(&["10", "20", "thirty", "40"]);
    assert!(result.is_err());
}
