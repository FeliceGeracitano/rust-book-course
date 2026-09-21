//! Composition root: the only crate that knows which concrete adapter satisfies
//! which port. Builds the shared HTTP client, the services, and one merged router.

mod config;

pub use config::Config;

use axum::Router;
use axum::routing::get;
use std::future::Future;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct App {
    pub router: Router,
}

impl App {
    pub fn new(config: &Config) -> anyhow::Result<Self> {
        // One client: one connection pool and one timeout policy shared by every adapter.
        let http = reqwest::Client::builder()
            .timeout(config.upstream_timeout)
            .build()?;
        let identity = Arc::new(integrations::identity::Client::new(
            http.clone(),
            config.identity_url.clone(),
        ));
        let payments = Arc::new(integrations::payments::Client::new(
            http,
            config.payments_url.clone(),
        ));

        // The identity adapter satisfies two ports owned by two different domains.
        let customers = Arc::new(customers::Service::new(identity.clone()));
        let billing = Arc::new(billing::Service::new(payments));
        let orders = Arc::new(orders::Service::new(identity, billing.clone()));

        let router = Router::new()
            .route("/health", get(|| async { "ok" }))
            .merge(transport_http::customers_routes(customers))
            .merge(transport_http::billing_routes(billing))
            .merge(transport_http::orders_routes(orders));
        Ok(Self {
            router: observability::instrument(router),
        })
    }
}

/// Serve until `shutdown` resolves, then let in-flight requests finish.
pub async fn serve(
    router: Router,
    listener: TcpListener,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> anyhow::Result<()> {
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown)
        .await?;
    Ok(())
}
