# 17.1 Futures and the Async Syntax

A *future* is a value that represents work that is not finished yet. In real
Rust you write `async fn` and `.await`, and the compiler turns each function
into a hidden state machine implementing the `Future` trait. The heart of that
trait is one method, `poll`, which an executor calls over and over. Each call
returns either `Pending` ("not done, ask me again") or `Ready(value)` ("here is
the result").

Because this crate has no async runtime, we build that machinery by hand. A
future is just a struct with a `poll` method, and its internal fields *are* the
state machine.

```rust
enum Poll<T> { Pending, Ready(T) }

trait SimpleFuture {
    type Output;
    fn poll(&mut self) -> Poll<Self::Output>;
}

struct Countdown { remaining: u32 }

impl SimpleFuture for Countdown {
    type Output = &'static str;
    fn poll(&mut self) -> Poll<&'static str> {
        if self.remaining == 0 { Poll::Ready("done") }
        else { self.remaining -= 1; Poll::Pending }
    }
}
```

Nothing runs until something polls the future — futures are *lazy*.

### Exercise
In `chapters/ch17_async/src/lib.rs`, implement `Countdown::new` and its `poll`
so it returns `Pending` until the counter hits zero, then `Ready(label)`. Run:

```bash
cargo test -p ch17_async
```
