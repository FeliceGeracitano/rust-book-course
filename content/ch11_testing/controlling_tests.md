# 11.2 Controlling How Tests Are Run

`cargo test` compiles your tests into a binary and runs them in parallel by
default. Several flags let you control that behavior. Flags before `--` go to
Cargo; flags after `--` go to the test binary.

Run tests one at a time (useful when tests share state or you want ordered
output):

```bash
cargo test -- --test-threads=1
```

By default, `println!` output from *passing* tests is hidden. Show it with:

```bash
cargo test -- --show-output
```

You can run a subset by name. Any test whose name *contains* the argument runs,
so you can target one test or a whole group:

```bash
cargo test guess          # runs every test with "guess" in its name
cargo test guess_in_range # runs just that one
```

Mark expensive tests with `#[ignore]` so they're skipped normally and run only
with `cargo test -- --ignored`.

```rust
#[test]
#[ignore]
fn slow_full_sweep() {
    // run only on demand
}
```

### Exercise

Finish the implementations in `chapters/ch11_testing/src/lib.rs`, then practice
filtering — e.g. `cargo test -p ch11_testing guess` — before the full run:

```bash
cargo test -p ch11_testing
```
