# 5.2 An Example Program Using Structs

Structs make code clearer by giving meaning to grouped data. Imagine computing
the area of a rectangle. You *could* pass two loose `u32` values, but nothing
ties the width and height together, and it is easy to swap them by mistake.

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height // two unrelated parameters
}
```

Bundling the dimensions into a `Rectangle` makes the relationship explicit, and
the function now takes a single, self-describing argument:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

Notice the `&`: `area` *borrows* the rectangle instead of taking ownership, so
the caller can keep using its value afterward. Deriving `Debug` with
`#[derive(Debug)]` also lets you print the whole struct for inspection with
`println!("{rectangle:?}")` — handy while developing.

This grouping is the first step. The dimensions and the operation on them
clearly belong together, which naturally leads to attaching the behavior to the
type itself (next section).

### Exercise

In `chapters/ch05_structs/src/lib.rs`, implement the free function `area`, which
takes `&Rectangle` and returns `width * height`. Then run:

```bash
cargo test -p ch05_structs
```
