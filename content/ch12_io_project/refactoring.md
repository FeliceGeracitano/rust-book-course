# 12.3 Refactoring to Improve Modularity and Error Handling

A `main` that parses arguments, opens files, and searches all at once is hard to
test and easy to break. The fix is to group related data into a `Config` struct
and give it a constructor that *validates* the input. Instead of panicking on a
missing argument, the constructor returns a `Result`, so the caller decides how
to report the problem.

```rust
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[&str]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[0].to_string(),
            file_path: args[1].to_string(),
        })
    }
}
```

Two ideas are doing the heavy lifting here. First, *modularity*: configuration
parsing now lives in one named place, separate from the work it configures.
Second, *error handling as values*: a wrong argument count produces an `Err`
with a clear message rather than a crash. This is the pattern the Book uses to
move logic out of `main` and into a library that tests can exercise directly.

### Exercise
Implement `Config::build` in `chapters/ch12_io_project/src/lib.rs` so it returns
the right `Err` messages for too few and too many arguments, then run:

```bash
cargo test -p ch12_io_project
```
