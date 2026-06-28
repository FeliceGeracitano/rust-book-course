//! Request router: the JSON API, plus a static-file fallback for everything else.
//!
//! tiny_http has no routing layer, so we match on the URL's path segments
//! ourselves. Each arm returns a [`Body`].

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use tiny_http::{Method, Response};

use crate::chapters::{cargo_json, lib_path, run_cargo, touch_sources};
use crate::http::{header, json, not_found, serve_static, valid_seg, Body};

pub fn handle(
    method: &Method,
    segs: &[&str],
    path: &str,
    body: &str,
    client_dir: &str,
    content_dir: &str,
    chapters_dir: &str,
    progress: &Mutex<HashMap<String, bool>>,
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
            // Lock only to record the result — never while cargo runs, so a slow
            // check doesn't block other requests.
            progress.lock().unwrap().insert((*krate).to_string(), pass);
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
            let snapshot = progress.lock().unwrap();
            json(serde_json::to_string(&*snapshot).unwrap_or_else(|_| "{}".to_string()))
        }
        ["api", "config"] if is_get => {
            // Lets the client build an editor deep-link to the file on the HOST.
            let host_repo_dir = env::var("HOST_REPO_DIR").unwrap_or_default();
            let editor_scheme = env::var("EDITOR_SCHEME").unwrap_or_else(|_| "vscode".to_string());
            let lsp_url = env::var("LSP_URL").unwrap_or_default();
            // The container path rust-analyzer indexes; the client builds file://
            // document URIs from it so RA resolves them inside the workspace.
            let chapters_for_lsp = env::var("CHAPTERS_DIR").unwrap_or_else(|_| "chapters".to_string());
            json(
                serde_json::json!({
                    "hostRepoDir": host_repo_dir,
                    "editorScheme": editor_scheme,
                    "lspUrl": lsp_url,
                    "chaptersDir": chapters_for_lsp,
                })
                .to_string(),
            )
        }
        _ if is_get => serve_static(client_dir, path),
        _ => not_found(),
    }
}
