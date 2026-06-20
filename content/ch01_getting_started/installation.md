# 1.1 Installation

Rust is installed and managed by **`rustup`**, the toolchain installer.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version   # verify
rustup update     # update later
```

> In this course you don't strictly need Rust on your host — the Docker container
> ships the toolchain. But installing locally makes editing and running snippets nicer.

Next: write your first program in **1.2 Hello, World!**
