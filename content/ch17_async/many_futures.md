# 17.3 Working With Any Number of Futures

Often you have several futures and want them to make progress *together*. Two
patterns dominate. `join` runs futures concurrently and waits for **all** of
them, giving you every result. `race` (sometimes called `select`) waits for the
**first** to finish and discards the rest.

Building `join` by hand shows how cheap this concurrency is. On each poll we
nudge every sub-future that is not finished and cache its result. We only report
`Ready` once every result is in hand.

```rust
fn poll(&mut self) -> Poll<(A::Output, B::Output)> {
    if self.a_done.is_none() {
        if let Poll::Ready(v) = self.a.poll() { self.a_done = Some(v); }
    }
    if self.b_done.is_none() {
        if let Poll::Ready(v) = self.b.poll() { self.b_done = Some(v); }
    }
    if self.a_done.is_some() && self.b_done.is_some() {
        Poll::Ready((self.a_done.take().unwrap(), self.b_done.take().unwrap()))
    } else { Poll::Pending }
}
```

`race` is the mirror image: return the first sub-future that is `Ready`. Polling
order makes ties deterministic — poll `a` before `b`, and `a` wins a tie.

### Exercise
In `chapters/ch17_async/src/lib.rs`, implement `Join` (wait for both) and `Race`
(first to finish wins, tagged `Left`/`Right`). Run:

```bash
cargo test -p ch17_async
```
