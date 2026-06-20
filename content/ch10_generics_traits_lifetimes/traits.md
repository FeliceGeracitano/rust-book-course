# 10.2 Defining Shared Behavior with Traits

A **trait** describes behavior shared across types — a set of method signatures a
type can promise to provide. It's Rust's answer to interfaces. You declare the
required methods, and each type spells out its own behavior in an `impl Trait
for Type` block.

Traits can also supply **default implementations**. A default method can call
other methods of the same trait, so an implementor only has to fill in a small
required piece to unlock richer behavior for free. Any type may override a
default with its own version.

Once a trait exists, you can use it as a **bound** to accept "any type that
behaves this way." The `impl Trait` syntax in a parameter position —
`fn notify(item: &impl Summary)` — is shorthand for a generic with a trait
bound, and it keeps signatures readable.

```rust
trait Greet {
    fn name(&self) -> String;
    fn hello(&self) -> String {
        format!("Hello, {}!", self.name())
    }
}

struct Dog;
impl Greet for Dog {
    fn name(&self) -> String { String::from("Rex") }
}

assert_eq!(Dog.hello(), "Hello, Rex!");
```

### Exercise
Define the `Summary` trait's default method and the `Tweet`/`Article` impls in
`chapters/ch10_generics_traits_lifetimes/src/lib.rs`, then run:

```bash
cargo test -p ch10_generics_traits_lifetimes
```
