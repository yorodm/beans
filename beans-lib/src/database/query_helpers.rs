//! Helper utilities for building SQL queries with sql_query_builder.

use crate::error::{BeansError, BeansResult};
use rusqlite::{params_from_iter, Connection, ToSql, Transaction};
use sql_query_builder as sql;

/// Executes a query string with parameters and returns the number of affected rows.
///
/// This is a helper to bridge sql_query_builder output with rusqlite execution.
pub fn execute_query(
    conn: &Connection,
    query: &str,
    params: &[&dyn ToSql],
) -> BeansResult<usize> {
    conn.execute(query, params)
        .map_err(|e| BeansError::database(format!("Failed to execute query: {}", e)))
}

/// Executes a query within a transaction.
pub fn execute_query_tx(
    tx: &Transaction,
    query: &str,
    params: &[&dyn ToSql],
) -> BeansResult<usize> {
    tx.execute(query, params)
        .map_err(|e| BeansError::database(format!("Failed to execute query: {}", e)))
}

/// Helper to build INSERT query with sql_query_builder
pub fn build_insert(table: &str, columns: &[&str]) -> sql::Insert {
    let mut insert = sql::Insert::new().insert_into(table);
    
    for column in columns {
        insert = insert.columns(column);
    }
    
    insert
}

/// Helper to build UPDATE query with sql_query_builder
pub fn build_update(table: &str) -> sql::Update {
    sql::Update::new().update(table)
}

/// Helper to build DELETE query with sql_query_builder
pub fn build_delete(table: &str) -> sql::Delete {
    sql::Delete::new().delete_from(table)
}

/// Helper to build SELECT query with sql_query_builder
pub fn build_select(columns: &str, table: &str) -> sql::Select {
    sql::Select::new()
        .select(columns)
        .from(table)
}

/// Creates parameter placeholders for a given count (e.g., "?, ?, ?")
pub fn create_placeholders(count: usize) -> String {
    vec!["?"; count].join(", ")
}

/// Converts a vector of boxed ToSql values to references for rusqlite
pub fn params_to_refs(params: &[Box<dyn ToSql>]) -> Vec<&dyn ToSql> {
    params
        .iter()
        .map(|p| p.as_ref() as &dyn ToSql)
        .collect()
}

