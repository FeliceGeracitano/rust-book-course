# 16.3 Shared-State Concurrency

Sometimes several threads really do need to touch the *same* value. For that,
Rust pairs two types. A `Mutex<T>` ("mutual exclusion") wraps data and hands out
access through a lock: call `.lock()`, get a guard, and only the holder of that
guard may read or write. The lock is released automatically when the guard goes
out of scope, so you can't forget to unlock.

But a `Mutex<T>` still needs multiple *owners* to be useful across threads.
`Rc<T>` can't cross thread boundaries, so we reach for `Arc<T>` — an *atomically*
reference-counted pointer that is safe to share. Cloning an `Arc` is cheap and
gives every thread a handle to the same underlying `Mutex`.

The combination `Arc<Mutex<T>>` is the canonical shared, mutable state. Because
every increment takes the lock, there are no lost updates: spawn N threads that
each bump a counter, join them all, and the total is exact.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];
for _ in 0..10 {
    let counter = Arc::clone(&counter);
    handles.push(thread::spawn(move || *counter.lock().unwrap() += 1));
}
for h in handles { h.join().unwrap(); }
assert_eq!(*counter.lock().unwrap(), 10);
```

### Exercise
Complete `SharedCounter` and `concurrent_increments` in
`chapters/ch16_concurrency/src/lib.rs`, then run:

```bash
cargo test -p ch16_concurrency
```
