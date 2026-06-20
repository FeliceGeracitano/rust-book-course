# 21.1 Building a Single-Threaded Web Server

A web server speaks HTTP, a text protocol. A request begins with a *request
line* of three space-separated fields: a method, a target path, and a version,
such as `GET /index.html HTTP/1.1`. The server's reply starts with a *status
line* like `HTTP/1.1 200 OK`, followed by headers, a blank line, and the body.
Every line ends with `\r\n` (carriage-return + line-feed).

In the Book this logic is tangled up with a `TcpListener` and a `TcpStream`. But
the interesting part — turning bytes into a decision and back into bytes — is
pure string work you can write and test without ever opening a socket.

```rust
let line = "GET / HTTP/1.1";
let mut parts = line.split(' ');
let method = parts.next().unwrap();   // "GET"
let target = parts.next().unwrap();   // "/"

let body = "hello";
let response = format!(
    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    body.len(), body,
);
```

Note that `Content-Length` is the body's length **in bytes** (`body.len()`),
not its character count — they differ for non-ASCII text.

### Exercise

Implement `parse_request_line` and `build_response` in
`chapters/ch21_web_server/src/lib.rs`, then run:

```bash
cargo test -p ch21_web_server
```
