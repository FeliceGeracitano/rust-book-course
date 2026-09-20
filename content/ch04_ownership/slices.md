# 4.3 The Slice Type

A **slice** is a reference to a *contiguous range* of elements inside a
collection, rather than the whole thing. Because a slice is a kind of reference,
it borrows its data and copies nothing. A string slice has type `&str`; a slice
of a `[i32]` array or `Vec<i32>` has type `&[i32]`.

You build a slice with range syntax: `&data[start..end]` covers indices `start`
up to but not including `end`. `&data[..]` is the whole thing.

Slices stay in sync with their source. If you return the index of a word and then
the string is cleared, the index is stale — but a slice borrows the data, so the
borrow checker stops you from invalidating it while the slice is alive. That is
why `first_word` returns a `&str` pointing into the input instead of a number.

```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];   // &str borrowing the first 5 bytes
    let world = &s[6..11];
    println!("{hello} {world}");

    let nums = [1, 2, 3, 4, 5];
    let middle = &nums[1..4]; // &[i32] == [2, 3, 4]
    println!("{}", middle.len());
}
```

Watch range bounds: indexing past the end panics, so clamp lengths when the range
might overrun.

```quiz
{
  "id": "discovery",
  "question": "What does &text[..5] give you for text = String::from(\"hello rust\")?",
  "options": [
    "Ownership of text",
    "A borrowed view of the first five bytes",
    "A newly allocated String"
  ],
  "answer": 1,
  "explain": "A &str slice borrows a valid UTF-8 range of the existing string. It does not copy or take ownership of the allocation."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Implement `first_word` and `window` in
`chapters/ch04_ownership/src/lib.rs`, then run:

```bash
cargo test -p ch04_ownership
```
