# 3.1 Variables and Mutability

In Rust, a `let` binding is immutable by default. Once a value is bound, you
cannot reassign it unless you opt in with `mut`. This is a safety feature: the
compiler guarantees a value you expect to stay fixed really does.

```rust
let x = 5;       // immutable
let mut y = 10;  // mutable
y = 20;          // OK because of `mut`
```

`const` is stricter still: it must have a type annotation, is computed at compile
time, and can live at item scope. Use it for fixed values like
`const SECONDS_PER_HOUR: u32 = 60 * 60;`.

**Shadowing** is different from mutation. A new `let` with the same name creates a
fresh binding, and the new value may even have a different type:

```rust
let spaces = "   ";        // &str
let spaces = spaces.len(); // now usize — a new variable
```

Shadowing lets you transform a value through a small pipeline while keeping one
descriptive name, without making it `mut`.

### Exercise
Complete `parse_and_square` in
`chapters/ch03_common_concepts/src/lib.rs` using shadowing, then run:

```bash
cargo test -p ch03_common_concepts
```
