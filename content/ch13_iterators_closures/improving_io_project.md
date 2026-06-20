# 13.3 Improving Our I/O Project

Chapter 12 built the `minigrep` tool with explicit `for` loops and a mutable
`results` vector. Now that we have iterators, we can rewrite the searching logic
in a more declarative style: describe *what* we want rather than *how* to
accumulate it.

The original case-insensitive search looped over every line, lowercased it,
checked for a match, and pushed survivors into a growing `Vec`. With iterators
that whole block collapses into a single chain of `lines()`, `filter`, and
`collect`. The intermediate mutable state disappears, which removes a class of
off-by-one and forgotten-`push` bugs.

```rust
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

let text = "Rust\nTrust\nOther";
assert_eq!(search("rust", text), vec!["Rust", "Trust"]);
```

Note the borrow: the returned `&str` slices point into `contents`, so the
lifetime `'a` ties the output to the input. We test the *pure* function here —
no files, args, or terminal — exactly the part worth unit-testing.

### Exercise
Implement `search_insensitive` in
`chapters/ch13_iterators_closures/src/lib.rs`, then run:

```bash
cargo test -p ch13_iterators_closures
```
