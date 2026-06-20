# 14.5 Extending Cargo with Custom Commands

Cargo is designed to be extended without modifying Cargo itself. The rule is
simple: if you run `cargo something` and `something` is not a built-in command,
Cargo searches your `PATH` for an executable named `cargo-something` and runs
it. So an installed binary called `cargo-nm` is invoked as `cargo nm`, feeling
exactly like a native subcommand.

This naming convention is the whole mechanism. Combined with
`cargo install` (14.4), anyone can publish a tool that slots straight into the
`cargo` workflow:

```bash
cargo install cargo-expand   # installs the `cargo-expand` binary
cargo expand                 # Cargo finds and runs cargo-expand
```

Running `cargo --list` shows both built-in commands and any `cargo-*` binaries
it found on your `PATH`. The convention keeps the ecosystem open: the core tool
stays small, while the community ships everything from linters to release
automation as ordinary executables. The only "magic" is the `cargo-` prefix —
which is also why your own subcommand names must *not* already start with it.

```text
cargo widget   ->  looks for an executable named `cargo-widget`
```

### Exercise
Implement `subcommand_binary` in `chapters/ch14_cargo_crates_io/src/lib.rs` so
it turns a subcommand name into the `cargo-<name>` binary Cargo would look for
(and rejects bad input), then run:

```bash
cargo test -p ch14_cargo_crates_io
```
