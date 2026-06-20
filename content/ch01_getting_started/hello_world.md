# 1.2 Hello, World!

A Rust program starts at `main`. Save this as `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

Compile and run:

```bash
rustc main.rs
./main
```

`println!` is a **macro** (note the `!`), not a function — more on that in Chapter 20.

### Exercise
In `chapters/ch01_getting_started/src/lib.rs`, make `greeting()` return
`"Hello, world!"`, then:

```bash
cargo test -p ch01_getting_started
```
