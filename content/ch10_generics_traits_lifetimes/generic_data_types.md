# 10.1 Generic Data Types

Generics let you write one definition that works for *many* concrete types
instead of copy-pasting near-identical code. You introduce a type parameter —
conventionally `T` — in angle brackets, then use it like a real type inside the
item.

A generic function names its parameter right after the function name. A generic
struct or enum names it after the type name and reuses it in the fields. Methods
on a generic type repeat the parameter on the `impl` block.

Generics are zero-cost: at compile time Rust *monomorphizes* your code,
generating a specialized copy for each concrete type actually used. There is no
runtime dispatch and no boxing.

To *do* anything with a `T` (compare it, add it, copy it) you must constrain it
with trait bounds, like `T: PartialOrd`. Without a bound, the compiler only
knows the value exists — it can't assume any behavior.

```rust
fn first<T: Copy>(items: &[T]) -> Option<T> {
    items.first().copied()
}

struct Wrapper<T> {
    value: T,
}

assert_eq!(first(&[10, 20, 30]), Some(10));
assert_eq!(first(&['a', 'b']), Some('a'));
```

### Exercise
Implement `largest` and the `Pair<T>` methods in
`chapters/ch10_generics_traits_lifetimes/src/lib.rs`, then run:

```bash
cargo test -p ch10_generics_traits_lifetimes
```
