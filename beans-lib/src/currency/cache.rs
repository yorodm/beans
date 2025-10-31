//! Cache for exchange rates.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Cache for exchange rates with time-to-live (TTL).
#[derive(Debug, Clone)]
pub struct ExchangeRateCache {
    // Placeholder implementation - will be expanded in final version
    cache: Arc<Mutex<HashMap<String, (f64, Instant)>>>,
    ttl: Duration,
}

impl ExchangeRateCache {
    /// Creates a new cache with the given TTL.
    pub fn new(ttl: Duration) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            ttl,
        }
    }

    /// Creates a new cache with a default TTL of 24 hours.
    pub fn default() -> Self {
        Self::new(Duration::from_secs(24 * 60 * 60))
    }

    /// Gets a rate from the cache.
    pub fn get(&self, from: &str, to: &str) -> Option<f64> {
        // Placeholder implementation - will be expanded in final version
        let key = Self::make_key(from, to);
        let cache = self.cache.lock().unwrap();
        
        cache.get(&key).and_then(|(rate, timestamp)| {
            if timestamp.elapsed() < self.ttl {
                Some(*rate)
            } else {
                None
            }
        })
    }

    /// Puts a rate into the cache.
    pub fn put(&self, from: &str, to: &str, rate: f64) {
        // Placeholder implementation - will be expanded in final version
        let key = Self::make_key(from, to);
        let mut cache = self.cache.lock().unwrap();
        
        cache.insert(key, (rate, Instant::now()));
    }

    /// Clears the cache.
    pub fn clear(&self) {
        // Placeholder implementation - will be expanded in final version
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Creates a cache key from currency codes.
    fn make_key(from: &str, to: &str) -> String {
        format!("{}:{}", from.to_uppercase(), to.to_uppercase())
    }
}

