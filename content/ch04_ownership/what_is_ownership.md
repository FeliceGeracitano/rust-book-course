# 4.1 What is Ownership?

**Ownership** is the set of rules the Rust compiler checks to manage memory. There
is no garbage collector and no manual `free` — instead the compiler proves, at
compile time, exactly when each value is cleaned up.

Three rules drive everything:

- Each value has a single **owner**.
- There can be only one owner at a time.
- When the owner goes out of scope, the value is dropped (its memory is freed).

For heap data like `String`, assigning or passing a value **moves** it. After a
move the old binding is invalid, so two owners can never free the same memory.
Simple stack types (`i32`, `bool`, `char`, …) implement `Copy` and are duplicated
instead of moved.

```rust
fn main() {
    let s1 = String::from("hi");
    let s2 = s1;            // s1 is MOVED into s2
    // println!("{s1}");    // compile error: s1 no longer owns the data
    println!("{s2}");       // ok

    let n = 5;
    let m = n;              // i32 is Copy: both n and m are valid
    println!("{n} {m}");
}
```

Passing an owned value to a function moves it in; returning a value moves it back
out — exactly what the `combine` exercise practices.

```trace
{
  "title": "One allocation, a new owner",
  "code": "fn main() {\n    let first = String::from(\"hello\");\n    let second = first;\n    println!(\"{second}\");\n}",
  "steps": [
    {
      "line": 2,
      "note": "String::from allocates the text. first owns the allocation.",
      "state": {
        "first": "owns \"hello\"",
        "second": "not bound yet"
      },
      "output": ""
    },
    {
      "line": 3,
      "note": "The assignment moves ownership. It does not duplicate the allocation.",
      "state": {
        "first": "moved; cannot be read",
        "second": "owns \"hello\""
      },
      "output": ""
    },
    {
      "line": 4,
      "note": "println! borrows second to format it, so second keeps ownership.",
      "state": {
        "first": "moved",
        "second": "still owns \"hello\""
      },
      "output": "hello\n"
    },
    {
      "line": 5,
      "note": "At the end of the scope, second is dropped and the allocation is freed once.",
      "state": {
        "first": "moved",
        "second": "dropped"
      },
      "output": "hello\n"
    }
  ]
}
```

```quiz
{
  "id": "discovery",
  "question": "Which binding can still be used after this move?",
  "options": [
    "second only",
    "Both first and second",
    "first only"
  ],
  "answer": 0,
  "explain": "String does not implement Copy. Assigning it to second transfers ownership; first is no longer usable. clone() would explicitly create another owned String.",
  "code": "fn main() {\n    let first = String::from(\"hello\");\n    let second = first;\n}"
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Implement `combine` in `chapters/ch04_ownership/src/lib.rs`, then run:

```bash
cargo test -p ch04_ownership
```
