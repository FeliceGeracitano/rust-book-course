# Chapter 15 — Smart Pointers

> Book: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html

## Summary
Heap allocation and ownership patterns via smart pointers — `Box`, `Deref`,
`Drop`, `Rc`, `RefCell`, and reference cycles.

## You will learn
- `Box<T>` for heap data
- The `Deref` and `Drop` traits
- Shared ownership with `Rc<T>`
- Interior mutability with `RefCell<T>`
- How reference cycles leak memory

## Subchapters
- 15.1 Using `Box<T>` to Point to Data on the Heap
- 15.2 Treating Smart Pointers Like Regular References (`Deref`)
- 15.3 Running Code on Cleanup with the `Drop` Trait
- 15.4 `Rc<T>`, the Reference Counted Smart Pointer
- 15.5 `RefCell<T>` and the Interior Mutability Pattern
- 15.6 Reference Cycles Can Leak Memory

## Exercises
Open `src/lib.rs`, complete each `todo!()`, then make the tests pass:

```bash
cargo test -p ch15_smart_pointers
```

Stuck? See `SOLUTION.md`.
