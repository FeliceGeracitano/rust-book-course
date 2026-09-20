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

```quiz
{
  "id": "discovery",
  "question": "What does an associated type in a trait express?",
  "options": [
    "A value allocated by every method call",
    "An automatically inherited struct field",
    "A type chosen by an implementation and used by its trait contract"
  ],
  "answer": 2,
  "explain": "For example, Iterator::Item names the type yielded by that iterator implementation, avoiding an extra type parameter at each use."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

In `chapters/ch20_advanced_features/src/lib.rs`, implement `Add` for `Point` so
that `+` adds the `x` and `y` fields component-wise. Then run:

```bash
cargo test -p ch20_advanced_features
```
