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

```quiz
{
  "id": "discovery",
  "question": "Why does a generic function that compares T values need a trait bound?",
  "options": [
    "Generic functions run without type checking",
    "The compiler needs to know T supports the comparison",
    "Every generic type supports all operators"
  ],
  "answer": 1,
  "explain": "A bound such as T: PartialOrd states the operations the implementation requires. Rust checks generic code against those stated capabilities."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Implement `largest` and the `Pair<T>` methods in
`chapters/ch10_generics_traits_lifetimes/src/lib.rs`, then run:

```bash
cargo test -p ch10_generics_traits_lifetimes
```
