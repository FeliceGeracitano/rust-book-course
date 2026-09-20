# 19.2 Refutability: Whether a Pattern Might Fail to Match

Patterns come in two flavors. An **irrefutable** pattern always matches: `let x =
5;` or `let (a, b) = pair;` can never fail, because every possible value fits the
shape. A **refutable** pattern *can* fail for some values: `Some(n)` does not
match `None`, so it might not bind.

Where a pattern is used decides which flavor is allowed. `let` and function
parameters require **irrefutable** patterns — there is no "else" branch to fall
through to, so the match must be guaranteed. `if let`, `while let`, and the
pattern before `else` in `let...else` *expect* **refutable** patterns, because
their whole job is to handle the "did not match" case.

```rust
let maybe: Option<i32> = Some(7);

// `Some(n)` is refutable -> use `if let`, not plain `let`:
if let Some(n) = maybe {
    println!("got {n}");
}

// `let...else` binds on success and must diverge on failure:
let Some(n) = maybe else {
    return;
};
println!("n is {n}");
```

Mixing them up is a compile error: an irrefutable pattern in `if let` is useless,
and a refutable one in plain `let` cannot guarantee a binding. The compiler even
suggests the right form.

```quiz
{
  "id": "discovery",
  "question": "Why is let Some(x) = maybe; rejected for an Option<i32>?",
  "options": [
    "Some cannot contain i32",
    "let cannot bind integers",
    "The pattern does not match None"
  ],
  "answer": 2,
  "explain": "A plain let binding requires an irrefutable pattern. Use if let, match, or let...else to handle the case where the pattern does not match."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Complete the `todo!()` bodies in
`chapters/ch19_patterns_matching/src/lib.rs`, then run:

`cargo test -p ch19_patterns_matching`
