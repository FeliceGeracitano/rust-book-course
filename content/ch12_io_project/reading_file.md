# 12.2 Reading a File

Once `minigrep` knows which file to open, it needs the file's text. The standard
library makes this a one-liner: `std::fs::read_to_string` takes a path and hands
back a `Result<String, std::io::Error>` containing the whole file as a single
`String`.

```rust
use std::fs;

fn main() {
    let contents = fs::read_to_string("poem.txt")
        .expect("should have been able to read the file");
    println!("With text:\n{contents}");
}
```

Holding the entire file in one `String` is convenient: you can iterate its
`.lines()`, search substrings, or count words without juggling a reader. The
trade-off is memory — for huge files you'd stream instead — but for a learning
tool it keeps the code clear.

Notice that the search logic never actually cares *where* the text came from. It
only needs a `&str`. That separation is what lets our tests pass literal strings
instead of touching the filesystem, so they stay fast and deterministic. In the
exercises you'll work directly on that in-memory text, exactly the value
`read_to_string` would have produced.

### Exercise
In `chapters/ch12_io_project/src/lib.rs`, the search functions receive the file
`contents` as a `&str`. Implement them, then run:

```bash
cargo test -p ch12_io_project
```
