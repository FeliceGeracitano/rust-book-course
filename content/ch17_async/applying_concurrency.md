# 17.2 Applying Concurrency with Async

A future does nothing on its own; something has to *drive* it. That something is
an *executor*. The simplest possible executor is a loop that polls one future
until it stops returning `Pending` — the book calls this `block_on`, and it is
how you cross from synchronous `main` into async land.

```rust
fn block_on<F: SimpleFuture>(mut future: F) -> F::Output {
    loop {
        match future.poll() {
            Poll::Ready(value) => return value,
            Poll::Pending => continue,
        }
    }
}
```

A real executor parks the thread between polls and relies on a `Waker` to wake
it back up, so it is not a busy spin. But the shape is identical: poll, and if
the answer is `Pending`, try again. The power of async appears when *one*
executor drives *many* futures, interleaving their progress on a single thread.
That is concurrency without extra threads: each `poll` is a cooperative chance
for a task to advance a little and then yield control back.

### Exercise
In `chapters/ch17_async/src/lib.rs`, implement `block_on` so it polls the future
in a loop and returns the value once it is `Ready`. Run:

```bash
cargo test -p ch17_async
```
