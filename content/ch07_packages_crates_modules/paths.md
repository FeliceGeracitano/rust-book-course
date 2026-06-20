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

### Exercise

Implement `eat_at_restaurant` in `src/lib.rs` so it reaches
`add_to_waitlist` through both an absolute and a relative path, then run:

```bash
cargo test -p ch07_packages_crates_modules
```
