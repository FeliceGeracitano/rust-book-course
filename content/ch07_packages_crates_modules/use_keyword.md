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

### Exercise

Add the `use` and `pub use` lines in `src/lib.rs` and finish `describe`, which
relies on the re-exported `Appetizer`, then run:

```bash
cargo test -p ch07_packages_crates_modules
```
