# Getting Started with Beans

Welcome to Beans, a modern ledger library for tracking your income and expenses built in Rust! This guide will help you get up and running quickly.

## Installation

Add Beans to your `Cargo.toml`:

```toml
[dependencies]
beans-lib = "0.1.0"
tokio = { version = "1.0", features = ["full"] }  # Required for async features
```

## Your First Ledger

Let's create a simple program that demonstrates the core features of Beans:

```rust
use beans_lib::prelude::*;
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> BeansResult<()> {
    // Create a new ledger file
    let ledger = LedgerManager::open("./my_ledger.bean")?;
    
    // Add an income entry
    let salary = LedgerEntryBuilder::new()
        .name("Monthly Salary")
        .amount(dec!(5000.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Income)
        .add_tag("salary")?
        .add_tag("monthly")?
        .build()?;
    
    ledger.add_entry(&salary)?;
    
    // Add an expense entry
    let rent = LedgerEntryBuilder::new()
        .name("Rent")
        .amount(dec!(1200.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .add_tag("housing")?
        .add_tag("monthly")?
        .build()?;
    
    ledger.add_entry(&rent)?;
    
    // List all entries
    let entries = ledger.list_entries(&EntryFilter::default())?;
    
    for entry in entries {
        println!("{} - {} - {} {}", 
            entry.date().format("%Y-%m-%d"),
            entry.name(),
            entry.amount(),
            entry.currency_code()
        );
    }
    
    Ok(())
}
```

## Core Concepts

### 1. Ledger Manager

The `LedgerManager` is your main entry point. It manages the ledger file and provides high-level operations:

```rust
// Open or create a ledger file
let ledger = LedgerManager::open("./finances.bean")?;

// For testing, use an in-memory ledger
let ledger = LedgerManager::in_memory()?;
```

**Important**: Ledger files must have the `.bean` extension.

### 2. Ledger Entries

Each entry represents a financial transaction and includes:

- **Date**: When the transaction occurred
- **Name**: A descriptive title
- **Amount**: The transaction amount (using `Decimal` for precision)
- **Currency**: ISO 4217 currency code (e.g., USD, EUR, GBP)
- **Entry Type**: `Income` or `Expense`
- **Description**: Optional detailed description
- **Tags**: For categorization and filtering

### 3. Builder Pattern

Create entries using the builder pattern:

```rust
let entry = LedgerEntryBuilder::new()
    .name("Groceries")
    .amount(dec!(150.00))
    .currency(Currency::usd())
    .entry_type(EntryType::Expense)
    .description("Weekly shopping")
    .add_tag("food")?
    .add_tag("groceries")?
    .build()?;
```

### 4. Tags

Tags help you categorize and filter entries:

```rust
// Add tags when building
builder.add_tag("food")?
       .add_tag("groceries")?

// Filter by tags
let filter = EntryFilter {
    tags: vec!["food".to_string()],
    ..Default::default()
};

let food_entries = ledger.list_entries(&filter)?;
```

Tags are automatically normalized (lowercase, trimmed) and validated.

### 5. Filtering

Query entries using various criteria:

```rust
use chrono::{Duration, Utc};

let now = Utc::now();
let last_month = now - Duration::days(30);

let filter = EntryFilter {
    start_date: Some(last_month),
    end_date: Some(now),
    entry_type: Some(EntryType::Expense),
    tags: vec!["monthly".to_string()],
    currency_code: Some("USD".to_string()),
    ..Default::default()
};

let filtered = ledger.list_entries(&filter)?;
```

### 6. Currency Support

Beans supports multiple currencies with built-in conversion:

```rust
use std::time::Duration as StdDuration;

// Create a converter with 1-hour cache
let converter = CurrencyConverter::new(StdDuration::from_secs(3600));

// Convert amounts
let eur_amount = converter
    .convert(dec!(100.00), &Currency::usd(), &Currency::eur())
    .await?;
```

### 7. Reports and Analytics

Generate reports for your financial data:

```rust
use chrono::Utc;

let now = Utc::now();
let start = now - Duration::days(30);

// Create a report generator
let generator = ReportGenerator::new(&ledger);

// Generate monthly income/expense report
let report = generator
    .income_expense_report(
        start,
        now,
        TimePeriod::Monthly,
        None,  // No currency conversion
        None,  // No tag filter
    )
    .await?;

println!("Total Income: {}", report.summary.income);
println!("Total Expenses: {}", report.summary.expenses);
println!("Net: {}", report.summary.net);
```

## Error Handling

Beans uses a custom error type `BeansError` that implements `std::error::Error`:

```rust
match ledger.add_entry(&entry) {
    Ok(id) => println!("Entry added with ID: {}", id),
    Err(BeansError::ValidationError(msg)) => {
        eprintln!("Validation failed: {}", msg);
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

Common error types:
- `ValidationError`: Invalid data (e.g., empty name, invalid currency)
- `DatabaseError`: Storage issues
- `NotFound`: Entry doesn't exist
- `CurrencyConversionError`: Exchange rate problems
- `InvalidDateRange`: Invalid date parameters

## Next Steps

Now that you understand the basics, check out these resources:

- **[Personal Budget Tutorial](PERSONAL_BUDGET_TUTORIAL.md)**: Complete workflow for managing personal finances
- **[Multi-Currency Guide](MULTI_CURRENCY_GUIDE.md)**: Working with multiple currencies
- **[Custom Reports Guide](CUSTOM_REPORTS_GUIDE.md)**: Creating custom analytics
- **[Examples](../beans-lib/examples/)**: Runnable example programs
- **[API Reference](API_REFERENCE.md)**: Complete API documentation

## Common Patterns

### Pattern 1: Monthly Budget Tracking

```rust
// Add monthly recurring entries
let monthly_expenses = vec![
    ("Rent", dec!(1200.00)),
    ("Utilities", dec!(200.00)),
    ("Internet", dec!(60.00)),
];

for (name, amount) in monthly_expenses {
    let entry = LedgerEntryBuilder::new()
        .name(name)
        .amount(amount)
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .add_tag("monthly")?
        .add_tag("fixed")?
        .build()?;
    
    ledger.add_entry(&entry)?;
}
```

### Pattern 2: Expense Analysis

```rust
// Get expenses for last month
let expenses = ledger.list_entries(&EntryFilter {
    entry_type: Some(EntryType::Expense),
    start_date: Some(last_month),
    end_date: Some(now),
    ..Default::default()
})?;

// Calculate total
let total: Decimal = expenses.iter().map(|e| e.amount()).sum();
println!("Total expenses: ${:.2}", total);
```

### Pattern 3: Tag-Based Budgeting

```rust
// Track food spending
let food_filter = EntryFilter {
    tags: vec!["food".to_string()],
    entry_type: Some(EntryType::Expense),
    ..Default::default()
};

let food_expenses = ledger.list_entries(&food_filter)?;
let food_total: Decimal = food_expenses.iter().map(|e| e.amount()).sum();

println!("Food budget spent: ${:.2}", food_total);
```

## Tips

1. **Use Tags Consistently**: Establish a tagging system early (e.g., category tags, frequency tags)
2. **Regular Entries**: Add entries regularly to keep your ledger up-to-date
3. **Backup**: Regularly backup your `.bean` files
4. **Currency Precision**: Use `rust_decimal::Decimal` for all currency amounts to avoid floating-point errors
5. **Async Operations**: Currency conversion and reporting are async - use `tokio` runtime

## Troubleshooting

**Problem**: "Invalid ledger format" error  
**Solution**: Ensure your file has the `.bean` extension

**Problem**: Currency conversion fails  
**Solution**: Check internet connectivity - live rates are fetched from an external API

**Problem**: Tag validation fails  
**Solution**: Tags must be lowercase, contain only alphanumeric characters and hyphens, and be non-empty

**Problem**: Date range errors in reports  
**Solution**: Ensure `start_date` is before `end_date`

## Getting Help

- Check the [examples directory](../beans-lib/examples/) for working code
- Read the [API documentation](https://docs.rs/beans-lib)
- Review the [architecture documentation](ARCHITECTURE.md) for system design details

Happy tracking! 📊

