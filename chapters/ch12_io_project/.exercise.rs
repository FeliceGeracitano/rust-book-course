//! Chapter 12 — An I/O Project: Building a Command Line Program
//!
//! These exercises distill the heart of `minigrep`: the pure search logic and
//! the configuration parsing that the Book builds across the chapter. They use
//! no real files, stdin, or environment access, so they stay fast and
//! deterministic. Complete each function/method, then run
//! `cargo test -p ch12_io_project`.

/// 12.4 Case-sensitive search.
///
/// Return every line in `contents` that contains `query` as a substring, in
/// their original order. The match is case-sensitive: `"rust"` does not match
/// `"Rust"`. Lines are borrowed from `contents`, so no allocation of the line
/// text is needed.
///
/// ```
/// use ch12_io_project::search;
/// let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
/// assert_eq!(search("ru", contents), vec!["Trust me."]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // TODO: split `contents` into lines, keep the ones that `contains(query)`,
    // and `collect()` them into a Vec (case-sensitive comparison).
    todo!("return the lines of `contents` that contain `query`")
}

/// 12.5 Case-insensitive search.
///
/// Like [`search`], but the comparison ignores ASCII case, so `"rUsT"` matches
/// `"Rust"`, `"RUST"`, and `"rust"`. The returned slices still borrow the
/// original, un-lowercased lines from `contents`.
///
/// ```
/// use ch12_io_project::search_case_insensitive;
/// let contents = "Rust:\nTrust me.\nNOBODY expects\nthe Rusty knife.";
/// assert_eq!(
///     search_case_insensitive("rUsT", contents),
///     vec!["Rust:", "Trust me.", "the Rusty knife."],
/// );
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // TODO: lowercase the query once, then keep each line whose lowercased
    // form `contains` it. Return the original (un-lowercased) lines.
    todo!("return the lines of `contents` that contain `query`, ignoring case")
}

/// 12.3 / 12.5 Parsed command-line configuration for `minigrep`.
///
/// In the Book this is built by reading `std::env::args` and the
/// `IGNORE_CASE` environment variable. Here the same logic lives in a pure
/// constructor so it can be tested without touching the real environment.
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    /// The pattern to search for.
    pub query: String,
    /// The path of the file to search (stored, never opened, in these tests).
    pub file_path: String,
    /// Whether the search should ignore case.
    pub ignore_case: bool,
}

impl Config {
    /// 12.3 Build a `Config` from the program's arguments.
    ///
    /// `args` is the argument list *excluding* the program name (i.e. what the
    /// Book skips with the first `args.next()`). It must contain exactly the
    /// query followed by the file path; `ignore_case` mirrors the
    /// `IGNORE_CASE` environment variable.
    ///
    /// Returns `Err` with a human-readable message when the wrong number of
    /// arguments is supplied:
    /// - fewer than two: `"not enough arguments"`
    /// - more than two: `"too many arguments"`
    ///
    /// ```
    /// use ch12_io_project::Config;
    /// let cfg = Config::build(&["needle", "poem.txt"], true).unwrap();
    /// assert_eq!(cfg.query, "needle");
    /// assert_eq!(cfg.file_path, "poem.txt");
    /// assert!(cfg.ignore_case);
    ///
    /// assert_eq!(Config::build(&["only_one"], false), Err("not enough arguments"));
    /// ```
    pub fn build(args: &[&str], ignore_case: bool) -> Result<Config, &'static str> {
        // TODO: return Err("not enough arguments") when fewer than 2 args,
        // Err("too many arguments") when more than 2, otherwise build the
        // Config from args[0] (query) and args[1] (file_path).
        todo!("validate the argument count and construct a Config")
    }

    /// 12.4 Run the search described by this config against `contents`.
    ///
    /// Dispatches to [`search`] or [`search_case_insensitive`] depending on
    /// [`Config::ignore_case`]. This is the pure core of the Book's `run`
    /// function, with the file already loaded into `contents`.
    ///
    /// ```
    /// use ch12_io_project::Config;
    /// let cfg = Config::build(&["rust", "x.txt"], true).unwrap();
    /// assert_eq!(cfg.matches("Trust\nno one"), vec!["Trust"]);
    /// ```
    pub fn matches<'a>(&self, contents: &'a str) -> Vec<&'a str> {
        // TODO: call search_case_insensitive when self.ignore_case is true,
        // otherwise call search, passing &self.query and contents.
        todo!("dispatch to the case-sensitive or case-insensitive search")
    }
}
