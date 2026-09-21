# Multiple endpoints with one application

Every endpoint needs the same three things: parse the request, call a service, map the
result to a status and a body. When one router file holds forty handlers, those steps
blur into each other and domain logic leaks into the transport. The example gives each
domain its own router builder and lets `app` merge them once.

## Register routes by domain

`transport-http` exposes three functions: `customers_routes`, `billing_routes`,
`orders_routes`. Each takes the service it needs and returns an axum `Router` with its
state already attached, so the caller merges them without knowing what is inside:

```rust
// Source: examples/large-service/crates/app/src/lib.rs
        let router = Router::new()
            .route("/health", get(|| async { "ok" }))
            .merge(transport_http::customers_routes(customers))
            .merge(transport_http::billing_routes(billing))
            .merge(transport_http::orders_routes(orders));
```

[`Router::merge`](https://docs.rs/axum/latest/axum/struct.Router.html#method.merge)
combines routers that already carry their own state; a path registered twice panics at
startup, which is what you want for a wiring bug. Adding a domain means adding one
`merge` line, not editing a shared handler file.

## A thin handler still has responsibilities

```rust
// Source: examples/large-service/crates/transport-http/src/billing_routes.rs
/// A thin handler still has three jobs: parse, call, map. Nothing else.
async fn quote(
    State(service): State<Arc<billing::Service>>,
    Query(query): Query<QuoteQuery>,
) -> Result<Json<billing::Quote>, ApiError> {
    let subtotal = Cents::new(query.cents)?;
    Ok(Json(service.quote(subtotal).await?))
}
```

Parse: the `Query` extractor rejects a missing or non-integer `cents` with 400 before
the handler body runs, and `Cents::new` rejects negative or oversized amounts. Call: one
service method. Map: `?` converts every domain error into `ApiError`, and `Json`
serializes the domain's own `Quote` type, so the transport adds no data-transfer type
of its own.

## One error mapping, not one per handler

```rust
// Source: examples/large-service/crates/transport-http/src/error.rs
impl From<OrdersError> for ApiError {
    fn from(error: OrdersError) -> Self {
        match error {
            OrdersError::Customer(inner) => inner.into(),
            OrdersError::UnknownCustomer(id) => {
                ApiError::NotFound(format!("unknown customer {id}"))
            }
            OrdersError::Billing(inner) => inner.into(),
            OrdersError::Money(inner) => inner.into(),
        }
    }
}
```

| Domain outcome | HTTP |
|---|---|
| invalid id, negative or oversized amount | 400 |
| unknown customer | 404 |
| provider unavailable or malformed | 502 |
| provider deadline exceeded | 504 |

The table lives in one file because it is a transport decision. Domains return their
own enums and never learn what a status code is; the same enums could feed a gRPC or a
CLI transport unchanged. rust-analyzer applies the rule one level up: only its outermost
crate knows about LSP and JSON
([architecture.md](https://github.com/rust-lang/rust-analyzer/blob/master/docs/book/src/contributing/architecture.md)).

## When to add a handler, and when not to

Add a route when a client needs a new resource. Do not add a route to expose a service
method because it exists; every endpoint is a contract you will keep. Validation that
belongs to the domain (`Cents::new`, `CustomerId::parse`) stays in the domain, so a
second transport cannot forget it. Validation that belongs to HTTP (is `cents` an
integer at all?) stays in the extractor.

```quiz
{
  "id": "discovery",
  "question": "A request arrives at /billing/quote?cents=abc. Where is it rejected?",
  "options": [
    "In billing::Service, which returns BillingError::Money",
    "In the Query extractor, before the handler body runs",
    "In ApiError::from, which maps the parse failure"
  ],
  "answer": 1,
  "explain": "Query<QuoteQuery> fails to deserialize abc as i64 and axum answers 400. The handler body and the domain never see the request."
}
```

```quiz
{
  "id": "tradeoff",
  "question": "Why do the domain error enums not carry an HTTP status?",
  "options": [
    "Status codes are integers, and enums cannot hold integers",
    "So the same domain crates can serve another transport without changes",
    "Because axum requires errors to implement IntoResponse directly"
  ],
  "answer": 1,
  "explain": "The mapping from outcome to status is a transport concern kept in one place. A domain crate that knew about 404 would be coupled to HTTP."
}
```
