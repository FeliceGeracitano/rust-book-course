# 20.5 Macros

Macros write code for you at compile time. **Declarative macros**, defined with
`macro_rules!`, work by pattern matching: you give one or more *rules* that match
the macro's input and expand into Rust source. They are how `vec!` and `println!`
accept any number of arguments — something a regular function cannot do.

The key syntax is the *repetition*: `$( ... ),*` matches a comma-separated list,
binding a *metavariable* like `$x:expr` each time, and the matching `$( ... )*` in
the body emits the expansion once per match.

```rust
macro_rules! my_vec {
    ( $( $x:expr ),* $(,)? ) => {{
        let mut v = Vec::new();
        $( v.push($x); )*
        v
    }};
}

let v = my_vec![1, 2, 3];
assert_eq!(v, vec![1, 2, 3]);
```

The `$(,)?` allows an optional trailing comma. Rust also has **procedural
macros** (custom `derive`, attribute, and function-like macros) that operate on a
token stream in their own crate — more powerful, but heavier to set up than the
declarative macros you write here.

```quiz
{
  "id": "discovery",
  "question": "What is a key difference between a declarative macro and an ordinary function?",
  "options": [
    "A macro skips all type checking of generated Rust",
    "A macro always runs in a separate process",
    "A macro expands syntax before ordinary execution"
  ],
  "answer": 2,
  "explain": "macro_rules! matches token patterns and expands them into Rust syntax. The resulting code is still subject to the compiler’s normal checks."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

In `chapters/ch20_advanced_features/src/lib.rs`, complete the `string_vec!`
macro so it builds a `Vec<String>` by calling `.to_string()` on each input.
Then run:

```bash
cargo test -p ch20_advanced_features
```
