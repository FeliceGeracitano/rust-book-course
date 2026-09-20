# 10.3 Validating References with Lifetimes

Every reference in Rust has a **lifetime** — the span during which it stays
valid. Usually the compiler infers them, but when a function returns a reference
derived from several inputs, it can't always tell which input the result borrows
from. Lifetime annotations give names to those relationships so the borrow
checker can prove no reference outlives its data.

A lifetime parameter looks like `'a` and is declared in angle brackets, just
like a generic type. Writing `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`
tells Rust: the returned slice lives at least as long as *both* inputs.
Annotations don't change how long values live — they only describe constraints
the compiler then enforces.

Structs that hold references also need a lifetime, ensuring the struct can't
outlive the data it borrows. This is how you safely keep a slice of a larger
string around.

```rust
struct Holder<'a> {
    text: &'a str,
}

let s = String::from("hello world");
let h = Holder { text: &s[..5] };
assert_eq!(h.text, "hello");
```

```quiz
{
  "id": "discovery",
  "question": "Do lifetime annotations make a borrowed value live longer?",
  "options": [
    "Yes, they allocate the value on the heap",
    "Yes, they disable dropping",
    "No, they describe relationships between reference lifetimes"
  ],
  "answer": 2,
  "explain": "Lifetimes help the compiler check that references remain valid. An annotation cannot extend storage duration or make a dangling reference safe."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Implement `longest` and the `Excerpt<'a>` methods in
`chapters/ch10_generics_traits_lifetimes/src/lib.rs`, then run:

```bash
cargo test -p ch10_generics_traits_lifetimes
```
