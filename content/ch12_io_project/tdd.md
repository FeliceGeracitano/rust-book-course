# 12.4 Adding Functionality with Test Driven Development

Test-driven development flips the usual order: you write a failing test that
describes the behaviour you want, then write just enough code to make it pass.
For `minigrep` the behaviour is searching — return every line of some text that
contains a query string, in their original order.

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

The lifetime `'a` ties the returned slices to `contents`: the results *borrow*
the original text, so nothing is copied. `.lines()` splits on newlines,
`.filter()` keeps matching lines, and `.collect()` gathers them into a `Vec`.

Writing the test first forces you to pin down the exact expected output — which
lines match, and in what order — before you touch the implementation. When the
red test turns green, you have a precise, regression-proof definition of
"search." That confidence is exactly what lets you refactor the body later
(say, swapping the loop for iterator adapters) without fear.

```quiz
{
  "id": "discovery",
  "question": "What comes first when adding a new search behavior with test-driven development?",
  "options": [
    "Optimizing allocations before defining behavior",
    "A test that fails because the behavior is missing",
    "Deleting existing tests"
  ],
  "answer": 1,
  "explain": "Start with a failing test, implement enough to make it pass, then refactor while preserving the passing behavior."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Implement `search` in `chapters/ch12_io_project/src/lib.rs` to make the
case-sensitive search tests pass, then run:

```bash
cargo test -p ch12_io_project
```
