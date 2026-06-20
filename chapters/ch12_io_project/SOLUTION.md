# Chapter 12 — Solutions

```rust
/// 12.4 Case-sensitive search: every line containing `query`, in order.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// 12.5 Case-insensitive search (ASCII case-folded comparison).
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

/// 12.3 / 12.5 Parsed command-line configuration for `minigrep`.
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// 12.3 Build a `Config` from the args (program name already stripped).
    pub fn build(args: &[&str], ignore_case: bool) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        if args.len() > 2 {
            return Err("too many arguments");
        }
        Ok(Config {
            query: args[0].to_string(),
            file_path: args[1].to_string(),
            ignore_case,
        })
    }

    /// 12.4 Run the search described by this config against `contents`.
    pub fn matches<'a>(&self, contents: &'a str) -> Vec<&'a str> {
        if self.ignore_case {
            search_case_insensitive(&self.query, contents)
        } else {
            search(&self.query, contents)
        }
    }
}
```
