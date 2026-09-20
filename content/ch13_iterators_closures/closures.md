# 13.1 Closures

A **closure** is an anonymous function you can store in a variable or hand to
another function. Unlike a regular `fn`, a closure can *capture* values from the
scope where it is defined. Rust infers the parameter and return types from how
the closure is used, so you usually write them with terse `|args| body` syntax.

Closures implement one of three traits depending on how they use captured
values: `FnOnce` (consumes captures, callable once), `FnMut` (mutates captures),
or `Fn` (only reads captures). Most standard-library methods that take a closure
accept the most permissive trait they can.

A great example is `Option::unwrap_or_else`, which only runs its closure when the
value is `None` — so an expensive fallback stays lazy:

```rust
let preference: Option<&str> = None;
let chosen = preference.unwrap_or_else(|| {
    // only evaluated because preference is None
    "default"
});
assert_eq!(chosen, "default");
```

Here the closure captures nothing, but it could just as easily read a field of
`self` to compute the fallback. That is exactly the pattern the shirt-store
`Inventory::giveaway` uses.

```quiz
{
  "id": "discovery",
  "question": "Why can this closure read offset without an explicit parameter?",
  "options": [
    "offset becomes a global variable",
    "Closures can capture values from their surrounding scope",
    "All Rust functions automatically read local variables"
  ],
  "answer": 1,
  "explain": "Unlike an ordinary named function, a closure can capture its environment. Here it only needs shared access to offset.",
  "code": "fn main() {\n    let offset = 2;\n    let add = |n| n + offset;\n    println!(\"{}\", add(3));\n}"
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Implement `Inventory::giveaway` and `Inventory::most_stocked` in
`chapters/ch13_iterators_closures/src/lib.rs`, then run:

```bash
cargo test -p ch13_iterators_closures
```
