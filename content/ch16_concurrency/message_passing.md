# 16.2 Transfer Data Between Threads with Message Passing

One safe way to share work is to *not* share memory at all: send messages
instead. The standard library's `mpsc` channel — *multiple producer, single
consumer* — is built for exactly this. `mpsc::channel()` returns a transmitter
(`tx`) and a receiver (`rx`).

Each `tx.send(value)` hands a value down the channel, transferring ownership to
the receiving end. The consumer reads values with `rx.recv()` (which blocks for
one message) or by iterating `rx`. The iterator ends automatically once *every*
sender has been dropped, which is how the consumer knows the work is finished.

To get multiple producers, clone the transmitter — one clone per thread.
Remember to drop the original `tx` after cloning, or the receiver loop will
wait forever for a sender that never sends.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();
for i in 1..=3 {
    let tx = tx.clone();
    thread::spawn(move || tx.send(i).unwrap());
}
drop(tx); // no more senders after the clones finish

let total: i32 = rx.iter().sum(); // 6, in any arrival order
assert_eq!(total, 6);
```

### Exercise
Complete `channel_sum` in
`chapters/ch16_concurrency/src/lib.rs`, then run:

```bash
cargo test -p ch16_concurrency
```
