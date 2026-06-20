# 20.2 Advanced Traits

Traits carry more than methods. An **associated type** is a placeholder type a
trait declares and each implementor fills in — `Iterator::Item` is the classic
example. Associated types appear all over the standard library, including in the
operator traits.

**Operator overloading** means implementing one of the traits in `std::ops`, such
as `Add`, so your type works with `+`, `*`, and friends. `Add` has an associated
type `Output` that names what `+` produces, letting you decide the result type.

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Millimeters(u32);

impl Add for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Millimeters) -> Millimeters {
        Millimeters(self.0 + other.0)
    }
}

assert_eq!(Millimeters(2) + Millimeters(3), Millimeters(5));
```

Because `Output` is associated rather than generic, the compiler knows there is
exactly one result type per `Add` impl, which keeps inference clean and error
messages clear.

### Exercise

In `chapters/ch20_advanced_features/src/lib.rs`, implement `Add` for `Point` so
that `+` adds the `x` and `y` fields component-wise. Then run:

```bash
cargo test -p ch20_advanced_features
```
