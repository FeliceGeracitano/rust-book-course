use crate::ApiError;
use axum::Router;
use axum::extract::{Query, State};
use axum::response::Json;
use axum::routing::get;
use money::Cents;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct QuoteQuery {
    cents: i64,
}

pub fn billing_routes(service: Arc<billing::Service>) -> Router {
    Router::new()
        .route("/billing/quote", get(quote))
        .with_state(service)
}

/// A thin handler still has three jobs: parse, call, map. Nothing else.
async fn quote(
    State(service): State<Arc<billing::Service>>,
    Query(query): Query<QuoteQuery>,
) -> Result<Json<billing::Quote>, ApiError> {
    let subtotal = Cents::new(query.cents)?;
    Ok(Json(service.quote(subtotal).await?))
}
