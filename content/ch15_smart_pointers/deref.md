# 15.2 Treating Smart Pointers Like Regular References

The `Deref` trait is what lets `*pointer` reach the value a smart pointer wraps.
When you implement `Deref`, you teach the `*` operator how to turn your type
into a reference to its inner value, so your type behaves like a plain
reference.

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

let b = MyBox(5);
assert_eq!(*b, 5); // really *(b.deref())
```

`Deref` also unlocks *deref coercion*: when you pass `&SomeType` to a function
expecting `&Target`, the compiler inserts `deref` calls for you. That is why a
`&MyBox<String>` can be handed to a function taking `&str` — the compiler
chains `MyBox<String>` -> `String` -> `str`. This keeps APIs flexible without
manual conversions.

### Exercise
In `chapters/ch15_smart_pointers/src/lib.rs`, finish `MyBox::new`, the `deref`
method, and the `hello` function so deref coercion lets a `&MyBox<String>` be
greeted as a `&str`. Then run:

```bash
cargo test -p ch15_smart_pointers
```
