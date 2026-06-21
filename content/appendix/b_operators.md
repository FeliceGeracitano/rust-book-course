# Appendix B — Operators and Symbols

> Book: https://doc.rust-lang.org/book/appendix-02-operators.html

A quick reference to the operators you'll meet most.

| Group | Operators |
|-------|-----------|
| Arithmetic | `+` `-` `*` `/` `%` |
| Comparison | `==` `!=` `<` `>` `<=` `>=` |
| Logical | `&&` `\|\|` `!` |
| Bitwise | `&` `\|` `^` `!` `<<` `>>` |
| Assignment | `=` `+=` `-=` `*=` `/=` `%=` `&=` `\|=` `^=` `<<=` `>>=` |
| Ranges | `a..b` (exclusive) · `a..=b` (inclusive) |
| Error propagation | `?` (return early on `Err`/`None`) |

### Other symbols you'll see
- `&` / `&mut` — borrow (shared / mutable reference)
- `*` — dereference
- `->` — function return type · `=>` — match arm
- `::` — path separator · `!` — macro call (`println!`)
- `_` — ignore / wildcard pattern · `#[...]` — attribute
- `'a` — lifetime · `|x| x + 1` — closure

---

📖 **Read it on the official Rust Book** — the full, authoritative version: <https://doc.rust-lang.org/book/appendix-02-operators.html>
