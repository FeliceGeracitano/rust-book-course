use ch21_web_server::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "21");
}
