# 18.1 Characteristics of Object-Oriented Languages

There is no single definition of "object-oriented," but most accounts agree on
a few traits: objects bundle data with the methods that act on it,
encapsulation hides internal state behind a public interface, and inheritance
lets one type reuse another's behavior. How much does Rust qualify?

Rust gives you the first two cleanly. A `struct` or `enum` holds data, `impl`
blocks attach methods, and the `pub` keyword decides what the outside world can
touch. Everything else stays private, so you can change internals freely.

```rust
pub struct AveragedCollection {
    list: Vec<i32>,  // private
    average: f64,    // private
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    fn update_average(&mut self) { /* ... */ }
}
```

Inheritance is the part Rust deliberately omits. There is no way to inherit a
parent struct's fields. Instead Rust reaches the same goals two ways: *traits*
provide shared behavior (with default methods for reuse), and *trait objects*
provide polymorphism. The next sections build on exactly those tools.

### Exercise
Open `chapters/ch18_oop/src/lib.rs` and complete the encapsulated `Button` and
`SelectBox` `draw` methods (Exercise 1). Then run:

```bash
cargo test -p ch18_oop
```
