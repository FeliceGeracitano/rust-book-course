use ch14_cargo_crates_io::{compatible_update, opt_level, subcommand_binary, SemVer};

fn v(s: &str) -> SemVer {
    SemVer::parse(s).expect("valid version")
}

// ---- SemVer::parse -------------------------------------------------------

#[test]
fn parses_a_three_part_version() {
    let parsed = SemVer::parse("1.4.0").unwrap();
    assert_eq!(
        parsed,
        SemVer {
            major: 1,
            minor: 4,
            patch: 0
        }
    );
}

#[test]
fn parses_multi_digit_components() {
    let parsed = SemVer::parse("10.20.300").unwrap();
    assert_eq!((parsed.major, parsed.minor, parsed.patch), (10, 20, 300));
}

#[test]
fn rejects_malformed_versions() {
    assert!(SemVer::parse("1.2").is_err()); // too few parts
    assert!(SemVer::parse("1.2.3.4").is_err()); // too many parts
    assert!(SemVer::parse("1.2.x").is_err()); // non-numeric
    assert!(SemVer::parse("1..0").is_err()); // empty part
    assert!(SemVer::parse("").is_err()); // empty input
}

// ---- ordering ------------------------------------------------------------

#[test]
fn orders_versions_field_by_field() {
    assert!(v("1.0.0") < v("1.0.1"));
    assert!(v("1.2.0") < v("1.10.0"));
    assert!(v("2.0.0") > v("1.99.99"));
    assert_eq!(v("1.2.3"), v("1.2.3"));
}

#[test]
fn max_picks_the_newest_version() {
    let mut versions = [v("1.4.1"), v("1.9.0"), v("1.2.0")];
    versions.sort();
    assert_eq!(versions, [v("1.2.0"), v("1.4.1"), v("1.9.0")]);
}

// ---- is_compatible_with --------------------------------------------------

#[test]
fn caret_compatibility_for_stable_releases() {
    let base = v("1.2.0");
    assert!(base.is_compatible_with(&v("1.2.0"))); // itself
    assert!(base.is_compatible_with(&v("1.5.0"))); // newer minor
    assert!(base.is_compatible_with(&v("1.2.9"))); // newer patch
    assert!(!base.is_compatible_with(&v("2.0.0"))); // major bump breaks
    assert!(!base.is_compatible_with(&v("1.1.0"))); // older than base
}

#[test]
fn zero_x_releases_are_stricter() {
    let base = v("0.3.0");
    assert!(base.is_compatible_with(&v("0.3.5"))); // patch ok
    assert!(!base.is_compatible_with(&v("0.4.0"))); // minor bump breaks in 0.x
    assert!(!base.is_compatible_with(&v("0.2.0"))); // older than base
}

// ---- compatible_update ---------------------------------------------------

#[test]
fn chooses_highest_compatible_release() {
    let published = [v("1.2.0"), v("1.4.1"), v("1.9.0"), v("2.0.0")];
    assert_eq!(compatible_update(&published, &v("1.2.0")), Some(v("1.9.0")));
}

#[test]
fn returns_none_when_nothing_is_compatible() {
    let published = [v("2.0.0"), v("3.1.0")];
    assert_eq!(compatible_update(&published, &v("1.0.0")), None);
}

// ---- opt_level -----------------------------------------------------------

#[test]
fn maps_profiles_to_default_opt_levels() {
    assert_eq!(opt_level("dev"), Some(0));
    assert_eq!(opt_level("test"), Some(0));
    assert_eq!(opt_level("release"), Some(3));
    assert_eq!(opt_level("bench"), Some(3));
    assert_eq!(opt_level("ship-it"), None);
}

// ---- subcommand_binary ---------------------------------------------------

#[test]
fn builds_the_cargo_prefixed_binary_name() {
    assert_eq!(subcommand_binary("nm"), Ok("cargo-nm".to_string()));
    assert_eq!(
        subcommand_binary("frobnicate"),
        Ok("cargo-frobnicate".to_string())
    );
}

#[test]
fn rejects_empty_or_already_prefixed_subcommands() {
    assert!(subcommand_binary("").is_err());
    assert!(subcommand_binary("cargo-nm").is_err());
}
