# 17.5 A Closer Look at the Traits for Async

Now that you have built futures, executors, and streams by hand, the real
standard-library traits should feel familiar. `std::future::Future` looks almost
exactly like our `SimpleFuture`:

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Two pieces we left out:

* **`Context` and `Waker`.** A real executor should not busy-spin. When a future
  returns `Pending`, it stashes the `Waker` from the `Context` and calls
  `wake()` once it can make progress, so the executor can sleep in between
  instead of polling in a tight loop.
* **`Pin`.** Async state machines can hold references into their own data, so
  they must not be moved in memory once polling starts. `Pin` is the type-level
  promise that the future stays put.

`Stream` mirrors `Future`: it adds `poll_next` returning `Poll<Option<Item>>`,
which is exactly the signature you implemented. The `async`/`await` keywords are
sugar — the compiler generates a `Future` whose `poll` resumes the function at
the last `.await` point, just like our hand-written state machines.

### Exercise
There is no new code here. Re-read your `SimpleFuture` and `SimpleStream` impls
in `chapters/ch17_async/src/lib.rs` and note how each maps onto `Future` and
`Stream`. Then confirm everything still passes:

```bash
cargo test -p ch17_async
```
