//! rust-book-course learning server.
//!
//! A small synchronous `tiny_http` server that:
//!   - serves the built React client (`CLIENT_DIR`),
//!   - exposes a tiny JSON API for the table of contents, lessons, and progress,
//!   - runs `cargo test -p <crate>` for the "Check" button.
//!
//! It is intentionally single-threaded: one local learner, one request at a time.
//! Reading this file is also a preview of Chapter 21 (building a web server).

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Command;

use tiny_http::{Header, Method, Response, Server};

type Body = Response<Cursor<Vec<u8>>>;

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn header(content_type: &str) -> Header {
    Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes())
        .expect("static header is always valid")
}

/// Path segments must be lowercase alphanumeric + underscore — blocks traversal.
fn valid_seg(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn content_type_for(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" | "map" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "ico" => "image/x-icon",
        "woff2" => "font/woff2",
        "woff" => "font/woff",
        "wasm" => "application/wasm",
        _ => "application/octet-stream",
    }
}

fn json(body: String) -> Body {
    Response::from_string(body).with_header(header("application/json"))
}

fn not_found() -> Body {
    Response::from_string("not found").with_status_code(404)
}

/// Serve a static asset from `client_dir`, falling back to `index.html` for
/// extension-less paths (client-side routing).
fn serve_static(client_dir: &str, url_path: &str) -> Body {
    let rel = url_path.trim_start_matches('/');
    let rel = if rel.is_empty() { "index.html" } else { rel };
    if rel.contains("..") {
        return not_found();
    }
    let mut path = PathBuf::from(client_dir);
    path.push(rel);
    if path.is_file() {
        return match fs::read(&path) {
            Ok(bytes) => Response::from_data(bytes).with_header(header(content_type_for(rel))),
            Err(_) => not_found(),
        };
    }
    // SPA fallback: a path without a file extension is a client route.
    if !rel.contains('.') {
        if let Ok(bytes) = fs::read(Path::new(client_dir).join("index.html")) {
            return Response::from_data(bytes).with_header(header("text/html; charset=utf-8"));
        }
    }
    not_found()
}

/// Run the chapter's tests and report pass/fail with captured output.
fn run_check(chapters_dir: &str, krate: &str) -> (bool, String, String) {
    match Command::new("cargo")
        .args(["test", "-p", krate, "--quiet"])
        .current_dir(chapters_dir)
        .output()
    {
        Ok(o) => (
            o.status.success(),
            String::from_utf8_lossy(&o.stdout).into_owned(),
            String::from_utf8_lossy(&o.stderr).into_owned(),
        ),
        Err(e) => (false, String::new(), format!("failed to run cargo: {e}")),
    }
}

fn handle(
    method: &Method,
    segs: &[&str],
    path: &str,
    client_dir: &str,
    content_dir: &str,
    chapters_dir: &str,
    progress: &mut HashMap<String, bool>,
) -> Body {
    let is_get = method == &Method::Get;
    let is_post = method == &Method::Post;

    match segs {
        ["api", "chapters"] if is_get => match fs::read_to_string(Path::new(content_dir).join("course.json")) {
            Ok(s) => json(s),
            Err(_) => not_found(),
        },
        ["api", "lesson", krate, sub] if is_get => {
            if !valid_seg(krate) || !valid_seg(sub) {
                return not_found();
            }
            let file = Path::new(content_dir).join(krate).join(format!("{sub}.md"));
            match fs::read_to_string(file) {
                Ok(s) => Response::from_string(s).with_header(header("text/markdown; charset=utf-8")),
                Err(_) => not_found(),
            }
        }
        ["api", "check", krate] if is_post => {
            if !valid_seg(krate) {
                return not_found();
            }
            let (pass, stdout, stderr) = run_check(chapters_dir, krate);
            progress.insert((*krate).to_string(), pass);
            json(serde_json::json!({ "pass": pass, "stdout": stdout, "stderr": stderr }).to_string())
        }
        ["api", "progress"] if is_get => {
            json(serde_json::to_string(progress).unwrap_or_else(|_| "{}".to_string()))
        }
        _ if is_get => serve_static(client_dir, path),
        _ => not_found(),
    }
}

fn main() {
    let port = env_or("PORT", "8080");
    let client_dir = env_or("CLIENT_DIR", "client/dist");
    let content_dir = env_or("CONTENT_DIR", "content");
    let chapters_dir = env_or("CHAPTERS_DIR", "chapters");
    let addr = format!("0.0.0.0:{port}");

    let server = Server::http(&addr).expect("failed to bind address");
    println!("rust-book-course server → http://{addr}");
    println!("  CLIENT_DIR={client_dir}  CONTENT_DIR={content_dir}  CHAPTERS_DIR={chapters_dir}");

    let mut progress: HashMap<String, bool> = HashMap::new();

    for request in server.incoming_requests() {
        let url = request.url().to_string();
        let path = url.split('?').next().unwrap_or("/").to_string();
        let method = request.method().clone();
        let segs: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        let response = handle(
            &method,
            &segs,
            &path,
            &client_dir,
            &content_dir,
            &chapters_dir,
            &mut progress,
        );
        let _ = request.respond(response);
    }
}
