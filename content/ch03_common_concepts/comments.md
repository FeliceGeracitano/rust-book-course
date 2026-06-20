# 3.4 Comments

Comments document intent for the humans reading your code; the compiler ignores
them. The everyday comment starts with `//` and runs to the end of the line.

```rust
// Convert Fahrenheit to Celsius before storing.
let c = (f - 32.0) * 5.0 / 9.0; // formula from 3.2
```

Rust also has **documentation comments** that tooling understands. `///` documents
the item that follows it, and `//!` documents the enclosing item (such as a whole
module or crate). Doc comments support Markdown and can contain runnable code
examples that `cargo test` actually executes — so your docs cannot silently drift
from reality.

```rust
/// Returns `n` squared.
///
/// ```
/// assert_eq!(square(4), 16);
/// ```
pub fn square(n: i64) -> i64 { n * n }
```

Good comments explain *why*, not *what* — the code already shows what it does.
Prefer clarifying tricky decisions over narrating obvious lines.

### Exercise
Read the `///` doc comments above each function in
`chapters/ch03_common_concepts/src/lib.rs`: they describe exactly what to build.
Implement them, then run:

```bash
cargo test -p ch03_common_concepts
```
