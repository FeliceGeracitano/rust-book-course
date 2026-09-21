use integrations::{Origin, OriginError};

#[test]
fn accepts_bare_http_origins() {
    let origin = Origin::parse("http://127.0.0.1:9090").unwrap();
    assert_eq!(
        origin.join("/customers/c1").as_str(),
        "http://127.0.0.1:9090/customers/c1"
    );
    assert_eq!(
        origin.join("/fees?cents=5").as_str(),
        "http://127.0.0.1:9090/fees?cents=5"
    );
    assert!(Origin::parse("https://example.com/").is_ok());
}

#[test]
fn rejects_other_schemes_and_extra_parts() {
    assert_eq!(Origin::parse("ftp://x"), Err(OriginError::Scheme));
    for extra in [
        "http://u:p@x",
        "http://x/api",
        "http://x?y=1",
        "http://x#frag",
    ] {
        assert_eq!(Origin::parse(extra), Err(OriginError::Extra), "{extra}");
    }
    assert!(matches!(
        Origin::parse("not a url"),
        Err(OriginError::Invalid(_))
    ));
}
