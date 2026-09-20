# 6.3 Concise Control Flow with if let and let...else

Sometimes a full `match` is overkill: you only care about *one* pattern and want
to ignore everything else. `if let` is the shorthand. It runs its block only when
the pattern matches, binding the inner data, and an optional `else` covers the
rest:

```rust
let config: Option<u8> = Some(3);

if let Some(max) = config {
    println!("max is {max}");
} else {
    println!("no max set");
}
```

You trade exhaustiveness for brevity, so reach for `if let` when the other cases
genuinely need no handling.

`let...else` flips the shape for the common "extract or bail" case. The happy-path
binding lives in the normal `let`, and the `else` block — which **must diverge**
(`return`, `break`, `continue`, or `panic!`) — handles the miss. Crucially, the
binding stays in scope *after* the statement, keeping the rest of the function flat
instead of nested:

```rust
fn first_char(s: &str) -> char {
    let Some(c) = s.chars().next() else {
        return '?';
    };
    c // `c` is available here, no rightward drift
}
```

```quiz
{
  "id": "discovery",
  "question": "What must the else branch of let...else do?",
  "options": [
    "Continue normally to the next statement",
    "Diverge, for example by returning from the function",
    "Produce a replacement value for the pattern"
  ],
  "answer": 1,
  "explain": "The else branch must not continue into code where the pattern bindings are assumed to exist. return, break in a suitable loop, or panic can diverge."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Complete `describe_quarter` (with `if let`) and `username_or_guest` (with
`let...else`) in `chapters/ch06_enums_pattern_matching/src/lib.rs`, then run:

`cargo test -p ch06_enums_pattern_matching`
