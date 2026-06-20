# 16.1 Using Threads to Run Code Simultaneously

A thread runs a slice of your program independently of the rest. Rust's
standard library gives each spawned thread its own stack and lets the operating
system schedule them, so work can overlap on multiple cores.

You start a thread with `thread::spawn`, passing a closure. Spawning returns a
`JoinHandle`. Calling `.join()` on that handle blocks until the thread finishes
and hands back whatever value its closure returned. Joining is also what makes
results predictable: nothing is guaranteed about *when* a thread runs, but once
you have joined it, its work is definitely done.

Closures usually need the `move` keyword so the thread *owns* the data it
captures. That ownership transfer is the compiler's way of guaranteeing the
data outlives the thread.

```rust
use std::thread;

let handle = thread::spawn(move || {
    let n = 21;
    n * 2
});

let result = handle.join().unwrap(); // waits, then yields 42
assert_eq!(result, 42);
```

Spawn many handles, collect them in a `Vec`, then join them in order to gather
deterministic results.

### Exercise
Complete `parallel_squares` in
`chapters/ch16_concurrency/src/lib.rs`, then run:

```bash
cargo test -p ch16_concurrency
```
