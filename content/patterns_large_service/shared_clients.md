# Shared external clients

Two providers, one HTTP client. A `reqwest::Client` owns a connection pool and a
timeout policy; creating one per request throws the pool away and lets every call site
pick its own timeout. The example constructs the client once, in `app`, and hands a
clone to each adapter.

## Construct once, inject explicitly

```rust
// Source: examples/large-service/crates/app/src/lib.rs
        // One client: one connection pool and one timeout policy shared by every adapter.
        let http = reqwest::Client::builder()
            .timeout(config.upstream_timeout)
            .build()?;
        let identity = Arc::new(integrations::identity::Client::new(
            http.clone(),
            config.identity_url.clone(),
        ));
        let payments = Arc::new(integrations::payments::Client::new(
            http,
            config.payments_url.clone(),
        ));
```

[`reqwest::Client`](https://docs.rs/reqwest/latest/reqwest/struct.Client.html) is an
`Arc` around the pool, so `clone` is cheap and both adapters share the same connections.
The timeout is set once because it is a policy of this process, not of one call.
Adapters receive the client through their constructor:

```rust
// Source: examples/large-service/crates/integrations/src/identity.rs
    /// Takes the shared `reqwest::Client`; the caller owns its lifetime and timeout policy.
    pub fn new(http: reqwest::Client, base: Origin) -> Self {
        Self { http, base }
    }
```

Nothing in `integrations` builds a client or reads an environment variable. That is
what makes the adapter testable: its tests build a client with a 200 ms timeout and
point it at a [wiremock](https://docs.rs/wiremock) server that answers slowly, with
garbage, or with two megabytes.

## Share the mechanics, not the meaning

Both adapters need the same bounded JSON GET: classify timeouts, treat 404 as absence,
cap the body, decode. It lives once, in a `pub(crate)` helper:

```rust
// Source: examples/large-service/crates/integrations/src/httpjson.rs
pub(crate) async fn get_json<T: DeserializeOwned>(
    client: &reqwest::Client,
    url: Url,
) -> Result<Fetched<T>, HttpJsonError> {
```

`pub(crate)` keeps it an implementation detail of `integrations`. Each adapter then
translates `HttpJsonError` into the error type of the port it implements: a timeout
becomes `CustomersError::Timeout` for identity and `BillingError::Timeout` for payments.
The mechanics are shared; what a failure means belongs to each domain.

## What a global would cost

A `static CLIENT: OnceLock<reqwest::Client>` looks simpler. It also means every test
shares one timeout, nothing can run two configurations in one process, and the
dependency is invisible in constructors, so a reader cannot tell which crates talk to
the network. Explicit injection costs one parameter and buys that visibility. Vector
follows the same discipline at larger scale: a component's external clients are built
from its own configuration, not fetched from a global
([ARCHITECTURE.md](https://github.com/vectordotdev/vector/blob/master/docs/ARCHITECTURE.md)).

```quiz
{
  "id": "discovery",
  "question": "Why is cloning reqwest::Client for each adapter not wasteful?",
  "options": [
    "Each clone opens its own connection pool, which is fine for two adapters",
    "Client wraps an Arc, so clones share one pool and one configuration",
    "reqwest caches connections globally regardless of client instances"
  ],
  "answer": 1,
  "explain": "Client::clone is an Arc clone. Both adapters reuse the same connections and inherit the timeout set once in app."
}
```

```quiz
{
  "id": "tradeoff",
  "question": "Where should the upstream timeout be decided?",
  "options": [
    "In each adapter, so identity and payments can differ",
    "Once, where the client is built, as process configuration",
    "In the domain services, because they know how long a quote may take"
  ],
  "answer": 1,
  "explain": "The example treats it as process policy: UPSTREAM_TIMEOUT_MS is read by app::Config. Per-call overrides exist (RequestBuilder::timeout), but the default lives in one place."
}
```
