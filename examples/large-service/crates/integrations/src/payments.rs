//! Payment provider adapter: implements `billing::FeeProvider`.

use crate::httpjson::{Fetched, HttpJsonError, get_json};
use crate::origin::Origin;
use async_trait::async_trait;
use billing::{BillingError, FeeProvider};
use money::Cents;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FeeBody {
    fee_cents: i64,
}

#[derive(Clone)]
pub struct Client {
    http: reqwest::Client,
    base: Origin,
}

impl Client {
    pub fn new(http: reqwest::Client, base: Origin) -> Self {
        Self { http, base }
    }
}

fn to_billing_error(error: HttpJsonError) -> BillingError {
    match error {
        HttpJsonError::Timeout => BillingError::Timeout,
        other => BillingError::Unavailable(other.to_string()),
    }
}

#[async_trait]
impl FeeProvider for Client {
    async fn fee(&self, subtotal: Cents) -> Result<Cents, BillingError> {
        let url = self.base.join(&format!("/fees?cents={}", subtotal.value()));
        let body = match get_json::<FeeBody>(&self.http, url)
            .await
            .map_err(to_billing_error)?
        {
            Fetched::Found(body) => body,
            Fetched::NotFound => {
                return Err(BillingError::Unavailable("fee endpoint not found".into()));
            }
        };
        Cents::new(body.fee_cents).map_err(|error| {
            BillingError::Unavailable(format!("provider returned an invalid fee: {error}"))
        })
    }
}
