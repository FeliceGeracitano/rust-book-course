# 3.3 Functions

Functions are declared with `fn`. Each parameter must have a type annotation, and
if the function returns a value you declare its type after `->`.

```rust
fn square(n: i64) -> i64 {
    n * n
}
```

Rust distinguishes **statements** from **expressions**. A statement performs an
action and returns nothing (`let x = 5;`). An expression evaluates to a value
(`5 + 1`, a block `{ ... }`, an `if`). The last expression in a function body —
written *without* a trailing semicolon — becomes its return value. Adding a
semicolon turns it into a statement that returns `()`, which is a common
beginner mistake.

```rust
fn five() -> i32 {
    5      // no semicolon: this is the return value
}
```

You can also return early with the `return` keyword, but the trailing-expression
style is idiomatic for the final value. Functions can be defined in any order;
the compiler sees them all.

### Exercise
The functions in `chapters/ch03_common_concepts/src/lib.rs` all rely on returning
a final expression. Complete them, paying attention to the missing semicolons,
then run:

```bash
cargo test -p ch03_common_concepts
```
