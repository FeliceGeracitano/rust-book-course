# 1.1 Installation

Start exploring right here: read an example, make a prediction, and use the
feedback to understand what Rust is doing. Your progress stays in this browser.

For optional terminal practice, Rust is installed and managed by **`rustup`**,
the toolchain installer.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version   # verify
rustup update     # update later
```

> This browser course needs no Rust installation. Install the toolchain only if
> you want to run examples or try the optional terminal exercises.

Next: write your first program in **1.2 Hello, World!**

```quiz
{
  "id": "discovery",
  "question": "Which tool manages installed Rust toolchains?",
  "options": [
    "rustup",
    "rustc",
    "cargo check"
  ],
  "answer": 0,
  "explain": "rustup installs and updates toolchains. rustc compiles Rust, while Cargo manages projects and their dependencies."
}
```
