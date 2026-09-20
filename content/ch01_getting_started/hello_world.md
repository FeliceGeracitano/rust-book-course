# 1.2 Hello, World!

A Rust program starts at `main`. Save this as `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Compile and run:

```bash
rustc main.rs
./main
```

`println!` is a **macro** (note the `!`), not a function — more on that in Chapter 20.

```quiz
{
  "id": "discovery",
  "question": "What does the exclamation mark in println! tell you?",
  "options": [
    "It means the call can panic",
    "It makes the function asynchronous",
    "It invokes a macro"
  ],
  "answer": 2,
  "explain": "The ! distinguishes macro invocations from ordinary function calls. println! expands into code that formats and prints its arguments."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

In `chapters/ch01_getting_started/src/lib.rs`, make `greeting()` return
`"Hello, world!"`, then:

```bash
cargo test -p ch01_getting_started
```
