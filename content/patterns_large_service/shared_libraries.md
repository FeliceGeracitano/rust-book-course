# Shared libraries with clear owners

Three crates handle amounts. Without a shared type, each would check "not negative, not
absurdly large" on its own, and one would forget. The example has two shared crates
with two different reasons to exist: `money` holds an **invariant**, and `observability`
holds a **mechanism**.

## Share an invariant, not just similar syntax

```rust
// Source: examples/large-service/crates/money/src/lib.rs
    pub fn new(value: i64) -> Result<Self, MoneyError> {
        if value < 0 {
            return Err(MoneyError::Negative);
        }
        if value > MAX_CENTS {
            return Err(MoneyError::TooLarge);
        }
        Ok(Self(value))
    }
```

`Cents` is a newtype whose constructor is the only way in, so every crate that holds a
`Cents` holds a valid one. `checked_add` re-applies the cap, `serde` goes through the
same constructor (`#[serde(try_from = "i64")]`), and the transport maps `MoneyError` to
400 once. This is worth a crate because the rule has an owner and would otherwise be
duplicated. polars does the same with `polars-error` and `polars-utils`: tiny leaf crates
that every layer depends on and that depend on nothing
([pola-rs/polars](https://github.com/pola-rs/polars)).

## Share a mechanism that knows no domain

```rust
// Source: examples/large-service/crates/observability/src/lib.rs
/// Logs method and path only. Query strings can carry identifiers, so they stay out of logs.
pub fn instrument(router: Router) -> Router {
    router.layer(TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        tracing::info_span!("request", method = %request.method(), path = %request.uri().path())
    }))
}
```

`observability` has no idea what a customer is. That is the test for a mechanism crate:
if it needs a domain type, it belongs to that domain instead. Because it depends on
nothing of ours, any crate can use it without creating an edge that points sideways.

## Decide when to extract

Extract when a rule has **one owner and several users**. Keep it local when only one
crate uses it, even if a second crate has something that looks alike: two similar
functions are cheaper than a shared crate whose two users pull it in different
directions. rust-analyzer names its boundary crates on purpose (`syntax`, `hir` and
`ide` are marked "API Boundary" in its architecture doc), so contributors know which
interfaces are meant to be stable and which are internal
([architecture.md](https://github.com/rust-lang/rust-analyzer/blob/master/docs/book/src/contributing/architecture.md)).

A `common` or `utils` crate with no owner is the failure mode: it grows by accretion,
every crate depends on it, and it becomes the reason the whole workspace rebuilds.
matklad's [Fast Rust Builds](https://matklad.github.io/2021/09/04/fast-rust-builds.html)
makes the build-time case: keep the crate graph wide, keep leaf crates small, and keep
proc-macro heavy dependencies out of the crates everything depends on.

## Privacy is the cheapest boundary

```rust
// compile-fail: E0603
mod billing {
    pub struct Quote(pub u64);
    fn fee(cents: u64) -> u64 {
        cents / 40
    }
}

pub fn total(cents: u64) -> u64 {
    cents + billing::fee(cents)
}
```

`fee` is private to `billing`, so the call from outside fails with E0603 and the rule
stays in one place. `pub(crate)` widens that to the current crate; `pub` widens it to
the world, and "once a crate item is public, it can't be made private again without
breaking any code" ([Effective Rust](https://www.lurklurk.org/effective-rust/visibility.html)).

```quiz
{
  "id": "discovery",
  "question": "Why is money a separate crate instead of a module inside billing?",
  "options": [
    "Because billing is the only crate that uses amounts",
    "Because orders and transport-http also need the invariant, and billing must not depend on them",
    "Because newtypes must live in their own crate"
  ],
  "answer": 1,
  "explain": "Three crates hold amounts. A leaf crate lets each depend on the rule without depending on each other. If only billing used it, a module would do."
}
```

```quiz
{
  "id": "tradeoff",
  "question": "A helper in observability starts needing CustomerId to redact ids. What should happen?",
  "options": [
    "Add customers as a dependency of observability",
    "Move that helper into the customers crate; observability stays domain-free",
    "Copy CustomerId into observability to avoid the dependency"
  ],
  "answer": 1,
  "explain": "A mechanism crate that imports a domain creates a sideways edge and pulls every user of observability into the customers build. The redaction belongs with the type it redacts."
}
```
