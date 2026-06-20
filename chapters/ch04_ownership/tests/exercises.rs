use ch04_ownership::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "4");
}
