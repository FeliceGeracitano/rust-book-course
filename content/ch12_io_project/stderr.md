# 12.6 Redirecting Errors to Standard Error

Command-line programs have *two* output streams. Standard output (stdout) is for
the program's real results; standard error (stderr) is for diagnostics. Keeping
them separate matters because users often redirect stdout to a file —
`minigrep query poem.txt > results.txt` — and error messages should not end up
mixed into that file.

In Rust, `println!` writes to stdout, while `eprintln!` writes to stderr. Use
`eprintln!` for anything that reports a problem.

```rust
fn main() {
    let args = ["query"]; // pretend the file path is missing
    let config = build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });
    // print real results with println! ...
}
# fn build(_: &[&str]) -> Result<(), &'static str> { Err("not enough arguments") }
```

Now `> results.txt` captures only matches, while the error still appears on the
terminal. The lesson generalizes: results go to stdout, problems go to stderr,
and a non-zero exit code signals failure to the shell. Our `Config::build`
already returns those error *messages* as values — choosing the right stream to
print them on is the final polish that makes a tool behave well in pipelines.

### Exercise
Finish every `todo!()` in `chapters/ch12_io_project/src/lib.rs`, including the
error messages returned by `Config::build`, then run:

```bash
cargo test -p ch12_io_project
```
