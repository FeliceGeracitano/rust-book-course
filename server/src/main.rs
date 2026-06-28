//! rust-book-course learning server.
//!
//! A small `tiny_http` server that:
//!   - serves the built React client (`CLIENT_DIR`) — see [`http`],
//!   - exposes a tiny JSON API for the table of contents, lessons, and progress,
//!     and runs `cargo test -p <crate>` for the "Check" button — see [`api`] and
//!     [`chapters`],
//!
//! It runs a small pool of worker threads sharing the `tiny_http` server, so a
//! cold first page load's burst of parallel chunk requests is served in parallel
//! instead of serialized behind one another (a single-threaded loop leaves some
//! chunks "pending" until a refresh). Shared progress is behind a `Mutex`.
//! Reading this is also a preview of Chapter 21 (building a web server).

mod api;
mod chapters;
mod http;

use std::collections::HashMap;
use std::env;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

use tiny_http::{Method, Server};

use crate::api::handle;
use crate::chapters::ensure_working_files;

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn main() {
    let port = env_or("PORT", "8080");
    // Default to loopback so the host-file write API is never exposed on the LAN.
    // In Docker the container sets BIND_ADDR=0.0.0.0 and the host publishes the
    // port on 127.0.0.1 only (see docker-compose.yml).
    let bind = env_or("BIND_ADDR", "127.0.0.1");
    let client_dir = env_or("CLIENT_DIR", "client/dist");
    let content_dir = env_or("CONTENT_DIR", "content");
    let chapters_dir = env_or("CHAPTERS_DIR", "chapters");
    let addr = format!("{bind}:{port}");

    ensure_working_files(&chapters_dir);

    let server = Arc::new(Server::http(&addr).expect("failed to bind address"));
    // 0.0.0.0 is a bind wildcard, not a browseable host (Safari refuses it),
    // so advertise localhost when we're listening on every interface.
    let display_host = if bind == "0.0.0.0" { "localhost" } else { &bind };
    println!("rust-book-course server → http://{display_host}:{port}");
    println!("  CLIENT_DIR={client_dir}  CONTENT_DIR={content_dir}  CHAPTERS_DIR={chapters_dir}");

    let progress = Arc::new(Mutex::new(HashMap::<String, bool>::new()));

    // A handful of workers share the server so a cold first load's parallel chunk
    // requests are served concurrently, not serialized behind one another.
    // Bounded so a burst of "Check" (cargo) requests can't fan out into too many
    // simultaneous compiles.
    let workers = thread::available_parallelism().map(|n| n.get()).unwrap_or(4).clamp(2, 8);
    let mut handles = Vec::with_capacity(workers);
    for _ in 0..workers {
        let server = Arc::clone(&server);
        let progress = Arc::clone(&progress);
        let client_dir = client_dir.clone();
        let content_dir = content_dir.clone();
        let chapters_dir = chapters_dir.clone();
        handles.push(thread::spawn(move || {
            serve(&server, &progress, &client_dir, &content_dir, &chapters_dir)
        }));
    }
    for handle in handles {
        let _ = handle.join();
    }
}

/// One worker: pull requests off the shared server and handle them until shutdown.
fn serve(
    server: &Server,
    progress: &Mutex<HashMap<String, bool>>,
    client_dir: &str,
    content_dir: &str,
    chapters_dir: &str,
) {
    loop {
        let mut request = match server.recv() {
            Ok(req) => req,
            Err(_) => break,
        };
        let url = request.url().to_string();
        let path = url.split('?').next().unwrap_or("/").to_string();
        let method = request.method().clone();
        let segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        // Only PUT carries a body we care about (saving the editor's contents).
        // Cap the read — an exercise file is at most a few KB.
        const MAX_BODY: u64 = 1024 * 1024;
        let mut body = String::new();
        if method == Method::Put {
            let _ = request.as_reader().take(MAX_BODY).read_to_string(&mut body);
        }

        let response = handle(
            &method,
            &segs,
            &path,
            &body,
            client_dir,
            content_dir,
            chapters_dir,
            progress,
        );
        let _ = request.respond(response);
    }
}
