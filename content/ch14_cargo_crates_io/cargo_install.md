# 14.4 Installing Binaries with cargo install

`cargo install` downloads a crate from crates.io, compiles it in release mode,
and drops the resulting executable into your local Cargo bin directory —
usually `~/.cargo/bin`, which you add to your `PATH`. It's how you install
command-line tools written in Rust without a separate package manager:

```bash
cargo install ripgrep      # installs the `rg` binary
cargo install --list       # see what you've installed
```

Only crates with a **binary target** (a `src/main.rs` or a `[[bin]]`) can be
installed this way; library-only crates have nothing to run. Each install is
versioned like any dependency, so re-running `cargo install` upgrades the tool
to the newest compatible release. Because everything lands in one bin directory
on your `PATH`, the installed tools behave just like any other shell command.

This is also the mechanism that powers the next section: tools you install with
`cargo install` can become new `cargo` subcommands automatically, purely from
their binary names.

```bash
cargo install cargo-audit  # adds a `cargo audit` subcommand
```

```quiz
{
  "id": "discovery",
  "question": "What is cargo install used for?",
  "options": [
    "Installing a Rust binary tool for use from the command line",
    "Adding a library dependency to Cargo.toml",
    "Installing the Rust compiler itself"
  ],
  "answer": 0,
  "explain": "cargo install builds and installs binary crates. Project dependencies belong in Cargo.toml (or can be added with cargo add); rustup manages the compiler toolchain."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

There's no separate function for this section — the `subcommand_binary`
exercise (14.5) builds directly on the naming convention that makes installed
binaries discoverable. Continue in `chapters/ch14_cargo_crates_io/src/lib.rs`:

```bash
cargo test -p ch14_cargo_crates_io
```
