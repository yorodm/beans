//! Currency conversion using external API.

use crate::currency::ExchangeRateCache;
use crate::error::{BeansError, BeansResult};
use crate::models::Currency;
use rust_decimal::Decimal;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

/// Converts between currencies using exchange rates.
#[derive(Debug, Clone)]
pub struct CurrencyConverter {
    cache: ExchangeRateCache,
    base_url: String,
    fallback_url: Option<String>,
    client: reqwest::Client,
}

impl CurrencyConverter {
    /// Creates a new converter with the given cache TTL.
    pub fn new(ttl: Duration) -> Self {
        Self {
            cache: ExchangeRateCache::new(ttl),
            base_url: "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1".to_string(),
            fallback_url: None,
            client: reqwest::Client::new(),
        }
    }

    /// Creates a new converter with a default cache TTL of 24 hours.
    pub fn default() -> Self {
        Self::new(Duration::from_secs(24 * 60 * 60))
    }

    /// Sets the base URL for the API.
    /// 
    /// This is primarily used for testing.
    pub fn set_base_url(&mut self, url: String) {
        self.base_url = url;
    }

    /// Sets the fallback URL for the API.
    /// 
    /// This is used if the primary URL fails.
    pub fn set_fallback_url(&mut self, url: String) {
        self.fallback_url = Some(url);
    }

    /// Gets the exchange rate between two currencies.
    pub async fn get_exchange_rate<'a>(&self, from: &Currency<'a>, to: &Currency<'a>) -> BeansResult<f64> {
        let from_code = from.code().to_lowercase();
        let to_code = to.code().to_lowercase();
        
        // If converting to the same currency, return 1.0
        if from_code == to_code {
            return Ok(1.0);
        }
        
        // Check cache first
        if let Some(rate) = self.cache.get(&from_code, &to_code) {
            return Ok(rate);
        }
        
        // Fetch from API
        let rates = self.fetch_rates(&from_code).await?;
        
        // Get the specific rate we need
        let rate = rates.get(&to_code).ok_or_else(|| {
            BeansError::ExchangeRateUnavailable {
                from: from_code.clone(),
                to: to_code.clone(),
            }
        })?;
        
        Ok(*rate)
    }

    /// Converts an amount from one currency to another.
    pub async fn convert_amount<'a>(
        &self,
        amount: Decimal,
        from: &Currency<'a>,
        to: &Currency<'a>,
    ) -> BeansResult<Decimal> {
        let from_code = from.code();
        let to_code = to.code();
        
        if from_code == to_code {
            return Ok(amount);
        }
        
        let rate = self.get_exchange_rate(from, to).await?;
        let rate_decimal = Decimal::try_from(rate)
            .map_err(|e| BeansError::Other(format!("Failed to convert rate to Decimal: {}", e)))?;
        
        Ok(amount * rate_decimal)
    }

    /// Fetches all exchange rates for a given base currency.
    /// 
    /// This method fetches rates from the API and caches them.
    async fn fetch_rates(&self, base_currency: &str) -> BeansResult<HashMap<String, f64>> {
        // Build the URL
        let url = format!("{}/currencies/{}.json", self.base_url, base_currency);
        
        // Try to fetch from the primary URL
        let response = match self.client.get(&url).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp
                } else if let Some(fallback) = &self.fallback_url {
                    // If primary fails, try fallback
                    let fallback_url = format!("{}/currencies/{}.json", fallback, base_currency);
                    self.client.get(&fallback_url).send().await
                        .map_err(|e| BeansError::Network(e))?
                } else {
                    // No fallback, return the error
                    return Err(BeansError::Other(format!(
                        "API request failed with status: {}", 
                        resp.status()
                    )));
                }
            },
            Err(e) if self.fallback_url.is_some() => {
                // If primary fails with an error, try fallback
                let fallback_url = format!("{}/currencies/{}.json", self.fallback_url.as_ref().unwrap(), base_currency);
                self.client.get(&fallback_url).send().await
                    .map_err(|e| BeansError::Network(e))?
            },
            Err(e) => return Err(BeansError::Network(e)),
        };
        
        // Parse the JSON response
        let json: Value = response.json().await
            .map_err(|e| BeansError::Network(e))?;
        
        // Extract the rates
        let rates = json.get(base_currency)
            .ok_or_else(|| BeansError::Json(serde_json::Error::custom(
                format!("Missing base currency '{}' in response", base_currency)
            )))?;
        
        // Convert to HashMap
        let mut rate_map = HashMap::new();
        
        if let Value::Object(obj) = rates {
            for (currency, rate) in obj {
                if let Value::Number(num) = rate {
                    if let Some(n) = num.as_f64() {
                        rate_map.insert(currency.clone(), n);
                    }
                }
            }
        } else {
            return Err(BeansError::Json(serde_json::Error::custom(
                format!("Expected object for '{}' rates, got: {:?}", base_currency, rates)
            )));
        }
        
        // Cache all the rates
        self.cache.put_all(base_currency, rate_map.clone());
        
        Ok(rate_map)
    }
}
