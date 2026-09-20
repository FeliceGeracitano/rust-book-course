# 2.1 Building the Guessing Game

The Book's guessing game ties together input, parsing, comparison, and looping.
Two of those pieces — reading stdin and rolling a random secret — are I/O and
randomness, which make code hard to test. So here we practice the *pure* logic
that sits underneath them.

**Parsing.** Text from a user is always a `String`, even when it looks like a
number. To compare it numerically you must `parse()` it into an integer. Parsing
can fail, so `parse()` returns a `Result` — handle the error instead of trusting
the input. Don't forget to `trim()` away the trailing newline.

**Comparing.** Once you have a number, `cmp` returns an
[`Ordering`](https://doc.rust-lang.org/std/cmp/enum.Ordering.html), and `match`
forces you to handle every case:

```rust
use std::cmp::Ordering;

fn describe(guess: u32, secret: u32) -> &'static str {
    match guess.cmp(&secret) {
        Ordering::Less => "Too small!",
        Ordering::Greater => "Too big!",
        Ordering::Equal => "You win!",
    }
}
```

The real game wraps this in a loop that keeps asking until `Equal`. Your
`play_round` does the same over a slice of pretend "typed" lines, skipping any
that fail to parse — mirroring the Book's `continue` on bad input.

```quiz
{
  "id": "discovery",
  "question": "A player enters letters instead of a number. What should the game do with parse::<u32>()?",
  "options": [
    "Handle Err and ask for another guess",
    "Use the text directly as a u32",
    "Assume parse always succeeds"
  ],
  "answer": 0,
  "explain": "parse returns a Result. Handling Err lets the game recover from invalid user input rather than terminating with unwrap."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Complete `parse_guess`, `check_guess`, and `play_round` in
`chapters/ch02_guessing_game/src/lib.rs`, then run:

```bash
cargo test -p ch02_guessing_game
```
