# 3.2 Data Types

Rust is statically typed: every value has a type known at compile time. The
compiler infers most of them, but sometimes you annotate explicitly.

**Scalar types** hold a single value:
- integers (`i32`, `u32`, `u64`, `usize`, ...),
- floating point (`f32`, `f64`),
- booleans (`bool`),
- characters (`char`, a Unicode scalar).

```rust
let temperature: f64 = 98.6;
let is_warm: bool = temperature > 70.0;
let grade: char = 'A';
```

**Compound types** group several values. A **tuple** packs values of possibly
different types and can be returned to hand back more than one result. An
**array** holds a fixed number of values of the *same* type, with its length
baked into the type:

```rust
let point: (i32, i32) = (3, 7);
let (x, y) = point;          // destructuring

let weekdays: [i32; 5] = [1, 2, 3, 4, 5];
let first = weekdays[0];     // indexing
```

Tuples are great for returning a `(min, max)` pair; arrays are great when the
count is fixed and known up front.

```quiz
{
  "id": "discovery",
  "question": "What type describes a tuple containing an integer and a boolean?",
  "options": [
    "Vec<bool>",
    "(i32, bool)",
    "[i32; 2]"
  ],
  "answer": 1,
  "explain": "A tuple groups values that can have different types. Arrays and vectors hold elements of one type."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Complete `fahrenheit_to_celsius` and `min_and_max` in
`chapters/ch03_common_concepts/src/lib.rs`, then run:

```bash
cargo test -p ch03_common_concepts
```
