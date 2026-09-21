use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use billing::BillingError;
use customers::CustomersError;
use money::MoneyError;
use orders::OrdersError;
use serde_json::json;

/// Every domain error lands on one of four HTTP outcomes. The mapping lives
/// here, once, instead of in each handler.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Upstream(String),
    #[error("upstream deadline exceeded")]
    Timeout,
}

impl ApiError {
    pub fn status(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Upstream(_) => StatusCode::BAD_GATEWAY,
            ApiError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status(), Json(json!({ "error": self.to_string() }))).into_response()
    }
}

impl From<MoneyError> for ApiError {
    fn from(error: MoneyError) -> Self {
        ApiError::BadRequest(error.to_string())
    }
}

impl From<CustomersError> for ApiError {
    fn from(error: CustomersError) -> Self {
        match error {
            CustomersError::InvalidId(_) => ApiError::BadRequest(error.to_string()),
            CustomersError::Unavailable(_) => ApiError::Upstream(error.to_string()),
            CustomersError::Timeout => ApiError::Timeout,
        }
    }
}

impl From<BillingError> for ApiError {
    fn from(error: BillingError) -> Self {
        match error {
            BillingError::Money(inner) => inner.into(),
            BillingError::Unavailable(_) => ApiError::Upstream(error.to_string()),
            BillingError::Timeout => ApiError::Timeout,
        }
    }
}

impl From<OrdersError> for ApiError {
    fn from(error: OrdersError) -> Self {
        match error {
            OrdersError::Customer(inner) => inner.into(),
            OrdersError::UnknownCustomer(id) => {
                ApiError::NotFound(format!("unknown customer {id}"))
            }
            OrdersError::Billing(inner) => inner.into(),
            OrdersError::Money(inner) => inner.into(),
        }
    }
}
