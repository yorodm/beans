//! Database schema for the ledger.

use crate::error::{BeansError, BeansResult};
use rusqlite::Connection;
use sql_query_builder as sql;
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
    // Create entries table
    let create_entries_table = sql::CreateTable::new()
        .create_table_if_not_exists("entries")
        .column("id TEXT PRIMARY KEY")
        .column("date TEXT NOT NULL")
        .column("name TEXT NOT NULL")
        .column("currency TEXT NOT NULL")
        .column("amount TEXT NOT NULL")
        .column("description TEXT")
        .column("entry_type TEXT NOT NULL")
        .column("created_at TEXT NOT NULL")
        .column("updated_at TEXT NOT NULL")
        .as_string();

    conn.execute(&create_entries_table, [])
        .map_err(|e| BeansError::database(format!("Failed to create entries table: {}", e)))?;

    // Create tags table
    let create_tags_table = sql::CreateTable::new()
        .create_table_if_not_exists("tags")
        .column("id INTEGER PRIMARY KEY AUTOINCREMENT")
        .column("name TEXT NOT NULL UNIQUE")
        .as_string();

    conn.execute(&create_tags_table, [])
        .map_err(|e| BeansError::database(format!("Failed to create tags table: {}", e)))?;

    // Create entry_tags junction table
    let create_entry_tags_table = sql::CreateTable::new()
        .create_table_if_not_exists("entry_tags")
        .column("entry_id TEXT NOT NULL")
        .column("tag_id INTEGER NOT NULL")
        .column("PRIMARY KEY (entry_id, tag_id)")
        .column("FOREIGN KEY (entry_id) REFERENCES entries (id) ON DELETE CASCADE")
        .column("FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE")
        .as_string();

    conn.execute(&create_entry_tags_table, [])
        .map_err(|e| BeansError::database(format!("Failed to create entry_tags table: {}", e)))?;

    // Create indexes
    let create_idx_entries_date = sql::CreateIndex::new()
        .create_index_if_not_exists("idx_entries_date")
        .on("entries")
        .column("date")
        .as_string();

    conn.execute(&create_idx_entries_date, [])
        .map_err(|e| BeansError::database(format!("Failed to create idx_entries_date: {}", e)))?;

    let create_idx_entries_entry_type = sql::CreateIndex::new()
        .create_index_if_not_exists("idx_entries_entry_type")
        .on("entries")
        .column("entry_type")
        .as_string();

    conn.execute(&create_idx_entries_entry_type, [])
        .map_err(|e| {
            BeansError::database(format!("Failed to create idx_entries_entry_type: {}", e))
        })?;

    let create_idx_entries_currency = sql::CreateIndex::new()
        .create_index_if_not_exists("idx_entries_currency")
        .on("entries")
        .column("currency")
        .as_string();

    conn.execute(&create_idx_entries_currency, [])
        .map_err(|e| {
            BeansError::database(format!("Failed to create idx_entries_currency: {}", e))
        })?;

    let create_idx_tags_name = sql::CreateIndex::new()
        .create_index_if_not_exists("idx_tags_name")
        .on("tags")
        .column("name")
        .as_string();

    conn.execute(&create_idx_tags_name, [])
        .map_err(|e| BeansError::database(format!("Failed to create idx_tags_name: {}", e)))?;

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
    let check_table_query = sql::Select::new()
        .select("1")
        .from("sqlite_master")
        .where_clause("type='table'")
        .where_clause("name='schema_version'")
        .as_string();

    let table_exists: bool = conn
        .query_row(&check_table_query, [], |_| Ok(true))
        .unwrap_or(false);

    if !table_exists {
        return Ok(0);
    }

    // Get the version
    let get_version_query = sql::Select::new()
        .select("version")
        .from("schema_version")
        .where_clause("id = 1")
        .as_string();

    let version: Result<i64, rusqlite::Error> =
        conn.query_row(&get_version_query, [], |row| row.get(0));

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

    // Note: INSERT OR REPLACE is SQLite-specific syntax
    // sql_query_builder doesn't have a direct method for this, so we use raw SQL for this specific case
    let insert_query = sql::Insert::new()
        .raw("INSERT OR REPLACE INTO schema_version (id, version, updated_at)")
        .values("(1, ?, ?)")
        .as_string();

    conn.execute(&insert_query, rusqlite::params![version, now])
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
        let check_table_query = sql::Select::new()
            .select("1")
            .from("sqlite_master")
            .where_clause("type='table'")
            .where_clause("name=?")
            .as_string();

        let exists: bool = conn
            .query_row(&check_table_query, [table], |_| Ok(true))
            .unwrap_or(false);

        if !exists {
            return Ok(false);
        }
    }

    // Check indexes
    for index in &required_indexes {
        let check_index_query = sql::Select::new()
            .select("1")
            .from("sqlite_master")
            .where_clause("type='index'")
            .where_clause("name=?")
            .as_string();

        let exists: bool = conn
            .query_row(&check_index_query, [index], |_| Ok(true))
            .unwrap_or(false);

        if !exists {
            return Ok(false);
        }
    }

    Ok(true)
}
