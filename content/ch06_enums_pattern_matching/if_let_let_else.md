# 6.3 Concise Control Flow with if let and let...else

Sometimes a full `match` is overkill: you only care about *one* pattern and want
to ignore everything else. `if let` is the shorthand. It runs its block only when
the pattern matches, binding the inner data, and an optional `else` covers the
rest:

```rust
let config: Option<u8> = Some(3);

if let Some(max) = config {
    println!("max is {max}");
} else {
    println!("no max set");
}
```

You trade exhaustiveness for brevity, so reach for `if let` when the other cases
genuinely need no handling.

`let...else` flips the shape for the common "extract or bail" case. The happy-path
binding lives in the normal `let`, and the `else` block — which **must diverge**
(`return`, `break`, `continue`, or `panic!`) — handles the miss. Crucially, the
binding stays in scope *after* the statement, keeping the rest of the function flat
instead of nested:

```rust
fn first_char(s: &str) -> char {
    let Some(c) = s.chars().next() else {
        return '?';
    };
    c // `c` is available here, no rightward drift
}
```

### Exercise
Complete `describe_quarter` (with `if let`) and `username_or_guest` (with
`let...else`) in `chapters/ch06_enums_pattern_matching/src/lib.rs`, then run:

`cargo test -p ch06_enums_pattern_matching`
