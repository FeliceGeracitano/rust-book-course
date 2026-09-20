# 7.2 Control Scope and Privacy with Modules

**Modules** let you organize code inside a crate and control which items are
visible. You declare one with `mod`, and modules can nest, forming the *module
tree* rooted at `crate`.

The key rule: items are **private to their parent module by default**. A child
module can see everything in its ancestors, but a parent cannot reach into a
child's private items. To expose an item, mark it `pub` — and remember that the
*module itself* must also be `pub` for outside code to path through it.

```rust
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(len: usize) -> usize {
            len + 1
        }
    }
    mod cleaning {                 // private: invisible outside front_of_house
        fn wipe_tables() {}
    }
}
```

Here `front_of_house`, `hosting`, and `add_to_waitlist` are all `pub`, so the
tests can call them. `cleaning` has no `pub`, so it stays an internal detail.
This is how you draw a clean boundary between a crate's public API and its
private implementation.

```quiz
{
  "id": "discovery",
  "question": "How can a parent module call a function in its child module?",
  "options": [
    "Every function is already public",
    "Use mut on the function",
    "Make that child function visible to the parent, for example with pub(super)"
  ],
  "answer": 2,
  "explain": "Items are private by default. pub(super) exposes an item to its parent; broader visibility such as pub can also be used when appropriate."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

In `src/lib.rs`, make sure the nested `hosting` functions are reachable and
return the right values, then run:

```bash
cargo test -p ch07_packages_crates_modules
```
