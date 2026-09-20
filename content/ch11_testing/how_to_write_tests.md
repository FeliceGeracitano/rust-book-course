# 11.1 How to Write Tests

A test in Rust is just a function marked with the `#[test]` attribute. When you
run `cargo test`, the test harness runs every such function on its own and
reports which ones passed. A test *passes* unless something inside it panics.

You rarely write `panic!` by hand. Instead you use assertion macros:

- `assert!(condition)` fails when the condition is `false`.
- `assert_eq!(left, right)` and `assert_ne!` compare two values and, on failure,
  print both so you can see what went wrong.
- You can add a custom message: `assert!(cond, "got {value}")`.

```rust
pub fn add_two(value: i32) -> i32 {
    value + 2
}

#[test]
fn it_adds_two() {
    assert_eq!(add_two(2), 4);
}
```

To test code that *should* fail, mark the test `#[should_panic]`. Add
`expected = "..."` so the test only passes when the panic message contains that
substring — this stops an unrelated panic from masquerading as success.

```quiz
{
  "id": "discovery",
  "question": "What makes a typical #[test] function fail?",
  "options": [
    "Returning normally",
    "A panic, such as one triggered by a failed assertion",
    "Printing to stdout"
  ],
  "answer": 1,
  "explain": "Assertions panic when their conditions are false, which the test harness reports as failure. Tests can also return a Result where Err indicates failure."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Open `chapters/ch11_testing/src/lib.rs` and implement the functions and the
`Guess` type so they behave as their doc comments describe. Then run:

```bash
cargo test -p ch11_testing
```
