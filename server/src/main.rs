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
use std::io::{Cursor, Read};
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

/// Bump source mtimes so cargo always recompiles. On macOS Docker bind mounts a
/// freshly-edited file's mtime can lag its content, so cargo's mtime fingerprint
/// may skip a rebuild and reuse a stale binary.
fn touch_sources(chapters_dir: &str, krate: &str) {
    let crate_dir = Path::new(chapters_dir).join(krate);
    let _ = Command::new("find")
        .args([crate_dir.to_string_lossy().as_ref(), "-name", "*.rs", "-exec", "touch", "{}", "+"])
        .status();
}

/// Run a cargo subcommand in the workspace and capture pass/stdout/stderr.
fn run_cargo(chapters_dir: &str, args: &[&str]) -> (bool, String, String) {
    match Command::new("cargo").args(args).current_dir(chapters_dir).output() {
        Ok(o) => (
            o.status.success(),
            String::from_utf8_lossy(&o.stdout).into_owned(),
            String::from_utf8_lossy(&o.stderr).into_owned(),
        ),
        Err(e) => (false, String::new(), format!("failed to run cargo: {e}")),
    }
}

fn cargo_json(pass: bool, stdout: String, stderr: String) -> String {
    serde_json::json!({ "pass": pass, "stdout": stdout, "stderr": stderr }).to_string()
}

/// Path to a chapter's editable exercise file.
fn lib_path(chapters_dir: &str, krate: &str) -> PathBuf {
    Path::new(chapters_dir).join(krate).join("src").join("lib.rs")
}

/// The working `src/lib.rs` is git-ignored (so a learner's edits never show up in
/// git). Materialize it from the committed pristine `.exercise.rs` when missing,
/// so cargo always has something to compile.
fn ensure_working_files(chapters_dir: &str) {
    let Ok(entries) = fs::read_dir(chapters_dir) else {
        return;
    };
    for entry in entries.flatten() {
        let dir = entry.path();
        if !dir.is_dir() {
            continue;
        }
        let original = dir.join(".exercise.rs");
        let lib = dir.join("src").join("lib.rs");
        if original.exists() && !lib.exists() {
            if let Some(parent) = lib.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::copy(&original, &lib);
        }
    }
}

fn handle(
    method: &Method,
    segs: &[&str],
    path: &str,
    body: &str,
    client_dir: &str,
    content_dir: &str,
    chapters_dir: &str,
    progress: &mut HashMap<String, bool>,
) -> Body {
    let is_get = method == &Method::Get;
    let is_post = method == &Method::Post;
    let is_put = method == &Method::Put;

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
            touch_sources(chapters_dir, krate);
            let (pass, stdout, stderr) = run_cargo(chapters_dir, &["test", "-p", krate]);
            progress.insert((*krate).to_string(), pass);
            json(cargo_json(pass, stdout, stderr))
        }
        ["api", "clippy", krate] if is_post => {
            if !valid_seg(krate) {
                return not_found();
            }
            touch_sources(chapters_dir, krate);
            // `-D warnings` makes clippy exit non-zero on any lint, so `pass`
            // truly means "idiomatic & warning-free" for the learning badge.
            let (pass, stdout, stderr) =
                run_cargo(chapters_dir, &["clippy", "-p", krate, "--tests", "--", "-D", "warnings"]);
            json(cargo_json(pass, stdout, stderr))
        }
        ["api", "file", krate] if is_get => {
            if !valid_seg(krate) {
                return not_found();
            }
            match fs::read_to_string(lib_path(chapters_dir, krate)) {
                Ok(s) => Response::from_string(s).with_header(header("text/plain; charset=utf-8")),
                Err(_) => not_found(),
            }
        }
        ["api", "file", krate] if is_put => {
            if !valid_seg(krate) {
                return not_found();
            }
            match fs::write(lib_path(chapters_dir, krate), body) {
                Ok(_) => json(serde_json::json!({ "ok": true }).to_string()),
                Err(e) => json(serde_json::json!({ "ok": false, "error": e.to_string() }).to_string())
                    .with_status_code(500),
            }
        }
        ["api", "solution", krate] if is_get => {
            if !valid_seg(krate) {
                return not_found();
            }
            let p = Path::new(chapters_dir).join(krate).join("SOLUTION.md");
            match fs::read_to_string(p) {
                Ok(s) => Response::from_string(s).with_header(header("text/markdown; charset=utf-8")),
                Err(_) => not_found(),
            }
        }
        ["api", "reset", krate] if is_post => {
            // Restore the pristine exercise (.exercise.rs is never edited) over the
            // learner's working src/lib.rs, and return the restored content.
            if !valid_seg(krate) {
                return not_found();
            }
            let original = Path::new(chapters_dir).join(krate).join(".exercise.rs");
            match fs::read_to_string(&original) {
                Ok(content) => match fs::write(lib_path(chapters_dir, krate), &content) {
                    Ok(_) => Response::from_string(content)
                        .with_header(header("text/plain; charset=utf-8")),
                    Err(e) => Response::from_string(e.to_string()).with_status_code(500),
                },
                Err(_) => not_found(),
            }
        }
        ["api", "progress"] if is_get => {
            json(serde_json::to_string(progress).unwrap_or_else(|_| "{}".to_string()))
        }
        ["api", "config"] if is_get => {
            // Lets the client build an editor deep-link to the file on the HOST.
            let host_repo_dir = env::var("HOST_REPO_DIR").unwrap_or_default();
            let editor_scheme = env::var("EDITOR_SCHEME").unwrap_or_else(|_| "vscode".to_string());
            json(serde_json::json!({ "hostRepoDir": host_repo_dir, "editorScheme": editor_scheme }).to_string())
        }
        _ if is_get => serve_static(client_dir, path),
        _ => not_found(),
    }
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

    let server = Server::http(&addr).expect("failed to bind address");
    println!("rust-book-course server → http://{addr}");
    println!("  CLIENT_DIR={client_dir}  CONTENT_DIR={content_dir}  CHAPTERS_DIR={chapters_dir}");

    let mut progress: HashMap<String, bool> = HashMap::new();

    for mut request in server.incoming_requests() {
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
            &client_dir,
            &content_dir,
            &chapters_dir,
            &mut progress,
        );
        let _ = request.respond(response);
    }
}
