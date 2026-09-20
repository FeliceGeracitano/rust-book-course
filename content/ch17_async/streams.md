# 17.4 Streams: Futures in Sequence

A future produces *one* value eventually. A *stream* produces *many* values over
time — it is the async cousin of an iterator. Where an iterator has `next()
-> Option<Item>`, a stream has `poll_next() -> Poll<Option<Item>>`. The extra
`Poll` layer means each step can say "not ready yet" (`Pending`) before
delivering the next item.

The return values encode the whole lifecycle:

* `Poll::Pending` — no item yet, poll again;
* `Poll::Ready(Some(item))` — here is the next item;
* `Poll::Ready(None)` — the stream is finished.

```rust
fn poll_next(&mut self) -> Poll<Option<u32>> {
    if self.stall { self.stall = false; return Poll::Pending; }
    self.stall = true;
    if self.next < self.end {
        let item = self.next;
        self.next += 1;
        Poll::Ready(Some(item))
    } else {
        Poll::Ready(None)
    }
}
```

Consuming a stream is the streaming analogue of `block_on`: poll in a loop, skip
over `Pending`, collect each `Some`, and stop at `None` — exactly what a
`while let Some(x) = stream.next().await` loop does.

```quiz
{
  "id": "discovery",
  "question": "How does an async stream differ from a future returning a single value?",
  "options": [
    "It yields a sequence of values over time",
    "It can never finish",
    "It always stores every value in memory"
  ],
  "answer": 0,
  "explain": "A stream supports repeatedly waiting for the next item and eventually signals the end of the sequence."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

In `chapters/ch17_async/src/lib.rs`, implement `CountStream::poll_next` (yield
`start..end` with `Pending` stalls between items) and `collect_stream` (drain it
into a `Vec`). Run:

```bash
cargo test -p ch17_async
```
