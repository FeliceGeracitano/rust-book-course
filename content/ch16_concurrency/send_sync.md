# 16.4 Extensible Concurrency with Send and Sync

Almost everything about Rust's concurrency lives in the library, not the
language. The two pieces baked into the language itself are the marker traits
`Send` and `Sync`. They carry no methods; they exist purely so the compiler can
reason about which types may cross thread boundaries.

A type is `Send` if it is safe to **move** to another thread. A type is `Sync`
if it is safe to **share by reference** across threads — formally, `T` is `Sync`
when `&T` is `Send`. Nearly every primitive and composite type is both, and any
type built entirely from `Send`/`Sync` parts inherits the traits automatically.

The interesting cases are the exceptions. `Rc<T>` is neither `Send` nor `Sync`,
because its reference count is not atomic — sharing it across threads could
corrupt the count. `Arc<T>` uses atomic counting, so it *is* both. This is
exactly why thread-spawning APIs require `Send`: the compiler rejects unsafe
sharing before your program ever runs.

```rust
fn assert_send<T: Send>(_: T) {}

assert_send(42u64);                 // ok: integers are Send
// assert_send(std::rc::Rc::new(1)); // compile error: Rc is not Send
```

### Exercise
Complete `assert_send` and `assert_sync` in
`chapters/ch16_concurrency/src/lib.rs`, then run:

```bash
cargo test -p ch16_concurrency
```
