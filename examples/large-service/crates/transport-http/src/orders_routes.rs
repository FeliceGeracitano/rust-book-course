use crate::ApiError;
use axum::Router;
use axum::extract::{Query, State};
use axum::response::Json;
use axum::routing::get;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct OrderQuery {
    customer_id: String,
    cents: i64,
}

pub fn orders_routes(service: Arc<orders::Service>) -> Router {
    Router::new()
        .route("/orders/quote", get(quote))
        .with_state(service)
}

async fn quote(
    State(service): State<Arc<orders::Service>>,
    Query(query): Query<OrderQuery>,
) -> Result<Json<orders::OrderQuote>, ApiError> {
    Ok(Json(service.quote(&query.customer_id, query.cents).await?))
}
