use ch06_enums_pattern_matching::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "6");
}
