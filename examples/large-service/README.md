# Large Rust service: a runnable architecture example

A read-only quote API with three domains: customers, billing, and orders. It is a
Cargo workspace of ten small crates so the boundaries are visible; a smaller team
could fold several of them into modules of one crate (lesson 8.6 discusses when).
Rust 1.85+ (edition 2024) is required.

## Run

From this directory, start the local provider simulator:

```bash
cargo run -p demo-upstream
```

In another terminal, from this directory:

```bash
export IDENTITY_URL=http://127.0.0.1:9090
export PAYMENTS_URL=http://127.0.0.1:9090
cargo run -p api
```

In a third terminal:

```bash
curl http://127.0.0.1:8080/health
curl http://127.0.0.1:8080/customers/c1
curl 'http://127.0.0.1:8080/billing/quote?cents=1000'
curl 'http://127.0.0.1:8080/orders/quote?customer_id=c1&cents=1000'
```

The order response is:

```json
{"customer_id":"c1","price":{"subtotal_cents":1000,"fee_cents":25,"total_cents":1025}}
```

Unknown customers return 404. Invalid input returns 400. A failed provider returns
502; a provider deadline returns 504. The simulator knows only customer `c1` (Ada)
and always quotes a 25-cent fee. It never charges money. Stop both with Ctrl-C; the
API finishes in-flight requests first. `BIND_ADDR` (default `127.0.0.1:8080`) and
`UPSTREAM_TIMEOUT_MS` (default 2000) are optional. Provider URLs must be bare
`http(s)://host:port` origins, with no path, query, or credentials.

## Boundaries

- `crates/api`: process entry point. Logging, config, bind, signals, exit code. Nothing else.
- `crates/app`: composition root. Builds the one shared `reqwest::Client`, the adapters,
  the services, and merges the routers. The only crate that knows concrete adapters.
- `crates/customers`: `CustomerId` rules and the `Directory` port. No HTTP.
- `crates/billing`: the `FeeProvider` port and the quoted-total calculation. No HTTP.
- `crates/orders`: the quote workflow and its own `CustomerDirectory` port. Depends on
  `customers` (types) and `billing` (quote); neither depends back on `orders`.
- `crates/transport-http`: one axum router per domain, `ApiError` mapping to 400/404/502/504.
- `crates/integrations`: identity and payments adapters over the shared client, plus the
  bounded JSON GET they share. Implements the three ports.
- `crates/money`: nonnegative cents with a 100000000 cap. A leaf every domain uses.
- `crates/observability`: request tracing by method and path only. Knows no domain.
- `crates/demo-upstream`: the local simulator used by `cargo run` and by the app tests.

Dependency direction: `api → app → {transport-http, integrations, observability}`;
`transport-http → {orders, customers, billing, money}`; `orders → {customers, billing, money}`;
`integrations → {orders, customers, billing, money}`. Nothing points back up. The identity
adapter is constructed once and satisfies two ports owned by two domains. No crate uses a
global registry or a static client.

## Verify

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

Tests cover money bounds, id rules, quote math, the unknown-customer short-circuit,
error mapping at the transport, malformed / oversized / slow / unavailable providers
(wiremock), the assembled router, config validation, and graceful shutdown draining an
in-flight request. No network access or credentials are needed.

From the repository's `client/` directory, `npm run check:rust` runs these checks and
verifies that every lesson excerpt headed `// Source: examples/large-service/...`
still matches its source file.

## Scope

This demonstrates crate boundaries, not a production commerce platform. It binds to
loopback, has no authentication or database, and never creates orders or moves money.
Quotes can go stale between calls. Real writes need persistence, authorization,
idempotency, and recovery policies. Per-call HTTP timeouts are not a workflow deadline.
