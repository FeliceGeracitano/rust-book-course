//! Billing domain: a quoted total is the subtotal plus a provider fee.
//! The fee comes through the `FeeProvider` port; this crate knows no HTTP.

use async_trait::async_trait;
use money::{Cents, MoneyError};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Quote {
    pub subtotal_cents: Cents,
    pub fee_cents: Cents,
    pub total_cents: Cents,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum BillingError {
    #[error(transparent)]
    Money(#[from] MoneyError),
    #[error("fee provider unavailable: {0}")]
    Unavailable(String),
    #[error("fee provider timed out")]
    Timeout,
}

/// Port: who decides the fee for a subtotal. Implemented by an adapter crate.
#[async_trait]
pub trait FeeProvider: Send + Sync {
    async fn fee(&self, subtotal: Cents) -> Result<Cents, BillingError>;
}

pub struct Service {
    fees: Arc<dyn FeeProvider>,
}

impl Service {
    pub fn new(fees: Arc<dyn FeeProvider>) -> Self {
        Self { fees }
    }

    pub async fn quote(&self, subtotal: Cents) -> Result<Quote, BillingError> {
        let fee = self.fees.fee(subtotal).await?;
        Ok(Quote {
            subtotal_cents: subtotal,
            fee_cents: fee,
            total_cents: subtotal.checked_add(fee)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use money::MAX_CENTS;

    struct Fixed(Result<Cents, BillingError>);

    #[async_trait]
    impl FeeProvider for Fixed {
        async fn fee(&self, _subtotal: Cents) -> Result<Cents, BillingError> {
            self.0.clone()
        }
    }

    fn cents(value: i64) -> Cents {
        Cents::new(value).unwrap()
    }

    #[tokio::test]
    async fn adds_the_provider_fee_to_the_subtotal() {
        let service = Service::new(Arc::new(Fixed(Ok(cents(25)))));
        let quote = service.quote(cents(1000)).await.unwrap();
        assert_eq!(
            (quote.subtotal_cents, quote.fee_cents, quote.total_cents),
            (cents(1000), cents(25), cents(1025))
        );
        assert_eq!(
            serde_json::to_string(&quote).unwrap(),
            r#"{"subtotal_cents":1000,"fee_cents":25,"total_cents":1025}"#
        );
    }

    #[tokio::test]
    async fn propagates_provider_failures_and_money_bounds() {
        let timeout = Service::new(Arc::new(Fixed(Err(BillingError::Timeout))));
        assert_eq!(timeout.quote(cents(1)).await, Err(BillingError::Timeout));
        let capped = Service::new(Arc::new(Fixed(Ok(cents(1)))));
        assert_eq!(
            capped.quote(cents(MAX_CENTS)).await,
            Err(BillingError::Money(MoneyError::TooLarge))
        );
    }
}
