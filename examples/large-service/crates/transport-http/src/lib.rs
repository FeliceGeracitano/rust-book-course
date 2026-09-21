//! HTTP transport: one router per domain, merged once by `app`. Handlers parse
//! input, call a service, and map the result. They never build clients or services.

mod billing_routes;
mod customers_routes;
mod error;
mod orders_routes;

pub use billing_routes::billing_routes;
pub use customers_routes::customers_routes;
pub use error::ApiError;
pub use orders_routes::orders_routes;
