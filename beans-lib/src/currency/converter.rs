//! Currency conversion using external API.

use crate::currency::ExchangeRateCache;
use crate::error::{BeansError, BeansResult};
use crate::models::Currency;
use rust_decimal::Decimal;
use std::time::Duration;

/// Converts between currencies using exchange rates.
#[derive(Debug, Clone)]
pub struct CurrencyConverter {
    // Placeholder implementation - will be expanded in final version
    cache: ExchangeRateCache,
    base_url: String,
}

impl CurrencyConverter {
    /// Creates a new converter with the given cache TTL.
    pub fn new(ttl: Duration) -> Self {
        Self {
            cache: ExchangeRateCache::new(ttl),
            base_url: "https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies".to_string(),
        }
    }

    /// Creates a new converter with a default cache TTL of 24 hours.
    pub fn default() -> Self {
        Self::new(Duration::from_secs(24 * 60 * 60))
    }

    /// Gets the exchange rate between two currencies.
    pub async fn get_exchange_rate(&self, from: &Currency, to: &Currency) -> BeansResult<f64> {
        // Placeholder implementation - will be expanded in final version
        if from.code() == to.code() {
            return Ok(1.0);
        }
        
        // Check cache first
        if let Some(rate) = self.cache.get(from.code(), to.code()) {
            return Ok(rate);
        }
        
        // Placeholder for API call
        Err(BeansError::NotImplemented("CurrencyConverter::get_exchange_rate".to_string()))
    }

    /// Converts an amount from one currency to another.
    pub async fn convert_amount(
        &self,
        amount: Decimal,
        from: &Currency,
        to: &Currency,
    ) -> BeansResult<Decimal> {
        // Placeholder implementation - will be expanded in final version
        if from.code() == to.code() {
            return Ok(amount);
        }
        
        let rate = self.get_exchange_rate(from, to).await?;
        let rate_decimal = Decimal::try_from(rate)
            .map_err(|e| BeansError::ConversionError(e.to_string()))?;
        
        Ok(amount * rate_decimal)
    }
}

