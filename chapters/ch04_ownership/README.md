# Chapter 4 — Understanding Ownership

> Book: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

## Summary
Rust's defining feature — ownership, moves, borrowing, and slices — the rules
that guarantee memory safety without a garbage collector.

## You will learn
- Move vs. copy semantics
- How the borrow checker works
- Shared (`&`) and mutable (`&mut`) references
- Slice types

## Subchapters
- 4.1 What is Ownership?
- 4.2 References and Borrowing
- 4.3 The Slice Type

## Exercises
Open `src/lib.rs`, complete each `todo!()`, then make the tests pass:

```bash
cargo test -p ch04_ownership
```

Stuck? See `SOLUTION.md`.
