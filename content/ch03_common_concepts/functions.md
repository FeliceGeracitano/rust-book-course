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

```quiz
{
  "id": "discovery",
  "question": "Why does this function fail to compile?",
  "options": [
    "The semicolon makes the body return ()",
    "Returning i32 requires mut",
    "Functions cannot multiply"
  ],
  "answer": 0,
  "explain": "A trailing expression supplies the return value. Removing the semicolon makes x * 2 the expression returned from double.",
  "code": "fn double(x: i32) -> i32 {\n    x * 2;\n}"
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

The functions in `chapters/ch03_common_concepts/src/lib.rs` all rely on returning
a final expression. Complete them, paying attention to the missing semicolons,
then run:

```bash
cargo test -p ch03_common_concepts
```
