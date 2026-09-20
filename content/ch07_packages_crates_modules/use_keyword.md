# 7.4 Bringing Paths Into Scope with use

Writing the full path every time gets noisy. The `use` keyword brings a path
into scope so you can refer to it with a shorter name — like a shortcut in your
filesystem. The idiomatic convention is to `use` the *parent module* of a
function (so calls read `hosting::add_to_waitlist`), but to `use` structs, enums,
and traits by their *full* name.

```rust
use crate::front_of_house::hosting;

pub fn seat() -> usize {
    hosting::add_to_waitlist(0)   // shorter than the whole path
}
```

You can rename an import with `as` (`use std::fmt::Result as FmtResult;`) to
avoid clashes. And `pub use` *re-exports*: it brings a name into scope **and**
makes it available to outside code under the new path. This lets you present a
tidy public API even when the internals are nested deep.

```rust
pub use crate::back_of_house::Appetizer;   // re-exported at the crate root
```

Now callers reach it as `ch07_packages_crates_modules::Appetizer` instead of the
longer internal path.

```quiz
{
  "id": "discovery",
  "question": "Does use move or copy an item into a new module?",
  "options": [
    "No, it brings a path into scope",
    "Yes, it creates a new implementation",
    "Yes, it makes private items public"
  ],
  "answer": 0,
  "explain": "use provides a convenient name for an existing item. It does not bypass privacy; pub use can deliberately re-export an accessible item."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Add the `use` and `pub use` lines in `src/lib.rs` and finish `describe`, which
relies on the re-exported `Appetizer`, then run:

```bash
cargo test -p ch07_packages_crates_modules
```
