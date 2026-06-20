# 15.3 Running Code on Cleanup with the Drop Trait

The `Drop` trait lets you run code right before a value is freed — closing a
file, releasing a lock, or, here, recording that cleanup happened. You
implement a single method, `drop`, and Rust calls it automatically when the
value goes out of scope.

```rust
struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

let _a = Guard("a");
let _b = Guard("b");
// prints "dropping b" then "dropping a"
```

Two rules matter. First, values are dropped in *reverse* order of declaration —
the last one created is the first cleaned up. Second, you may **not** call
`.drop()` yourself; Rust forbids it to prevent double frees. To release a value
early, hand it to `std::mem::drop` (`drop(value)`), which takes ownership and
lets the destructor run immediately.

### Exercise
In `chapters/ch15_smart_pointers/src/lib.rs`, implement `Tracker::new`, its
`Drop` impl (push the label to the log), and `drop_now`. The tests assert the
reverse-order cleanup sequence. Then run:

```bash
cargo test -p ch15_smart_pointers
```
