mod support;

use beans_lib::database::{EntryFilter, Repository, SQLiteRepository};
use beans_lib::error::BeansResult;
use beans_lib::models::{LedgerEntry, LedgerEntryBuilder, Tag};
use chrono::{DateTime, TimeZone, Utc};
use rust_decimal::Decimal;
use rusqlite::Connection;
use sql_query_builder as sql;
use std::str::FromStr;
use uuid::Uuid;

#[tokio::test]
async fn test_schema_version_table_creation() -> BeansResult<()> {
    // Create an in-memory database
    let conn = Connection::open_in_memory()?;
    
    // Create schema_version table using sql_query_builder
    let create_schema_version_table = sql::CreateTable::new()
        .create_table_if_not_exists("schema_version")
        .column("id INTEGER PRIMARY KEY CHECK (id = 1)")
        .column("version INTEGER NOT NULL")
        .column("updated_at TEXT NOT NULL")
        .as_string();
        
    conn.execute(&create_schema_version_table, [])?;
    
    // Verify the table was created
    let check_table_query = sql::Select::new()
        .select("1")
        .from("sqlite_master")
        .where_clause("type='table'")
        .where_clause("name='schema_version'")
        .as_string();
    
    let table_exists: bool = conn
        .query_row(&check_table_query, [], |_| Ok(true))
        .unwrap_or(false);
    
    assert!(table_exists, "schema_version table should exist");
    
    Ok(())
}

#[tokio::test]
async fn test_insert_or_replace_into() -> BeansResult<()> {
    // Create an in-memory database
    let conn = Connection::open_in_memory()?;
    
    // Create schema_version table
    let create_schema_version_table = sql::CreateTable::new()
        .create_table_if_not_exists("schema_version")
        .column("id INTEGER PRIMARY KEY CHECK (id = 1)")
        .column("version INTEGER NOT NULL")
        .column("updated_at TEXT NOT NULL")
        .as_string();
        
    conn.execute(&create_schema_version_table, [])?;
    
    // Insert a record using insert_or_replace_into
    let now = chrono::Utc::now().to_rfc3339();
    let insert_query = sql::Insert::new()
        .insert_or_replace_into("schema_version (id, version, updated_at)")
        .values("(1, ?, ?)")
        .as_string();
    
    conn.execute(&insert_query, rusqlite::params![42, &now])?;
    
    // Verify the record was inserted
    let select_query = sql::Select::new()
        .select("version")
        .from("schema_version")
        .where_clause("id = 1")
        .as_string();
    
    let version: i64 = conn.query_row(&select_query, [], |row| row.get(0))?;
    
    assert_eq!(version, 42, "Version should be 42");
    
    // Replace the record
    let new_now = chrono::Utc::now().to_rfc3339();
    conn.execute(&insert_query, rusqlite::params![99, &new_now])?;
    
    // Verify the record was replaced
    let new_version: i64 = conn.query_row(&select_query, [], |row| row.get(0))?;
    
    assert_eq!(new_version, 99, "Version should be updated to 99");
    
    Ok(())
}

#[tokio::test]
async fn test_pragma_foreign_keys() -> BeansResult<()> {
    // Create an in-memory database
    let conn = Connection::open_in_memory()?;
    
    // Enable foreign keys using sql_query_builder
    let pragma_query = sql::Pragma::new()
        .pragma("foreign_keys = ON")
        .as_string();
        
    conn.execute(&pragma_query, [])?;
    
    // Verify foreign keys are enabled
    let check_pragma_query = sql::Pragma::new()
        .pragma("foreign_keys")
        .as_string();
    
    let foreign_keys_enabled: i64 = conn.query_row(&check_pragma_query, [], |row| row.get(0))?;
    
    assert_eq!(foreign_keys_enabled, 1, "Foreign keys should be enabled");
    
    Ok(())
}

#[tokio::test]
async fn test_repository_operations_with_query_builder() -> BeansResult<()> {
    // Create a test repository
    let repo = support::create_test_repository()?;
    
    // Create a test entry
    let id = Uuid::new_v4();
    let date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let entry = LedgerEntryBuilder::new()
        .id(id)
        .date(date)
        .name("Test Entry")
        .currency(support::usd())
        .amount(Decimal::from_str("100.00")?)
        .entry_type(beans_lib::models::EntryType::Income)
        .tag(Tag::new("test")?)
        .build()?;
    
    // Save the entry
    repo.create(&entry)?;
    
    // Retrieve the entry
    let retrieved = repo.get(id)?;
    
    // Verify the entry was saved and retrieved correctly
    assert_eq!(retrieved.id(), id);
    assert_eq!(retrieved.name(), "Test Entry");
    assert_eq!(retrieved.amount(), &Decimal::from_str("100.00")?);
    
    // Update the entry
    let updated_entry = LedgerEntryBuilder::new()
        .id(id)
        .date(date)
        .name("Updated Entry")
        .currency(support::usd())
        .amount(Decimal::from_str("200.00")?)
        .entry_type(beans_lib::models::EntryType::Income)
        .tag(Tag::new("test")?)
        .tag(Tag::new("updated")?)
        .build()?;
    
    repo.update(&updated_entry)?;
    
    // Retrieve the updated entry
    let retrieved = repo.get(id)?;
    
    // Verify the entry was updated correctly
    assert_eq!(retrieved.name(), "Updated Entry");
    assert_eq!(retrieved.amount(), &Decimal::from_str("200.00")?);
    assert_eq!(retrieved.tags().len(), 2);
    
    // Test filtering
    let filter = EntryFilter::new()
        .with_tag("updated");
    
    let filtered = repo.list(&filter)?;
    
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].id(), id);
    
    // Delete the entry
    repo.delete(id)?;
    
    // Verify the entry was deleted
    let empty_filter = EntryFilter::new();
    let all_entries = repo.list(&empty_filter)?;
    
    assert_eq!(all_entries.len(), 0);
    
    Ok(())
}

