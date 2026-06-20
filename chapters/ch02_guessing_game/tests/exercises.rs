use ch02_guessing_game::{check_guess, parse_guess, play_round, Outcome};

#[test]
fn parse_guess_accepts_plain_number() {
    assert_eq!(parse_guess("42"), Ok(42));
}

#[test]
fn parse_guess_trims_whitespace_and_newline() {
    assert_eq!(parse_guess("  7\n"), Ok(7));
}

#[test]
fn parse_guess_accepts_range_bounds() {
    assert_eq!(parse_guess("1"), Ok(1));
    assert_eq!(parse_guess("100"), Ok(100));
}

#[test]
fn parse_guess_rejects_non_numbers() {
    assert!(parse_guess("hello").is_err());
    assert!(parse_guess("").is_err());
}

#[test]
fn parse_guess_rejects_out_of_range() {
    assert!(parse_guess("0").is_err());
    assert!(parse_guess("101").is_err());
}

#[test]
fn check_guess_reports_too_small() {
    assert_eq!(check_guess(10, 50), Outcome::TooSmall);
}

#[test]
fn check_guess_reports_too_big() {
    assert_eq!(check_guess(90, 50), Outcome::TooBig);
}

#[test]
fn check_guess_reports_correct() {
    assert_eq!(check_guess(50, 50), Outcome::Correct);
}

#[test]
fn play_round_finds_winning_line() {
    // "abc" is skipped (unparsable), "30" is too small, "50" wins on line 3.
    assert_eq!(play_round(50, &["abc", "30", "50"]), Some(3));
}

#[test]
fn play_round_wins_on_first_line() {
    assert_eq!(play_round(7, &["7", "99"]), Some(1));
}

#[test]
fn play_round_returns_none_when_never_correct() {
    assert_eq!(play_round(50, &["10", "20", "bad", "200"]), None);
}
