# 1.3 Hello, Cargo!

**Cargo** is Rust's build system and package manager. You'll use it for everything
from here on.

```bash
cargo new hello_cargo   # create a project
cargo build             # compile
cargo run               # compile + run
cargo check             # type-check without producing a binary
cargo test              # run tests
```

### Exercise
Make `build_tool()` return `"cargo"`:

```bash
cargo test -p ch01_getting_started
```

When both ch01 tests pass, the chapter shows ✅ in the UI.
