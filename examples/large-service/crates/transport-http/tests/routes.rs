use async_trait::async_trait;
use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use billing::{BillingError, FeeProvider};
use customers::{CustomerId, CustomersError, Directory};
use http_body_util::BodyExt;
use money::Cents;
use orders::{CustomerDirectory, OrdersError};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tower::ServiceExt;
use transport_http::{billing_routes, customers_routes, orders_routes};

/// Knows `c1`; any other id maps to the configured failure or "absent".
struct FakeIdentity(Option<CustomersError>);

#[async_trait]
impl Directory for FakeIdentity {
    async fn name(&self, id: &CustomerId) -> Result<Option<String>, CustomersError> {
        match (&self.0, id.as_str()) {
            (Some(error), _) => Err(error.clone()),
            (None, "c1") => Ok(Some("Ada".into())),
            _ => Ok(None),
        }
    }
}

#[async_trait]
impl CustomerDirectory for FakeIdentity {
    async fn exists(&self, id: &CustomerId) -> Result<bool, OrdersError> {
        Ok(self.name(id).await?.is_some())
    }
}

struct FakeFees(Result<Cents, BillingError>, AtomicUsize);

#[async_trait]
impl FeeProvider for FakeFees {
    async fn fee(&self, _subtotal: Cents) -> Result<Cents, BillingError> {
        self.1.fetch_add(1, Ordering::SeqCst);
        self.0.clone()
    }
}

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

fn fees(result: Result<Cents, BillingError>) -> Arc<FakeFees> {
    Arc::new(FakeFees(result, AtomicUsize::new(0)))
}

fn cents(value: i64) -> Cents {
    Cents::new(value).unwrap()
}

async fn call(app: Router, uri: &str) -> (StatusCode, String) {
    let response = app
        .oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8(bytes.to_vec()).unwrap())
}

#[tokio::test]
async fn customers_route_maps_found_missing_and_invalid_ids() {
    let ok = fees(Ok(cents(25)));
    assert_eq!(
        call(app(None, ok.clone()), "/customers/c1").await,
        (StatusCode::OK, r#"{"id":"c1","name":"Ada"}"#.into())
    );
    assert_eq!(
        call(app(None, ok.clone()), "/customers/zz").await.0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        call(app(None, ok.clone()), "/customers/BAD!").await.0,
        StatusCode::BAD_REQUEST
    );
}

#[tokio::test]
async fn provider_failures_become_502_and_504() {
    let ok = fees(Ok(cents(25)));
    let (status, body) = call(
        app(Some(CustomersError::Timeout), ok.clone()),
        "/customers/c1",
    )
    .await;
    assert_eq!(
        (status, body),
        (
            StatusCode::GATEWAY_TIMEOUT,
            r#"{"error":"upstream deadline exceeded"}"#.into()
        )
    );
    let down = Some(CustomersError::Unavailable("boom".into()));
    assert_eq!(
        call(app(down, ok), "/customers/c1").await.0,
        StatusCode::BAD_GATEWAY
    );
    assert_eq!(
        call(
            app(None, fees(Err(BillingError::Timeout))),
            "/billing/quote?cents=1"
        )
        .await
        .0,
        StatusCode::GATEWAY_TIMEOUT
    );
}

#[tokio::test]
async fn billing_route_validates_the_query() {
    let ok = fees(Ok(cents(25)));
    assert_eq!(
        call(app(None, ok.clone()), "/billing/quote?cents=1000").await,
        (
            StatusCode::OK,
            r#"{"subtotal_cents":1000,"fee_cents":25,"total_cents":1025}"#.into()
        )
    );
    assert_eq!(
        call(app(None, ok.clone()), "/billing/quote?cents=-1")
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        call(app(None, ok), "/billing/quote").await.0,
        StatusCode::BAD_REQUEST
    );
}

#[tokio::test]
async fn orders_route_quotes_known_customers_and_short_circuits_unknown_ones() {
    let ok = fees(Ok(cents(25)));
    assert_eq!(
        call(app(None, ok.clone()), "/orders/quote?customer_id=c1&cents=1000").await,
        (
            StatusCode::OK,
            r#"{"customer_id":"c1","price":{"subtotal_cents":1000,"fee_cents":25,"total_cents":1025}}"#.into()
        )
    );
    let (status, body) = call(
        app(None, ok.clone()),
        "/orders/quote?customer_id=zz&cents=1000",
    )
    .await;
    assert_eq!(
        (status, body),
        (
            StatusCode::NOT_FOUND,
            r#"{"error":"unknown customer zz"}"#.into()
        )
    );
    assert_eq!(
        ok.1.load(Ordering::SeqCst),
        1,
        "fee provider asked once, for c1 only"
    );
}
