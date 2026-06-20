# 19.1 All the Places Patterns Can Be Used

A *pattern* describes the **shape** of data. You have already used patterns
without naming them: every `match` arm is a pattern, and so is the left side of a
`let`. Once you see them as one idea, they show up everywhere.

Patterns appear in:

- **`match` arms** — each arm is a pattern, and `match` must be exhaustive.
- **`let` statements** — `let (a, b) = pair;` destructures into two bindings.
- **`if let` / `while let` / `let...else`** — match one shape and act on it.
- **Function and closure parameters** — `fn dist(&(x, y): &(i32, i32))` binds the
  tuple's parts straight from the argument.
- **`for` loops** — `for (i, v) in items.iter().enumerate()` destructures the pair.

```rust
let point = (3, 5);
let (x, y) = point;            // `let` pattern
println!("{x}, {y}");

for (i, c) in "ab".chars().enumerate() {
    println!("{i}: {c}");      // `for` pattern
}
```

The same syntax binds names, ignores parts, and pulls fields out of structs,
tuples, and enums no matter where it sits. Learning patterns once pays off across
all of these positions.

### Exercise
Open `chapters/ch19_patterns_matching/src/lib.rs` and complete the `todo!()`
bodies, then run:

`cargo test -p ch19_patterns_matching`
