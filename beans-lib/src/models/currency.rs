//! Currency type for representing ISO 4217 currency codes.

use crate::error::{BeansError, BeansResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents an ISO 4217 currency code.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Currency {
    code: String,
}

impl Currency {
    /// Creates a new currency with the given ISO 4217 code.
    ///
    /// The code is normalized to uppercase and must be 3 letters.
    pub fn new(code: impl AsRef<str>) -> BeansResult<Self> {
        let code = code.as_ref().to_uppercase();
        // Placeholder validation - will be more robust in final implementation
        if code.len() != 3 {
            return Err(BeansError::ValidationError(
                "Currency code must be 3 letters".to_string(),
            ));
        }
        
        Ok(Self { code })
    }

    /// Returns the currency code.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Creates a USD currency.
    pub fn usd() -> Self {
        Self { code: "USD".to_string() }
    }

    /// Creates a EUR currency.
    pub fn eur() -> Self {
        Self { code: "EUR".to_string() }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_new() {
        let currency = Currency::new("USD").unwrap();
        assert_eq!(currency.code(), "USD");
    }

    #[test]
    fn test_currency_normalize() {
        let currency = Currency::new("usd").unwrap();
        assert_eq!(currency.code(), "USD");
    }
}

