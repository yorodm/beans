//! High-level ledger management service.
//!
//! The LedgerManager provides the business logic layer for the Beans application.
//! It handles file operations, validation, and delegates persistence to the Repository.

use crate::database::{initialize_schema, EntryFilter, Repository, SQLiteRepository};
use crate::error::{BeansError, BeansResult};
use crate::models::LedgerEntry;
use chrono::Utc;
use std::fs;
use std::path::Path;
use uuid::Uuid;

/// Manages ledger operations.
#[derive(Debug)]
pub struct LedgerManager {
    /// The underlying repository for data persistence.
    repository: Box<dyn Repository>,
}

impl LedgerManager {
    /// Opens a ledger file or creates it if it doesn't exist.
    ///
    /// The file must have a `.bean` extension.
    pub fn open<P: AsRef<Path>>(path: P) -> BeansResult<Self> {
        let path = path.as_ref();

        // Validate file extension
        if let Some(ext) = path.extension() {
            if ext != "bean" {
                return Err(BeansError::InvalidLedgerFormat(format!(
                    "Ledger file must have .bean extension, got: {:?}",
                    ext
                )));
            }
        } else {
            return Err(BeansError::InvalidLedgerFormat(
                "Ledger file must have .bean extension".to_string(),
            ));
        }

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| BeansError::Io(e))?;
            }
        }

        // Open or create the SQLite database
        let repository = SQLiteRepository::open(path)?;

        // Initialize the schema
        let conn = repository.conn.lock().unwrap();
        initialize_schema(&conn)?;
        drop(conn);

        Ok(Self {
            repository: Box::new(repository),
        })
    }

    /// Creates an in-memory ledger for testing.
    pub fn in_memory() -> BeansResult<Self> {
        // Create an in-memory SQLite repository
        let repository = SQLiteRepository::in_memory()?;

        // Initialize the schema
        let conn = repository.conn.lock().unwrap();
        initialize_schema(&conn)?;
        drop(conn);

        Ok(Self {
            repository: Box::new(repository),
        })
    }

    /// Adds a new entry to the ledger.
    ///
    /// Returns the UUID of the created entry.
    pub fn add_entry(&self, entry: &LedgerEntry) -> BeansResult<Uuid> {
        // Validate the entry (additional business logic validation can be added here)
        self.validate_entry(entry)?;

        // Create the entry in the repository
        self.repository.create(entry)?;

        Ok(entry.id())
    }

    /// Retrieves an entry by its ID.
    pub fn get_entry(&self, id: Uuid) -> BeansResult<LedgerEntry> {
        self.repository.get(id)
    }

    /// Updates an existing entry.
    ///
    /// This will fail if the entry doesn't exist or if the entry is invalid.
    pub fn update_entry(&self, entry: &LedgerEntry) -> BeansResult<()> {
        // Validate the entry
        self.validate_entry(entry)?;

        // Update the entry with the current timestamp
        let updated_entry = entry.with_updated_at(Utc::now());

        // Update the entry in the repository
        self.repository.update(&updated_entry)
    }

    /// Deletes an entry by its ID.
    pub fn delete_entry(&self, id: Uuid) -> BeansResult<()> {
        self.repository.delete(id)
    }

    /// Lists entries matching the given filter.
    pub fn list_entries(&self, filter: &EntryFilter) -> BeansResult<Vec<LedgerEntry>> {
        self.repository.list(filter)
    }

    /// Counts entries matching the given filter.
    pub fn count_entries(&self, filter: &EntryFilter) -> BeansResult<usize> {
        self.repository.count(filter)
    }

    /// Gets all entries in the ledger.
    pub fn get_all_entries(&self) -> BeansResult<Vec<LedgerEntry>> {
        let filter = EntryFilter::default();
        self.repository.list(&filter)
    }

    /// Validates an entry according to business rules.
    ///
    /// This is separate from the model validation and can include additional
    /// business logic specific to the ledger.
    fn validate_entry(&self, entry: &LedgerEntry) -> BeansResult<()> {
        // Basic validation is already done in the LedgerEntry::build method
        // Additional business logic validation can be added here

        // For example, we could check if the entry date is in the future
        let now = Utc::now();
        if entry.date() > now {
            return Err(BeansError::validation(
                "Entry date cannot be in the future".to_string(),
            ));
        }

        Ok(())
    }
}
