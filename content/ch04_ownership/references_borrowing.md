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

```trace
{
  "title": "An exclusive borrow ends after its last use",
  "code": "fn main() {\n    let mut text = String::from(\"hi\");\n    let borrowed = &mut text;\n    borrowed.push('!');\n    text.push('?');\n    println!(\"{text}\");\n}",
  "steps": [
    {
      "line": 2,
      "note": "text owns a mutable String.",
      "state": {
        "text": "\"hi\"",
        "borrowed": "not bound yet"
      },
      "output": ""
    },
    {
      "line": 3,
      "note": "borrowed has exclusive access. text cannot be used directly while this borrow is needed.",
      "state": {
        "text": "owned, exclusively borrowed",
        "borrowed": "&mut String"
      },
      "output": ""
    },
    {
      "line": 4,
      "note": "The String is changed through borrowed. This is the last use of the reference.",
      "state": {
        "text": "\"hi!\"",
        "borrowed": "last use; borrow can end"
      },
      "output": ""
    },
    {
      "line": 5,
      "note": "Because borrowed is not used again, direct mutable access to text is allowed.",
      "state": {
        "text": "\"hi!?\"",
        "borrowed": "no longer used"
      },
      "output": ""
    },
    {
      "line": 6,
      "note": "Formatting reads the completed String.",
      "state": {
        "text": "\"hi!?\""
      },
      "output": "hi!?\n"
    }
  ]
}
```

```quiz
{
  "id": "discovery",
  "question": "Can the second push happen before the later use of r?",
  "options": [
    "Yes: both operations add a character",
    "Only if s was declared without mut",
    "No: it conflicts with an active mutable borrow"
  ],
  "answer": 2,
  "explain": "r is used after s.push, so its exclusive borrow must still be valid at that point. Mutating s directly while that borrow is active is rejected.",
  "code": "fn main() {\n    let mut s = String::from(\"hi\");\n    let r = &mut s;\n    s.push('!');\n    r.push('?');\n}"
}
```

### Optional terminal practice

Run `node scripts/prepare-exercises.mjs` once from the repository root, then `cd chapters` before running the commands below. Source paths are relative to the repository root. You can complete the browser lesson without installing Rust.

Implement `Counter::new`, `Counter::value`, and `count_up` in
`chapters/ch04_ownership/src/lib.rs`, then run:

```bash
cargo test -p ch04_ownership
```
