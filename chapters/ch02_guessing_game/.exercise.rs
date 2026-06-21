//! Chapter 2 — Programming a Guessing Game
//!
//! The Book builds a guessing game around stdin input and a random secret
//! number. Those two pieces are I/O and randomness, so here we exercise the
//! *pure* logic underneath them instead: turning text into a number and
//! comparing a guess against a secret with `match` over [`std::cmp::Ordering`].
//!
//! Complete each item below by replacing the `todo!()` bodies, then run:
//!
//! ```text
//! cargo test -p ch02_guessing_game
//! ```

// You'll need `Ordering` to implement `check_guess`.
#[allow(unused_imports)]
use std::cmp::Ordering;

/// Parse a line of user input into a guess between 1 and 100.
///
/// Mirrors `let guess: u32 = guess.trim().parse()` from the Book, but returns
/// a `Result` instead of panicking. Leading/trailing whitespace (including the
/// trailing newline you'd get from stdin) is ignored.
///
/// Returns `Err` with a short message when the text is not a number, or when
/// the number is outside the inclusive range `1..=100`.
///
/// # Examples
///
/// ```
/// use ch02_guessing_game::parse_guess;
/// assert_eq!(parse_guess("42\n"), Ok(42));
/// assert!(parse_guess("hello").is_err());
/// assert!(parse_guess("0").is_err());
/// ```
pub fn parse_guess(input: &str) -> Result<u32, String> {
    // TODO: trim the input, `parse()` it into a u32, and return `Err` with a
    // message if parsing fails or the number is outside `1..=100`.
    let _ = input;
    todo!("trim, parse to u32, and bounds-check 1..=100")
}

/// The result of comparing a guess against the secret number.
///
/// This is the domain-specific stand-in for the three `match` arms in the
/// Book's game: too small, too big, or exactly right.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// The guess is smaller than the secret number ("Too small!").
    TooSmall,
    /// The guess is larger than the secret number ("Too big!").
    TooBig,
    /// The guess equals the secret number ("You win!").
    Correct,
}

/// Compare a `guess` against the `secret` and report the [`Outcome`].
///
/// This is the heart of the Book's `match guess.cmp(&secret)` block, lifted out
/// so it can be tested without stdin or randomness. Implement it by matching on
/// [`Ordering`].
///
/// # Examples
///
/// ```
/// use ch02_guessing_game::{check_guess, Outcome};
/// assert_eq!(check_guess(10, 50), Outcome::TooSmall);
/// assert_eq!(check_guess(90, 50), Outcome::TooBig);
/// assert_eq!(check_guess(50, 50), Outcome::Correct);
/// ```
pub fn check_guess(guess: u32, secret: u32) -> Outcome {
    // TODO: `match` on `guess.cmp(&secret)` and map each `Ordering` arm to the
    // matching `Outcome` variant.
    let _ = (guess, secret);
    todo!("match on Ordering and return the right Outcome")
}

/// Play a full round of the game by replaying a sequence of typed guesses.
///
/// Each entry in `inputs` is treated like a line the player typed. The game
/// loops in order: invalid lines (rejected by [`parse_guess`]) are skipped —
/// just as the Book's `continue` skips a bad parse — and the first valid guess
/// that equals `secret` wins.
///
/// Returns the **1-based position within `inputs`** of the winning line, or
/// `None` if the player never guessed the secret.
///
/// # Examples
///
/// ```
/// use ch02_guessing_game::play_round;
/// // "abc" is skipped, "30" is too small, "50" wins on the 3rd line.
/// assert_eq!(play_round(50, &["abc", "30", "50"]), Some(3));
/// assert_eq!(play_round(50, &["10", "20"]), None);
/// ```
pub fn play_round(secret: u32, inputs: &[&str]) -> Option<usize> {
    // TODO: loop over `inputs`, skip lines that `parse_guess` rejects, and
    // return `Some(1-based index)` for the first line whose `check_guess` is
    // `Outcome::Correct`; return `None` if no line wins.
    let _ = (secret, inputs);
    todo!("loop, skip bad lines, return the 1-based winning position")
}
