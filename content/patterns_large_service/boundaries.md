# Structuring a large Rust codebase

A growing service needs boundaries that let someone change one business capability
without reading every crate. This six-lesson chapter builds one runnable quote service,
`examples/large-service/`: **customers** looks up a name, **billing** computes a quoted
total, **orders** checks a customer and then asks billing. Every operation is read-only.
A quote never creates an order or charges a card.

Start after **Chapter 7 (Packages, Crates, and Modules)** and **Chapter 14 (Workspaces)**.
The layout below is one honest answer, not a mandated template: it makes the boundaries
visible by giving each one a crate. A smaller team can fold several of these into modules
of one crate; lesson 8.6 says when.

## A concrete crate map

```text
examples/large-service/
  Cargo.toml                 virtual workspace: shared versions, deps, lints
  crates/
    api/                     binary: config, logging, bind, signals, exit code
    app/                     composition root: builds adapters, services, one Router
    transport-http/          axum handlers per domain, error → status mapping
    integrations/            identity + payments clients over one reqwest::Client
    orders/                  quote workflow; owns the CustomerDirectory port
    customers/               CustomerId rules; owns the Directory port
    billing/                 fee port and quoted-total rules
    money/                   Cents: nonnegative, capped; a leaf everyone uses
    observability/           request tracing by method and path only
    demo-upstream/           local provider simulator used by cargo run and tests
```

Two rules make the map navigable. **Domains own their ports**: `customers`, `billing`
and `orders` declare traits for what they need and contain no HTTP, no JSON, no
`reqwest`. **Nothing points up**: `billing` and `customers` never import `orders`, no
domain imports `transport-http`, and only `app` knows which adapter satisfies which port.
Cargo enforces the absence of cycles; code review enforces the direction.

## Explore the dependency direction

Open the chapter visualization at the end of this lesson. The compile-time view lists
every `Cargo.toml` edge; the runtime view walks one `/orders/quote` request through the
crates and shows where a trait call crosses a boundary.

## The workspace root

```toml
[workspace]
resolver = "2"
members = ["crates/*"]
default-members = ["crates/api"]
```

A **virtual workspace** (no root package) with a flat `crates/` directory is what
rust-analyzer, polars, bevy, zed and meilisearch use. matklad's
[Large Rust Workspaces](https://matklad.github.io/2021/08/22/large-rust-workspaces.html)
makes the case: folder name equals crate name, the binary is just another crate, and
`default-members` makes plain `cargo run` start the API. cargo and ripgrep keep a root
package plus `crates/` libraries instead, so both shapes are common. `[workspace.package]`
and `[workspace.dependencies]` keep one version and one feature set per dependency
([Cargo reference](https://doc.rust-lang.org/cargo/reference/workspaces.html)).

## Start at the executable

```rust
// Source: examples/large-service/crates/api/src/main.rs
async fn run() -> anyhow::Result<()> {
    let config = app::Config::from_env()?;
    let app = app::App::new(&config)?;
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;
    tracing::info!(address = %config.bind_addr, "starting api");
    app::serve(app.router, listener, shutdown_signal()).await
}
```

`main` reads configuration, asks `app` for a router, binds, and serves until a signal.
The Book's advice for command-line tools applies to servers too: keep `main` thin so
everything else is testable
([ch12-03](https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html)).
The `api` crate cannot even name a domain type: it depends on `app`, `observability`,
`tokio` and `anyhow`, nothing else.

## Run the complete example

From `examples/large-service/`, in three terminals:

```bash
cargo run -p demo-upstream
```

```bash
IDENTITY_URL=http://127.0.0.1:9090 PAYMENTS_URL=http://127.0.0.1:9090 cargo run -p api
```

```bash
curl 'http://127.0.0.1:8080/orders/quote?customer_id=c1&cents=1000'
```

The response is
`{"customer_id":"c1","price":{"subtotal_cents":1000,"fee_cents":25,"total_cents":1025}}`.
`cargo test --workspace` runs every boundary test without network access.

```quiz
{
  "id": "discovery",
  "question": "Which crate is allowed to import both orders and integrations?",
  "options": [
    "billing, because it needs the fee",
    "app, because it wires adapters to ports",
    "customers, because orders depends on it"
  ],
  "answer": 1,
  "explain": "Only the composition root knows concrete adapters. Domains depend on their own traits; billing and customers never depend on orders."
}
```

```quiz
{
  "id": "tradeoff",
  "question": "What does default-members = [\"crates/api\"] change?",
  "options": [
    "Which crates cargo test --workspace runs",
    "What plain cargo run and cargo build act on at the workspace root",
    "Which crates are published"
  ],
  "answer": 1,
  "explain": "default-members picks the targets for cargo commands run at the root without -p or --workspace. Tests still run everywhere with --workspace, and publish = false keeps every crate private."
}
```
