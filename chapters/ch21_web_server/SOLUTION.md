# Chapter 21 — Solutions

```rust
use std::sync::{Arc, Mutex};

// --- Exercise 1: parsing an HTTP request line ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestLine {
    pub method: String,
    pub target: String,
    pub version: String,
}

pub fn parse_request_line(line: &str) -> Option<RequestLine> {
    let mut parts = line.split(' ');
    let method = parts.next()?;
    let target = parts.next()?;
    let version = parts.next()?;

    // A valid request line has exactly three parts — reject anything extra.
    if parts.next().is_some() {
        return None;
    }

    Some(RequestLine {
        method: method.to_string(),
        target: target.to_string(),
        version: version.to_string(),
    })
}

// --- Exercise 2: building an HTTP response string ---

pub fn build_response(status_code: u16, reason: &str, body: &str) -> String {
    let status_line = format!("HTTP/1.1 {status_code} {reason}");
    let length = body.len();
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{body}")
}

// --- Exercise 3: routing a request ---

pub fn route(request: &RequestLine) -> (u16, &'static str, &'static str) {
    if request.method != "GET" {
        return (405, "METHOD NOT ALLOWED", "Method Not Allowed");
    }

    match request.target.as_str() {
        "/" => (200, "OK", "Welcome"),
        "/sleep" => (200, "OK", "Slept"),
        _ => (404, "NOT FOUND", "Not Found"),
    }
}

// --- Exercise 4: an Arc<Mutex<T>> job counter ---

#[derive(Clone)]
pub struct JobCounter {
    counts: Arc<Mutex<Counts>>,
}

#[derive(Default)]
struct Counts {
    submitted: u64,
    completed: u64,
}

impl JobCounter {
    pub fn new() -> JobCounter {
        JobCounter {
            counts: Arc::new(Mutex::new(Counts::default())),
        }
    }

    pub fn submit(&self) {
        let mut counts = self.counts.lock().expect("mutex poisoned");
        counts.submitted += 1;
    }

    pub fn complete(&self) {
        let mut counts = self.counts.lock().expect("mutex poisoned");
        counts.completed += 1;
    }

    pub fn submitted(&self) -> u64 {
        self.counts.lock().expect("mutex poisoned").submitted
    }

    pub fn completed(&self) -> u64 {
        self.counts.lock().expect("mutex poisoned").completed
    }

    pub fn pending(&self) -> u64 {
        let counts = self.counts.lock().expect("mutex poisoned");
        counts.submitted.saturating_sub(counts.completed)
    }
}

impl Default for JobCounter {
    fn default() -> Self {
        JobCounter::new()
    }
}
```
