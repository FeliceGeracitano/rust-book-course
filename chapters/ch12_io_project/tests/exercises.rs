use ch12_io_project::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "12");
}
