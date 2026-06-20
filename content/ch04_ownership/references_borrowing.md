# 4.2 References and Borrowing

Moving a value every time you want to use it is tedious. A **reference** lets you
access a value *without* taking ownership — this is called **borrowing**. Write
`&value` to create a reference and `&mut value` to create a mutable one.

References come with two guarantees the borrow checker enforces:

- You may have **any number of shared (`&`) references** at once, but they are
  read-only.
- You may have **exactly one mutable (`&mut`) reference**, and no shared ones at
  the same time.

These rules prevent data races at compile time: nothing can read a value while it
is being written. A function taking `&self` can only read; one taking
`&mut self` (or a `&mut T` argument) can change the value in place, and the caller
keeps ownership.

```rust
fn len(s: &String) -> usize {   // borrows, does not take ownership
    s.len()
}

fn push_bang(s: &mut String) {  // exclusive borrow: may mutate
    s.push('!');
}

fn main() {
    let mut s = String::from("hi");
    println!("{}", len(&s));     // shared borrow
    push_bang(&mut s);           // mutable borrow
    println!("{s}");             // "hi!" — s is still owned here
}
```

### Exercise
Implement `Counter::new`, `Counter::value`, and `count_up` in
`chapters/ch04_ownership/src/lib.rs`, then run:

```bash
cargo test -p ch04_ownership
```
