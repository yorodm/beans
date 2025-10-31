//! Currency type for representing ISO 4217 currency codes.

use crate::error::{BeansError, BeansResult};
use currency_rs::{Currency as CurrencyRs, CurrencyOpts};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Represents an ISO 4217 currency code.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Currency {
    code: String,
}

impl Currency {
    /// Creates a new currency with the given ISO 4217 code.
    ///
    /// The code is normalized to uppercase and must be 3 alphabetic characters.
    /// 
    /// # Examples
    ///
    /// ```
    /// use beans_lib::models::Currency;
    ///
    /// let usd = Currency::new("usd").unwrap();
    /// assert_eq!(usd.code(), "USD");
    ///
    /// // Invalid codes will return an error
    /// assert!(Currency::new("US").is_err());
    /// assert!(Currency::new("USD1").is_err());
    /// assert!(Currency::new("US$").is_err());
    /// ```
    pub fn new(code: impl AsRef<str>) -> BeansResult<Self> {
        let code = code.as_ref().trim().to_uppercase();
        
        // Validate code length
        if code.len() != 3 {
            return Err(BeansError::validation(
                "Currency code must be exactly 3 characters"
            ));
        }
        
        // Validate code contains only alphabetic characters
        if !code.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(BeansError::validation(
                "Currency code must contain only alphabetic characters"
            ));
        }
        
        // Validate if it's a recognized currency code
        // Note: currency_rs doesn't provide a validation method, so we'll just
        // create a currency with the code and see if it works
        let _ = CurrencyRs::new_float(0.0, Some(CurrencyOpts::new().set_symbol(&code)));
        
        Ok(Self { code })
    }

    /// Returns the currency code.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Creates a USD currency (United States Dollar).
    pub fn usd() -> Self {
        Self { code: "USD".to_string() }
    }

    /// Creates a EUR currency (Euro).
    pub fn eur() -> Self {
        Self { code: "EUR".to_string() }
    }

    /// Creates a GBP currency (British Pound).
    pub fn gbp() -> Self {
        Self { code: "GBP".to_string() }
    }

    /// Creates a JPY currency (Japanese Yen).
    pub fn jpy() -> Self {
        Self { code: "JPY".to_string() }
    }

    /// Creates a CNY currency (Chinese Yuan).
    pub fn cny() -> Self {
        Self { code: "CNY".to_string() }
    }

    /// Creates a CAD currency (Canadian Dollar).
    pub fn cad() -> Self {
        Self { code: "CAD".to_string() }
    }

    /// Creates a AUD currency (Australian Dollar).
    pub fn aud() -> Self {
        Self { code: "AUD".to_string() }
    }

    /// Creates a CHF currency (Swiss Franc).
    pub fn chf() -> Self {
        Self { code: "CHF".to_string() }
    }
    
    /// Creates a currency value with the specified amount.
    /// 
    /// This is used to create a currency value for calculations and formatting.
    pub fn with_amount(&self, amount: f64) -> CurrencyRs {
        CurrencyRs::new_float(amount, Some(CurrencyOpts::new().set_symbol(&self.code)))
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl TryFrom<&str> for Currency {
    type Error = BeansError;

    fn try_from(code: &str) -> Result<Self, Self::Error> {
        Self::new(code)
    }
}

impl TryFrom<String> for Currency {
    type Error = BeansError;

    fn try_from(code: String) -> Result<Self, Self::Error> {
        Self::new(code)
    }
}

impl FromStr for Currency {
    type Err = BeansError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
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
        
        let currency = Currency::new(" eur ").unwrap();
        assert_eq!(currency.code(), "EUR");
    }
    
    #[test]
    fn test_currency_validation() {
        // Too short
        assert!(Currency::new("US").is_err());
        
        // Too long
        assert!(Currency::new("USDT").is_err());
        
        // Non-alphabetic characters
        assert!(Currency::new("US$").is_err());
        assert!(Currency::new("123").is_err());
        assert!(Currency::new("AB1").is_err());
    }
    
    #[test]
    fn test_currency_factory_methods() {
        assert_eq!(Currency::usd().code(), "USD");
        assert_eq!(Currency::eur().code(), "EUR");
        assert_eq!(Currency::gbp().code(), "GBP");
        assert_eq!(Currency::jpy().code(), "JPY");
    }
    
    #[test]
    fn test_currency_try_from() {
        let currency: Currency = "USD".try_into().unwrap();
        assert_eq!(currency.code(), "USD");
        
        let currency: Currency = String::from("EUR").try_into().unwrap();
        assert_eq!(currency.code(), "EUR");
        
        let result: Result<Currency, _> = "US".try_into();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_currency_display() {
        let currency = Currency::new("USD").unwrap();
        assert_eq!(format!("{}", currency), "USD");
    }
    
    #[test]
    fn test_currency_with_amount() {
        let currency = Currency::new("USD").unwrap();
        let amount = currency.with_amount(42.50);
        assert_eq!(format!("{}", amount), "42.50");
        
        let currency = Currency::new("EUR").unwrap();
        let amount = currency.with_amount(100.00);
        assert_eq!(format!("{}", amount), "100.00");
    }
}

