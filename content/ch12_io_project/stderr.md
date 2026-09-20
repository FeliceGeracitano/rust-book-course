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

```quiz
{
  "id": "discovery",
  "question": "Where should command-line diagnostics go so piped search results stay clean?",
  "options": [
    "Standard output mixed into matching lines",
    "Into every line of the input file",
    "Standard error via eprintln!"
  ],
  "answer": 2,
  "explain": "stderr separates diagnostics from the program’s data output. Shell users can redirect stdout without capturing errors in the same file."
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Finish every `todo!()` in `chapters/ch12_io_project/src/lib.rs`, including the
error messages returned by `Config::build`, then run:

```bash
cargo test -p ch12_io_project
```
