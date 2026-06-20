# 8.1 Storing Lists of Values with Vectors

A `Vec<T>` is a growable list that stores values of a single type next to each
other on the heap. Create one with `Vec::new()` or the `vec!` macro, then push
new values onto the end. Read elements by index with `v[i]` (panics if out of
bounds) or `v.get(i)`, which returns an `Option<&T>` so you can handle a missing
index gracefully.

Because a `Vec` owns its data, you usually iterate over it by reference. Use
`&v` for read-only access and `&mut v` when you want to change each element in
place.

```rust
let mut scores = vec![10, 20, 30];
scores.push(40);

let mut total = 0;
for &s in &scores {
    total += s;
}

assert_eq!(total, 100);
assert_eq!(scores.get(99), None);
```

Building a brand-new vector from an existing slice is a common pattern: walk the
input, compute something, and `push` the result onto a fresh `Vec`. Reserving
capacity up front with `Vec::with_capacity` avoids repeated reallocations when
you already know the length.

### Exercise

Implement `running_totals` in `chapters/ch08_collections/src/lib.rs` so it
returns a vector of prefix sums. Then run:

```bash
cargo test -p ch08_collections
```
