use ch18_oop::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "18");
}
