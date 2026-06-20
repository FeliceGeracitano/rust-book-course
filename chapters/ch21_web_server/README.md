# Chapter 21 — Final Project: Building a Multithreaded Web Server

> Book: https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

## Summary
Capstone — build a multithreaded HTTP server from scratch, the same family as
this course's own server.

## You will learn
- `TcpListener` and `TcpStream`
- Parsing HTTP requests by hand
- Building a thread pool
- Graceful shutdown via the `Drop` trait

## Subchapters
- 21.1 Building a Single-Threaded Web Server
- 21.2 From Single-Threaded to Multithreaded Server
- 21.3 Graceful Shutdown and Cleanup

## Exercises
Open `src/lib.rs`, complete each `todo!()`, then make the tests pass:

```bash
cargo test -p ch21_web_server
```

Stuck? See `SOLUTION.md`.
