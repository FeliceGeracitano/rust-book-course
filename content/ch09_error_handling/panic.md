# 9.1 Unrecoverable Errors with panic!

Some failures mean your program has hit a state it was never designed to handle —
a bug, a broken invariant, something it cannot reasonably continue past. For these,
Rust offers `panic!`. When a panic fires, the program prints an error message,
unwinds the stack (cleaning up as it goes), and exits.

```rust
fn main() {
    let scores = [10, 20, 30];
    // This panics: index 5 is out of bounds.
    let value = scores[5];
    println!("{value}");
}
```

Many standard methods panic for you. `Vec` and slice indexing panics on an
out-of-bounds index; `Option::unwrap` and `Result::unwrap` panic when there's no
value or an error. That convenience is fine in a quick prototype or a test, but in
real code a panic is a one-way door: the caller gets no chance to recover.

Set `RUST_BACKTRACE=1` to see exactly where a panic originated — invaluable when
debugging. The key idea for the rest of this chapter: panicking is for *truly
unrecoverable* situations. When a caller might sensibly handle the failure,
return a value instead.

```quiz
{
  "id": "discovery",
  "question": "What does panic! usually communicate?",
  "options": [
    "A successful early return",
    "An unrecoverable failure in the current computation",
    "An ordinary missing search result"
  ],
  "answer": 1,
  "explain": "panic! stops normal execution. Depending on the build configuration it unwinds or aborts; expected failures are usually represented with Result."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Open `chapters/ch09_error_handling/src/lib.rs`. Notice that every function returns
a `Result` rather than panicking — that's the pattern this chapter teaches. Run:

```bash
cargo test -p ch09_error_handling
```
