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
//! # Examples
//!
//! ## Basic Usage
//!
//! ```no_run
//! use beans_lib::prelude::*;
//! use rust_decimal_macros::dec;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create or open a ledger
//! let ledger = LedgerManager::open("my_finances.bean")?;
//!
//! // Create an entry
//! let entry = LedgerEntryBuilder::new()
//!     .name("Monthly Salary")
//!     .amount(dec!(5000.00))
//!     .currency(Currency::usd())
//!     .entry_type(EntryType::Income)
//!     .description("April salary")
//!     .tag(Tag::new("salary")?)
//!     .build()?;
//!
//! // Add to ledger
//! let id = ledger.add_entry(&entry)?;
//!
//! // Query entries
//! let filter = EntryFilter::default();
//! let entries = ledger.list_entries(&filter)?;
//!
//! println!("Found {} entries", entries.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Currency Conversion
//!
//! ```no_run
//! use beans_lib::prelude::*;
//! use rust_decimal_macros::dec;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let converter = CurrencyConverter::default();
//! let amount_in_eur = converter.convert_amount(
//!     dec!(100.00),
//!     &Currency::usd(),
//!     &Currency::eur()
//! ).await?;
//!
//! println!("100 USD = {} EUR", amount_in_eur);
//! # Ok(())
//! # }
//! ```
//!
//! ## Generating Reports
//!
//! ```no_run
//! use beans_lib::prelude::*;
//! use chrono::{Duration, Utc};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let ledger = LedgerManager::open("my_finances.bean")?;
//! let converter = CurrencyConverter::default();
//! let generator = ReportGenerator::new(&ledger).with_converter(converter);
//!
//! let end = Utc::now();
//! let start = end - Duration::days(30);
//!
//! let report = generator.income_expense_report(
//!     start,
//!     end,
//!     TimePeriod::Daily,
//!     Some(Currency::usd()),
//!     None
//! ).await?;
//!
//! println!("Income: {}", report.summary.income);
//! println!("Expenses: {}", report.summary.expenses);
//! println!("Net: {}", report.summary.net);
//! # Ok(())
//! # }
//! ```

// Public modules
pub mod error;
pub mod currency;
pub mod database;
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
    pub use crate::error::{BeansError, BeansResult};
    pub use crate::currency::{CurrencyConverter, ExchangeRateCache};
    pub use crate::database::{EntryFilter, Repository};
    pub use crate::ledger::LedgerManager;
    pub use crate::models::{Currency, EntryType, LedgerEntry, LedgerEntryBuilder, Tag};
    pub use crate::reporting::{
        IncomeExpenseReport, PeriodSummary, ReportGenerator, TimePeriod, TimeSeriesData,
        TimeSeriesPoint,
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
