use billing::{BillingError, FeeProvider};
use customers::{CustomerId, CustomersError, Directory};
use integrations::{Origin, identity, payments};
use money::Cents;
use orders::CustomerDirectory;
use serde_json::json;
use std::time::Duration;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn http() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_millis(200))
        .build()
        .unwrap()
}

fn id(raw: &str) -> CustomerId {
    CustomerId::parse(raw).unwrap()
}

async fn identity(server: &MockServer) -> identity::Client {
    identity::Client::new(http(), Origin::parse(&server.uri()).unwrap())
}

#[tokio::test]
async fn identity_maps_found_and_missing_customers_for_both_ports() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/customers/c1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"name": "Ada"})))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/customers/zz"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    let client = identity(&server).await;
    assert_eq!(client.name(&id("c1")).await, Ok(Some("Ada".into())));
    assert_eq!(client.name(&id("zz")).await, Ok(None));
    assert!(client.exists(&id("c1")).await.unwrap());
    assert!(!client.exists(&id("zz")).await.unwrap());
}

#[tokio::test]
async fn identity_classifies_provider_failures() {
    let server = MockServer::start().await;
    Mock::given(path("/customers/err"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;
    Mock::given(path("/customers/slow"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"name": "x"}))
                .set_delay(Duration::from_millis(600)),
        )
        .mount(&server)
        .await;
    Mock::given(path("/customers/bad"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"nope": 1})))
        .mount(&server)
        .await;
    Mock::given(path("/customers/big"))
        .respond_with(ResponseTemplate::new(200).set_body_string("x".repeat(2 * 1024 * 1024)))
        .mount(&server)
        .await;
    let client = identity(&server).await;
    assert!(matches!(
        client.name(&id("err")).await,
        Err(CustomersError::Unavailable(_))
    ));
    assert_eq!(client.name(&id("slow")).await, Err(CustomersError::Timeout));
    assert!(matches!(
        client.name(&id("bad")).await,
        Err(CustomersError::Unavailable(_))
    ));
    assert!(matches!(
        client.name(&id("big")).await,
        Err(CustomersError::Unavailable(_))
    ));
    assert_eq!(
        client.exists(&id("slow")).await,
        Err(CustomersError::Timeout.into())
    );
}

#[tokio::test]
async fn one_shared_client_serves_concurrent_calls() {
    let server = MockServer::start().await;
    Mock::given(path("/customers/c1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"name": "Ada"})))
        .mount(&server)
        .await;
    let client = identity(&server).await;
    let c1 = id("c1");
    let (first, second) = tokio::join!(client.name(&c1), client.exists(&c1));
    assert_eq!(first, Ok(Some("Ada".into())));
    assert!(second.unwrap());
}

#[tokio::test]
async fn payments_parses_the_fee_and_rejects_invalid_ones() {
    let server = MockServer::start().await;
    Mock::given(path("/fees"))
        .and(query_param("cents", "1000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"fee_cents": 25})))
        .mount(&server)
        .await;
    Mock::given(path("/fees"))
        .and(query_param("cents", "7"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"fee_cents": -1})))
        .mount(&server)
        .await;
    let client = payments::Client::new(http(), Origin::parse(&server.uri()).unwrap());
    assert_eq!(
        client.fee(Cents::new(1000).unwrap()).await,
        Ok(Cents::new(25).unwrap())
    );
    assert!(matches!(
        client.fee(Cents::new(7).unwrap()).await,
        Err(BillingError::Unavailable(_))
    ));
    assert!(matches!(
        client.fee(Cents::new(9).unwrap()).await,
        Err(BillingError::Unavailable(_))
    ));
}
