# 5.3 Method Syntax

When a function logically belongs to a type, define it inside an `impl` block as
a **method**. Its first parameter is `self`, which represents the value the
method is called on. Usually you want `&self` to borrow it, just like passing
`&Rectangle` to a free function — but now the call reads `rect.area()`.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

let r = Rectangle { width: 30, height: 50 };
println!("{}", r.area()); // 1500
```

Methods keep behavior next to the data it operates on, and Rust's *automatic
referencing* means you write `r.area()` rather than `(&r).area()`.

An `impl` block can also hold **associated functions** that take no `self`.
These are often constructors, called with `::` on the type:

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

let sq = Rectangle::square(20);
```

```quiz
{
  "id": "discovery",
  "question": "What does &self mean in a method signature?",
  "options": [
    "Create a fresh instance",
    "Borrow this instance immutably",
    "Move this instance into the method"
  ],
  "answer": 1,
  "explain": "&self is shorthand for self: &Self. Use &mut self for exclusive mutable access or self to consume the instance."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

In `chapters/ch05_structs/src/lib.rs`, implement the `area` and `can_hold`
methods plus the `square` associated function on `Rectangle`, then run:

```bash
cargo test -p ch05_structs
```
