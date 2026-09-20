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

```quiz
{
  "id": "discovery",
  "question": "Why can text.lines().filter(...) replace a manual search loop?",
  "options": [
    "It automatically spawns one thread per line",
    "It always allocates a String for each line",
    "It describes a sequence of borrowed lines and retains only matching ones"
  ],
  "answer": 2,
  "explain": "lines yields borrowed string slices. filter selects items without requiring an intermediate vector; a later collect can gather the matches."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Implement `search_insensitive` in
`chapters/ch13_iterators_closures/src/lib.rs`, then run:

```bash
cargo test -p ch13_iterators_closures
```
