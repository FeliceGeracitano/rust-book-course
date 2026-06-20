use ch19_patterns_matching::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "19");
}
