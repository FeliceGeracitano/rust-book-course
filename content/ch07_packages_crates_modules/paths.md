# 7.3 Paths for Referring to an Item in the Module Tree

To call an item you name it with a **path**, much like a filesystem path. A
path can be **absolute**, starting from the crate root with the `crate` keyword,
or **relative**, starting from the current module (optionally with `self` or
`super`). Both forms name the same item; pick whichever reads better and survives
future moves.

```rust
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(len: usize) -> usize { len + 1 }
    }
}

pub fn eat_at_restaurant() -> usize {
    // Absolute path — from the crate root.
    let a = crate::front_of_house::hosting::add_to_waitlist(0);
    // Relative path — from this module.
    let b = front_of_house::hosting::add_to_waitlist(0);
    a + b
}
```

Privacy still applies along the path: every segment you traverse must be `pub`
(or an ancestor you already have access to). Use `super::` to refer to the
parent module — handy when a child needs a sibling defined one level up.

```quiz
{
  "id": "discovery",
  "question": "What does crate:: refer to at the start of a path?",
  "options": [
    "The current function",
    "The root of the current crate",
    "The parent module"
  ],
  "answer": 1,
  "explain": "crate:: starts an absolute path at this crate’s root. super:: refers to the parent module and self:: to the current module."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Implement `eat_at_restaurant` in `src/lib.rs` so it reaches
`add_to_waitlist` through both an absolute and a relative path, then run:

```bash
cargo test -p ch07_packages_crates_modules
```
