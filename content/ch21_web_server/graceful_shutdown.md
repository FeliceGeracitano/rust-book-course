# 21.3 Graceful Shutdown and Cleanup

A server that stops mid-request leaves clients hanging. *Graceful* shutdown
means: stop accepting new work, let in-flight jobs finish, then exit. The Book
implements this with the `Drop` trait, signalling workers and joining their
threads when the pool goes out of scope.

Underneath the threads, the decision a worker makes for each request is pure: it
matches the method and path and produces a status, reason, and body. Keeping
that *routing* separate from any I/O makes it trivial to test, and it's also
where you decide which requests are cheap and which (like a `/sleep` endpoint)
are the slow ones that motivated the thread pool in the first place.

```rust
fn route(method: &str, target: &str) -> (u16, &'static str) {
    if method != "GET" {
        return (405, "METHOD NOT ALLOWED");
    }
    match target {
        "/" => (200, "OK"),
        _ => (404, "NOT FOUND"),
    }
}

assert_eq!(route("GET", "/"), (200, "OK"));
assert_eq!(route("POST", "/"), (405, "METHOD NOT ALLOWED"));
```

Returning a plain tuple — instead of writing bytes to a socket — lets you assert
on the exact decision and pair it with `build_response` to produce the reply.

### Exercise

Implement `route` in `chapters/ch21_web_server/src/lib.rs`, then run:

```bash
cargo test -p ch21_web_server
```
