# 15.4 Rc<T>, the Reference Counted Smart Pointer

Ownership in Rust is usually singular: one value, one owner. But sometimes a
value genuinely has *multiple* owners — think a node in a graph shared by
several edges. `Rc<T>` (reference counted) enables this for single-threaded
programs. It keeps a count of how many owners exist and frees the value only
when the last one goes away.

```rust
use std::rc::Rc;

let a = Rc::new(5);
let b = Rc::clone(&a); // shares ownership, count is now 2
println!("{}", Rc::strong_count(&a)); // 2
```

`Rc::clone` does *not* deep-copy the data; it just bumps the reference count and
hands back another pointer to the same value. That is cheap. Use
`Rc::strong_count` to inspect how many handles are live. As each clone is
dropped, the count drops; at zero the heap data is released.

Note `Rc<T>` only hands out *shared, immutable* references. To mutate shared
data you combine it with `RefCell<T>`, covered next.

### Exercise
In `chapters/ch15_smart_pointers/src/lib.rs`, build the `Counter` type on
`Rc<RefCell<i32>>` and implement `handles` using `Rc::strong_count`. The tests
check that the count rises and falls as clones are created and dropped. Then
run:

```bash
cargo test -p ch15_smart_pointers
```
