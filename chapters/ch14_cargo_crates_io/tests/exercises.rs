use ch14_cargo_crates_io::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "14");
}
