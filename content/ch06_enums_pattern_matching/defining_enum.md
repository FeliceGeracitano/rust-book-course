# 6.1 Defining an Enum

A struct says "this AND that". An **enum** says "this OR that": a value is exactly
one of a fixed set of *variants*. That makes illegal states unrepresentable —
a coin is a penny, nickel, dime, or quarter, never two at once.

Variants can be bare names, or they can carry data. Different variants may even
carry different types, which is something a struct cannot do:

```rust
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // this variant holds extra data
}

let c = Coin::Quarter(String::from("Alaska"));
println!("{c:?}");
```

The standard library's `Option<T>` is just an enum — `Some(T)` or `None` — which
is why Rust has no null. Instead of a value that might secretly be null, you hold
an `Option<T>` and the compiler forces you to handle the empty case before you can
reach the inner value. Absence becomes part of the type, checked at compile time.

You can also attach methods to an enum with `impl`, exactly like a struct.

```quiz
{
  "id": "discovery",
  "question": "How should a value that may be missing be represented?",
  "options": [
    "Option<T> with Some(value) or None",
    "A null T that needs no checking",
    "Always the number zero"
  ],
  "answer": 0,
  "explain": "Option makes presence and absence distinct variants in the type system, so callers must account for the missing case."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Open `chapters/ch06_enums_pattern_matching/src/lib.rs` and define/complete the
`Coin`, `UsState`, and `Command` enums plus their functions. Then run:

`cargo test -p ch06_enums_pattern_matching`
