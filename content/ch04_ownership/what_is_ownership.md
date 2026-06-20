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

### Exercise
Implement `combine` in `chapters/ch04_ownership/src/lib.rs`, then run:

```bash
cargo test -p ch04_ownership
```
