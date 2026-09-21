//! Orders domain: a read-only quote workflow across customers and billing.
//! `CustomerDirectory` is a consumer-owned port: orders declares exactly what it
//! needs (existence), and the identity adapter implements it alongside
//! `customers::Directory`. Billing and customers never depend on this crate.

use async_trait::async_trait;
use customers::{CustomerId, CustomersError};
use money::{Cents, MoneyError};
use serde::Serialize;
use std::sync::Arc;

#[async_trait]
pub trait CustomerDirectory: Send + Sync {
    async fn exists(&self, id: &CustomerId) -> Result<bool, OrdersError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OrderQuote {
    pub customer_id: String,
    pub price: billing::Quote,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum OrdersError {
    #[error(transparent)]
    Customer(#[from] CustomersError),
    #[error("unknown customer {0}")]
    UnknownCustomer(CustomerId),
    #[error(transparent)]
    Billing(#[from] billing::BillingError),
    #[error(transparent)]
    Money(#[from] MoneyError),
}

pub struct Service {
    customers: Arc<dyn CustomerDirectory>,
    billing: Arc<billing::Service>,
}

impl Service {
    pub fn new(customers: Arc<dyn CustomerDirectory>, billing: Arc<billing::Service>) -> Self {
        Self { customers, billing }
    }

    /// Validate input, confirm the customer exists, then ask billing. Unknown
    /// customers short-circuit: the fee provider is never called for them.
    pub async fn quote(&self, raw_id: &str, cents: i64) -> Result<OrderQuote, OrdersError> {
        let id = CustomerId::parse(raw_id)?;
        let subtotal = Cents::new(cents)?;
        if !self.customers.exists(&id).await? {
            return Err(OrdersError::UnknownCustomer(id));
        }
        let price = self.billing.quote(subtotal).await?;
        Ok(OrderQuote {
            customer_id: id.to_string(),
            price,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use billing::{BillingError, FeeProvider};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct KnownOnly;

    #[async_trait]
    impl CustomerDirectory for KnownOnly {
        async fn exists(&self, id: &CustomerId) -> Result<bool, OrdersError> {
            Ok(id.as_str() == "c1")
        }
    }

    struct CountingFees(AtomicUsize);

    #[async_trait]
    impl FeeProvider for CountingFees {
        async fn fee(&self, _subtotal: Cents) -> Result<Cents, BillingError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(Cents::new(25).unwrap())
        }
    }

    fn service() -> (Service, Arc<CountingFees>) {
        let fees = Arc::new(CountingFees(AtomicUsize::new(0)));
        let billing = Arc::new(billing::Service::new(fees.clone()));
        (Service::new(Arc::new(KnownOnly), billing), fees)
    }

    #[tokio::test]
    async fn quotes_a_known_customer() {
        let (service, fees) = service();
        let quote = service.quote("c1", 1000).await.unwrap();
        assert_eq!(quote.customer_id, "c1");
        assert_eq!(quote.price.total_cents, Cents::new(1025).unwrap());
        assert_eq!(fees.0.load(Ordering::SeqCst), 1);
        assert_eq!(
            serde_json::to_string(&quote).unwrap(),
            r#"{"customer_id":"c1","price":{"subtotal_cents":1000,"fee_cents":25,"total_cents":1025}}"#
        );
    }

    #[tokio::test]
    async fn unknown_customers_short_circuit_before_billing() {
        let (service, fees) = service();
        assert_eq!(
            service.quote("zz", 1000).await,
            Err(OrdersError::UnknownCustomer(
                CustomerId::parse("zz").unwrap()
            ))
        );
        assert_eq!(fees.0.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn rejects_bad_input_before_any_port() {
        let (service, fees) = service();
        assert_eq!(
            service.quote("BAD!", 1000).await,
            Err(OrdersError::Customer(CustomersError::InvalidId(
                "BAD!".into()
            )))
        );
        assert_eq!(
            service.quote("c1", -1).await,
            Err(OrdersError::Money(MoneyError::Negative))
        );
        assert_eq!(fees.0.load(Ordering::SeqCst), 0);
    }
}
