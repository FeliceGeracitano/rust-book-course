use ch13_iterators_closures::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "13");
}
