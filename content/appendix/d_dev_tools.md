# Appendix D — Useful Development Tools

> Book: https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html

The tooling that makes day-to-day Rust pleasant:

- **rustfmt** — automatic formatting. `cargo fmt` rewrites your code to the
  community style.
- **Clippy** — a linter with hundreds of checks for common mistakes and
  non-idiomatic code. `cargo clippy`. (The **Clippy** button in this course runs
  it for you.)
- **rust-analyzer** — the language server powering IDE features: completion,
  go-to-definition, inline errors, and types.
- **`cargo fix`** — applies compiler-suggested fixes automatically (great for
  edition upgrades).
- **`rustup component add <name>`** — installs components like `clippy`,
  `rustfmt`, or `rust-src`.
- **`rustup doc`** — opens the offline docs (the Book, std library, and more).

```bash
cargo fmt        # format
cargo clippy     # lint
cargo fix        # apply suggestions
```

---

📖 **Read it on the official Rust Book** — the full, authoritative version: <https://doc.rust-lang.org/book/appendix-04-useful-development-tools.html>
