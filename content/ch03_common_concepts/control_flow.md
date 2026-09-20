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

```quiz
{
  "id": "discovery",
  "question": "What is printed?",
  "options": [
    "The loop never stops",
    "6",
    "3"
  ],
  "answer": 1,
  "explain": "A loop can return a value through break. At n == 3, break n * 2 ends the loop and produces 6.",
  "code": "fn main() {\n    let mut n = 0;\n    let result = loop {\n        n += 1;\n        if n == 3 { break n * 2; }\n    };\n    println!(\"{result}\");\n}"
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Complete `fib` (a `while` loop) and `classify` (an `if`/`else if` chain) in
`chapters/ch03_common_concepts/src/lib.rs`, then run:

```bash
cargo test -p ch03_common_concepts
```
