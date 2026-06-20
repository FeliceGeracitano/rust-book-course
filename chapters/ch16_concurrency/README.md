# Chapter 16 — Fearless Concurrency

> Book: https://doc.rust-lang.org/book/ch16-00-concurrency.html

## Summary
Fearless concurrency — threads, message passing, shared state, and the `Send` /
`Sync` traits.

## You will learn
- Spawn threads
- `mpsc` channels for message passing
- `Mutex<T>` + `Arc<T>` for shared state
- The `Send` and `Sync` marker traits

## Subchapters
- 16.1 Using Threads to Run Code Simultaneously
- 16.2 Transfer Data Between Threads with Message Passing
- 16.3 Shared-State Concurrency
- 16.4 Extensible Concurrency with `Send` and `Sync`

## Exercises
Open `src/lib.rs`, complete each `todo!()`, then make the tests pass:

```bash
cargo test -p ch16_concurrency
```

Stuck? See `SOLUTION.md`.
