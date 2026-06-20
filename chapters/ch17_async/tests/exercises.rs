use ch17_async::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "17");
}
