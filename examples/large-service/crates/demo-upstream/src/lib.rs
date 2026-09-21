//! Local simulator standing in for the identity and payment providers.
//! Knows one customer (`c1` → Ada) and always quotes a 25-cent fee. It never charges money.

use axum::Router;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use axum::routing::get;
use serde_json::json;
use std::collections::HashMap;

pub fn router() -> Router {
    Router::new()
        .route("/customers/{id}", get(customer))
        .route("/fees", get(fee))
}

async fn customer(Path(id): Path<String>) -> impl IntoResponse {
    if id == "c1" {
        (StatusCode::OK, Json(json!({"name": "Ada"})))
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "unknown customer"})),
        )
    }
}

async fn fee(Query(query): Query<HashMap<String, String>>) -> impl IntoResponse {
    let cents = query.get("cents").and_then(|raw| raw.parse::<i64>().ok());
    match cents {
        Some(cents) if cents >= 0 => (StatusCode::OK, Json(json!({"fee_cents": 25}))),
        _ => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "cents must be a nonnegative integer"})),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn call(uri: &str) -> (StatusCode, String) {
        let response = router()
            .oneshot(Request::get(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn serves_the_documented_fixtures() {
        assert_eq!(
            call("/customers/c1").await,
            (StatusCode::OK, r#"{"name":"Ada"}"#.into())
        );
        assert_eq!(call("/customers/zz").await.0, StatusCode::NOT_FOUND);
        assert_eq!(
            call("/fees?cents=1000").await,
            (StatusCode::OK, r#"{"fee_cents":25}"#.into())
        );
        assert_eq!(call("/fees").await.0, StatusCode::BAD_REQUEST);
        assert_eq!(call("/fees?cents=-3").await.0, StatusCode::BAD_REQUEST);
    }
}
