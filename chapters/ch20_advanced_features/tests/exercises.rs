use ch20_advanced_features::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "20");
}
