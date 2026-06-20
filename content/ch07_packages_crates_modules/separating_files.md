# 7.5 Separating Modules into Different Files

As a crate grows, keeping every module in one file becomes unwieldy. Rust lets
you move a module's body into its own file. When you write `mod front_of_house;`
(with a semicolon instead of a block), the compiler looks for the module's
contents in `src/front_of_house.rs`, or in `src/front_of_house/mod.rs`.

```rust
// src/lib.rs
mod front_of_house;          // load the body from another file

pub use crate::front_of_house::hosting;
```

```rust
// src/front_of_house.rs
pub mod hosting;             // load this submodule too
```

```rust
// src/front_of_house/hosting.rs
pub fn add_to_waitlist(len: usize) -> usize { len + 1 }
```

The `mod` declaration is *not* an "include": you write it once, and paths,
privacy, and `use` all behave exactly as if the code were inline. Splitting files
is purely about organization — the module tree stays identical.

In this chapter the crate is small enough to live in one `src/lib.rs`, but the
same module tree would split cleanly along these lines.

### Exercise

Complete the remaining `todo!()`s in `src/lib.rs` so the whole module tree
works, then run:

```bash
cargo test -p ch07_packages_crates_modules
```
