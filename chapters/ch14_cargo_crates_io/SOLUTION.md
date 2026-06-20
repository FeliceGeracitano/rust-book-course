# Chapter 14 — Solutions

```rust
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemVer {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

impl SemVer {
    pub fn parse(input: &str) -> Result<SemVer, String> {
        let mut parts = input.split('.');
        let major = next_number(&mut parts)?;
        let minor = next_number(&mut parts)?;
        let patch = next_number(&mut parts)?;
        if parts.next().is_some() {
            return Err(format!("too many parts in version: {input:?}"));
        }
        Ok(SemVer { major, minor, patch })
    }

    pub fn is_compatible_with(&self, other: &SemVer) -> bool {
        if other < self {
            return false;
        }
        if self.major != other.major {
            return false;
        }
        // 0.x.y is special: each minor bump may break the API.
        if self.major == 0 && self.minor != other.minor {
            return false;
        }
        true
    }
}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SemVer {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
    }
}

fn next_number<'a>(parts: &mut impl Iterator<Item = &'a str>) -> Result<u64, String> {
    let part = parts.next().ok_or_else(|| "missing version part".to_string())?;
    if part.is_empty() {
        return Err("empty version part".to_string());
    }
    part.parse::<u64>()
        .map_err(|_| format!("not a number: {part:?}"))
}

pub fn compatible_update(versions: &[SemVer], req: &SemVer) -> Option<SemVer> {
    versions
        .iter()
        .copied()
        .filter(|candidate| req.is_compatible_with(candidate))
        .max()
}

pub fn opt_level(profile: &str) -> Option<u8> {
    match profile {
        "dev" | "test" => Some(0),
        "release" | "bench" => Some(3),
        _ => None,
    }
}

pub fn subcommand_binary(subcommand: &str) -> Result<String, String> {
    if subcommand.is_empty() {
        return Err("subcommand name cannot be empty".to_string());
    }
    if subcommand.starts_with("cargo-") {
        return Err(format!(
            "subcommand {subcommand:?} should not include the cargo- prefix"
        ));
    }
    Ok(format!("cargo-{subcommand}"))
}
```
