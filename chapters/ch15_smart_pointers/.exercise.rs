//! Chapter 15 — Smart Pointers
//!
//! These exercises drill the five ideas from the chapter: heap allocation and
//! recursion with `Box<T>`, custom dereferencing with `Deref`, deterministic
//! cleanup ordering with `Drop`, shared ownership with `Rc<T>`, and interior
//! mutability with `RefCell<T>`.
//!
//! Complete each `todo!()` in the items below, then run:
//!
//! ```text
//! cargo test -p ch15_smart_pointers
//! ```

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

// ---------------------------------------------------------------------------
// Exercise 1 — Box<T>: a recursive cons list
// ---------------------------------------------------------------------------

/// A classic *cons list*: each node holds an `i32` and the rest of the list.
///
/// Because a `List` could contain another `List`, the type is recursive and
/// would have infinite size if stored inline. Wrapping the tail in `Box<T>`
/// gives the node a known, pointer-sized field while the data lives on the heap.
///
/// # Examples
///
/// ```
/// use ch15_smart_pointers::List::{Cons, Nil};
///
/// let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
/// assert_eq!(list.sum(), 3);
/// ```
#[derive(Debug)]
pub enum List {
    /// A node holding a value and the boxed remainder of the list.
    Cons(i32, Box<List>),
    /// The end of the list.
    Nil,
}

impl List {
    /// Sums every value stored in the list, returning `0` for `Nil`.
    pub fn sum(&self) -> i32 {
        // TODO: match on `self`. For `Cons(value, rest)` return
        // `value + rest.sum()`; for `Nil` return 0.
        todo!("sum the cons list recursively")
    }
}

// ---------------------------------------------------------------------------
// Exercise 2 — Deref: build your own smart pointer
// ---------------------------------------------------------------------------

/// A minimal stand-in for `Box<T>` that owns a value on the (conceptual) heap.
///
/// Implementing [`Deref`] lets `*pointer` reach the inner value and unlocks
/// *deref coercion*, so a `&MyBox<String>` can be passed where `&str` is
/// expected.
///
/// # Examples
///
/// ```
/// use ch15_smart_pointers::MyBox;
///
/// let b = MyBox::new(5);
/// assert_eq!(*b, 5);
/// ```
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    /// Wraps `value` in a `MyBox`.
    pub fn new(value: T) -> MyBox<T> {
        // TODO: construct a `MyBox` holding `value` in its single field.
        todo!("wrap `value` in a MyBox")
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    /// Returns a reference to the wrapped value so `*my_box` works.
    fn deref(&self) -> &T {
        // TODO: return a reference to the inner value (`&self.0`).
        todo!("return a reference to the wrapped value")
    }
}

/// Greets `name`. Thanks to deref coercion, callers can pass a
/// `&MyBox<String>` and it will coerce to `&str`.
pub fn hello(name: &str) -> String {
    // TODO: return the string "Hello, {name}!" using `format!`.
    todo!("format a greeting for `name`")
}

// ---------------------------------------------------------------------------
// Exercise 3 — Drop: observe deterministic cleanup order
// ---------------------------------------------------------------------------

/// A guard that records its label into a shared log when it is dropped.
///
/// Values are dropped in *reverse* order of declaration, so this type lets a
/// test observe Rust's deterministic cleanup ordering without any real I/O.
///
/// # Examples
///
/// ```
/// use std::cell::RefCell;
/// use ch15_smart_pointers::Tracker;
///
/// let log = RefCell::new(Vec::new());
/// {
///     let _a = Tracker::new("a", &log);
///     let _b = Tracker::new("b", &log);
/// } // _b drops first, then _a
/// assert_eq!(*log.borrow(), vec!["b", "a"]);
/// ```
pub struct Tracker<'a> {
    label: &'static str,
    log: &'a RefCell<Vec<&'static str>>,
}

impl<'a> Tracker<'a> {
    /// Creates a tracker that will push `label` to `log` when dropped.
    pub fn new(label: &'static str, log: &'a RefCell<Vec<&'static str>>) -> Tracker<'a> {
        // TODO: build a `Tracker` storing both `label` and `log`.
        todo!("construct a Tracker holding the label and log")
    }
}

impl<'a> Drop for Tracker<'a> {
    /// Records this tracker's label at the moment of cleanup.
    fn drop(&mut self) {
        // TODO: push `self.label` onto the log via `self.log.borrow_mut()`.
        todo!("record the label when this tracker is dropped")
    }
}

/// Drops `tracker` immediately by handing ownership to [`std::mem::drop`].
///
/// This forces an early cleanup, which is the only way to drop a value before
/// the end of its scope (you may not call `tracker.drop()` directly).
pub fn drop_now(tracker: Tracker<'_>) {
    // TODO: call `drop(tracker)` to release it now.
    todo!("drop `tracker` early")
}

// ---------------------------------------------------------------------------
// Exercise 4 — Rc<T> + RefCell<T>: shared, mutable counters
// ---------------------------------------------------------------------------

/// A shared, mutable counter built from `Rc<RefCell<i32>>`.
///
/// `Rc<T>` allows multiple owners of the same value, and `RefCell<T>` permits
/// mutation through a shared reference (interior mutability). Cloning a
/// [`Counter`] shares the *same* underlying cell, so every clone observes the
/// same count.
///
/// # Examples
///
/// ```
/// use ch15_smart_pointers::Counter;
///
/// let a = Counter::new(0);
/// let b = a.clone();
/// a.increment();
/// b.increment();
/// assert_eq!(a.get(), 2); // both clones share one cell
/// ```
#[derive(Clone)]
pub struct Counter {
    value: Rc<RefCell<i32>>,
}

impl Counter {
    /// Creates a counter starting at `start`.
    pub fn new(start: i32) -> Counter {
        // TODO: wrap `start` in `Rc::new(RefCell::new(start))`.
        todo!("create a shared counter starting at `start`")
    }

    /// Adds one to the shared count.
    pub fn increment(&self) {
        // TODO: borrow the cell mutably and add 1 (`*self.value.borrow_mut() += 1`).
        todo!("increment the shared count")
    }

    /// Reads the current count.
    pub fn get(&self) -> i32 {
        // TODO: borrow the cell and return the value it holds.
        todo!("read the current count")
    }

    /// Reports how many [`Counter`] handles currently share this cell.
    ///
    /// This is the live strong reference count from `Rc::strong_count`.
    pub fn handles(&self) -> usize {
        // TODO: return `Rc::strong_count(&self.value)`.
        todo!("report the strong reference count")
    }
}
