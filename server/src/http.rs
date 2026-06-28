//! HTTP plumbing: response helpers and static-file serving.

use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use tiny_http::{Header, Response};

/// A fully-buffered response body — what every handler returns.
pub type Body = Response<Cursor<Vec<u8>>>;

pub fn header(content_type: &str) -> Header {
    Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes())
        .expect("static header is always valid")
}

/// Path segments must be lowercase alphanumeric + underscore — blocks traversal.
pub fn valid_seg(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// Hashed assets are immutable and cache forever; everything else (notably
/// index.html) must revalidate so a rebuild's new asset hashes are always picked
/// up — otherwise a stale cached index.html points at chunks that no longer exist
/// (blank page until a manual refresh).
fn cache_header(rel: &str) -> Header {
    let value = if rel.starts_with("assets/") {
        "public, max-age=31536000, immutable"
    } else {
        "no-cache"
    };
    Header::from_bytes(&b"Cache-Control"[..], value.as_bytes()).expect("static header is valid")
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

pub fn json(body: String) -> Body {
    Response::from_string(body).with_header(header("application/json"))
}

pub fn not_found() -> Body {
    Response::from_string("not found").with_status_code(404)
}

/// Serve a static asset from `client_dir`, falling back to `index.html` for
/// extension-less paths (client-side routing).
pub fn serve_static(client_dir: &str, url_path: &str) -> Body {
    let rel = url_path.trim_start_matches('/');
    let rel = if rel.is_empty() { "index.html" } else { rel };
    if rel.contains("..") {
        return not_found();
    }
    let mut path = PathBuf::from(client_dir);
    path.push(rel);
    if path.is_file() {
        return match fs::read(&path) {
            Ok(bytes) => Response::from_data(bytes)
                .with_header(header(content_type_for(rel)))
                .with_header(cache_header(rel)),
            Err(_) => not_found(),
        };
    }
    // SPA fallback: a path without a file extension is a client route.
    if !rel.contains('.') {
        if let Ok(bytes) = fs::read(Path::new(client_dir).join("index.html")) {
            return Response::from_data(bytes)
                .with_header(header("text/html; charset=utf-8"))
                .with_header(cache_header("index.html"));
        }
    }
    not_found()
}
