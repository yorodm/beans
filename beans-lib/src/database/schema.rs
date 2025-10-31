//! Database schema for the ledger.

use crate::error::{BeansError, BeansResult};
use rusqlite::Connection;
use std::collections::HashMap;

/// Current schema version.
pub const CURRENT_SCHEMA_VERSION: i64 = 1;

/// Initializes the database schema.
///
/// This creates the necessary tables and indexes if they don't exist.
/// It also handles schema migrations if the database already exists but has an older schema version.
pub fn initialize_schema(conn: &Connection) -> BeansResult<()> {
    // Create schema_version table if it doesn't exist
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS schema_version (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            version INTEGER NOT NULL,
            updated_at TEXT NOT NULL
        );
        ",
    )
    .map_err(|e| BeansError::database(format!("Failed to create schema_version table: {}", e)))?;

    // Get current schema version from database
    let db_version = get_schema_version(conn)?;

    // If the database is new (version 0), create the initial schema
    if db_version == 0 {
        create_initial_schema(conn)?;
        set_schema_version(conn, CURRENT_SCHEMA_VERSION)?;
        return Ok(());
    }

    // If the database has an older version, run migrations
    if db_version < CURRENT_SCHEMA_VERSION {
        run_migrations(conn, db_version, CURRENT_SCHEMA_VERSION)?;
        set_schema_version(conn, CURRENT_SCHEMA_VERSION)?;
    }

    Ok(())
}

/// Creates the initial schema (version 1).
fn create_initial_schema(conn: &Connection) -> BeansResult<()> {
    conn.execute_batch(
        "
        -- Create entries table
        CREATE TABLE IF NOT EXISTS entries (
            id TEXT PRIMARY KEY,
            date TEXT NOT NULL,
            name TEXT NOT NULL,
            currency TEXT NOT NULL,
            amount TEXT NOT NULL,
            description TEXT,
            entry_type TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        -- Create tags table
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );

        -- Create entry_tags junction table
        CREATE TABLE IF NOT EXISTS entry_tags (
            entry_id TEXT NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (entry_id, tag_id),
            FOREIGN KEY (entry_id) REFERENCES entries (id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
        );

        -- Create indexes
        CREATE INDEX IF NOT EXISTS idx_entries_date ON entries (date);
        CREATE INDEX IF NOT EXISTS idx_entries_entry_type ON entries (entry_type);
        CREATE INDEX IF NOT EXISTS idx_entries_currency ON entries (currency);
        CREATE INDEX IF NOT EXISTS idx_tags_name ON tags (name);
        ",
    )
    .map_err(|e| BeansError::database(format!("Failed to create initial schema: {}", e)))?;

    Ok(())
}

/// Runs migrations to upgrade the schema from one version to another.
fn run_migrations(conn: &Connection, from_version: i64, to_version: i64) -> BeansResult<()> {
    // Define migrations as a map from version to migration function
    let migrations: HashMap<i64, fn(&Connection) -> BeansResult<()>> = HashMap::new();

    // Run migrations in order
    for version in from_version + 1..=to_version {
        if let Some(migration) = migrations.get(&version) {
            migration(conn)?;
        }
    }

    Ok(())
}

/// Returns the current schema version from the database.
///
/// Returns 0 if the schema_version table doesn't exist or is empty.
pub fn get_schema_version(conn: &Connection) -> BeansResult<i64> {
    // Check if the schema_version table exists
    let table_exists: bool = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='schema_version'",
            [],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !table_exists {
        return Ok(0);
    }

    // Get the version
    let version: Result<i64, rusqlite::Error> = conn.query_row(
        "SELECT version FROM schema_version WHERE id = 1",
        [],
        |row| row.get(0),
    );

    match version {
        Ok(v) => Ok(v),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
        Err(e) => Err(BeansError::database(format!(
            "Failed to get schema version: {}",
            e
        ))),
    }
}

/// Sets the schema version in the database.
fn set_schema_version(conn: &Connection, version: i64) -> BeansResult<()> {
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT OR REPLACE INTO schema_version (id, version, updated_at) VALUES (1, ?, ?)",
        (version, now),
    )
    .map_err(|e| BeansError::database(format!("Failed to set schema version: {}", e)))?;

    Ok(())
}

/// Validates the database schema.
///
/// This checks that all required tables and indexes exist.
pub fn validate_schema(conn: &Connection) -> BeansResult<bool> {
    // List of required tables
    let required_tables = vec!["entries", "tags", "entry_tags", "schema_version"];

    // List of required indexes
    let required_indexes = vec![
        "idx_entries_date",
        "idx_entries_entry_type",
        "idx_entries_currency",
        "idx_tags_name",
    ];

    // Check tables
    for table in &required_tables {
        let exists: bool = conn
            .query_row(
                "SELECT 1 FROM sqlite_master WHERE type='table' AND name=?",
                [table],
                |_| Ok(true),
            )
            .unwrap_or(false);

        if !exists {
            return Ok(false);
        }
    }

    // Check indexes
    for index in &required_indexes {
        let exists: bool = conn
            .query_row(
                "SELECT 1 FROM sqlite_master WHERE type='index' AND name=?",
                [index],
                |_| Ok(true),
            )
            .unwrap_or(false);

        if !exists {
            return Ok(false);
        }
    }

    Ok(true)
}
