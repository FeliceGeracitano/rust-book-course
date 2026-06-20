# 19.3 Pattern Syntax

This is where patterns earn their keep. A handful of pieces combine to express
rich conditions concisely.

**Literals and `|`** match exact values, with `|` meaning "or":
`1 | 2 => ...`. **Ranges** match an inclusive interval: `'a'..='z'` or `1..=89`,
far cleaner than spelling out every value.

**Destructuring** pulls data out of compound types. Structs, tuples, and enums
all open up inside a pattern, and you can pin a field to a literal to match an
exact case:

```rust
struct Point { x: i32, y: i32 }

match (Point { x: 0, y: 7 }) {
    Point { x: 0, y } => println!("on the y axis at {y}"),
    Point { x, y }    => println!("at ({x}, {y})"),
}
```

**Ignoring**: `_` discards one value, `..` skips the rest of a struct or tuple.

**Match guards** add an `if` condition after the pattern, splitting one shape into
cases the pattern alone cannot express:

```rust
let n = 4;
match n {
    x if x % 2 == 0 => println!("{x} is even"),
    x              => println!("{x} is odd"),
}
```

**`@` bindings** test a value *and* capture it at once: `id @ 1..=100` matches the
range while binding the number to `id` for use in the arm — something neither the
range nor a plain binding can do alone.

### Exercise
Complete the `todo!()` bodies in
`chapters/ch19_patterns_matching/src/lib.rs`, then run:

`cargo test -p ch19_patterns_matching`
