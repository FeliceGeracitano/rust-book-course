//! Chapter 14 — More about Cargo and Crates.io
//!
//! This is a conceptual chapter, so the exercises drill the *pure logic* that
//! sits underneath Cargo's features — no real builds, network, or file I/O:
//!
//! - [`SemVer`]: parse and compare `MAJOR.MINOR.PATCH` versions the way
//!   crates.io and `cargo update` reason about releases.
//! - [`compatible_update`]: pick the highest version a caret (`^`) requirement
//!   allows, mirroring Cargo's default dependency resolution.
//! - [`opt_level`]: map a build profile name to its default optimization level,
//!   the heart of "Customizing Builds with Release Profiles".
//! - [`subcommand_binary`]: turn `cargo frobnicate` into the binary name Cargo
//!   actually looks for, the rule behind custom Cargo commands.
//!
//! Complete each `todo!()`, then run `cargo test -p ch14_cargo_crates_io`.

use std::cmp::Ordering;

/// A parsed [Semantic Version](https://semver.org/): `MAJOR.MINOR.PATCH`.
///
/// crates.io versions and Cargo's resolver are built on SemVer, so being able
/// to parse and order versions is the foundation for everything in this chapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SemVer {
    /// Breaking-change counter. Bumped for incompatible API changes.
    pub major: u64,
    /// Feature counter. Bumped for backwards-compatible additions.
    pub minor: u64,
    /// Fix counter. Bumped for backwards-compatible bug fixes.
    pub patch: u64,
}

impl SemVer {
    /// Parse a version string like `"1.4.0"` into a [`SemVer`].
    ///
    /// Returns `Err` with a short message if the string is not exactly three
    /// dot-separated, non-empty, base-10 numbers (e.g. `"1.2"`, `"1.2.x"`,
    /// or `"1..0"` are all errors). Pre-release/build metadata is out of scope.
    ///
    /// ```
    /// use ch14_cargo_crates_io::SemVer;
    /// let v = SemVer::parse("1.4.0").unwrap();
    /// assert_eq!((v.major, v.minor, v.patch), (1, 4, 0));
    /// assert!(SemVer::parse("1.2").is_err());
    /// ```
    pub fn parse(input: &str) -> Result<SemVer, String> {
        // TODO: split `input` on '.', read exactly three base-10 numbers, and
        // error on missing/extra/empty/non-numeric parts. The `next_number`
        // helper below is handy here.
        let _ = input;
        todo!("parse a MAJOR.MINOR.PATCH version string into a SemVer")
    }

    /// Is `other` a backwards-compatible upgrade *from* `self` under SemVer's
    /// caret rules?
    ///
    /// For `1.0.0` and above, compatible means the same `major` and a version
    /// that is greater than or equal to `self`. For `0.x` releases the API is
    /// considered unstable, so compatibility also requires the same `minor`.
    /// A version is always compatible with itself.
    ///
    /// ```
    /// use ch14_cargo_crates_io::SemVer;
    /// let base = SemVer::parse("1.2.0").unwrap();
    /// assert!(base.is_compatible_with(&SemVer::parse("1.5.0").unwrap()));
    /// assert!(!base.is_compatible_with(&SemVer::parse("2.0.0").unwrap()));
    /// assert!(!base.is_compatible_with(&SemVer::parse("1.1.0").unwrap()));
    /// ```
    pub fn is_compatible_with(&self, other: &SemVer) -> bool {
        // TODO: return false if `other` is older than `self` or has a different
        // `major`; for 0.x versions also require an equal `minor`. Otherwise true.
        let _ = other;
        todo!("decide caret (^) compatibility between two versions")
    }
}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SemVer {
    /// Order versions field by field: `major`, then `minor`, then `patch`.
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: compare `major`, then `minor`, then `patch` (Ordering::then helps).
        let _ = other;
        todo!("order two versions field by field")
    }
}

/// Parse the next dot-separated component as a `u64`, rejecting empty or
/// non-numeric segments.
fn next_number<'a>(parts: &mut impl Iterator<Item = &'a str>) -> Result<u64, String> {
    // TODO: pull the next component; error if it's missing, empty, or not a u64.
    let _ = parts;
    todo!("parse the next dot-separated component as a u64")
}

/// Given a list of published `versions` and a caret requirement `req` (as in
/// `cargo`'s default `version = "1.2.0"`, which means `^1.2.0`), return the
/// highest version that satisfies the requirement, or `None` if nothing fits.
///
/// This mirrors how `cargo update` chooses an upgrade: stay
/// SemVer-compatible with the requirement and pick the newest such release.
///
/// ```
/// use ch14_cargo_crates_io::{compatible_update, SemVer};
/// let published = ["1.2.0", "1.4.1", "1.9.0", "2.0.0"]
///     .iter()
///     .map(|v| SemVer::parse(v).unwrap())
///     .collect::<Vec<_>>();
/// let req = SemVer::parse("1.2.0").unwrap();
/// assert_eq!(compatible_update(&published, &req), SemVer::parse("1.9.0").ok());
/// ```
pub fn compatible_update(versions: &[SemVer], req: &SemVer) -> Option<SemVer> {
    // TODO: keep only versions compatible with `req`, then return the maximum.
    let _ = (versions, req);
    todo!("pick the highest version satisfying the caret requirement")
}

/// Return the default optimization level Cargo applies to a build `profile`.
///
/// These are Cargo's documented defaults that you'd otherwise tweak in
/// `Cargo.toml` under `[profile.*]`:
///
/// | profile   | `opt-level` |
/// |-----------|-------------|
/// | `dev`     | `0`         |
/// | `test`    | `0`         |
/// | `release` | `3`         |
/// | `bench`   | `3`         |
///
/// Unknown profile names return `None`.
///
/// ```
/// use ch14_cargo_crates_io::opt_level;
/// assert_eq!(opt_level("dev"), Some(0));
/// assert_eq!(opt_level("release"), Some(3));
/// assert_eq!(opt_level("ship-it"), None);
/// ```
pub fn opt_level(profile: &str) -> Option<u8> {
    // TODO: match the profile name to its default opt-level (see the table above).
    let _ = profile;
    todo!("map a profile name to its default opt-level")
}

/// Resolve the external binary name Cargo would invoke for a custom subcommand.
///
/// When you run `cargo something`, and `something` is not built in, Cargo looks
/// on your `PATH` for an executable named `cargo-something`. Return that binary
/// name, or `Err` if `subcommand` is empty or itself already starts with the
/// `cargo-` prefix (which would be a user mistake).
///
/// ```
/// use ch14_cargo_crates_io::subcommand_binary;
/// assert_eq!(subcommand_binary("nm"), Ok("cargo-nm".to_string()));
/// assert!(subcommand_binary("").is_err());
/// assert!(subcommand_binary("cargo-nm").is_err());
/// ```
pub fn subcommand_binary(subcommand: &str) -> Result<String, String> {
    // TODO: reject empty names and names already starting with "cargo-";
    // otherwise return the "cargo-<subcommand>" binary name.
    let _ = subcommand;
    todo!("build the cargo-<subcommand> binary name (or error)")
}
