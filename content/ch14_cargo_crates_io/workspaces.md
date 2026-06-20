# 14.3 Cargo Workspaces

A **workspace** is a set of packages that share one `Cargo.lock` and one
`target/` output directory. It's how you split a large project into several
crates that build together. A top-level `Cargo.toml` lists the members instead
of describing a package:

```toml
[workspace]
resolver = "2"
members = ["adder", "add_one"]
```

Each member is its own crate in its own directory. Because they share one lock
file, every crate resolves dependencies to the *same* versions, keeping the
build consistent. One crate depends on a sibling with a path:

```toml
# adder/Cargo.toml
[dependencies]
add_one = { path = "../add_one" }
```

From the workspace root, `cargo build` compiles everything, and
`cargo test -p add_one` runs just one member's tests (the `-p` you use
throughout this course works because each chapter crate is a workspace member).
Running `cargo run` from the root requires `-p <name>` to choose which binary,
since the workspace itself has no default. Workspaces shine when crates evolve
together but you still want clean module boundaries and independent test suites.

### Exercise
Implement `compatible_update` in `chapters/ch14_cargo_crates_io/src/lib.rs` —
the resolver logic that keeps shared dependencies on the newest compatible
version — then run:

```bash
cargo test -p ch14_cargo_crates_io
```
