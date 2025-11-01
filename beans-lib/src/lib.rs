//! Beans - A ledger library for tracking income and expenses.
//!
//! This library provides functionality for:
//! - Creating and managing ledger files (.bean extension)
//! - Adding, updating, and deleting ledger entries
//! - Filtering and querying entries by date, tags, currency, and type
//! - Currency conversion using external exchange rates
//! - Generating reports and analytics (income vs expenses over time)
//!
//! # Architecture
//!
//! The library is organized into several modules:
//!
//! - `models`: Core domain models (Currency, Tag, LedgerEntry, EntryType)
//! - `database`: Repository pattern for SQLite persistence
//! - `ledger`: High-level ledger management API
//! - `currency`: Exchange rate conversion and caching
//! - `reporting`: Time series data and report generation
//! - `error`: Error types and handling
//!

// Public modules
pub mod currency;
pub mod database;
pub mod error;
pub mod ledger;
pub mod models;
pub mod reporting;

// Prelude for convenient imports
pub mod prelude {
    //! Prelude module for convenient imports.
    //!
    //! Import everything you need with a single use statement:
    //!
    //! ```
    //! use beans_lib::prelude::*;
    //! ```

    // Re-export core types
    pub use crate::currency::{CurrencyConverter, ExchangeRateCache};
    pub use crate::database::{EntryFilter, Repository};
    pub use crate::error::{BeansError, BeansResult};
    pub use crate::ledger::LedgerManager;
    pub use crate::models::{Currency, EntryType, LedgerEntry, LedgerEntryBuilder, Tag};
    pub use crate::reporting::{
        ExportFormat, GroupBy, IncomeExpenseReport, PeriodSummary, ReportGenerator, 
        TagReport, TagSummary, TimePeriod, TimeSeriesData, TimeSeriesPoint,
    };

    // Re-export commonly used external types
    pub use chrono::{DateTime, Utc};
    pub use rust_decimal::Decimal;
    pub use uuid::Uuid;
}

// Re-export commonly used types at the crate root
pub use error::{BeansError, BeansResult};
pub use ledger::LedgerManager;
pub use models::{Currency, EntryType, LedgerEntry, LedgerEntryBuilder, Tag};
