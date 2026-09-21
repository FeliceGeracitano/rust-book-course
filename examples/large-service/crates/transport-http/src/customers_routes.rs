use crate::ApiError;
use axum::Router;
use axum::extract::{Path, State};
use axum::response::Json;
use axum::routing::get;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
struct CustomerBody {
    id: String,
    name: String,
}

pub fn customers_routes(service: Arc<customers::Service>) -> Router {
    Router::new()
        .route("/customers/{id}", get(get_customer))
        .with_state(service)
}

async fn get_customer(
    State(service): State<Arc<customers::Service>>,
    Path(id): Path<String>,
) -> Result<Json<CustomerBody>, ApiError> {
    match service.name(&id).await? {
        Some(name) => Ok(Json(CustomerBody { id, name })),
        None => Err(ApiError::NotFound(format!("unknown customer {id}"))),
    }
}
