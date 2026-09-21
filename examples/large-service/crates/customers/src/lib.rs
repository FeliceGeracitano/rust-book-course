//! Customer domain: identifier rules and the directory port.
//! No HTTP, no JSON: an adapter crate implements `Directory`.

use async_trait::async_trait;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomerId(String);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum CustomersError {
    #[error("invalid customer id: {0:?}")]
    InvalidId(String),
    #[error("customer directory unavailable: {0}")]
    Unavailable(String),
    #[error("customer directory timed out")]
    Timeout,
}

impl CustomerId {
    pub const MAX_LEN: usize = 64;

    /// Non-empty, at most 64 bytes, lowercase ASCII letters, digits, `_` or `-`.
    pub fn parse(raw: &str) -> Result<Self, CustomersError> {
        let valid = !raw.is_empty()
            && raw.len() <= Self::MAX_LEN
            && raw
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-');
        if valid {
            Ok(Self(raw.to_owned()))
        } else {
            Err(CustomersError::InvalidId(raw.to_owned()))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CustomerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Port: where customer names come from. Implemented by an adapter crate.
#[async_trait]
pub trait Directory: Send + Sync {
    async fn name(&self, id: &CustomerId) -> Result<Option<String>, CustomersError>;
}

pub struct Service {
    directory: Arc<dyn Directory>,
}

impl Service {
    pub fn new(directory: Arc<dyn Directory>) -> Self {
        Self { directory }
    }

    /// Validates the raw id before asking the directory, so bad input never leaves the process.
    pub async fn name(&self, raw_id: &str) -> Result<Option<String>, CustomersError> {
        let id = CustomerId::parse(raw_id)?;
        self.directory.name(&id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn parses_only_well_formed_ids() {
        assert_eq!(
            CustomerId::parse("c1").map(|id| id.to_string()),
            Ok("c1".into())
        );
        for bad in ["", "C1", "a b", &"x".repeat(65)] {
            assert_eq!(
                CustomerId::parse(bad),
                Err(CustomersError::InvalidId(bad.to_owned()))
            );
        }
    }

    struct Fake {
        calls: AtomicUsize,
    }

    #[async_trait]
    impl Directory for Fake {
        async fn name(&self, id: &CustomerId) -> Result<Option<String>, CustomersError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok((id.as_str() == "c1").then(|| "Ada".to_owned()))
        }
    }

    #[tokio::test]
    async fn service_validates_before_calling_the_directory() {
        let fake = Arc::new(Fake {
            calls: AtomicUsize::new(0),
        });
        let service = Service::new(fake.clone());
        assert_eq!(service.name("c1").await, Ok(Some("Ada".into())));
        assert_eq!(service.name("zz").await, Ok(None));
        assert_eq!(
            service.name("BAD!").await,
            Err(CustomersError::InvalidId("BAD!".into()))
        );
        assert_eq!(fake.calls.load(Ordering::SeqCst), 2);
    }
}
