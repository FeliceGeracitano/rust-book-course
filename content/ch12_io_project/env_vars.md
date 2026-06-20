# 12.5 Working with Environment Variables

Not every option belongs on the command line. A case-insensitive search, for
example, is a preference a user might want to set once for their whole session.
Environment variables are perfect for this. The Book reads `IGNORE_CASE` with
`std::env::var`, which returns a `Result`: `Ok` when the variable is set, `Err`
when it isn't. Often you only care *whether* it exists, so `.is_ok()` is enough.

```rust
use std::env;

fn main() {
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    // ...later, pick which search to run based on this flag.
}
```

The case-insensitive search itself lowercases both sides before comparing, so
`"rUsT"` matches `Rust`, `RUST`, and `rust` alike:

```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```

Because real environment access isn't deterministic, the exercises pass the
`ignore_case` flag in directly — the same boolean `env::var(...).is_ok()` would
have produced — keeping the tested logic pure.

### Exercise
Implement `search_case_insensitive` and have `Config::matches` honour the
`ignore_case` flag in `chapters/ch12_io_project/src/lib.rs`, then run:

```bash
cargo test -p ch12_io_project
```
