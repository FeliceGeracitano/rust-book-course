# 6.2 The match Control Flow Construct

`match` compares a value against a series of patterns and runs the first arm that
fits. Its superpower is being **exhaustive**: the compiler refuses to build unless
every possible variant is covered, so adding a new enum variant later turns silent
gaps into compile errors.

Each arm is `pattern => expression`. A pattern can *bind* the data inside a variant
to a name, pulling it out for use in that arm:

```rust
enum Coin { Penny, Quarter(String) }

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Quarter(state) => {
            println!("Quarter from {state}!");
            25
        }
    }
}
```

Matching on `Option<T>` is the idiomatic way to handle "maybe a value": the
`Some(n)` arm binds `n`, the `None` arm handles absence. When you don't want to
list every case, the catch-all `_ => ...` (or a name like `other => ...`) handles
the rest. Struct-like variants destructure with braces: `Command::Move { dx, dy }`.

`match` is an expression, so every arm must produce the same type, and that value
becomes the result.

### Exercise
Complete `value_in_cents`, `plus_one`, and `apply_command` in
`chapters/ch06_enums_pattern_matching/src/lib.rs`, then run:

`cargo test -p ch06_enums_pattern_matching`
