//! Error types for the Beans library.
//!
//! This module defines all error types used throughout the library,
//! providing clear and actionable error messages.

use thiserror::Error;

/// Result type alias for Beans operations.
pub type BeansResult<T> = Result<T, BeansError>;

/// Main error type for the Beans library.
#[derive(Error, Debug)]
pub enum BeansError {
    /// Database-related errors from SQLite.
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    /// Custom database errors with detailed messages.
    #[error("Database error: {0}")]
    DatabaseCustom(String),

    /// I/O errors when working with files.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Validation errors for ledger entries.
    #[error("Validation error: {0}")]
    Validation(String),

    /// Currency conversion errors.
    #[error("Currency error: {0}")]
    Currency(String),

    /// Network errors when fetching exchange rates.
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON parsing errors.
    #[error("JSON error: {0}")]
    Json(String),

    /// Ledger file not found.
    #[error("Ledger file not found: {0}")]
    LedgerNotFound(String),

    /// Invalid ledger file format.
    #[error("Invalid ledger file format: {0}")]
    InvalidLedgerFormat(String),

    /// Entry not found in the ledger.
    #[error("Entry not found: {0}")]
    NotFound(String),

    /// Currency conversion rate not available.
    #[error("Exchange rate not available for {from} to {to}")]
    ExchangeRateUnavailable { from: String, to: String },

    /// Invalid date range for queries.
    #[error("Invalid date range: start date must be before end date")]
    InvalidDateRange,

    /// Generic error for other cases.
    #[error("Operation failed: {0}")]
    Other(String),

    /// Feature not yet implemented (placeholder).
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Error converting between types.
    #[error("Conversion error: {0}")]
    ConversionError(String),
}

impl BeansError {
    /// Creates a validation error with a custom message.
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Creates a currency error with a custom message.
    pub fn currency(msg: impl Into<String>) -> Self {
        Self::Currency(msg.into())
    }

    /// Creates a database error with a custom message.
    pub fn database(msg: impl Into<String>) -> Self {
        Self::DatabaseCustom(msg.into())
    }

    /// Creates a not found error with a custom message.
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    /// Creates a generic error with a custom message.
    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }
}
