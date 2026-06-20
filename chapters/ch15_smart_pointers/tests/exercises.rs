use std::cell::RefCell;

use ch15_smart_pointers::List::{Cons, Nil};
use ch15_smart_pointers::{Counter, MyBox, Tracker, drop_now, hello};

// --- Exercise 1: Box<T> recursive cons list ---------------------------------

#[test]
fn cons_list_sums_values() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    assert_eq!(list.sum(), 6);
}

#[test]
fn empty_list_sums_to_zero() {
    let list = Nil;
    assert_eq!(list.sum(), 0);
}

// --- Exercise 2: Deref and deref coercion -----------------------------------

#[test]
fn mybox_derefs_to_inner_value() {
    let b = MyBox::new(5);
    assert_eq!(*b, 5);
}

#[test]
fn deref_coercion_passes_mybox_string_as_str() {
    let name = MyBox::new(String::from("Ferris"));
    // &MyBox<String> -> &String -> &str via two deref coercions.
    assert_eq!(hello(&name), "Hello, Ferris!");
}

// --- Exercise 3: Drop order -------------------------------------------------

#[test]
fn values_drop_in_reverse_declaration_order() {
    let log = RefCell::new(Vec::new());
    {
        let _a = Tracker::new("a", &log);
        let _b = Tracker::new("b", &log);
        let _c = Tracker::new("c", &log);
    } // dropped here: c, then b, then a
    assert_eq!(*log.borrow(), vec!["c", "b", "a"]);
}

#[test]
fn drop_now_cleans_up_early() {
    let log = RefCell::new(Vec::new());
    let first = Tracker::new("first", &log);
    let second = Tracker::new("second", &log);
    drop_now(first); // "first" recorded immediately
    assert_eq!(*log.borrow(), vec!["first"]);
    drop(second); // "second" recorded now
    assert_eq!(*log.borrow(), vec!["first", "second"]);
}

// --- Exercise 4: Rc<T> + RefCell<T> -----------------------------------------

#[test]
fn clones_share_one_counter() {
    let a = Counter::new(0);
    let b = a.clone();
    a.increment();
    b.increment();
    a.increment();
    assert_eq!(a.get(), 3);
    assert_eq!(b.get(), 3);
}

#[test]
fn strong_count_tracks_live_handles() {
    let a = Counter::new(10);
    assert_eq!(a.handles(), 1);
    let b = a.clone();
    assert_eq!(a.handles(), 2);
    {
        let _c = a.clone();
        assert_eq!(a.handles(), 3);
    } // _c dropped here
    assert_eq!(a.handles(), 2);
    drop(b);
    assert_eq!(a.handles(), 1);
}
