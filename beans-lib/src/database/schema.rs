//! Database schema for the ledger.

use crate::error::BeansResult;
use rusqlite::Connection;

/// Initializes the database schema.
///
/// This creates the necessary tables and indexes if they don't exist.
pub fn initialize_schema(conn: &Connection) -> BeansResult<()> {
    // Placeholder implementation - will be expanded in final version
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
        "
    )?;

    Ok(())
}

/// Returns the current schema version.
pub fn get_schema_version(_conn: &Connection) -> BeansResult<i64> {
    // Placeholder implementation - will be expanded in final version
    Ok(1)
}
