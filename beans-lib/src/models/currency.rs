//! Currency type for representing ISO 4217 currency codes.

use crate::error::{BeansError, BeansResult};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::collections::HashSet;
use once_cell::sync::Lazy;

/// Common currency codes based on ISO 4217
static COMMON_CURRENCIES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut set = HashSet::new();
    // Major global currencies
    set.insert("USD"); // US Dollar
    set.insert("EUR"); // Euro
    set.insert("GBP"); // British Pound
    set.insert("JPY"); // Japanese Yen
    set.insert("CNY"); // Chinese Yuan
    set.insert("AUD"); // Australian Dollar
    set.insert("CAD"); // Canadian Dollar
    set.insert("CHF"); // Swiss Franc
    set.insert("HKD"); // Hong Kong Dollar
    set.insert("SGD"); // Singapore Dollar
    
    // Other common currencies
    set.insert("INR"); // Indian Rupee
    set.insert("RUB"); // Russian Ruble
    set.insert("ZAR"); // South African Rand
    set.insert("BRL"); // Brazilian Real
    set.insert("MXN"); // Mexican Peso
    set.insert("ARS"); // Argentine Peso
    set.insert("SEK"); // Swedish Krona
    set.insert("NOK"); // Norwegian Krone
    set.insert("DKK"); // Danish Krone
    set.insert("NZD"); // New Zealand Dollar
    set.insert("KRW"); // South Korean Won
    set.insert("TRY"); // Turkish Lira
    set.insert("PLN"); // Polish Złoty
    set.insert("THB"); // Thai Baht
    set.insert("IDR"); // Indonesian Rupiah
    set.insert("MYR"); // Malaysian Ringgit
    set.insert("PHP"); // Philippine Peso
    set.insert("TWD"); // Taiwan Dollar
    set.insert("AED"); // UAE Dirham
    set.insert("SAR"); // Saudi Riyal
    set
});

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
        
        Ok(Self { code })
    }

    /// Returns the currency code.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Checks if this is a common/major currency.
    ///
    /// This can be useful for UI prioritization or default selections.
    pub fn is_common(&self) -> bool {
        COMMON_CURRENCIES.contains(self.code.as_str())
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

    /// Returns a list of common currency codes.
    pub fn common_currencies() -> Vec<&'static str> {
        COMMON_CURRENCIES.iter().copied().collect()
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
    fn test_currency_is_common() {
        assert!(Currency::usd().is_common());
        assert!(Currency::eur().is_common());
        assert!(Currency::new("GBP").unwrap().is_common());
        
        // Create a likely uncommon currency
        let uncommon = Currency::new("XYZ").unwrap();
        assert!(!uncommon.is_common());
    }
    
    #[test]
    fn test_currency_common_currencies() {
        let common = Currency::common_currencies();
        assert!(common.contains(&"USD"));
        assert!(common.contains(&"EUR"));
        assert!(common.contains(&"GBP"));
        assert!(common.contains(&"JPY"));
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
}
