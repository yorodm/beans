// Helper functions for returning currency codes
pub fn usd<'a>() -> &'a str {
    rusty_money::iso::USD.iso_alpha_code
}

pub fn bgn<'a>() -> &'a str {
    rusty_money::iso::BGN.iso_alpha_code
}

pub fn eur<'a>() -> &'a str {
    rusty_money::iso::EUR.iso_alpha_code
}

use beans_lib::database::{initialize_schema, SQLiteRepository};
use beans_lib::error::BeansResult;
use rusqlite::Connection;

/// Creates an in-memory SQLite repository with initialized schema.
pub fn create_test_repository() -> BeansResult<SQLiteRepository> {
    let repo = SQLiteRepository::in_memory()?;

    // Initialize schema
    let conn = repo.conn.lock().unwrap();
    initialize_schema(&conn)?;
    drop(conn);

    Ok(repo)
}

/// Validates that a file path has a .bean extension
pub fn validate_bean_extension(path: &std::path::Path) -> BeansResult<()> {
    use beans_lib::error::BeansError;

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

    Ok(())
}
