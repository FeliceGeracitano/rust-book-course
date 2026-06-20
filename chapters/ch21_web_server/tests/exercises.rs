use std::thread;

use ch21_web_server::{JobCounter, RequestLine, build_response, parse_request_line, route};

// --- Exercise 1: parsing an HTTP request line -------------------------------

#[test]
fn parses_a_well_formed_request_line() {
    let parsed = parse_request_line("GET /index.html HTTP/1.1").unwrap();
    assert_eq!(
        parsed,
        RequestLine {
            method: "GET".to_string(),
            target: "/index.html".to_string(),
            version: "HTTP/1.1".to_string(),
        }
    );
}

#[test]
fn parses_a_post_to_the_root_path() {
    let parsed = parse_request_line("POST / HTTP/1.1").unwrap();
    assert_eq!(parsed.method, "POST");
    assert_eq!(parsed.target, "/");
    assert_eq!(parsed.version, "HTTP/1.1");
}

#[test]
fn rejects_a_line_with_too_few_parts() {
    assert!(parse_request_line("GET /only-two").is_none());
    assert!(parse_request_line("GET").is_none());
    assert!(parse_request_line("").is_none());
}

#[test]
fn rejects_a_line_with_too_many_parts() {
    // A trailing field (or an extra space) is malformed.
    assert!(parse_request_line("GET / HTTP/1.1 extra").is_none());
    assert!(parse_request_line("GET  / HTTP/1.1").is_none());
}

// --- Exercise 2: building an HTTP response string ---------------------------

#[test]
fn builds_a_200_response_with_correct_length() {
    let response = build_response(200, "OK", "hello");
    assert_eq!(response, "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello");
}

#[test]
fn builds_a_404_response() {
    let response = build_response(404, "NOT FOUND", "Not Found");
    assert_eq!(
        response,
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 9\r\n\r\nNot Found"
    );
}

#[test]
fn empty_body_has_zero_content_length() {
    let response = build_response(204, "NO CONTENT", "");
    assert_eq!(
        response,
        "HTTP/1.1 204 NO CONTENT\r\nContent-Length: 0\r\n\r\n"
    );
}

#[test]
fn content_length_counts_bytes_not_chars() {
    // "é" is two bytes in UTF-8, so the byte length is 3, not 2.
    let response = build_response(200, "OK", "aé");
    assert_eq!(response, "HTTP/1.1 200 OK\r\nContent-Length: 3\r\n\r\naé");
}

// --- Exercise 3: routing a request ------------------------------------------

#[test]
fn routes_root_to_welcome() {
    let req = parse_request_line("GET / HTTP/1.1").unwrap();
    assert_eq!(route(&req), (200, "OK", "Welcome"));
}

#[test]
fn routes_sleep_path_without_sleeping() {
    let req = parse_request_line("GET /sleep HTTP/1.1").unwrap();
    assert_eq!(route(&req), (200, "OK", "Slept"));
}

#[test]
fn routes_unknown_path_to_404() {
    let req = parse_request_line("GET /missing HTTP/1.1").unwrap();
    assert_eq!(route(&req), (404, "NOT FOUND", "Not Found"));
}

#[test]
fn routes_non_get_method_to_405() {
    let req = parse_request_line("POST / HTTP/1.1").unwrap();
    assert_eq!(
        route(&req),
        (405, "METHOD NOT ALLOWED", "Method Not Allowed")
    );
}

#[test]
fn route_and_build_response_compose() {
    let req = parse_request_line("GET / HTTP/1.1").unwrap();
    let (code, reason, body) = route(&req);
    assert_eq!(
        build_response(code, reason, body),
        "HTTP/1.1 200 OK\r\nContent-Length: 7\r\n\r\nWelcome"
    );
}

// --- Exercise 4: Arc<Mutex<T>> job counter ----------------------------------

#[test]
fn fresh_counter_starts_at_zero() {
    let counter = JobCounter::new();
    assert_eq!(counter.submitted(), 0);
    assert_eq!(counter.completed(), 0);
    assert_eq!(counter.pending(), 0);
}

#[test]
fn clones_share_one_set_of_counts() {
    let counter = JobCounter::new();
    let worker = counter.clone();

    counter.submit();
    counter.submit();
    worker.complete();

    assert_eq!(counter.submitted(), 2);
    assert_eq!(counter.completed(), 1);
    assert_eq!(counter.pending(), 1);
    // The clone observes the same shared state.
    assert_eq!(worker.pending(), 1);
}

#[test]
fn pending_never_goes_negative() {
    let counter = JobCounter::new();
    counter.complete();
    counter.complete();
    assert_eq!(counter.pending(), 0);
}

#[test]
fn many_threads_submit_and_complete_without_lost_updates() {
    let counter = JobCounter::new();

    let threads = 8;
    let per_thread = 1000;

    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        let counter = counter.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..per_thread {
                counter.submit();
                counter.complete();
            }
        }));
    }

    for handle in handles {
        handle.join().expect("worker thread panicked");
    }

    // Joining every thread before asserting makes this deterministic.
    let total = (threads * per_thread) as u64;
    assert_eq!(counter.submitted(), total);
    assert_eq!(counter.completed(), total);
    assert_eq!(counter.pending(), 0);
}
