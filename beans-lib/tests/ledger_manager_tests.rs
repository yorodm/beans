//! Integration tests for the LedgerManager.
mod support;
use beans_lib::database::EntryFilter;
use beans_lib::error::BeansResult;
use beans_lib::ledger::LedgerManager;
use beans_lib::models::{EntryType, LedgerEntry, LedgerEntryBuilder, Tag};
use rust_decimal_macros::dec;
use support::*;
use tempfile::tempdir;

/// Creates a test entry with the given name and entry type.
fn create_test_entry(name: &str, entry_type: EntryType) -> BeansResult<LedgerEntry> {
    let amount = match entry_type {
        EntryType::Income => dec!(100.00),
        EntryType::Expense => dec!(50.00),
    };

    let mut builder = LedgerEntryBuilder::new()
        .name(name)
        .amount(amount)
        .currency_code(usd().to_owned())
        .entry_type(entry_type);

    // Add tags based on entry type
    if entry_type == EntryType::Income {
        builder = builder.tag(Tag::new("income").unwrap());
        builder = builder.tag(Tag::new("salary").unwrap());
    } else {
        builder = builder.tag(Tag::new("expense").unwrap());
        builder = builder.tag(Tag::new("food").unwrap());
    }

    builder.build()
}

#[test]
fn test_in_memory_ledger() -> BeansResult<()> {
    // Create an in-memory ledger
    let ledger = LedgerManager::in_memory()?;

    // Create a test entry
    let entry = create_test_entry("Test Income", EntryType::Income)?;

    // Add the entry to the ledger
    let id = ledger.add_entry(&entry)?;

    // Retrieve the entry
    let retrieved = ledger.get_entry(id)?;

    // Verify the entry was stored correctly
    assert_eq!(retrieved.name(), entry.name());
    assert_eq!(retrieved.amount(), entry.amount());
    assert_eq!(retrieved.currency_code(), entry.currency_code());
    assert_eq!(retrieved.entry_type(), entry.entry_type());

    Ok(())
}

#[test]
fn test_file_ledger() -> BeansResult<()> {
    // Create a temporary directory for the test
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.bean");

    // Create a file-based ledger
    let ledger = LedgerManager::open(&file_path)?;

    // Create a test entry
    let entry = create_test_entry("Test Expense", EntryType::Expense)?;

    // Add the entry to the ledger
    let id = ledger.add_entry(&entry)?;

    // Retrieve the entry
    let retrieved = ledger.get_entry(id)?;

    // Verify the entry was stored correctly
    assert_eq!(retrieved.name(), entry.name());
    assert_eq!(retrieved.amount(), entry.amount());
    assert_eq!(retrieved.currency_code(), entry.currency_code());
    assert_eq!(retrieved.entry_type(), entry.entry_type());

    // Verify the file was created
    assert!(file_path.exists());

    Ok(())
}

#[test]
fn test_invalid_file_extension() {
    // Try to open a ledger with an invalid extension
    let result = LedgerManager::open("test.txt");

    // Verify the operation failed with the expected error
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(err.to_string().contains("must have .bean extension"));
    }
}

#[test]
fn test_crud_operations() -> BeansResult<()> {
    // Create an in-memory ledger
    let ledger = LedgerManager::in_memory()?;

    // Create a test entry
    let entry = create_test_entry("Original Entry", EntryType::Income)?;

    // Add the entry to the ledger
    let id = ledger.add_entry(&entry)?;

    // Retrieve the entry
    let retrieved = ledger.get_entry(id)?;
    assert_eq!(retrieved.name(), "Original Entry");

    // Update the entry
    let updated_entry = LedgerEntryBuilder::from_entry(&retrieved)
        .name("Updated Entry")
        .build()?;

    ledger.update_entry(&updated_entry)?;

    // Retrieve the updated entry
    let retrieved = ledger.get_entry(id)?;
    assert_eq!(retrieved.name(), "Updated Entry");

    // Delete the entry
    ledger.delete_entry(id)?;

    // Verify the entry was deleted
    let result = ledger.get_entry(id);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_list_and_filter() -> BeansResult<()> {
    // Create an in-memory ledger
    let ledger = LedgerManager::in_memory()?;

    // Create test entries
    let income1 = create_test_entry("Income 1", EntryType::Income)?;
    let income2 = create_test_entry("Income 2", EntryType::Income)?;
    let expense1 = create_test_entry("Expense 1", EntryType::Expense)?;
    let expense2 = create_test_entry("Expense 2", EntryType::Expense)?;

    // Add entries to the ledger
    ledger.add_entry(&income1)?;
    ledger.add_entry(&income2)?;
    ledger.add_entry(&expense1)?;
    ledger.add_entry(&expense2)?;

    // Get all entries
    let all_entries = ledger.get_all_entries()?;
    assert_eq!(all_entries.len(), 4);

    // Filter by entry type
    let filter = EntryFilter {
        entry_type: Some(EntryType::Income),
        ..Default::default()
    };

    let income_entries = ledger.list_entries(&filter)?;
    assert_eq!(income_entries.len(), 2);
    assert!(income_entries
        .iter()
        .all(|e| e.entry_type() == EntryType::Income));

    // Filter by tag
    let filter = EntryFilter {
        tags: vec!["food".to_string()],
        ..Default::default()
    };

    let food_entries = ledger.list_entries(&filter)?;
    assert_eq!(food_entries.len(), 2);
    assert!(food_entries.iter().all(|e| e.has_tag("food")));

    // Count entries
    let count = ledger.count_entries(&filter)?;
    assert_eq!(count, 2);

    Ok(())
}

#[test]
fn test_transaction_atomicity() -> BeansResult<()> {
    // Create an in-memory ledger
    let ledger = LedgerManager::in_memory()?;

    // Create a valid entry and an invalid entry (negative amount)
    let valid_entry = create_test_entry("Valid Entry", EntryType::Income)?;

    let invalid_entry = LedgerEntryBuilder::new()
        .name("Invalid Entry")
        .amount(dec!(-50.00)) // Negative amount, should fail validation
        .currency_code(usd().to_owned())
        .entry_type(EntryType::Expense)
        .build();

    // The invalid entry should fail to build
    assert!(invalid_entry.is_err());

    // Add the valid entry
    let id = ledger.add_entry(&valid_entry)?;

    // Verify the entry was added
    let retrieved = ledger.get_entry(id)?;
    assert_eq!(retrieved.name(), "Valid Entry");

    // Get all entries - should only have the valid one
    let all_entries = ledger.get_all_entries()?;
    assert_eq!(all_entries.len(), 1);

    Ok(())
}
