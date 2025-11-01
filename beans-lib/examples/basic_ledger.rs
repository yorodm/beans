//! Basic Ledger Example
//!
//! This example demonstrates the fundamental operations of the Beans ledger library:
//! - Creating a new ledger file
//! - Adding entries (income and expenses)
//! - Retrieving entries by ID
//! - Updating entries
//! - Deleting entries
//!
//! Run with: `cargo run --example basic_ledger`

use beans_lib::prelude::*;
use chrono::Utc;
use rust_decimal_macros::dec;

fn main() -> BeansResult<()> {
    println!("=== Beans Ledger: Basic Operations Example ===\n");

    // Create a new in-memory ledger for this example
    // In a real application, you would use LedgerManager::open("path/to/file.bean")
    let ledger = LedgerManager::in_memory()?;
    println!("✓ Created in-memory ledger\n");

    // === ADDING ENTRIES ===
    println!("--- Adding Entries ---");

    // Create an income entry using the builder pattern
    let salary = LedgerEntryBuilder::new()
        .name("Monthly Salary")
        .amount(dec!(5000.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Income)
        .description("January salary payment")
        .add_tag("salary")?
        .add_tag("monthly")?
        .build()?;

    let salary_id = ledger.add_entry(&salary)?;
    println!("✓ Added income entry: {} (ID: {})", salary.name(), salary_id);

    // Create an expense entry
    let rent = LedgerEntryBuilder::new()
        .name("Apartment Rent")
        .amount(dec!(1200.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .description("January rent payment")
        .add_tag("housing")?
        .add_tag("monthly")?
        .add_tag("fixed")?
        .build()?;

    let rent_id = ledger.add_entry(&rent)?;
    println!("✓ Added expense entry: {} (ID: {})", rent.name(), rent_id);

    // Add more expenses
    let groceries = LedgerEntryBuilder::new()
        .name("Weekly Groceries")
        .amount(dec!(150.75))
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .description("Supermarket shopping")
        .add_tag("food")?
        .add_tag("groceries")?
        .build()?;

    let groceries_id = ledger.add_entry(&groceries)?;
    println!("✓ Added expense entry: {} (ID: {})\n", groceries.name(), groceries_id);

    // === RETRIEVING ENTRIES ===
    println!("--- Retrieving Entries ---");

    let retrieved_salary = ledger.get_entry(salary_id)?;
    println!("Retrieved: {}", retrieved_salary.name());
    println!("  Amount: {} {}", retrieved_salary.amount(), retrieved_salary.currency_code());
    println!("  Type: {}", retrieved_salary.entry_type());
    println!("  Tags: {:?}\n", retrieved_salary.tags().iter().map(|t| t.name()).collect::<Vec<_>>());

    // === UPDATING ENTRIES ===
    println!("--- Updating Entries ---");

    // Update the groceries entry with a new amount and description
    let updated_groceries = groceries
        .with_amount(dec!(175.50))
        .with_description("Supermarket shopping + extra items");

    ledger.update_entry(&updated_groceries)?;
    println!("✓ Updated groceries amount to {}\n", updated_groceries.amount());

    // Verify the update
    let verified = ledger.get_entry(groceries_id)?;
    println!("Verified update:");
    println!("  New amount: {}", verified.amount());
    println!("  New description: {}\n", verified.description().unwrap_or(""));

    // === LISTING ALL ENTRIES ===
    println!("--- Listing All Entries ---");

    let all_entries = ledger.list_entries(&EntryFilter::default())?;
    println!("Total entries in ledger: {}", all_entries.len());
    
    for entry in &all_entries {
        let entry_type_symbol = match entry.entry_type() {
            EntryType::Income => "+",
            EntryType::Expense => "-",
        };
        println!("  {} {} {} - {} {}", 
            entry_type_symbol,
            entry.name(),
            entry.amount(),
            entry.currency_code(),
            entry.date().format("%Y-%m-%d")
        );
    }
    println!();

    // === DELETING ENTRIES ===
    println!("--- Deleting Entries ---");

    ledger.delete_entry(groceries_id)?;
    println!("✓ Deleted groceries entry\n");

    // Verify deletion
    let remaining_entries = ledger.list_entries(&EntryFilter::default())?;
    println!("Remaining entries: {}", remaining_entries.len());
    for entry in &remaining_entries {
        println!("  - {}", entry.name());
    }

    println!("\n=== Example Complete ===");
    Ok(())
}

