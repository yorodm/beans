//! Integration tests for the SQLiteRepository.

use beans_lib::database::{EntryFilter, Repository, SQLiteRepository, initialize_schema};
use beans_lib::error::BeansResult;
use beans_lib::models::{Currency, EntryType, LedgerEntry, LedgerEntryBuilder, Tag};
use beans_lib::models::currency::usd_with_amount;
use chrono::{Duration, Utc};
use rust_decimal_macros::dec;
use std::collections::HashSet;
use uuid::Uuid;

/// Creates a test repository with initialized schema.
fn create_test_repository() -> BeansResult<SQLiteRepository> {
    let repo = SQLiteRepository::in_memory()?;
    
    // Initialize schema
    let conn = repo.conn.lock().unwrap();
    initialize_schema(&conn)?;
    drop(conn);
    
    Ok(repo)
}

/// Creates a test entry with the given name and entry type.
fn create_test_entry(name: &str, entry_type: EntryType) -> BeansResult<LedgerEntry<'static>> {
    let amount = match entry_type {
        EntryType::Income => dec!(100.00),
        EntryType::Expense => dec!(50.00),
    };
    
    let mut builder = LedgerEntryBuilder::new()
        .name(name)
        .amount(amount)
        .currency(usd_with_amount(amount))
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
fn test_create_and_get_entry() -> BeansResult<()> {
    let repo = create_test_repository()?;
    
    // Create a test entry
    let entry = create_test_entry("Test Income", EntryType::Income)?;
    repo.create(&entry)?;
    
    // Get the entry
    let retrieved = repo.get(*entry.id())?;
    
    // Verify the entry
    assert_eq!(retrieved.id(), entry.id());
    assert_eq!(retrieved.name(), entry.name());
    assert_eq!(retrieved.amount(), entry.amount());
    assert_eq!(retrieved.currency().code(), entry.currency().code());
    assert_eq!(retrieved.entry_type(), entry.entry_type());
    
    // Verify tags
    let retrieved_tags: HashSet<String> = retrieved.tags().iter()
        .map(|t| t.name().to_string())
        .collect();
    
    let expected_tags: HashSet<String> = entry.tags().iter()
        .map(|t| t.name().to_string())
        .collect();
    
    assert_eq!(retrieved_tags, expected_tags);
    
    Ok(())
}

#[test]
fn test_update_entry() -> BeansResult<()> {
    let repo = create_test_repository()?;
    
    // Create a test entry
    let entry = create_test_entry("Test Income", EntryType::Income)?;
    repo.create(&entry)?;
    
    // Update the entry
    let updated = LedgerEntryBuilder::from(&entry)
        .name("Updated Entry")
        .amount(dec!(200.00))
        .description("Updated description")
        .tag(Tag::new("updated").unwrap())
        .build()?;
    
    repo.update(&updated)?;
    
    // Get the updated entry
    let retrieved = repo.get(*entry.id())?;
    
    // Verify the entry
    assert_eq!(retrieved.name(), "Updated Entry");
    assert_eq!(retrieved.amount(), dec!(200.00));
    assert_eq!(retrieved.description(), Some("Updated description".to_string()));
    
    // Verify tags - should only have the new tag
    let retrieved_tags: HashSet<String> = retrieved.tags().iter()
        .map(|t| t.name().to_string())
        .collect();
    
    let expected_tags: HashSet<String> = vec!["updated".to_string()].into_iter().collect();
    
    assert_eq!(retrieved_tags, expected_tags);
    
    Ok(())
}

#[test]
fn test_delete_entry() -> BeansResult<()> {
    let repo = create_test_repository()?;
    
    // Create a test entry
    let entry = create_test_entry("Test Income", EntryType::Income)?;
    repo.create(&entry)?;
    
    // Delete the entry
    repo.delete(*entry.id())?;
    
    // Try to get the entry - should fail
    let result = repo.get(*entry.id());
    assert!(result.is_err());
    
    Ok(())
}

#[test]
fn test_list_entries() -> BeansResult<()> {
    let repo = create_test_repository()?;
    
    // Create test entries
    let income_entry = create_test_entry("Income Entry", EntryType::Income)?;
    let expense_entry = create_test_entry("Expense Entry", EntryType::Expense)?;
    
    repo.create(&income_entry)?;
    repo.create(&expense_entry)?;
    
    // List all entries
    let all_entries = repo.list(&EntryFilter::default())?;
    assert_eq!(all_entries.len(), 2);
    
    // Filter by entry type
    let income_filter = EntryFilter {
        entry_type: Some(EntryType::Income),
        ..Default::default()
    };
    
    let income_entries = repo.list(&income_filter)?;
    assert_eq!(income_entries.len(), 1);
    assert_eq!(income_entries[0].name(), "Income Entry");
    
    // Filter by tag
    let tag_filter = EntryFilter {
        tags: vec!["food".to_string()],
        ..Default::default()
    };
    
    let tagged_entries = repo.list(&tag_filter)?;
    assert_eq!(tagged_entries.len(), 1);
    assert_eq!(tagged_entries[0].name(), "Expense Entry");
    
    Ok(())
}

#[test]
fn test_date_filtering() -> BeansResult<()> {
    let repo = create_test_repository()?;
    
    // Create entries with different dates
    let now = Utc::now();
    let yesterday = now - Duration::days(1);
    let tomorrow = now + Duration::days(1);
    
    let entry1 = LedgerEntryBuilder::new()
        .name("Yesterday Entry")
        .amount(dec!(100.00))
        .currency(usd_with_amount(dec!(100.00)))
        .entry_type(EntryType::Income)
        .date(yesterday)
        .build()?;
    
    let entry2 = LedgerEntryBuilder::new()
        .name("Tomorrow Entry")
        .amount(dec!(200.00))
        .currency(usd_with_amount(dec!(200.00)))
        .entry_type(EntryType::Income)
        .date(tomorrow)
        .build()?;
    
    repo.create(&entry1)?;
    repo.create(&entry2)?;
    
    // Filter by date range
    let date_filter = EntryFilter {
        start_date: Some(now),
        end_date: None,
        ..Default::default()
    };
    
    let future_entries = repo.list(&date_filter)?;
    assert_eq!(future_entries.len(), 1);
    assert_eq!(future_entries[0].name(), "Tomorrow Entry");
    
    // Filter by date range (both start and end)
    let date_range_filter = EntryFilter {
        start_date: Some(yesterday - Duration::hours(1)),
        end_date: Some(yesterday + Duration::hours(1)),
        ..Default::default()
    };
    
    let yesterday_entries = repo.list(&date_range_filter)?;
    assert_eq!(yesterday_entries.len(), 1);
    assert_eq!(yesterday_entries[0].name(), "Yesterday Entry");
    
    Ok(())
}

#[test]
fn test_multiple_tag_filtering() -> BeansResult<()> {
    let repo = create_test_repository()?;
    
    // Create an entry with multiple tags
    let entry = LedgerEntryBuilder::new()
        .name("Multi-tag Entry")
        .amount(dec!(100.00))
        .currency(usd_with_amount(dec!(100.00)))
        .entry_type(EntryType::Income)
        .tag(Tag::new("tag1").unwrap())
        .tag(Tag::new("tag2").unwrap())
        .tag(Tag::new("tag3").unwrap())
        .build()?;
    
    repo.create(&entry)?;
    
    // Filter by single tag
    let single_tag_filter = EntryFilter {
        tags: vec!["tag1".to_string()],
        ..Default::default()
    };
    
    let single_tag_entries = repo.list(&single_tag_filter)?;
    assert_eq!(single_tag_entries.len(), 1);
    
    // Filter by multiple tags (AND logic)
    let multi_tag_filter = EntryFilter {
        tags: vec!["tag1".to_string(), "tag2".to_string()],
        ..Default::default()
    };
    
    let multi_tag_entries = repo.list(&multi_tag_filter)?;
    assert_eq!(multi_tag_entries.len(), 1);
    
    // Filter by non-existent tag
    let non_existent_tag_filter = EntryFilter {
        tags: vec!["non-existent".to_string()],
        ..Default::default()
    };
    
    let non_existent_tag_entries = repo.list(&non_existent_tag_filter)?;
    assert_eq!(non_existent_tag_entries.len(), 0);
    
    Ok(())
}

#[test]
fn test_pagination() -> BeansResult<()> {
    let repo = create_test_repository()?;
    
    // Create multiple entries
    for i in 1..=10 {
        let entry = LedgerEntryBuilder::new()
            .name(&format!("Entry {}", i))
            .amount(dec!(100.00))
            .currency(usd_with_amount(dec!(100.00)))
            .entry_type(EntryType::Income)
            .build()?;
        
        repo.create(&entry)?;
    }
    
    // Test limit
    let limit_filter = EntryFilter {
        limit: Some(5),
        ..Default::default()
    };
    
    let limited_entries = repo.list(&limit_filter)?;
    assert_eq!(limited_entries.len(), 5);
    
    // Test offset
    let offset_filter = EntryFilter {
        offset: Some(5),
        ..Default::default()
    };
    
    let offset_entries = repo.list(&offset_filter)?;
    assert_eq!(offset_entries.len(), 5);
    
    // Test limit and offset
    let paginated_filter = EntryFilter {
        limit: Some(3),
        offset: Some(3),
        ..Default::default()
    };
    
    let paginated_entries = repo.list(&paginated_filter)?;
    assert_eq!(paginated_entries.len(), 3);
    
    Ok(())
}

#[test]
fn test_count() -> BeansResult<()> {
    let repo = create_test_repository()?;
    
    // Create entries
    for i in 1..=5 {
        let entry_type = if i % 2 == 0 { EntryType::Income } else { EntryType::Expense };
        let entry = create_test_entry(&format!("Entry {}", i), entry_type)?;
        repo.create(&entry)?;
    }
    
    // Count all entries
    let all_count = repo.count(&EntryFilter::default())?;
    assert_eq!(all_count, 5);
    
    // Count by entry type
    let income_filter = EntryFilter {
        entry_type: Some(EntryType::Income),
        ..Default::default()
    };
    
    let income_count = repo.count(&income_filter)?;
    assert_eq!(income_count, 2);
    
    let expense_filter = EntryFilter {
        entry_type: Some(EntryType::Expense),
        ..Default::default()
    };
    
    let expense_count = repo.count(&expense_filter)?;
    assert_eq!(expense_count, 3);
    
    Ok(())
}

#[test]
fn test_non_existent_entry() -> BeansResult<()> {
    let repo = create_test_repository()?;
    
    // Try to get a non-existent entry
    let non_existent_id = Uuid::new_v4();
    let result = repo.get(non_existent_id);
    
    assert!(result.is_err());
    
    // Try to update a non-existent entry
    let non_existent_entry = LedgerEntryBuilder::new()
        .id(non_existent_id)
        .name("Non-existent Entry")
        .amount(dec!(100.00))
        .currency(usd_with_amount(dec!(100.00)))
        .entry_type(EntryType::Income)
        .build()?;
    
    let update_result = repo.update(&non_existent_entry);
    assert!(update_result.is_err());
    
    // Try to delete a non-existent entry
    let delete_result = repo.delete(non_existent_id);
    assert!(delete_result.is_err());
    
    Ok(())
}

