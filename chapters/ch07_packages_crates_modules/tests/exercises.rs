use ch07_packages_crates_modules::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "7");
}
