use ch09_error_handling::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "9");
}
