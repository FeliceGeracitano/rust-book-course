//! Shared money invariant: nonnegative cents with an upper bound.
//! A leaf crate: every domain uses it, it depends on nothing of ours.

use serde::{Deserialize, Serialize};
use std::fmt;

pub const MAX_CENTS: i64 = 100_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "i64", into = "i64")]
pub struct Cents(i64);

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MoneyError {
    #[error("amount must not be negative")]
    Negative,
    #[error("amount exceeds {MAX_CENTS} cents")]
    TooLarge,
}

impl Cents {
    pub fn new(value: i64) -> Result<Self, MoneyError> {
        if value < 0 {
            return Err(MoneyError::Negative);
        }
        if value > MAX_CENTS {
            return Err(MoneyError::TooLarge);
        }
        Ok(Self(value))
    }

    pub fn value(self) -> i64 {
        self.0
    }

    pub fn checked_add(self, other: Cents) -> Result<Cents, MoneyError> {
        Cents::new(self.0.saturating_add(other.0))
    }
}

impl TryFrom<i64> for Cents {
    type Error = MoneyError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Cents::new(value)
    }
}

impl From<Cents> for i64 {
    fn from(cents: Cents) -> Self {
        cents.0
    }
}

impl fmt::Display for Cents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${}.{:02}", self.0 / 100, self.0 % 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_negative_and_too_large() {
        assert_eq!(Cents::new(-1), Err(MoneyError::Negative));
        assert_eq!(Cents::new(MAX_CENTS + 1), Err(MoneyError::TooLarge));
        assert_eq!(Cents::new(MAX_CENTS).map(Cents::value), Ok(MAX_CENTS));
    }

    #[test]
    fn checked_add_respects_the_cap() {
        let max = Cents::new(MAX_CENTS).unwrap();
        assert_eq!(
            max.checked_add(Cents::new(1).unwrap()),
            Err(MoneyError::TooLarge)
        );
        assert_eq!(
            Cents::new(1000)
                .unwrap()
                .checked_add(Cents::new(25).unwrap()),
            Cents::new(1025)
        );
    }

    #[test]
    fn serializes_as_an_integer() {
        assert_eq!(
            serde_json::to_string(&Cents::new(1025).unwrap()).unwrap(),
            "1025"
        );
        assert_eq!(
            serde_json::from_str::<Cents>("1025").unwrap(),
            Cents::new(1025).unwrap()
        );
        assert!(serde_json::from_str::<Cents>("-5").is_err());
    }

    #[test]
    fn displays_dollars_and_cents() {
        assert_eq!(Cents::new(1025).unwrap().to_string(), "$10.25");
    }
}
