//! Currency model for representing monetary values with currency codes.

use crate::error::{BeansError, BeansResult};
use rust_decimal::Decimal;
use rusty_money::{iso, FormattableCurrency, Money};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
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
        let iso_currency = iso::find(&sc.code).ok_or_else(|| {
            serde::de::Error::custom(format!("Invalid currency code: {}", sc.code))
        })?;

        // Create a new Currency with the static ISO currency
        Ok(Currency {
            money: Money::from_decimal(sc.amount, iso_currency),
        })
    }
}
