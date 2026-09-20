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

```trace
{
  "title": "Lazy transformation, eager consumption",
  "code": "fn main() {\n    let values = [1, 2, 3];\n    let doubled = values.iter().map(|n| n * 2);\n    let total: i32 = doubled.sum();\n    println!(\"{total}\");\n}",
  "steps": [
    {
      "line": 2,
      "note": "The array holds three integers.",
      "state": {
        "values": "[1, 2, 3]"
      },
      "output": ""
    },
    {
      "line": 3,
      "note": "Creating map sets up the transformation. No elements have been consumed yet.",
      "state": {
        "values": "[1, 2, 3]",
        "doubled": "lazy iterator; no results yet"
      },
      "output": ""
    },
    {
      "line": 4,
      "note": "sum consumes the iterator: map produces 2, 4, and 6, and sum accumulates 12.",
      "state": {
        "doubled": "consumed",
        "total": "12"
      },
      "output": ""
    },
    {
      "line": 5,
      "note": "The final total is printed.",
      "state": {
        "total": "12"
      },
      "output": "12\n"
    }
  ]
}
```

```quiz
{
  "id": "discovery",
  "question": "What is printed?",
  "options": [
    "12",
    "6",
    "The iterator cannot be used without a loop"
  ],
  "answer": 0,
  "explain": "map produces doubled items lazily. sum consumes the iterator and adds 2 + 4 + 6.",
  "code": "fn main() {\n    let total: i32 = [1, 2, 3].iter().map(|n| n * 2).sum();\n    println!(\"{total}\");\n}"
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Implement `map_collect`, `filter_collect`, and `total_length` in
`chapters/ch13_iterators_closures/src/lib.rs`, then run:

```bash
cargo test -p ch13_iterators_closures
```
