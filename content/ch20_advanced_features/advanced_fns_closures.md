# 20.4 Advanced Functions and Closures

You can pass a plain function where a closure is expected by using a **function
pointer**, written with the lowercase type `fn`. Because `fn` implements all
three closure traits (`Fn`, `FnMut`, `FnOnce`), any API taking a closure also
accepts a function pointer directly.

```rust
fn double(x: i32) -> i32 { x * 2 }

let nums = vec![1, 2, 3];
// Pass the function by name instead of a closure:
let doubled: Vec<i32> = nums.iter().map(|&x| double(x)).collect();
assert_eq!(doubled, vec![2, 4, 6]);
```

A tuple-struct or enum variant name is also a constructor function, so you can
write `.map(Wrapper)` to wrap each element.

**Returning closures** needs care: each closure has a unique, unnameable type, so
you return it behind a pointer — `-> impl Fn(i32) -> i32` when one concrete type
suffices, or `-> Box<dyn Fn(i32) -> i32>` when the returned type may vary at
runtime. The `Box<dyn ...>` form gives a fixed size the compiler is happy with.

### Exercise

This subchapter has no separate code task — the function-pointer and
closure-return ideas are reinforced by the other exercises. Keep working through
`chapters/ch20_advanced_features/src/lib.rs` and run:

```bash
cargo test -p ch20_advanced_features
```
