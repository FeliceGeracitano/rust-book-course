use ch11_testing::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "11");
}
