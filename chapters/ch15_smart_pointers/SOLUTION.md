# Chapter 15 — Solutions

Reference implementations for the smart-pointer exercises.

```rust
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

// Exercise 1 — Box<T>: a recursive cons list
#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn sum(&self) -> i32 {
        match self {
            List::Cons(value, rest) => value + rest.sum(),
            List::Nil => 0,
        }
    }
}

// Exercise 2 — Deref: build your own smart pointer
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

pub fn hello(name: &str) -> String {
    format!("Hello, {name}!")
}

// Exercise 3 — Drop: observe deterministic cleanup order
pub struct Tracker<'a> {
    label: &'static str,
    log: &'a RefCell<Vec<&'static str>>,
}

impl<'a> Tracker<'a> {
    pub fn new(label: &'static str, log: &'a RefCell<Vec<&'static str>>) -> Tracker<'a> {
        Tracker { label, log }
    }
}

impl<'a> Drop for Tracker<'a> {
    fn drop(&mut self) {
        self.log.borrow_mut().push(self.label);
    }
}

pub fn drop_now(tracker: Tracker<'_>) {
    drop(tracker);
}

// Exercise 4 — Rc<T> + RefCell<T>: shared, mutable counters
#[derive(Clone)]
pub struct Counter {
    value: Rc<RefCell<i32>>,
}

impl Counter {
    pub fn new(start: i32) -> Counter {
        Counter {
            value: Rc::new(RefCell::new(start)),
        }
    }

    pub fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }

    pub fn handles(&self) -> usize {
        Rc::strong_count(&self.value)
    }
}
```
