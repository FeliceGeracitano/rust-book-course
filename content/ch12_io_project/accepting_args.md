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

```quiz
{
  "id": "discovery",
  "question": "What is usually the first value from std::env::args()?",
  "options": [
    "An empty string on every platform",
    "The program’s invocation path or name",
    "The first search query"
  ],
  "answer": 1,
  "explain": "The first argument identifies the invoked program. User-supplied arguments come after it, so parsing must account for that position."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Open `chapters/ch12_io_project/src/lib.rs` and start filling in the `todo!()`
bodies, then run:

```bash
cargo test -p ch12_io_project
```
