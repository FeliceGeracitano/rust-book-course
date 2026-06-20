# 13.2 Processing a Series of Items with Iterators

An **iterator** produces a sequence of values one at a time. In Rust iterators
are *lazy*: building one does nothing until something asks for values. You drive
an iterator with two kinds of methods. **Adaptors** like `map` and `filter`
transform one iterator into another and stay lazy. **Consumers** like `collect`,
`sum`, and `fold` actually run the iterator and produce a final result.

`map` applies a closure to every item; `filter` keeps only items for which a
closure returns `true`; `collect` gathers the results into a collection such as
a `Vec`. `fold` is the general-purpose consumer: it threads an accumulator
through every item, combining them with a closure.

```rust
let nums = [1, 2, 3, 4, 5];
let evens_doubled: Vec<i32> = nums
    .iter()
    .filter(|n| *n % 2 == 0)
    .map(|n| n * 2)
    .collect();
assert_eq!(evens_doubled, vec![4, 8]);

let sum = nums.iter().fold(0, |acc, n| acc + n);
assert_eq!(sum, 15);
```

Because adaptors are lazy, chains like this allocate nothing until `collect`
runs — the compiler fuses them into a single efficient pass.

### Exercise
Implement `map_collect`, `filter_collect`, and `total_length` in
`chapters/ch13_iterators_closures/src/lib.rs`, then run:

```bash
cargo test -p ch13_iterators_closures
```
