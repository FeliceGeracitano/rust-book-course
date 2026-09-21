//! Identity provider adapter. One instance serves two ports: `customers::Directory`
//! (what is this customer's name?) and `orders::CustomerDirectory` (does it exist?).

use crate::httpjson::{Fetched, HttpJsonError, get_json};
use crate::origin::Origin;
use async_trait::async_trait;
use customers::{CustomerId, CustomersError, Directory};
use orders::{CustomerDirectory, OrdersError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CustomerBody {
    name: String,
}

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    base: Origin,
}

impl Client {
    /// Takes the shared `reqwest::Client`; the caller owns its lifetime and timeout policy.
    pub fn new(http: reqwest::Client, base: Origin) -> Self {
        Self { http, base }
    }

    pub async fn lookup(&self, id: &CustomerId) -> Result<Option<String>, HttpJsonError> {
        let url = self.base.join(&format!("/customers/{}", id.as_str()));
        match get_json::<CustomerBody>(&self.http, url).await? {
            Fetched::Found(body) => Ok(Some(body.name)),
            Fetched::NotFound => Ok(None),
        }
    }
}

fn to_customers_error(error: HttpJsonError) -> CustomersError {
    match error {
        HttpJsonError::Timeout => CustomersError::Timeout,
        other => CustomersError::Unavailable(other.to_string()),
    }
}

#[async_trait]
impl Directory for Client {
    async fn name(&self, id: &CustomerId) -> Result<Option<String>, CustomersError> {
        self.lookup(id).await.map_err(to_customers_error)
    }
}

#[async_trait]
impl CustomerDirectory for Client {
    async fn exists(&self, id: &CustomerId) -> Result<bool, OrdersError> {
        Ok(self.lookup(id).await.map_err(to_customers_error)?.is_some())
    }
}
