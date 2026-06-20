# 5.1 Defining and Instantiating Structs

A **struct** groups related values under one named type. Each piece of data is a
named *field*, so you read and write values by name rather than by position
(unlike a tuple).

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

let user = User {
    username: String::from("ferris"),
    email: String::from("ferris@example.com"),
    active: true,
    sign_in_count: 1,
};
```

When a variable and a field share a name, **field init shorthand** lets you
write the name once: `User { username, email, .. }` instead of
`username: username`. To build a value from an existing one, **struct update
syntax** copies the remaining fields:

```rust
let other = User {
    email: String::from("new@example.com"),
    ..user // take every other field from `user`
};
```

Note that `..user` *moves* non-`Copy` fields, so `user` may no longer be usable
afterward. Rust also offers *tuple structs* like `struct Point(i32, i32);` when
field names would add no clarity.

### Exercise

In `chapters/ch05_structs/src/lib.rs`, implement `build_rectangle` with field
init shorthand and `with_email` with struct update syntax, then run:

```bash
cargo test -p ch05_structs
```
