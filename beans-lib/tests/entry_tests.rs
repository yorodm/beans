mod support;
use beans_lib::models::{EntryType, LedgerEntryBuilder, Tag};
use chrono::{DateTime, Utc};
use rust_decimal::prelude::dec;
use std::str::FromStr;
use support::*;
use uuid::Uuid;

#[test]
fn test_entry_type_from_str() {
    assert_eq!(EntryType::from_str("income").unwrap(), EntryType::Income);
    assert_eq!(EntryType::from_str("INCOME").unwrap(), EntryType::Income);
    assert_eq!(EntryType::from_str(" income ").unwrap(), EntryType::Income);

    assert_eq!(EntryType::from_str("expense").unwrap(), EntryType::Expense);
    assert_eq!(EntryType::from_str("EXPENSE").unwrap(), EntryType::Expense);

    assert!(EntryType::from_str("invalid").is_err());
    assert!(EntryType::from_str("").is_err());
}

#[test]
fn test_entry_type_display() {
    assert_eq!(format!("{}", EntryType::Income), "income");
    assert_eq!(format!("{}", EntryType::Expense), "expense");
}

#[test]
fn test_entry_type_all() {
    let all = EntryType::all();
    assert_eq!(all.len(), 2);
    assert!(all.contains(&EntryType::Income));
    assert!(all.contains(&EntryType::Expense));
}

#[test]
fn test_entry_builder_basic() {
    let entry = LedgerEntryBuilder::new()
        .name("Groceries")
        .currency_code(usd().to_owned())
        .amount(dec!(42.50))
        .entry_type(EntryType::Expense)
        .build()
        .unwrap();

    assert_eq!(entry.name(), "Groceries");
    // Currency from rusty-money returns the amount as a string
    assert!(entry.currency().unwrap().to_string().contains("$42.50"));
    assert_eq!(entry.amount(), dec!(42.50));
    assert_eq!(entry.entry_type(), EntryType::Expense);
    assert!(entry.description().is_none());
    assert!(entry.tags().is_empty());
}

#[test]
fn test_entry_builder_full() {
    let tag1 = Tag::new("groceries").unwrap();
    let tag2 = Tag::new("food").unwrap();

    let id = Uuid::new_v4();
    let date = Utc::now();

    let entry = LedgerEntryBuilder::new()
        .id(id)
        .date(date)
        .name("Weekly Groceries")
        .currency_code(usd().to_owned())
        .amount(dec!(125.40))
        .description("Weekly grocery shopping")
        .tag(tag1.clone())
        .tag(tag2.clone())
        .entry_type(EntryType::Expense)
        .build()
        .unwrap();

    assert_eq!(entry.id(), id);
    assert_eq!(entry.date(), date);
    assert_eq!(entry.name(), "Weekly Groceries");
    // Currency from rusty-money returns the amount as a string
    assert!(entry.currency().unwrap().to_string().contains("$125.40"));
    assert_eq!(entry.amount(), dec!(125.40));
    assert_eq!(entry.description(), Some("Weekly grocery shopping"));
    assert_eq!(entry.tags().len(), 2);
    assert!(entry.tags().contains(&tag1));
    assert!(entry.tags().contains(&tag2));
    assert_eq!(entry.entry_type(), EntryType::Expense);
}

#[test]
fn test_entry_builder_validation() {
    // Missing name
    let result = LedgerEntryBuilder::new()
        .currency_code(usd().to_owned())
        .amount(dec!(42.50))
        .entry_type(EntryType::Expense)
        .build();
    assert!(result.is_err());

    // Empty name
    let result = LedgerEntryBuilder::new()
        .name("")
        .currency_code(usd().to_owned())
        .amount(dec!(42.50))
        .entry_type(EntryType::Expense)
        .build();
    assert!(result.is_err());

    // Missing currency
    let result = LedgerEntryBuilder::new()
        .name("Groceries")
        .amount(dec!(42.50))
        .entry_type(EntryType::Expense)
        .build();
    assert!(result.is_err());

    // Missing amount
    let result = LedgerEntryBuilder::new()
        .name("Groceries")
        .currency_code(usd().to_owned())
        .entry_type(EntryType::Expense)
        .build();
    assert!(result.is_err());

    // Zero amount
    let result = LedgerEntryBuilder::new()
        .name("Groceries")
        .currency_code(usd().to_owned())
        .amount(dec!(0))
        .entry_type(EntryType::Expense)
        .build();
    assert!(result.is_err());

    // Negative amount
    let result = LedgerEntryBuilder::new()
        .name("Groceries")
        .currency_code(usd().to_owned())
        .amount(dec!(-42.50))
        .entry_type(EntryType::Expense)
        .build();
    assert!(result.is_err());

    // Missing entry type
    let result = LedgerEntryBuilder::new()
        .name("Groceries")
        .currency_code(usd().to_owned())
        .amount(dec!(42.50))
        .build();
    assert!(result.is_err());
}

#[test]
fn test_entry_builder_from_entry() {
    let tag = Tag::new("groceries").unwrap();

    let original = LedgerEntryBuilder::new()
        .name("Groceries")
        .currency_code(usd().to_owned())
        .amount(dec!(42.50))
        .tag(tag.clone())
        .entry_type(EntryType::Expense)
        .build()
        .unwrap();

    // Create a modified copy
    let modified = LedgerEntryBuilder::from_entry(&original)
        .name("Updated Groceries")
        .amount(dec!(50.00))
        .build()
        .unwrap();

    // Check that modified values changed
    assert_eq!(modified.name(), "Updated Groceries");
    assert_eq!(modified.amount(), dec!(50.00));

    // Check that unmodified values stayed the same
    assert_eq!(modified.id(), original.id());
    // Currency from rusty-money returns the amount as a string
    assert!(modified.currency().unwrap().to_string().contains("$50.00"));
    assert_eq!(modified.entry_type(), EntryType::Expense);
    assert!(modified.tags().contains(&tag));
}

#[test]
fn test_entry_tags_methods() {
    let tag1 = Tag::new("groceries").unwrap();
    let tag2 = Tag::new("food").unwrap();

    let entry = LedgerEntryBuilder::new()
        .name("Groceries")
        .currency_code(usd().to_owned())
        .amount(dec!(42.50))
        .tag(tag1)
        .tag(tag2)
        .entry_type(EntryType::Expense)
        .build()
        .unwrap();

    assert!(entry.has_tag("groceries"));
    assert!(entry.has_tag("GROCERIES")); // Case insensitive
    assert!(entry.has_tag("food"));
    assert!(!entry.has_tag("household"));

    assert!(entry.has_all_tags(&["groceries", "food"]));
    assert!(!entry.has_all_tags(&["groceries", "household"]));

    assert!(entry.has_any_tag(&["household", "groceries"]));
    assert!(!entry.has_any_tag(&["household", "electronics"]));
}

#[test]
fn test_entry_summary_and_display() {
    let tag1 = Tag::new("groceries").unwrap();
    let tag2 = Tag::new("food").unwrap();

    let date = DateTime::parse_from_rfc3339("2023-01-15T12:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    let entry = LedgerEntryBuilder::new()
        .date(date)
        .name("Weekly Groceries")
        .currency_code(usd().to_owned())
        .amount(dec!(125.40))
        .tag(tag1)
        .tag(tag2)
        .entry_type(EntryType::Expense)
        .build()
        .unwrap();

    // The format has changed with rusty-money
    let summary = entry.summary();
    assert!(summary.contains("2023-01-15"));
    assert!(summary.contains("Weekly Groceries"));
    assert!(summary.contains("125.40"));
    assert!(summary.contains("food"));
    assert!(summary.contains("groceries"));

    // Test without tags
    let entry_no_tags = LedgerEntryBuilder::new()
        .date(date)
        .name("Weekly Groceries")
        .currency_code(usd().to_owned())
        .amount(dec!(125.40))
        .entry_type(EntryType::Expense)
        .build()
        .unwrap();

    let summary = entry_no_tags.summary();
    assert!(summary.contains("2023-01-15"));
    assert!(summary.contains("Weekly Groceries"));
    assert!(summary.contains("125.40"));
    assert!(!summary.contains("food"));
    assert!(!summary.contains("groceries"));
}

#[test]
fn test_entry_with_updated_at() {
    let entry = LedgerEntryBuilder::new()
        .name("Groceries")
        .currency_code(usd().to_owned())
        .amount(dec!(42.50))
        .entry_type(EntryType::Expense)
        .build()
        .unwrap();

    let original_updated_at = entry.updated_at();

    // Wait a moment to ensure different timestamp
    std::thread::sleep(std::time::Duration::from_millis(10));

    let new_time = Utc::now();
    let updated = entry.with_updated_at(new_time);

    // Check that updated_at changed
    assert_ne!(updated.updated_at(), original_updated_at);
    assert_eq!(updated.updated_at(), new_time);

    // Check that other fields stayed the same
    assert_eq!(updated.id(), entry.id());
    assert_eq!(updated.name(), "Groceries");
    assert_eq!(updated.amount(), dec!(42.50));
}

#[test]
fn test_entry_builder_tags_method() {
    let tag1 = Tag::new("groceries").unwrap();
    let tag2 = Tag::new("food").unwrap();
    let tag3 = Tag::new("household").unwrap();

    let tags = vec![tag1.clone(), tag2.clone(), tag3.clone()];

    let entry = LedgerEntryBuilder::new()
        .name("Groceries")
        .currency_code(usd().to_owned())
        .amount(dec!(42.50))
        .tags(tags)
        .entry_type(EntryType::Expense)
        .build()
        .unwrap();

    assert_eq!(entry.tags().len(), 3);
    assert!(entry.has_tag("groceries"));
    assert!(entry.has_tag("food"));
    assert!(entry.has_tag("household"));
}
