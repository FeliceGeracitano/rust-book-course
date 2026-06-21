# Appendix C — Derivable Traits

> Book: https://doc.rust-lang.org/book/appendix-03-derivable-traits.html

`#[derive(...)]` asks the compiler to generate a standard trait implementation
for your type, so you don't write the boilerplate.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }
```

| Trait | What you get |
|-------|--------------|
| `Debug` | `{:?}` formatting for printing/debugging |
| `Clone` | `.clone()` for an explicit deep copy |
| `Copy` | implicit bitwise copy (requires `Clone`) |
| `PartialEq` / `Eq` | `==` and `!=` |
| `PartialOrd` / `Ord` | `<`, `>`, sorting |
| `Hash` | use as a `HashMap`/`HashSet` key |
| `Default` | `T::default()` zero/empty value |

Derive only works when every field also implements the trait. For custom
behavior, implement the trait by hand instead.

---

📖 **Read it on the official Rust Book** — the full, authoritative version: <https://doc.rust-lang.org/book/appendix-03-derivable-traits.html>
