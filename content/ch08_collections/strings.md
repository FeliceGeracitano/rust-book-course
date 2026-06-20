# 8.2 Storing UTF-8 Encoded Text with Strings

Rust's `String` is a growable, heap-allocated, UTF-8 encoded buffer of bytes.
That UTF-8 detail matters: a single character like `é` may take more than one
byte, so you **cannot** index a `String` by integer position. Instead, you
choose how to view the text. Use `.chars()` to step over Unicode scalar values,
or `.bytes()` for the raw bytes.

Grow a string with `push` (one `char`) and `push_str` (a `&str` slice). Splitting
on whitespace with `.split_whitespace()` skips over runs of spaces and returns
each word as a `&str`.

```rust
let mut greeting = String::new();
greeting.push_str("caf");
greeting.push('é');

let upper: String = greeting.chars().next().unwrap().to_uppercase().collect();
assert_eq!(upper, "C");
assert_eq!(greeting.chars().count(), 4);
```

Note that `char::to_uppercase` returns an *iterator*, because uppercasing one
character can produce several (think of the German `ß`). Pushing each resulting
`char` keeps your code correct for any input.

### Exercise

Implement `capitalize_words` in `chapters/ch08_collections/src/lib.rs` so it
upper-cases the first character of every word while leaving the rest untouched.
Then run:

```bash
cargo test -p ch08_collections
```
