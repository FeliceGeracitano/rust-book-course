# 8.3 Storing Keys with Associated Values in Hash Maps

A `HashMap<K, V>` stores a mapping from keys to values, letting you look data up
by key instead of by position. It lives in `std::collections`, so bring it into
scope with `use std::collections::HashMap`. Insert with `insert`, and read with
`get`, which returns an `Option<&V>` so a missing key is `None` rather than a
panic.

The real power is the **entry API**. `entry(key)` returns a slot for that key;
`or_insert(default)` fills the slot with `default` only if the key is absent, and
either way hands back a mutable reference to the value. That makes
"increment-or-start-at-one" counting clean and allocation-free per existing key.

```rust
use std::collections::HashMap;

let mut counts: HashMap<&str, u32> = HashMap::new();
for word in "a b a".split_whitespace() {
    *counts.entry(word).or_insert(0) += 1;
}

assert_eq!(counts.get("a"), Some(&2));
assert_eq!(counts.get("z"), None);
```

Owned `String` keys work the same way; just call `.to_string()` on each `&str`
before inserting so the map owns its keys independently of the input.

### Exercise

Implement `word_count` in `chapters/ch08_collections/src/lib.rs` using the entry
API so it counts how often each word appears. Then run:

```bash
cargo test -p ch08_collections
```
