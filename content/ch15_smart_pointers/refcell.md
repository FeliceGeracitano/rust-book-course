# 15.5 RefCell<T> and the Interior Mutability Pattern

Rust's borrow checker normally enforces, at compile time, that you cannot mutate
data through a shared (`&`) reference. `RefCell<T>` relaxes this with *interior
mutability*: it lets you mutate the value inside even when the `RefCell` itself
is only shared. The borrowing rules still hold — they are just checked at
*runtime* instead of compile time.

You ask for access with `borrow()` (shared) or `borrow_mut()` (exclusive). If
you violate the rules — say, two mutable borrows at once — the program panics
rather than failing to compile.

```rust
use std::cell::RefCell;

let cell = RefCell::new(0);
*cell.borrow_mut() += 1;
assert_eq!(*cell.borrow(), 1);
```

The classic combination is `Rc<RefCell<T>>`: `Rc` gives many owners, and
`RefCell` lets any of them mutate the shared value. Because every clone points
at the same cell, a change made through one handle is visible through all of
them.

### Exercise
In `chapters/ch15_smart_pointers/src/lib.rs`, implement `Counter::increment` and
`Counter::get` using `borrow_mut()` and `borrow()`. Confirm that two clones
share one underlying count. Then run:

```bash
cargo test -p ch15_smart_pointers
```
