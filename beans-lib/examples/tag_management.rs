//! Tag Management Example
//!
//! This example demonstrates working with tags for entry categorization:
//! - Creating and validating tags
//! - Adding tags to entries
//! - Removing tags from entries
//! - Querying entries by tags
//! - Tag normalization and validation
//!
//! Run with: `cargo run --example tag_management`

use beans_lib::prelude::*;
use rust_decimal_macros::dec;

fn main() -> BeansResult<()> {
    println!("=== Beans Ledger: Tag Management Example ===\n");

    let ledger = LedgerManager::in_memory()?;
    println!("✓ Created in-memory ledger\n");

    // === CREATING TAGS ===
    println!("--- Creating Tags ---");
    
    // Tags are automatically normalized (lowercase, trimmed)
    let tag1 = Tag::new("Food")?;
    let tag2 = Tag::new("  HOUSING  ")?;
    let tag3 = Tag::new("monthly-bills")?;
    
    println!("Created tags:");
    println!("  Input: 'Food' -> Normalized: '{}'", tag1.name());
    println!("  Input: '  HOUSING  ' -> Normalized: '{}'", tag2.name());
    println!("  Input: 'monthly-bills' -> Normalized: '{}'", tag3.name());
    println!();

    // === TAG VALIDATION ===
    println!("--- Tag Validation ---");
    
    // Valid tags
    let valid_tags = vec!["groceries", "rent", "salary", "utilities", "food-dining"];
    println!("Valid tags:");
    for tag_str in valid_tags {
        match Tag::new(tag_str) {
            Ok(tag) => println!("  ✓ '{}'", tag.name()),
            Err(e) => println!("  ✗ '{}' - {}", tag_str, e),
        }
    }
    println!();
    
    // Invalid tags (empty, too long, invalid characters)
    let invalid_tags = vec!["", "   ", "tag with spaces", "tag!@#$%"];
    println!("Invalid tags:");
    for tag_str in invalid_tags {
        match Tag::new(tag_str) {
            Ok(tag) => println!("  ✓ '{}'", tag.name()),
            Err(e) => println!("  ✗ '{}' - {}", tag_str, e),
        }
    }
    println!();

    // === ADDING TAGS TO ENTRIES ===
    println!("--- Adding Tags to Entries ---");
    
    // Create an entry with tags using the builder
    let groceries = LedgerEntryBuilder::new()
        .name("Weekly Groceries")
        .amount(dec!(150.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .add_tag("food")?
        .add_tag("groceries")?
        .add_tag("weekly")?
        .build()?;
    
    let groceries_id = ledger.add_entry(&groceries)?;
    println!("✓ Added groceries with tags: {:?}", 
        groceries.tags().iter().map(|t| t.name()).collect::<Vec<_>>()
    );
    
    // Create an entry and add tags later
    let rent = LedgerEntryBuilder::new()
        .name("Monthly Rent")
        .amount(dec!(1200.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .add_tag("housing")?
        .build()?;
    
    let rent_id = ledger.add_entry(&rent)?;
    println!("✓ Added rent with tag: {:?}\n", 
        rent.tags().iter().map(|t| t.name()).collect::<Vec<_>>()
    );

    // === MODIFYING TAGS ===
    println!("--- Modifying Tags ---");
    
    // Get the entry and add more tags
    let mut updated_rent = ledger.get_entry(rent_id)?;
    updated_rent = updated_rent
        .with_tag(Tag::new("monthly")?)?
        .with_tag(Tag::new("fixed")?)?;
    
    ledger.update_entry(&updated_rent)?;
    println!("✓ Added tags to rent entry");
    
    let verified = ledger.get_entry(rent_id)?;
    println!("  Updated tags: {:?}\n", 
        verified.tags().iter().map(|t| t.name()).collect::<Vec<_>>()
    );

    // === REMOVING TAGS ===
    println!("--- Removing Tags ---");
    
    let mut modified = ledger.get_entry(groceries_id)?;
    println!("Before: {:?}", 
        modified.tags().iter().map(|t| t.name()).collect::<Vec<_>>()
    );
    
    // Remove a specific tag
    modified = modified.without_tag(&Tag::new("weekly")?);
    ledger.update_entry(&modified)?;
    
    let verified = ledger.get_entry(groceries_id)?;
    println!("After removing 'weekly': {:?}\n", 
        verified.tags().iter().map(|t| t.name()).collect::<Vec<_>>()
    );

    // === QUERYING BY TAGS ===
    println!("--- Querying Entries by Tags ---");
    
    // Add more entries with various tags
    setup_sample_entries(&ledger)?;
    
    // Find all "food" entries
    let food_filter = EntryFilter {
        tags: vec!["food".to_string()],
        ..Default::default()
    };
    
    let food_entries = ledger.list_entries(&food_filter)?;
    println!("Entries with 'food' tag: {}", food_entries.len());
    for entry in &food_entries {
        println!("  {} - ${} (tags: {:?})", 
            entry.name(),
            entry.amount(),
            entry.tags().iter().map(|t| t.name()).collect::<Vec<_>>()
        );
    }
    println!();
    
    // Find all "monthly" entries
    let monthly_filter = EntryFilter {
        tags: vec!["monthly".to_string()],
        ..Default::default()
    };
    
    let monthly_entries = ledger.list_entries(&monthly_filter)?;
    println!("Entries with 'monthly' tag: {}", monthly_entries.len());
    for entry in &monthly_entries {
        println!("  {} - ${}", entry.name(), entry.amount());
    }
    println!();
    
    // Find entries with multiple tags
    let multi_tag_filter = EntryFilter {
        tags: vec!["food".to_string(), "dining".to_string()],
        ..Default::default()
    };
    
    let multi_entries = ledger.list_entries(&multi_tag_filter)?;
    println!("Entries with 'food' AND 'dining' tags: {}", multi_entries.len());
    for entry in &multi_entries {
        println!("  {}", entry.name());
    }
    println!();

    // === TAG STATISTICS ===
    println!("--- Tag Usage Statistics ---");
    
    // Get all entries and count tag usage
    let all_entries = ledger.list_entries(&EntryFilter::default())?;
    let mut tag_counts = std::collections::HashMap::new();
    
    for entry in &all_entries {
        for tag in entry.tags() {
            *tag_counts.entry(tag.name().to_string()).or_insert(0) += 1;
        }
    }
    
    println!("Tag usage:");
    let mut tags_vec: Vec<_> = tag_counts.iter().collect();
    tags_vec.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending
    
    for (tag, count) in tags_vec {
        println!("  {}: {} entries", tag, count);
    }
    println!();

    // === TAG BEST PRACTICES ===
    println!("--- Tag Best Practices ---");
    println!("✓ Use lowercase tags for consistency");
    println!("✓ Use hyphens for multi-word tags (e.g., 'food-dining')");
    println!("✓ Keep tags short and descriptive");
    println!("✓ Create a tag hierarchy (e.g., 'food', 'food-groceries', 'food-dining')");
    println!("✓ Use tags for different purposes:");
    println!("  - Categories: 'food', 'housing', 'transportation'");
    println!("  - Frequency: 'monthly', 'weekly', 'one-time'");
    println!("  - Type: 'fixed', 'variable'");
    println!("  - Projects: 'project-alpha', 'vacation-2024'");
    println!();

    println!("=== Example Complete ===");
    Ok(())
}

/// Helper function to add sample entries with various tags
fn setup_sample_entries(ledger: &LedgerManager) -> BeansResult<()> {
    let entries = vec![
        ("Utilities", dec!(200.00), vec!["utilities", "monthly", "fixed"]),
        ("Dinner Out", dec!(75.00), vec!["food", "dining"]),
        ("Internet", dec!(60.00), vec!["utilities", "monthly", "fixed"]),
        ("Coffee Shop", dec!(25.00), vec!["food", "dining"]),
        ("Gas", dec!(50.00), vec!["transportation", "variable"]),
        ("Gym Membership", dec!(40.00), vec!["health", "monthly", "fixed"]),
        ("Movie Tickets", dec!(30.00), vec!["entertainment"]),
    ];
    
    for (name, amount, tags) in entries {
        let mut builder = LedgerEntryBuilder::new()
            .name(name)
            .amount(amount)
            .currency(Currency::usd())
            .entry_type(EntryType::Expense);
        
        for tag in tags {
            builder = builder.add_tag(tag)?;
        }
        
        let entry = builder.build()?;
        ledger.add_entry(&entry)?;
    }
    
    Ok(())
}

