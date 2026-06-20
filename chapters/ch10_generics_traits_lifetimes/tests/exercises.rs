use ch10_generics_traits_lifetimes::chapter_number;

#[test]
fn reports_chapter_number() {
    assert_eq!(chapter_number(), "10");
}
