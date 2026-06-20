# 3.5 Control Flow

Control flow decides which code runs. In Rust, `if` is an **expression**: its
condition must be a `bool` (there is no implicit truthiness), and every branch
can produce a value.

```rust
let label = if n % 2 == 0 { "even" } else { "odd" };
```

Chain conditions with `else if` to pick among several cases. For repetition, Rust
offers three loops:

- `loop` runs forever until you `break` (and `break value` can return a value);
- `while` repeats while a condition stays true;
- `for` iterates over a collection or range, which is the safest and most common.

```rust
let mut count = 0;
while count < 3 {
    count += 1;
}

let nums = [10, 20, 30];
for n in &nums {
    println!("{n}");
}

for i in 1..=5 {   // inclusive range 1,2,3,4,5
    println!("{i}");
}
```

Reach for `for` by default; use `while` when the stopping condition is dynamic,
and `loop` when you need to break with a computed value.

### Exercise
Complete `fib` (a `while` loop) and `classify` (an `if`/`else if` chain) in
`chapters/ch03_common_concepts/src/lib.rs`, then run:

```bash
cargo test -p ch03_common_concepts
```
