# Cross-domain workflows

Orders needs to know whether a customer exists before it asks billing for a quote. The
tempting shortcut is `orders` importing the identity client directly. Then `orders`
depends on `reqwest`, its tests need a network, and swapping the provider means editing
a domain crate. The example lets `orders` state what it needs as a trait it owns.

## Let the workflow state its dependencies

```rust
// Source: examples/large-service/crates/orders/src/lib.rs
#[async_trait]
pub trait CustomerDirectory: Send + Sync {
    async fn exists(&self, id: &CustomerId) -> Result<bool, OrdersError>;
}
```

This is a **consumer-owned port**: the trait lives in the crate that calls it, is as
small as that crate's need, and returns the caller's error type. `customers` has its own
port, `Directory`, which returns a name. One adapter implements both:

```rust
// Source: examples/large-service/crates/integrations/src/identity.rs
#[async_trait]
impl Directory for Client {
    async fn name(&self, id: &CustomerId) -> Result<Option<String>, CustomersError> {
        self.lookup(id).await.map_err(to_customers_error)
    }
}

#[async_trait]
impl CustomerDirectory for Client {
    async fn exists(&self, id: &CustomerId) -> Result<bool, OrdersError> {
        Ok(self.lookup(id).await.map_err(to_customers_error)?.is_some())
    }
}
```

`app` constructs the identity client once and passes the same `Arc` to both services.
Rust coerces `Arc<identity::Client>` to `Arc<dyn Directory>` and to
`Arc<dyn CustomerDirectory>` at each call site, so neither service knows the other exists.

## Keep ownership acyclic

```rust
// Source: examples/large-service/crates/orders/src/lib.rs
    pub async fn quote(&self, raw_id: &str, cents: i64) -> Result<OrderQuote, OrdersError> {
        let id = CustomerId::parse(raw_id)?;
        let subtotal = Cents::new(cents)?;
        if !self.customers.exists(&id).await? {
            return Err(OrdersError::UnknownCustomer(id));
        }
        let price = self.billing.quote(subtotal).await?;
        Ok(OrderQuote {
            customer_id: id.to_string(),
            price,
        })
    }
```

`orders` imports `customers` for `CustomerId` and `billing` for `Quote`: real
compile-time edges, pointing down. Neither of those crates imports `orders`. Cargo
rejects a cycle outright, but a cycle-free graph can still point the wrong way; the
rule "workflows depend on the domains they coordinate, never the reverse" is enforced by
review. Unknown customers stop the workflow before billing is asked, and the unit test
proves it with a counting fake.

tikv states the same rule for storage: `engine_traits` "must not have any transitive
dependencies on RocksDB"; the RocksDB implementation lives in `engine_rocks`
([engine_traits/src/lib.rs](https://github.com/tikv/tikv/blob/master/components/engine_traits/src/lib.rs)).
Ports in a leaf crate, adapters beside them.

## async traits and trait objects

`#[async_trait]` boxes the returned future so the trait can be used as
`dyn CustomerDirectory`. Since Rust 1.75, `async fn` in traits works natively, but a
trait with a native `async fn` cannot be a `dyn` object. Pick `async_trait` when `app`
wires `Arc<dyn Port>`; pick native `async fn` with generics
(`Service<D: CustomerDirectory>`) when monomorphization is fine and you never need a
trait object.

```quiz
{
  "id": "discovery",
  "question": "Which crate defines the CustomerDirectory trait?",
  "options": [
    "customers, because it is about customers",
    "integrations, because it implements it",
    "orders, because orders is the crate that needs it"
  ],
  "answer": 2,
  "explain": "Consumer-owned ports live where they are called. orders declares exactly what it needs (existence) and returns OrdersError; the adapter implements it."
}
```

```quiz
{
  "id": "tradeoff",
  "question": "What breaks first if billing imported orders to reuse OrderQuote?",
  "options": [
    "Nothing: Cargo allows any dependency between workspace members",
    "A dependency cycle, since orders already imports billing for Quote",
    "The async_trait macro, which cannot cross crate boundaries"
  ],
  "answer": 1,
  "explain": "orders → billing already exists. billing → orders would form a cycle, which Cargo rejects. The direction is not a style choice; it is what lets billing be tested and reused alone."
}
```
