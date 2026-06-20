# 7.1 Packages and Crates

A **crate** is the smallest unit the Rust compiler works with. There are two
flavors. A *binary crate* compiles to an executable and has a `main` function. A
*library crate* has no `main`; it exposes code that other crates can use. People
usually say "crate" to mean the library kind.

A **package** is a bundle of one or more crates, described by a `Cargo.toml`
file. A package can hold at most one library crate and any number of binary
crates. By convention `src/lib.rs` is the library crate root and `src/main.rs` is
a binary crate root — Cargo finds them automatically. The *crate root* is the
file the compiler starts from; it becomes the root module named `crate`.

```toml
# Cargo.toml — describes the package
[package]
name = "ch07_packages_crates_modules"
edition = "2024"
```

This chapter's crate is a single **library**, so everything lives under
`src/lib.rs` and other code reaches it through the crate name
`ch07_packages_crates_modules`.

### Exercise

Open `src/lib.rs` and complete the `todo!()`s in this library crate, then run:

```bash
cargo test -p ch07_packages_crates_modules
```
