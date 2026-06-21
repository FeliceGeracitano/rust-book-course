# Appendix E — Editions

> Book: https://doc.rust-lang.org/book/appendix-05-editions.html

Editions let Rust evolve (new keywords, defaults, idioms) without breaking
existing code. Each crate opts into an edition in its `Cargo.toml`:

```toml
[package]
edition = "2024"
```

Editions so far: **2015**, **2018**, **2021**, and **2024** (this course uses
2024).

Key points:
- The edition is **per crate**. A 2024 crate can depend on a 2015 crate and vice
  versa — the ecosystem never splits.
- The compiler keeps supporting all editions, so upgrading is opt-in and
  incremental.
- `cargo fix --edition` automates most of the migration to a newer edition.

---

📖 **Read it on the official Rust Book** — the full, authoritative version: <https://doc.rust-lang.org/book/appendix-05-editions.html>
