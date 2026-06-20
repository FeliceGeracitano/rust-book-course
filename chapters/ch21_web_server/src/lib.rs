//! Chapter 21 — Final Project: Building a Multithreaded Web Server
//!
//! The Book's capstone wires up a real `TcpListener`, reads bytes off a
//! `TcpStream`, and hands connections to a thread pool. Sockets and timing make
//! for flaky tests, so here we drill the *pure logic* that sits underneath the
//! networking:
//!
//! * **21.1** — parse an HTTP request line (`"GET /path HTTP/1.1"`) and build a
//!   response string (status line + headers + body) by hand, no sockets.
//! * **21.2** — a thread-pool-style [`JobCounter`] backed by `Arc<Mutex<T>>`
//!   that many worker threads can share to track queued and finished jobs.
//! * **21.3** — [`route`], the single-threaded routing decision a worker makes
//!   for each request, separated from any I/O.
//!
//! Every exercise is deterministic: the concurrency exercise joins all of its
//! threads before asserting, so there are no sleeps, no timing, and no
//! randomness.
//!
//! Complete each `todo!()` in the items below, then run:
//!
//! ```text
//! cargo test -p ch21_web_server
//! ```

use std::sync::{Arc, Mutex};

// ---------------------------------------------------------------------------
// Exercise 1 — Parsing an HTTP request line
// ---------------------------------------------------------------------------

/// The three space-separated fields of an HTTP/1.1 request line.
///
/// A request line looks like `GET /index.html HTTP/1.1`: a method, a request
/// target (path), and the protocol version, separated by single spaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestLine {
    /// The HTTP method, e.g. `GET` or `POST`.
    pub method: String,
    /// The request target (path), e.g. `/` or `/index.html`.
    pub target: String,
    /// The protocol version, e.g. `HTTP/1.1`.
    pub version: String,
}

/// Parses the first line of an HTTP request into its three fields.
///
/// Returns `None` when the line does not have exactly three
/// space-separated parts, so malformed input is rejected instead of panicking.
///
/// # Examples
///
/// ```
/// use ch21_web_server::{parse_request_line, RequestLine};
///
/// let parsed = parse_request_line("GET /index.html HTTP/1.1").unwrap();
/// assert_eq!(parsed, RequestLine {
///     method: "GET".to_string(),
///     target: "/index.html".to_string(),
///     version: "HTTP/1.1".to_string(),
/// });
///
/// assert!(parse_request_line("GET /only-two").is_none());
/// ```
pub fn parse_request_line(line: &str) -> Option<RequestLine> {
    // TODO: split `line` on spaces, take exactly three parts (method, target,
    // version), and return `None` if there are too few or too many. Use `?` on
    // the first three and check there is nothing left over.
    todo!("parse the request line into a RequestLine, or None if malformed")
}

// ---------------------------------------------------------------------------
// Exercise 2 — Building an HTTP response string
// ---------------------------------------------------------------------------

/// Builds a complete HTTP/1.1 response string for the given status and body.
///
/// The response is assembled by hand, exactly as the Book does it: a status
/// line, a `Content-Length` header, a blank line, then the body. Lines are
/// terminated with `\r\n` (CRLF) as the HTTP spec requires, and
/// `Content-Length` is the body's length **in bytes**.
///
/// ```text
/// HTTP/1.1 200 OK\r\n
/// Content-Length: 5\r\n
/// \r\n
/// hello
/// ```
///
/// # Examples
///
/// ```
/// use ch21_web_server::build_response;
///
/// let response = build_response(200, "OK", "hello");
/// assert_eq!(
///     response,
///     "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello",
/// );
/// ```
pub fn build_response(status_code: u16, reason: &str, body: &str) -> String {
    // TODO: build "HTTP/1.1 {code} {reason}", then a "Content-Length" header set
    // to the body's byte length (`body.len()`), a blank line, and the body.
    // Separate every line with "\r\n".
    todo!("assemble the status line, Content-Length header, blank line, and body")
}

// ---------------------------------------------------------------------------
// Exercise 3 — Routing a request to a (status, reason, body)
// ---------------------------------------------------------------------------

/// Decides the response for a parsed [`RequestLine`], with no I/O involved.
///
/// This is the pure decision a worker thread makes per request:
///
/// * `GET /`            → `(200, "OK", "Welcome")`
/// * `GET /sleep`       → `(200, "OK", "Slept")` (no real sleeping here)
/// * any other `GET`    → `(404, "NOT FOUND", "Not Found")`
/// * any non-`GET`      → `(405, "METHOD NOT ALLOWED", "Method Not Allowed")`
///
/// Returning a tuple keeps routing testable: feed it a [`RequestLine`] and
/// check the decision, then pair it with [`build_response`] to get bytes.
///
/// # Examples
///
/// ```
/// use ch21_web_server::{parse_request_line, route};
///
/// let req = parse_request_line("GET / HTTP/1.1").unwrap();
/// assert_eq!(route(&req), (200, "OK", "Welcome"));
///
/// let missing = parse_request_line("GET /nope HTTP/1.1").unwrap();
/// assert_eq!(route(&missing), (404, "NOT FOUND", "Not Found"));
/// ```
pub fn route(request: &RequestLine) -> (u16, &'static str, &'static str) {
    // TODO: return (405, "METHOD NOT ALLOWED", "Method Not Allowed") for any
    // method other than "GET". For GET, match the target: "/" → 200 "OK"
    // "Welcome", "/sleep" → 200 "OK" "Slept", anything else → 404 "NOT FOUND"
    // "Not Found".
    todo!("decide the (status, reason, body) for this request")
}

// ---------------------------------------------------------------------------
// Exercise 4 — A thread-pool-style job counter with Arc<Mutex<T>>
// ---------------------------------------------------------------------------

/// A shared counter that tracks queued and finished jobs across worker threads.
///
/// A real thread pool hands jobs to workers running on other threads. To know
/// how much work is outstanding, the pool needs state that every thread can
/// touch safely. [`JobCounter`] is that state: `Arc<T>` gives each worker a
/// cheap, atomically reference-counted handle to the *same* counts, and
/// `Mutex<T>` makes each update exclusive so no increment is ever lost.
///
/// Cloning a [`JobCounter`] clones the `Arc`, so every clone points at one set
/// of underlying counts.
///
/// # Examples
///
/// ```
/// use ch21_web_server::JobCounter;
///
/// let counter = JobCounter::new();
/// let worker = counter.clone();      // shares the same counts
///
/// counter.submit();                  // a job is queued
/// counter.submit();
/// worker.complete();                 // a worker finishes one
///
/// assert_eq!(counter.submitted(), 2);
/// assert_eq!(counter.completed(), 1);
/// assert_eq!(counter.pending(), 1);  // submitted − completed
/// ```
#[derive(Clone)]
pub struct JobCounter {
    // Both counts live behind one mutex so a single lock keeps them consistent.
    counts: Arc<Mutex<Counts>>,
}

#[derive(Default)]
struct Counts {
    submitted: u64,
    completed: u64,
}

impl JobCounter {
    /// Creates a new counter with no submitted or completed jobs.
    pub fn new() -> JobCounter {
        // TODO: wrap a default `Counts` in `Arc::new(Mutex::new(...))` and
        // store it in the `counts` field.
        todo!("build a JobCounter holding an Arc<Mutex<Counts>>")
    }

    /// Records that a job has been queued for the pool.
    pub fn submit(&self) {
        // TODO: lock the mutex and add one to `submitted`.
        todo!("lock the counts and increment `submitted`")
    }

    /// Records that a worker has finished a job.
    pub fn complete(&self) {
        // TODO: lock the mutex and add one to `completed`.
        todo!("lock the counts and increment `completed`")
    }

    /// Total number of jobs ever submitted.
    pub fn submitted(&self) -> u64 {
        // TODO: lock the mutex and read `submitted`.
        todo!("return the submitted count")
    }

    /// Total number of jobs ever completed.
    pub fn completed(&self) -> u64 {
        // TODO: lock the mutex and read `completed`.
        todo!("return the completed count")
    }

    /// Jobs still outstanding: `submitted - completed`.
    ///
    /// Both counts are read under a single lock so the pair is consistent, and
    /// `saturating_sub` keeps the result at `0` if `complete` is ever called
    /// more often than `submit`.
    pub fn pending(&self) -> u64 {
        // TODO: lock the mutex once, then return `submitted - completed` using
        // `saturating_sub` so the result never underflows below 0.
        todo!("return submitted.saturating_sub(completed)")
    }
}

impl Default for JobCounter {
    fn default() -> Self {
        JobCounter::new()
    }
}
