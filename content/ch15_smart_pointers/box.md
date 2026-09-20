# 15.1 Using Box<T> to Point to Data on the Heap

A *smart pointer* is a type that acts like a reference but carries extra
capabilities. The simplest is `Box<T>`: it stores its value on the heap and
keeps only a small pointer on the stack. There is no runtime overhead beyond
that indirection.

Why bother? The headline use case is *recursive types*. Consider a list where
each node owns the rest of the list. Written inline, the compiler cannot
compute a size — the type would nest forever. A `Box<T>` has a fixed,
pointer-sized footprint, so it breaks the cycle and gives the recursive type a
known size.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
```

The `Box` owns its contents: when the `Box` goes out of scope, both the pointer
and the heap data are freed. You dereference it with `*` just like a reference.

```quiz
{
  "id": "discovery",
  "question": "Why can Box make a recursive enum have a known size?",
  "options": [
    "The compiler reserves infinite stack space",
    "Box itself has a fixed size even when it points to another node",
    "Box removes recursion from the program"
  ],
  "answer": 1,
  "explain": "Putting the recursive part behind indirection breaks the infinite-size layout. The pointed-to data lives separately from the box value."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

In `chapters/ch15_smart_pointers/src/lib.rs`, complete `List::sum` so it walks
the cons list and adds every value (returning `0` for `Nil`). Then run:

```bash
cargo test -p ch15_smart_pointers
```
