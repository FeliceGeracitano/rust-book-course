# 15.6 Reference Cycles Can Leak Memory

`Rc<T>` frees its value when the strong count reaches zero. But if two
`Rc<RefCell<T>>` values point at each other, each keeps the other's count above
zero forever. Neither is ever freed, and you have leaked memory — a *reference
cycle*. Rust's safety guarantees prevent dangling pointers and data races, but
they do **not** prevent leaks like this.

```rust
use std::rc::Rc;
use std::cell::RefCell;

let a = Rc::new(RefCell::new(Vec::<Rc<RefCell<i32>>>::new()));
let b = Rc::new(RefCell::new(Vec::<Rc<RefCell<i32>>>::new()));
// If a holds b and b holds a, their strong counts never reach 0.
```

The fix is `Weak<T>`. A `Weak` reference, made with `Rc::downgrade`, does *not*
contribute to the strong count, so it never keeps a value alive. You upgrade it
back to an `Rc` with `.upgrade()`, which returns `Option<Rc<T>>` — `None` if the
value was already dropped. The common pattern is parent-child trees: children
hold a *strong* `Rc` to nothing upward, and a *weak* link back to the parent,
breaking the cycle.

### Exercise
You have already used `Rc::strong_count` in the `Counter` exercise. Review it
and confirm that dropping every handle returns the count to its starting point —
that is exactly how a leak would show up as a count that never falls. Run:

```bash
cargo test -p ch15_smart_pointers
```
