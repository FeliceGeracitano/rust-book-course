# 12.1 Accepting Command Line Arguments

A real command-line tool reacts to what the user typed. In Rust you reach the
arguments through `std::env::args`, which returns an iterator over the program
name followed by each argument as a `String`. The very first item is the path
to the binary itself, so most programs skip it.

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // args[0] is the program name; the real arguments start at args[1].
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {query} in {file_path}");
}
```

Collecting into a `Vec<String>` lets you index the pieces, but it also means a
missing argument will panic with an out-of-bounds error — something we'll fix
later with proper validation. For now the mental model is simple: arguments
arrive as an ordered list of strings, and it is up to you to give each position
a meaning. Because real `env::args` depends on how the program was launched, the
exercises work on a plain `&[&str]` slice instead, keeping the logic pure and
testable.

### Exercise
Open `chapters/ch12_io_project/src/lib.rs` and start filling in the `todo!()`
bodies, then run:

```bash
cargo test -p ch12_io_project
```
