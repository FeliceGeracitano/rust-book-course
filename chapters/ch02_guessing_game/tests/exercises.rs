use ch02_guessing_game::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "2");
}
