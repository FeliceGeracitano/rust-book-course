# 20.1 Unsafe Rust

Safe Rust guarantees memory safety by rejecting anything it cannot prove sound.
Sometimes you know more than the compiler — for example, that two `&mut`
references point at *different* parts of a slice. The `unsafe` keyword unlocks a
handful of extra powers, the most common being **dereferencing raw pointers**
(`*const T` and `*mut T`). Unsafe does not switch off the borrow checker; it just
lets you do a few things whose safety *you* must guarantee.

The idiom from the Book is to keep the `unsafe` block tiny and wrap it in a safe
function, so callers never have to write `unsafe` themselves.

```rust
fn split_first(slice: &mut [i32]) -> (&mut i32, &mut [i32]) {
    let ptr = slice.as_mut_ptr();
    // SAFETY: the head and tail never overlap.
    unsafe {
        (&mut *ptr, std::slice::from_raw_parts_mut(ptr.add(1), slice.len() - 1))
    }
}
```

Always write a `// SAFETY:` comment explaining why the invariants hold — that is
how reviewers trust your `unsafe`.

```quiz
{
  "id": "discovery",
  "question": "Does an unsafe block turn off the borrow checker for all ordinary references?",
  "options": [
    "No: it permits specific unsafe operations while other checks still apply",
    "Yes: all type and lifetime checks are disabled",
    "Yes: it makes every raw pointer valid"
  ],
  "answer": 0,
  "explain": "unsafe marks operations whose safety obligations the programmer must uphold. It does not validate pointers for you or disable ordinary reference rules."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

In `chapters/ch20_advanced_features/src/lib.rs`, complete `split_at_mut` so it
returns two disjoint mutable halves using raw pointers inside a safe wrapper.
Then run:

```bash
cargo test -p ch20_advanced_features
```
