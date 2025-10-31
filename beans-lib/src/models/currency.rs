//! Currency model for representing monetary values with currency codes.

use crate::error::{BeansError, BeansResult};
use rust_decimal::Decimal;
use rusty_money::{iso, Money, FormattableCurrency};
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::fmt;


/// Represents a monetary value with a specific currency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Currency<'a> {
    money: Money<'a, iso::Currency>,
}

impl<'a> Currency<'a> {
    /// Creates a new Currency with the specified amount and currency code.
    ///
    /// Returns an error if the currency code is invalid.
    pub fn new(amount: Decimal, code: &'a str) -> BeansResult<Self> {
        let currency = iso::find(code)
            .ok_or_else(|| BeansError::validation(format!("Invalid currency code: {}", code)))?;
        
        Ok(Self {
            money: Money::from_decimal(amount, currency),
        })
    }
    
    /// Creates a new Currency with zero amount and the specified currency code.
    ///
    /// Returns an error if the currency code is invalid.
    pub fn zero(code: &'a str) -> BeansResult<Self> {
        Self::new(Decimal::ZERO, code)
    }
    
    /// Returns the ISO currency code.
    pub fn code(&self) -> &str {
        self.money.currency().code()
    }
    
    /// Returns the amount as a Decimal.
    pub fn amount(&self) -> Decimal {
        *self.money.amount()
    }
    
    /// Returns a reference to the underlying Money object.
    pub fn as_money(&self) -> &Money<'a, iso::Currency> {
        &self.money
    }
    
    /// Returns true if the amount is zero.
    pub fn is_zero(&self) -> bool {
        self.money.is_zero()
    }
    
    /// Returns true if the amount is positive.
    pub fn is_positive(&self) -> bool {
        self.money.is_positive()
    }
    
    /// Returns true if the amount is negative.
    pub fn is_negative(&self) -> bool {
        self.money.is_negative()
    }
    
    /// Adds another Currency to this one.
    ///
    /// Returns an error if the currencies don't match.
    pub fn add(&self, other: &Self) -> BeansResult<Self> {
        if self.code() != other.code() {
            return Err(BeansError::validation(format!(
                "Cannot add currencies with different codes: {} and {}",
                self.code(),
                other.code()
            )));
        }
        
        let result = self.money.clone() + other.money.clone();
        Ok(Self { money: result })
    }
    
    /// Subtracts another Currency from this one.
    ///
    /// Returns an error if the currencies don't match.
    pub fn subtract(&self, other: &Self) -> BeansResult<Self> {
        if self.code() != other.code() {
            return Err(BeansError::validation(format!(
                "Cannot subtract currencies with different codes: {} and {}",
                self.code(),
                other.code()
            )));
        }
        
        let result = self.money.clone() - other.money.clone();
        Ok(Self { money: result })
    }
    
    /// Multiplies this Currency by a scalar value.
    pub fn multiply(&self, scalar: Decimal) -> Self {
        let result = self.money.clone() * scalar;
        Self { money: result }
    }
    
    /// Divides this Currency by a scalar value.
    ///
    /// Returns an error if the divisor is zero.
    pub fn divide(&self, scalar: Decimal) -> BeansResult<Self> {
        if scalar.is_zero() {
            return Err(BeansError::validation("Cannot divide by zero".to_string()));
        }
        
        let result = self.money.clone() / scalar;
        Ok(Self { money: result })
    }
}

impl<'a> fmt::Display for Currency<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.money)
    }
}

// Helper functions for creating common currencies with zero amount
pub fn usd<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "USD").unwrap()
}

// Helper function for creating USD currency with a specific amount
pub fn usd_with_amount<'a>(amount: Decimal) -> Currency<'a> {
    Currency::new(amount, "USD").unwrap()
}

pub fn eur<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "EUR").unwrap()
}

pub fn gbp<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "GBP").unwrap()
}

pub fn jpy<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "JPY").unwrap()
}

pub fn cny<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "CNY").unwrap()
}

pub fn cad<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "CAD").unwrap()
}

pub fn aud<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "AUD").unwrap()
}

pub fn chf<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "CHF").unwrap()
}

// Serialization support
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableCurrency {
    code: String,
    amount: Decimal,
}

// Module for serializing/deserializing Currency
pub mod currency_serde {
    use super::*;
    
    pub fn serialize<'a, S>(currency: &Currency<'a>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let sc = SerializableCurrency {
            code: currency.code().to_string(),
            amount: currency.amount(),
        };
        sc.serialize(serializer)
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Currency<'static>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let sc = SerializableCurrency::deserialize(deserializer)?;
        // Find the ISO currency code statically
        let iso_currency = iso::find(&sc.code)
            .ok_or_else(|| serde::de::Error::custom(format!("Invalid currency code: {}", sc.code)))?;
        
        // Create a new Currency with the static ISO currency
        Ok(Currency {
            money: Money::from_decimal(sc.amount, iso_currency),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    
    #[test]
    fn test_currency_creation() {
        let usd = Currency::new(dec!(42.50), "USD").unwrap();
        assert_eq!(usd.code(), "USD");
        assert_eq!(usd.amount(), dec!(42.50));
        
        let zero_usd = Currency::zero("USD").unwrap();
        assert_eq!(zero_usd.code(), "USD");
        assert_eq!(zero_usd.amount(), dec!(0));
        assert!(zero_usd.is_zero());
    }
    
    #[test]
    fn test_currency_invalid_code() {
        let result = Currency::new(dec!(42.50), "INVALID");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_currency_arithmetic() {
        let usd1 = Currency::new(dec!(10), "USD").unwrap();
        let usd2 = Currency::new(dec!(20), "USD").unwrap();
        
        // Addition
        let sum = usd1.add(&usd2).unwrap();
        assert_eq!(sum.amount(), dec!(30));
        assert_eq!(sum.code(), "USD");
        
        // Subtraction
        let diff = usd2.subtract(&usd1).unwrap();
        assert_eq!(diff.amount(), dec!(10));
        assert_eq!(diff.code(), "USD");
        
        // Multiplication
        let product = usd1.multiply(dec!(3));
        assert_eq!(product.amount(), dec!(30));
        assert_eq!(product.code(), "USD");
        
        // Division
        let quotient = usd2.divide(dec!(2)).unwrap();
        assert_eq!(quotient.amount(), dec!(10));
        assert_eq!(quotient.code(), "USD");
    }
    
    #[test]
    fn test_currency_arithmetic_errors() {
        let usd = Currency::new(dec!(10), "USD").unwrap();
        let eur = Currency::new(dec!(10), "EUR").unwrap();
        
        // Different currencies
        assert!(usd.add(&eur).is_err());
        assert!(usd.subtract(&eur).is_err());
        
        // Division by zero
        assert!(usd.divide(dec!(0)).is_err());
    }
    
    #[test]
    fn test_helper_functions() {
        assert_eq!(usd().code(), "USD");
        assert_eq!(eur().code(), "EUR");
        assert_eq!(gbp().code(), "GBP");
        assert_eq!(jpy().code(), "JPY");
        assert_eq!(cny().code(), "CNY");
        assert_eq!(cad().code(), "CAD");
        assert_eq!(aud().code(), "AUD");
        assert_eq!(chf().code(), "CHF");
    }
    
    #[test]
    fn test_positive_negative_predicates() {
        let positive = Currency::new(dec!(10), "USD").unwrap();
        let negative = Currency::new(dec!(-10), "USD").unwrap();
        let zero = Currency::zero("USD").unwrap();
        
        assert!(positive.is_positive());
        assert!(!positive.is_negative());
        assert!(!positive.is_zero());
        
        assert!(!negative.is_positive());
        assert!(negative.is_negative());
        assert!(!negative.is_zero());
        
        assert!(!zero.is_positive());
        assert!(!zero.is_negative());
        assert!(zero.is_zero());
    }
}
