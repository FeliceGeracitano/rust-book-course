use ch05_structs::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "5");
}
