# Chapter 12 — An I/O Project: Building a Command Line Program

> Book: https://doc.rust-lang.org/book/ch12-00-an-io-project.html

## Summary
Build `minigrep`, a real CLI — arguments, file I/O, modular refactor, TDD,
environment variables, and stderr.

## You will learn
- `std::env::args`
- Reading files
- Separating library and binary code
- Test-driven development
- `env::var` and `eprintln!`

## Subchapters
- 12.1 Accepting Command Line Arguments
- 12.2 Reading a File
- 12.3 Refactoring to Improve Modularity and Error Handling
- 12.4 Adding Functionality with Test Driven Development
- 12.5 Working with Environment Variables
- 12.6 Redirecting Errors to Standard Error

## Exercises
Open `src/lib.rs`, complete each `todo!()`, then make the tests pass:

```bash
cargo test -p ch12_io_project
```

Stuck? See `SOLUTION.md`.
