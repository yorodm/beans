//! Filtering Entries Example
//!
//! This example demonstrates how to query and filter ledger entries using various criteria:
//! - Filtering by date range
//! - Filtering by entry type (income vs expense)
//! - Filtering by tags
//! - Filtering by currency
//! - Combining multiple filters
//!
//! Run with: `cargo run --example filtering_entries`

use beans_lib::prelude::*;
use chrono::{Duration, Utc};
use rust_decimal_macros::dec;

fn main() -> BeansResult<()> {
    println!("=== Beans Ledger: Filtering Entries Example ===\n");

    // Create an in-memory ledger and populate it with sample data
    let ledger = setup_sample_ledger()?;
    
    // === FILTER BY ENTRY TYPE ===
    println!("--- Filter by Entry Type ---");
    
    // Get only income entries
    let income_filter = EntryFilter {
        entry_type: Some(EntryType::Income),
        ..Default::default()
    };
    
    let income_entries = ledger.list_entries(&income_filter)?;
    println!("Income entries: {}", income_entries.len());
    for entry in &income_entries {
        println!("  + {} - {} {}", 
            entry.name(), 
            entry.amount(), 
            entry.currency_code()
        );
    }
    println!();
    
    // Get only expense entries
    let expense_filter = EntryFilter {
        entry_type: Some(EntryType::Expense),
        ..Default::default()
    };
    
    let expense_entries = ledger.list_entries(&expense_filter)?;
    println!("Expense entries: {}", expense_entries.len());
    for entry in &expense_entries {
        println!("  - {} - {} {}", 
            entry.name(), 
            entry.amount(), 
            entry.currency_code()
        );
    }
    println!();
    
    // === FILTER BY DATE RANGE ===
    println!("--- Filter by Date Range ---");
    
    let now = Utc::now();
    let start_date = now - Duration::days(30);
    let end_date = now;
    
    let date_filter = EntryFilter {
        start_date: Some(start_date),
        end_date: Some(end_date),
        ..Default::default()
    };
    
    let recent_entries = ledger.list_entries(&date_filter)?;
    println!("Entries from last 30 days: {}", recent_entries.len());
    for entry in &recent_entries {
        println!("  {} - {} ({})", 
            entry.name(), 
            entry.date().format("%Y-%m-%d"),
            entry.entry_type()
        );
    }
    println!();
    
    // === FILTER BY TAGS ===
    println!("--- Filter by Tags ---");
    
    // Filter entries with "monthly" tag
    let monthly_filter = EntryFilter {
        tags: vec!["monthly".to_string()],
        ..Default::default()
    };
    
    let monthly_entries = ledger.list_entries(&monthly_filter)?;
    println!("Entries with 'monthly' tag: {}", monthly_entries.len());
    for entry in &monthly_entries {
        println!("  {} - {} (tags: {:?})", 
            entry.name(),
            entry.amount(),
            entry.tags().iter().map(|t| t.name()).collect::<Vec<_>>()
        );
    }
    println!();
    
    // Filter entries with "food" tag
    let food_filter = EntryFilter {
        tags: vec!["food".to_string()],
        ..Default::default()
    };
    
    let food_entries = ledger.list_entries(&food_filter)?;
    println!("Entries with 'food' tag: {}", food_entries.len());
    for entry in &food_entries {
        println!("  {} - {} {}", 
            entry.name(),
            entry.amount(),
            entry.currency_code()
        );
    }
    println!();
    
    // === FILTER BY CURRENCY ===
    println!("--- Filter by Currency ---");
    
    let usd_filter = EntryFilter {
        currency_code: Some("USD".to_string()),
        ..Default::default()
    };
    
    let usd_entries = ledger.list_entries(&usd_filter)?;
    println!("USD entries: {}", usd_entries.len());
    for entry in &usd_entries {
        println!("  {} - ${}", entry.name(), entry.amount());
    }
    println!();
    
    // === COMBINING FILTERS ===
    println!("--- Combining Multiple Filters ---");
    
    // Find monthly expenses in USD
    let combined_filter = EntryFilter {
        entry_type: Some(EntryType::Expense),
        currency_code: Some("USD".to_string()),
        tags: vec!["monthly".to_string()],
        ..Default::default()
    };
    
    let filtered_entries = ledger.list_entries(&combined_filter)?;
    println!("Monthly USD expenses: {}", filtered_entries.len());
    for entry in &filtered_entries {
        println!("  {} - ${}", entry.name(), entry.amount());
    }
    println!();
    
    // === PAGINATION ===
    println!("--- Pagination ---");
    
    // Get first page (limit 3)
    let page1_filter = EntryFilter {
        limit: Some(3),
        offset: Some(0),
        ..Default::default()
    };
    
    let page1 = ledger.list_entries(&page1_filter)?;
    println!("Page 1 (3 entries):");
    for entry in &page1 {
        println!("  {}", entry.name());
    }
    println!();
    
    // Get second page
    let page2_filter = EntryFilter {
        limit: Some(3),
        offset: Some(3),
        ..Default::default()
    };
    
    let page2 = ledger.list_entries(&page2_filter)?;
    println!("Page 2 (3 entries):");
    for entry in &page2 {
        println!("  {}", entry.name());
    }
    println!();
    
    // === COUNTING ENTRIES ===
    println!("--- Counting Entries ---");
    
    let total_count = ledger.count_entries(&EntryFilter::default())?;
    let income_count = ledger.count_entries(&income_filter)?;
    let expense_count = ledger.count_entries(&expense_filter)?;
    
    println!("Total entries: {}", total_count);
    println!("Income entries: {}", income_count);
    println!("Expense entries: {}", expense_count);
    
    println!("\n=== Example Complete ===");
    Ok(())
}

/// Helper function to set up a ledger with sample data
fn setup_sample_ledger() -> BeansResult<LedgerManager> {
    let ledger = LedgerManager::in_memory()?;
    
    // Add sample income entries
    let salary = LedgerEntryBuilder::new()
        .name("Monthly Salary")
        .amount(dec!(5000.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Income)
        .add_tag("salary")?
        .add_tag("monthly")?
        .build()?;
    ledger.add_entry(&salary)?;
    
    let freelance = LedgerEntryBuilder::new()
        .name("Freelance Project")
        .amount(dec!(1500.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Income)
        .add_tag("freelance")?
        .build()?;
    ledger.add_entry(&freelance)?;
    
    // Add sample expense entries
    let rent = LedgerEntryBuilder::new()
        .name("Apartment Rent")
        .amount(dec!(1200.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .add_tag("housing")?
        .add_tag("monthly")?
        .add_tag("fixed")?
        .build()?;
    ledger.add_entry(&rent)?;
    
    let groceries = LedgerEntryBuilder::new()
        .name("Weekly Groceries")
        .amount(dec!(150.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .add_tag("food")?
        .add_tag("groceries")?
        .build()?;
    ledger.add_entry(&groceries)?;
    
    let utilities = LedgerEntryBuilder::new()
        .name("Utilities")
        .amount(dec!(200.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .add_tag("utilities")?
        .add_tag("monthly")?
        .build()?;
    ledger.add_entry(&utilities)?;
    
    let restaurant = LedgerEntryBuilder::new()
        .name("Dinner Out")
        .amount(dec!(75.50))
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .add_tag("food")?
        .add_tag("dining")?
        .build()?;
    ledger.add_entry(&restaurant)?;
    
    let internet = LedgerEntryBuilder::new()
        .name("Internet Service")
        .amount(dec!(60.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .add_tag("utilities")?
        .add_tag("monthly")?
        .add_tag("fixed")?
        .build()?;
    ledger.add_entry(&internet)?;
    
    Ok(ledger)
}

