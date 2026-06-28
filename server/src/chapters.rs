//! The `chapters/` Cargo workspace: running cargo and materializing working files.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Bump source mtimes so cargo always recompiles. On macOS Docker bind mounts a
/// freshly-edited file's mtime can lag its content, so cargo's mtime fingerprint
/// may skip a rebuild and reuse a stale binary.
pub fn touch_sources(chapters_dir: &str, krate: &str) {
    let crate_dir = Path::new(chapters_dir).join(krate);
    let _ = Command::new("find")
        .args([crate_dir.to_string_lossy().as_ref(), "-name", "*.rs", "-exec", "touch", "{}", "+"])
        .status();
}

/// Run a cargo subcommand in the workspace and capture pass/stdout/stderr.
pub fn run_cargo(chapters_dir: &str, args: &[&str]) -> (bool, String, String) {
    match Command::new("cargo").args(args).current_dir(chapters_dir).output() {
        Ok(o) => (
            o.status.success(),
            String::from_utf8_lossy(&o.stdout).into_owned(),
            String::from_utf8_lossy(&o.stderr).into_owned(),
        ),
        Err(e) => (false, String::new(), format!("failed to run cargo: {e}")),
    }
}

pub fn cargo_json(pass: bool, stdout: String, stderr: String) -> String {
    serde_json::json!({ "pass": pass, "stdout": stdout, "stderr": stderr }).to_string()
}

/// Path to a chapter's editable exercise file.
pub fn lib_path(chapters_dir: &str, krate: &str) -> PathBuf {
    Path::new(chapters_dir).join(krate).join("src").join("lib.rs")
}

/// The working `src/lib.rs` is git-ignored (so a learner's edits never show up in
/// git). Materialize it from the committed pristine `.exercise.rs` when missing,
/// so cargo always has something to compile.
pub fn ensure_working_files(chapters_dir: &str) {
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
