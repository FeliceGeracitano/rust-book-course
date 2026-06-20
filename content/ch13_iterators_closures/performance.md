# 13.4 Performance: Loops vs. Iterators

Iterators look higher-level than hand-written loops, so it is natural to worry
they cost more at runtime. In Rust they do not. Iterators are a **zero-cost
abstraction**: the compiler turns an adaptor chain into essentially the same
machine code you would write by hand with a `for` loop, often even faster
because the bounds checks and indices vanish.

This happens because adaptors are generic and inlined. When you call `.map(...)`
then `.filter(...)` then `.sum()`, the optimizer fuses the closures into one
tight loop, unrolls it where profitable, and keeps everything in registers. The
abstraction exists only at compile time.

```rust
// These two compile to near-identical, equally fast code.
let data = [1, 2, 3, 4, 5];

let mut total = 0;
for &n in &data {
    total += n;
}

let total_iter: i32 = data.iter().sum();
assert_eq!(total, total_iter);
```

The practical takeaway: prefer the version that reads most clearly. Iterator
chains express intent directly and resist the small bookkeeping mistakes that
manual loops invite, with no speed penalty to pay for that clarity.

### Exercise
There is no new function for this section — it ties together the work you did in
13.1–13.3. Make sure everything still passes:

```bash
cargo test -p ch13_iterators_closures
```
