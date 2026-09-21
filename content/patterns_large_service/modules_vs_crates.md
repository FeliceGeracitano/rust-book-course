# Modules vs crates, and testing at boundaries

The example uses ten crates for a service that could fit in one. Both shapes are
legitimate. This lesson gives the rule of thumb and shows how the tests change
depending on where the boundary is.

## One crate, modules per domain

tokio is one crate. Its domains are modules behind features: `net`, `fs`, `sync`,
`time`, `rt`, with `full` as the umbrella
([tokio/Cargo.toml](https://github.com/tokio-rs/tokio/blob/master/tokio/Cargo.toml)).
The same service as a single crate would look like this:

```text
src/
  main.rs            thin: config, bind, serve
  lib.rs             mod customers; mod billing; mod orders; mod transport; pub use ...
  customers/         mod.rs, service.rs
  billing/
  orders/
  transport/
  integrations/
```

Boundaries are then enforced by visibility, not by Cargo: `pub(crate)` for what other
modules may see, private for the rest, and a curated `pub use` list in `lib.rs` for the
public surface. Two published positions disagree on the mechanism. Effective Rust
prefers `pub(crate)` on internals so the API cannot leak; Kobzol prefers private modules
with plain `pub` items and a single `pub use` block, so "the whole external interface of
your crate is visible at a single place"
([Two ways of interpreting visibility](https://kobzol.github.io/rust/2025/04/23/two-ways-of-interpreting-visibility-in-rust.html)).
Either works; pick one per workspace and write it down.

## Several crates, one workspace

Split when at least one of these is true:

- **Compile time.** Only changed crates and their dependents rebuild, and independent
  crates build in parallel. Wide graphs beat deep chains
  ([Fast Rust Builds](https://matklad.github.io/2021/09/04/fast-rust-builds.html)).
- **Different dependency sets.** `orders` should not compile `axum` and `reqwest`. A
  crate boundary makes that a fact; a module boundary makes it a convention.
- **Different owners or release cadence.** ripgrep publishes `ignore` and `globset` for
  other tools; zed keeps `git` and `git_ui` separate so the model crate never links UI.
- **A boundary you want Cargo to defend.** Cargo rejects cycles; nothing rejects a
  quiet `use crate::orders` inside `billing`.

Do not split for symmetry. A crate with one file and one user is ceremony. vector is a
useful middle: one big crate organised by domain modules (`sources`, `sinks`,
`transforms`, `topology`) plus forty `lib/` crates for the parts other components reuse
([vector](https://github.com/vectordotdev/vector)).

## Test behavior at each boundary

Ports make every boundary testable with a fake instead of a network. The transport
tests build the real routers over fakes and check status codes and bodies:

```rust
// Source: examples/large-service/crates/transport-http/tests/routes.rs
struct FakeFees(Result<Cents, BillingError>, AtomicUsize);

#[async_trait]
impl FeeProvider for FakeFees {
    async fn fee(&self, _subtotal: Cents) -> Result<Cents, BillingError> {
        self.1.fetch_add(1, Ordering::SeqCst);
        self.0.clone()
    }
}
```

```rust
// Source: examples/large-service/crates/transport-http/tests/routes.rs
fn app(identity: Option<CustomersError>, fees: Arc<FakeFees>) -> Router {
    let identity = Arc::new(FakeIdentity(identity));
    let customers = Arc::new(customers::Service::new(identity.clone()));
    let billing = Arc::new(billing::Service::new(fees));
    let orders = Arc::new(orders::Service::new(identity, billing.clone()));
    Router::new()
        .merge(customers_routes(customers))
        .merge(billing_routes(billing))
        .merge(orders_routes(orders))
}
```

The counting fake proves the short-circuit: an unknown customer yields 404 and the fee
provider was asked exactly once, for the known one. Each layer gets the test that fits
its boundary:

| Boundary | Test double | What it proves |
|---|---|---|
| domain service ↔ port | in-crate fake | rules and short-circuits |
| transport ↔ services | fakes + `tower::ServiceExt::oneshot` | parsing and status mapping |
| adapter ↔ provider | `wiremock` server | timeouts, 404, malformed and oversized bodies |
| `app` ↔ everything | the real simulator in-process | wiring and graceful shutdown |

Where the test file lives is the last decision. The Book puts each integration test in
its own `tests/*.rs` file, each compiled as a separate crate
([ch11-03](https://doc.rust-lang.org/book/ch11-03-test-organization.html)); matklad
measured that one `tests/it/main.rs` with modules compiles noticeably faster on large
projects ([Delete Cargo Integration Tests](https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html)).
With a handful of files per crate the Book's layout is simpler; past a dozen, merge them.

```quiz
{
  "id": "discovery",
  "question": "Which reason justifies a crate boundary that a module boundary cannot deliver?",
  "options": [
    "Keeping functions grouped by topic",
    "Guaranteeing that orders never compiles axum or reqwest",
    "Letting two developers edit the same file"
  ],
  "answer": 1,
  "explain": "Only a crate has its own dependency list. A module in one crate compiles everything the crate depends on, whatever it uses."
}
```

```quiz
{
  "id": "tradeoff",
  "question": "Where does the FakeFees provider belong?",
  "options": [
    "In the billing crate's public API so every test can reuse it",
    "Next to the tests that need it, or in a test-support crate with no production users",
    "In app, since app wires providers"
  ],
  "answer": 1,
  "explain": "Fakes are test code. Exposing them from a production crate widens its API; cargo and rust-analyzer keep such helpers in dedicated test-support crates instead."
}
```
