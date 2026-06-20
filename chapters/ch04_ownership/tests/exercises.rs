use ch04_ownership::{combine, count_up, first_word, window, Counter};

// --- Exercise 1: ownership & moves -----------------------------------------

#[test]
fn combine_joins_with_a_space() {
    let a = String::from("hello");
    let b = String::from("world");
    assert_eq!(combine(a, b), "hello world");
}

#[test]
fn combine_handles_empty_pieces() {
    assert_eq!(combine(String::new(), String::from("x")), " x");
    assert_eq!(combine(String::from("x"), String::new()), "x ");
    assert_eq!(combine(String::new(), String::new()), " ");
}

// --- Exercise 2: references & borrowing -------------------------------------

#[test]
fn value_reads_through_shared_borrow() {
    let c = Counter::new(7);
    // A shared borrow can be taken many times.
    assert_eq!(c.value(), 7);
    assert_eq!(c.value(), 7);
}

#[test]
fn count_up_mutates_through_mut_borrow() {
    let mut c = Counter::new(0);
    count_up(&mut c, 5);
    count_up(&mut c, 10);
    assert_eq!(c.value(), 15);
    // The caller still owns `c` after the mutable borrows ended.
    assert_eq!(c, Counter::new(15));
}

// --- Exercise 3: the slice type ---------------------------------------------

#[test]
fn first_word_returns_up_to_first_space() {
    assert_eq!(first_word("hello world"), "hello");
}

#[test]
fn first_word_handles_single_word_and_empty() {
    assert_eq!(first_word("solo"), "solo");
    assert_eq!(first_word(""), "");
    assert_eq!(first_word("  spaces first"), "");
}

#[test]
fn window_borrows_a_subslice() {
    let data = [10, 20, 30, 40, 50];
    assert_eq!(window(&data, 1, 3), &[20, 30, 40]);
    assert_eq!(window(&data, 0, 5), &data);
}

#[test]
fn window_clamps_instead_of_panicking() {
    let data = [10, 20, 30, 40, 50];
    assert_eq!(window(&data, 4, 10), &[50]);
    assert_eq!(window(&data, 9, 2), &[] as &[i32]);
    assert_eq!(window(&[] as &[i32], 0, 3), &[] as &[i32]);
}
