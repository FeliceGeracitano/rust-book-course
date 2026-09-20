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

```trace
{
  "title": "Watch shared ownership change",
  "code": "fn main() {\n    let a = std::rc::Rc::new(String::from(\"hi\"));\n    let b = std::rc::Rc::clone(&a);\n    drop(b);\n    println!(\"{}\", std::rc::Rc::strong_count(&a));\n}",
  "steps": [
    {
      "line": 2,
      "note": "a is the first strong owner of the allocation.",
      "state": {
        "a": "shared owner",
        "strong count": "1"
      },
      "output": ""
    },
    {
      "line": 3,
      "note": "Cloning the Rc adds an owner of the same allocation.",
      "state": {
        "a": "shared owner",
        "b": "shared owner",
        "strong count": "2"
      },
      "output": ""
    },
    {
      "line": 4,
      "note": "Dropping b removes one owner. The String stays alive because a still owns it.",
      "state": {
        "a": "shared owner",
        "b": "dropped",
        "strong count": "1"
      },
      "output": ""
    },
    {
      "line": 5,
      "note": "The remaining strong count is one.",
      "state": {
        "a": "shared owner",
        "strong count": "1"
      },
      "output": "1\n"
    },
    {
      "line": 6,
      "note": "The final owner leaves scope, so the String is dropped.",
      "state": {
        "a": "dropped",
        "strong count": "0; value freed"
      },
      "output": "1\n"
    }
  ]
}
```

```quiz
{
  "id": "discovery",
  "question": "What is the strong count after cloning this Rc?",
  "options": [
    "0",
    "2",
    "1"
  ],
  "answer": 1,
  "explain": "Rc::clone creates another shared owner of the same allocation. It increments the reference count rather than cloning the String itself.",
  "code": "fn main() {\n    let a = std::rc::Rc::new(String::from(\"hi\"));\n    let b = std::rc::Rc::clone(&a);\n    println!(\"{}\", std::rc::Rc::strong_count(&b));\n}"
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

In `chapters/ch15_smart_pointers/src/lib.rs`, build the `Counter` type on
`Rc<RefCell<i32>>` and implement `handles` using `Rc::strong_count`. The tests
check that the count rises and falls as clones are created and dropped. Then
run:

```bash
cargo test -p ch15_smart_pointers
```
