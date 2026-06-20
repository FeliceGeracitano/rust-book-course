# 20.3 Advanced Types

The **newtype pattern** wraps an existing type in a one-field tuple struct to
give it a fresh identity. This buys you type safety (the compiler will not let
you pass `Meters` where `Seconds` is expected), lets you attach your own methods
and trait impls, and can hide a type's internals behind a tidy public API — all
at zero runtime cost, since the wrapper compiles away to the inner value.

```rust
struct Meters(f64);

impl Meters {
    fn new(v: f64) -> Meters { Meters(v) }
    fn value(&self) -> f64 { self.0 }
}

let d = Meters::new(5.0);
assert_eq!(d.value(), 5.0);
```

Contrast this with a **type alias** (`type Kilometers = i32;`), which is just a
new *name* for the same type — interchangeable everywhere, with no extra safety.
The newtype is a genuinely distinct type. This chapter also touches the
never type `!` and dynamically sized types like `str`, but the newtype is the
workhorse you will reach for most often.

### Exercise

In `chapters/ch20_advanced_features/src/lib.rs`, finish the `Meters` newtype:
`new`, `value`, and its `Add` impl so two `Meters` add while keeping their unit.
Then run:

```bash
cargo test -p ch20_advanced_features
```
