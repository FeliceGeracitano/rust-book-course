use ch15_smart_pointers::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "15");
}
