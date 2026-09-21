use app::{App, Config};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::json;
use std::net::SocketAddr;
use std::time::Duration;
use tower::ServiceExt;
use wiremock::matchers::path;
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn simulator() -> SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, demo_upstream::router())
            .await
            .unwrap();
    });
    addr
}

fn config(upstream: String) -> Config {
    Config::from_lookup(|key| match key {
        "IDENTITY_URL" | "PAYMENTS_URL" => Some(upstream.clone()),
        "UPSTREAM_TIMEOUT_MS" => Some("500".into()),
        _ => None,
    })
    .unwrap()
}

async fn call(app: &App, uri: &str) -> (StatusCode, String) {
    let response = app
        .router
        .clone()
        .oneshot(Request::get(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8(bytes.to_vec()).unwrap())
}

#[tokio::test]
async fn assembles_every_domain_behind_one_router() {
    let upstream = simulator().await;
    let app = App::new(&config(format!("http://{upstream}"))).unwrap();
    assert_eq!(call(&app, "/health").await, (StatusCode::OK, "ok".into()));
    assert_eq!(
        call(&app, "/orders/quote?customer_id=c1&cents=1000").await,
        (
            StatusCode::OK,
            r#"{"customer_id":"c1","price":{"subtotal_cents":1000,"fee_cents":25,"total_cents":1025}}"#.into()
        )
    );
    assert_eq!(
        call(&app, "/customers/c1").await,
        (StatusCode::OK, r#"{"id":"c1","name":"Ada"}"#.into())
    );
    assert_eq!(call(&app, "/customers/zz").await.0, StatusCode::NOT_FOUND);
    assert_eq!(
        call(&app, "/billing/quote?cents=-5").await.0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(call(&app, "/nope").await.0, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn config_requires_valid_provider_origins() {
    assert!(Config::from_lookup(|_| None).is_err());
    let with_path = Config::from_lookup(|key| match key {
        "IDENTITY_URL" => Some("http://x/api".into()),
        "PAYMENTS_URL" => Some("http://x".into()),
        _ => None,
    });
    assert!(with_path.is_err());
    let config = config("http://127.0.0.1:9090".into());
    assert_eq!(
        config.bind_addr,
        "127.0.0.1:8080".parse::<SocketAddr>().unwrap()
    );
    assert_eq!(config.upstream_timeout, Duration::from_millis(500));
}

#[tokio::test]
async fn shutdown_drains_in_flight_requests() {
    let provider = MockServer::start().await;
    Mock::given(path("/customers/c1"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"name": "Ada"}))
                .set_delay(Duration::from_millis(300)),
        )
        .mount(&provider)
        .await;
    let app = App::new(&config(provider.uri())).unwrap();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
    let server = tokio::spawn(app::serve(app.router, listener, async {
        let _ = stopped.await;
    }));

    let request = tokio::spawn(reqwest::get(format!("http://{addr}/customers/c1")));
    tokio::time::sleep(Duration::from_millis(50)).await;
    stop.send(()).unwrap();

    let response = request.await.unwrap().unwrap();
    assert_eq!(response.status(), 200);
    assert_eq!(
        response.text().await.unwrap(),
        r#"{"id":"c1","name":"Ada"}"#
    );
    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
}
