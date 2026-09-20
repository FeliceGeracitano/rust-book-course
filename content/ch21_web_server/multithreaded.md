# 21.2 From Single-Threaded to Multithreaded Server

A single-threaded server handles one request at a time: a slow request blocks
everything behind it. The fix is a *thread pool* — a fixed set of worker threads
that pull jobs from a shared queue. To know how much work is outstanding, the
pool needs a counter that every worker can update safely.

That counter is shared mutable state, which Rust guards with two types working
together. `Arc<T>` (atomically reference-counted) gives each thread a cheap
handle to the *same* value; `Mutex<T>` ensures only one thread mutates it at a
time. Cloning the `Arc` hands out another handle to one underlying value, so no
increment is ever lost.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let count = Arc::new(Mutex::new(0u64));
let mut handles = Vec::new();
for _ in 0..4 {
    let count = Arc::clone(&count);
    handles.push(thread::spawn(move || {
        *count.lock().unwrap() += 1;
    }));
}
for h in handles { h.join().unwrap(); }
assert_eq!(*count.lock().unwrap(), 4);
```

Joining every handle before reading the count makes the result deterministic —
no sleeps or guesswork required.

```quiz
{
  "id": "discovery",
  "question": "Why use a bounded worker pool instead of spawning unlimited threads per request?",
  "options": [
    "To limit concurrent resource usage",
    "To guarantee every request is instantaneous",
    "To remove the need for synchronization"
  ],
  "answer": 0,
  "explain": "A fixed worker count bounds active execution resources. Queue capacity and overload behavior still need a deliberate design."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Implement the `JobCounter` methods in
`chapters/ch21_web_server/src/lib.rs`, then run:

```bash
cargo test -p ch21_web_server
```
