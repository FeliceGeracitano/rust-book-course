# Appendix G — How Rust is Made and "Nightly Rust"

> Book: https://doc.rust-lang.org/book/appendix-07-nightly-rust.html

Rust ships on a **train model** with three channels:

- **Stable** — a new release every 6 weeks. What you use for real work.
- **Beta** — the next stable, in testing.
- **Nightly** — built every night; includes **unstable features**.

A feature is developed on nightly behind a flag, gets tested through beta, and
only then becomes stable — so stable Rust never breaks.

### Using nightly
```bash
rustup toolchain install nightly
rustup override set nightly      # use nightly in this project
```

Unstable features must be opted into explicitly on nightly:

```rust
#![feature(some_unstable_feature)]
```

For this course, plain **stable** Rust is all you need.

---

📖 **Read it on the official Rust Book** — the full, authoritative version: <https://doc.rust-lang.org/book/appendix-07-nightly-rust.html>

```quiz
{
  "id": "discovery",
  "question": "When is the nightly toolchain useful?",
  "options": [
    "Only for formatting stable code",
    "When experimenting with unstable compiler features",
    "Whenever you want to run any Rust program at all"
  ],
  "answer": 1,
  "explain": "Stable is suitable for normal use. Nightly enables experimentation with unstable features whose behavior and availability can change."
}
```
