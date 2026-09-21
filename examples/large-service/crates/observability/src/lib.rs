//! Cross-cutting request logging. A mechanism, not a domain: it knows nothing
//! about customers, billing, or orders, so every crate can depend on it safely.

use axum::Router;
use axum::body::Body;
use axum::http::Request;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

/// Install the process-wide subscriber. Call once, from `main`.
pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();
}

/// Logs method and path only. Query strings can carry identifiers, so they stay out of logs.
pub fn instrument(router: Router) -> Router {
    router.layer(TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
        tracing::info_span!("request", method = %request.method(), path = %request.uri().path())
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::routing::get;
    use tower::ServiceExt;

    #[tokio::test]
    async fn instrumented_router_still_serves() {
        let router = instrument(Router::new().route("/health", get(|| async { "ok" })));
        let response = router
            .oneshot(
                Request::get("/health?secret=1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
