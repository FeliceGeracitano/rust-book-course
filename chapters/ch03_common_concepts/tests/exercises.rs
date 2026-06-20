use ch03_common_concepts::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "3");
}
