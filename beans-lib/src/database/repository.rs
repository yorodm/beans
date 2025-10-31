//! Repository pattern for database operations.

use crate::error::BeansResult;
use crate::models::LedgerEntry;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Filter for querying ledger entries.
#[derive(Debug, Clone, Default)]
pub struct EntryFilter {
    /// Start date for filtering (inclusive).
    pub start_date: Option<DateTime<Utc>>,
    /// End date for filtering (inclusive).
    pub end_date: Option<DateTime<Utc>>,
    /// Filter by entry type.
    pub entry_type: Option<crate::models::EntryType>,
    /// Filter by currency.
    pub currency: Option<String>,
    /// Filter by tags (entries must have all specified tags).
    pub tags: Vec<String>,
    /// Maximum number of entries to return.
    pub limit: Option<usize>,
    /// Number of entries to skip.
    pub offset: Option<usize>,
}

/// Repository trait for ledger entry operations.
pub trait Repository: std::fmt::Debug {
    /// Creates a new entry in the repository.
    fn create(&self, entry: &LedgerEntry) -> BeansResult<()>;
    
    /// Retrieves an entry by its ID.
    fn get(&self, id: Uuid) -> BeansResult<LedgerEntry>;
    
    /// Updates an existing entry.
    fn update(&self, entry: &LedgerEntry) -> BeansResult<()>;
    
    /// Deletes an entry by its ID.
    fn delete(&self, id: Uuid) -> BeansResult<()>;
    
    /// Lists entries matching the given filter.
    fn list(&self, filter: &EntryFilter) -> BeansResult<Vec<LedgerEntry>>;
    
    /// Counts entries matching the given filter.
    fn count(&self, filter: &EntryFilter) -> BeansResult<usize>;
}
