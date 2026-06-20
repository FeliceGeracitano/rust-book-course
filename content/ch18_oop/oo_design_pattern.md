# 18.3 Implementing an Object-Oriented Design Pattern

The *state pattern* models an object that behaves differently depending on its
internal state, while keeping that logic out of the object itself. A blog post
is the classic example: it moves through draft → pending review → published,
and each stage decides what operations mean. Rather than scatter `match`
statements everywhere, we give each state its own type and let the post
delegate to whichever state it currently holds.

```rust
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}
```

Each transition method takes `self: Box<Self>`, consuming the old state and
returning the next one, so an invalid state can never linger. The `Post`'s
`request_review` and `approve` just `take()` the current state and replace it
with the result. Content stays hidden until a `Published` state chooses to
reveal it; a `Draft` returns an empty string. Adding a new stage means adding a
type, not editing every method — the win the pattern is built for.

### Exercise
In `chapters/ch18_oop/src/lib.rs`, complete `Post` and the `Draft`,
`PendingReview`, and `Published` states (Exercise 3). Then run:

```bash
cargo test -p ch18_oop
```
