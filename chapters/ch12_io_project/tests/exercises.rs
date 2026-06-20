use ch12_io_project::{search, search_case_insensitive, Config};

const POEM: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

#[test]
fn search_is_case_sensitive() {
    // "Rust:" has a capital R, so a lowercase "rust" query must not match it;
    // only "Trust me." contains the substring "rust".
    assert_eq!(search("rust", POEM), vec!["Trust me."]);
}

#[test]
fn search_returns_empty_when_no_match() {
    assert_eq!(search("python", POEM), Vec::<&str>::new());
}

#[test]
fn search_can_match_multiple_lines_in_order() {
    let contents = "alpha\nbeta\nalpha-beta\ngamma";
    assert_eq!(search("alpha", contents), vec!["alpha", "alpha-beta"]);
}

#[test]
fn case_insensitive_matches_regardless_of_case() {
    let contents = "Rust:\nTrust me.\nNOBODY expects\nthe Rusty knife.";
    assert_eq!(
        search_case_insensitive("rUsT", contents),
        vec!["Rust:", "Trust me.", "the Rusty knife."],
    );
}

#[test]
fn case_insensitive_returns_original_lines() {
    // The returned slices borrow the original (un-lowercased) text.
    assert_eq!(search_case_insensitive("SAFE", POEM), vec!["safe, fast, productive."]);
}

#[test]
fn config_build_parses_query_and_path() {
    let cfg = Config::build(&["needle", "poem.txt"], false).unwrap();
    assert_eq!(
        cfg,
        Config {
            query: "needle".to_string(),
            file_path: "poem.txt".to_string(),
            ignore_case: false,
        }
    );
}

#[test]
fn config_build_records_ignore_case_flag() {
    let cfg = Config::build(&["needle", "poem.txt"], true).unwrap();
    assert!(cfg.ignore_case);
}

#[test]
fn config_build_rejects_too_few_arguments() {
    assert_eq!(Config::build(&["only_one"], false), Err("not enough arguments"));
    assert_eq!(Config::build(&[], false), Err("not enough arguments"));
}

#[test]
fn config_build_rejects_too_many_arguments() {
    assert_eq!(
        Config::build(&["a", "b", "c"], false),
        Err("too many arguments")
    );
}

#[test]
fn config_matches_uses_case_sensitive_search_by_default() {
    let cfg = Config::build(&["rust", "x.txt"], false).unwrap();
    assert_eq!(cfg.matches(POEM), vec!["Trust me."]);
}

#[test]
fn config_matches_uses_case_insensitive_search_when_flag_set() {
    let cfg = Config::build(&["rust", "x.txt"], true).unwrap();
    assert_eq!(cfg.matches(POEM), vec!["Rust:", "Trust me."]);
}
