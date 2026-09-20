# Appendix A — Keywords

> Book: https://doc.rust-lang.org/book/appendix-01-keywords.html

Rust reserves a set of words for the language. Most cannot be used as names.

**Strict keywords** (always reserved): `as`, `async`, `await`, `break`, `const`,
`continue`, `crate`, `dyn`, `else`, `enum`, `extern`, `false`, `fn`, `for`, `if`,
`impl`, `in`, `let`, `loop`, `match`, `mod`, `move`, `mut`, `pub`, `ref`, `return`,
`Self`, `self`, `static`, `struct`, `super`, `trait`, `true`, `type`, `unsafe`,
`use`, `where`, `while`.

**Reserved for the future**: `abstract`, `become`, `box`, `do`, `final`, `gen`,
`macro`, `override`, `priv`, `try`, `typeof`, `unsized`, `virtual`, `yield`.

**Weak keywords** (only special in context): `union`, `macro_rules`, `'static`.

### Raw identifiers
Need to use a keyword as a name (often when calling other languages)? Prefix it
with `r#`:

```rust
let r#fn = "I'm named fn";
println!("{}", r#fn);
```

---

📖 **Read it on the official Rust Book** — the full, authoritative version: <https://doc.rust-lang.org/book/appendix-01-keywords.html>

```quiz
{
  "id": "discovery",
  "question": "Why can a reserved Rust keyword not normally be used directly as a variable name?",
  "options": [
    "Only uppercase names are allowed",
    "It has a grammatical role or is reserved for language use",
    "All identifiers must contain a digit"
  ],
  "answer": 1,
  "explain": "Keywords have special meaning or are reserved. Raw identifiers such as r#type allow many keyword spellings when interoperability requires them."
}
```
