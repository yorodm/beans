# Personal Budget Management Tutorial

This tutorial walks you through setting up and managing a complete personal budget system using Beans. By the end, you'll have a working budget tracker with income tracking, expense categorization, and monthly reporting.

## Overview

We'll build a budget management system that:
- Tracks monthly income (salary, freelance work, etc.)
- Categorizes expenses (housing, food, transportation, etc.)
- Provides monthly summaries and trends
- Calculates savings rate
- Identifies spending patterns

## Step 1: Project Setup

Create a new Rust project:

```bash
cargo new my_budget
cd my_budget
```

Add dependencies to `Cargo.toml`:

```toml
[dependencies]
beans-lib = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
rust_decimal = "1.33"
rust_decimal_macros = "1.33"
chrono = "0.4"
```

## Step 2: Initialize Your Ledger

Create `src/main.rs`:

```rust
use beans_lib::prelude::*;
use rust_decimal_macros::dec;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> BeansResult<()> {
    // Open or create your personal ledger
    let ledger = LedgerManager::open("./my_budget.bean")?;
    
    println!("✓ Budget ledger initialized");
    
    Ok(())
}
```

## Step 3: Define Your Budget Categories

Create a tagging system for organizing expenses:

```rust
// Category tags
const CATEGORIES: &[&str] = &[
    "housing",      // Rent, mortgage, property taxes
    "utilities",    // Water, electricity, gas, internet
    "food",         // Groceries and dining
    "transportation", // Gas, public transit, car maintenance
    "health",       // Insurance, medical, gym
    "entertainment", // Movies, hobbies, subscriptions
    "shopping",     // Clothing, personal items
    "savings",      // Investments, emergency fund
];

// Frequency tags
const FREQUENCIES: &[&str] = &[
    "monthly",      // Recurring monthly
    "weekly",       // Recurring weekly
    "one-time",     // One-off expense
];
```

## Step 4: Add Income

Function to add income entries:

```rust
fn add_income(
    ledger: &LedgerManager,
    name: &str,
    amount: Decimal,
    tags: Vec<&str>,
) -> BeansResult<Uuid> {
    let mut builder = LedgerEntryBuilder::new()
        .name(name)
        .amount(amount)
        .currency(Currency::usd())
        .entry_type(EntryType::Income);
    
    for tag in tags {
        builder = builder.add_tag(tag)?;
    }
    
    let entry = builder.build()?;
    ledger.add_entry(&entry)
}

// Usage:
add_income(&ledger, "Monthly Salary", dec!(5000.00), vec!["salary", "monthly"])?;
add_income(&ledger, "Freelance Project", dec!(1500.00), vec!["freelance"])?;
```

## Step 5: Track Fixed Monthly Expenses

```rust
fn add_fixed_expenses(ledger: &LedgerManager) -> BeansResult<()> {
    let fixed_expenses = vec![
        ("Rent", dec!(1200.00), vec!["housing", "monthly"]),
        ("Internet", dec!(60.00), vec!["utilities", "monthly"]),
        ("Phone", dec!(50.00), vec!["utilities", "monthly"]),
        ("Car Insurance", dec!(100.00), vec!["transportation", "monthly"]),
        ("Health Insurance", dec!(150.00), vec!["health", "monthly"]),
        ("Gym Membership", dec!(40.00), vec!["health", "monthly"]),
        ("Streaming Services", dec!(30.00), vec!["entertainment", "monthly"]),
    ];
    
    for (name, amount, tags) in fixed_expenses {
        let mut builder = LedgerEntryBuilder::new()
            .name(name)
            .amount(amount)
            .currency(Currency::usd())
            .entry_type(EntryType::Expense);
        
        for tag in tags {
            builder = builder.add_tag(tag)?;
        }
        
        ledger.add_entry(&builder.build()?)?;
    }
    
    Ok(())
}
```

## Step 6: Track Variable Expenses

```rust
fn add_expense(
    ledger: &LedgerManager,
    name: &str,
    amount: Decimal,
    category: &str,
    frequency: &str,
) -> BeansResult<Uuid> {
    let entry = LedgerEntryBuilder::new()
        .name(name)
        .amount(amount)
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .add_tag(category)?
        .add_tag(frequency)?
        .build()?;
    
    ledger.add_entry(&entry)
}

// Usage examples:
add_expense(&ledger, "Groceries", dec!(150.00), "food", "weekly")?;
add_expense(&ledger, "Gas", dec!(45.00), "transportation", "weekly")?;
add_expense(&ledger, "Dinner Out", dec!(75.00), "food", "one-time")?;
add_expense(&ledger, "New Shoes", dec!(120.00), "shopping", "one-time")?;
```

## Step 7: Monthly Summary Report

```rust
async fn monthly_summary(ledger: &LedgerManager) -> BeansResult<()> {
    let now = Utc::now();
    let start_of_month = now - Duration::days(30);
    
    let generator = ReportGenerator::new(ledger);
    
    // Generate report
    let report = generator
        .income_expense_report(
            start_of_month,
            now,
            TimePeriod::Monthly,
            None,
            None,
        )
        .await?;
    
    // Display summary
    println!("\n=== Monthly Budget Summary ===");
    println!("Period: {} to {}", 
        start_of_month.format("%Y-%m-%d"),
        now.format("%Y-%m-%d")
    );
    println!("\nIncome:    ${:>10.2}", report.summary.income);
    println!("Expenses:  ${:>10.2}", report.summary.expenses);
    println!("────────────────────────");
    println!("Net:       ${:>10.2}", report.summary.net);
    
    // Calculate savings rate
    let savings_rate = if report.summary.income > dec!(0) {
        (report.summary.net / report.summary.income) * dec!(100)
    } else {
        dec!(0)
    };
    
    println!("\nSavings Rate: {:.1}%", savings_rate);
    
    Ok(())
}
```

## Step 8: Category Breakdown

```rust
async fn category_breakdown(ledger: &LedgerManager) -> BeansResult<()> {
    let now = Utc::now();
    let start = now - Duration::days(30);
    
    let generator = ReportGenerator::new(ledger);
    let tagged_report = generator.tagged_report(start, now, None).await?;
    
    println!("\n=== Spending by Category ===");
    
    // Sort by amount
    let mut expenses_vec: Vec<_> = tagged_report.expenses_by_tag.iter().collect();
    expenses_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    let total_expenses = tagged_report.summary.expenses;
    
    for (category, amount) in expenses_vec {
        let percentage = if total_expenses > dec!(0) {
            (amount / total_expenses) * dec!(100)
        } else {
            dec!(0)
        };
        
        println!("  {:.<20} ${:>8.2} ({:>5.1}%)", 
            category,
            amount,
            percentage
        );
    }
    
    Ok(())
}
```

## Step 9: Budget Alerts

```rust
fn check_budget_alerts(ledger: &LedgerManager, budgets: Vec<(&str, Decimal)>) -> BeansResult<()> {
    let now = Utc::now();
    let start = now - Duration::days(30);
    
    println!("\n=== Budget Alerts ===");
    
    for (category, budget_limit) in budgets {
        let filter = EntryFilter {
            tags: vec![category.to_string()],
            entry_type: Some(EntryType::Expense),
            start_date: Some(start),
            end_date: Some(now),
            ..Default::default()
        };
        
        let entries = ledger.list_entries(&filter)?;
        let total: Decimal = entries.iter().map(|e| e.amount()).sum();
        
        let percentage = (total / budget_limit) * dec!(100);
        
        let status = if percentage > dec!(100) {
            "⚠️  OVER BUDGET"
        } else if percentage > dec!(80) {
            "⚡ WARNING"
        } else {
            "✓ OK"
        };
        
        println!("  {:<15} ${:>8.2} / ${:>8.2} [{:>5.1}%] {}", 
            category,
            total,
            budget_limit,
            percentage,
            status
        );
    }
    
    Ok(())
}

// Usage:
let budgets = vec![
    ("food", dec!(600.00)),
    ("entertainment", dec!(200.00)),
    ("shopping", dec!(300.00)),
    ("transportation", dec!(250.00)),
];

check_budget_alerts(&ledger, budgets)?;
```

## Step 10: Complete Program

Here's the complete `main.rs`:

```rust
use beans_lib::prelude::*;
use rust_decimal_macros::dec;
use chrono::{Duration, Utc};

#[tokio::main]
async fn main() -> BeansResult<()> {
    // Initialize ledger
    let ledger = LedgerManager::open("./my_budget.bean")?;
    println!("✓ Budget system initialized\n");
    
    // Add monthly income
    println!("Adding income...");
    add_income(&ledger, "Monthly Salary", dec!(5000.00), vec!["salary", "monthly"])?;
    
    // Add fixed monthly expenses
    println!("Adding fixed expenses...");
    add_fixed_expenses(&ledger)?;
    
    // Add some variable expenses
    println!("Adding variable expenses...");
    add_expense(&ledger, "Groceries", dec!(150.00), "food", "weekly")?;
    add_expense(&ledger, "Gas", dec!(45.00), "transportation", "weekly")?;
    add_expense(&ledger, "Dinner Out", dec!(75.00), "food", "one-time")?;
    
    println!("\n✓ All entries added\n");
    
    // Generate reports
    monthly_summary(&ledger).await?;
    category_breakdown(&ledger).await?;
    
    // Check budget alerts
    let budgets = vec![
        ("food", dec!(600.00)),
        ("housing", dec!(1300.00)),
        ("transportation", dec!(250.00)),
    ];
    check_budget_alerts(&ledger, budgets)?;
    
    println!("\n✓ Budget analysis complete!");
    
    Ok(())
}

// ... (include all the helper functions from above)
```

## Best Practices

### 1. Consistent Entry Timing
- Add entries as soon as transactions occur
- Set up recurring reminders for monthly bills
- Review and reconcile weekly

### 2. Tag Strategy
- Use hierarchical tags (e.g., `food`, `food-groceries`, `food-dining`)
- Be consistent with tag names (always lowercase)
- Don't over-tag - 2-3 tags per entry is usually enough

### 3. Budget Categories
- Align categories with your actual spending patterns
- Start broad, then refine as needed
- Review and adjust categories quarterly

### 4. Regular Reviews
```rust
// Weekly review: Check variable expenses
// Monthly review: Full budget analysis
// Quarterly review: Adjust budgets and goals
// Yearly review: Long-term financial planning
```

### 5. Automation Ideas
- Write a CLI tool to quickly add entries
- Create scripts to import from bank statements
- Set up automated reports via email
- Build a web dashboard for visualization

## Advanced Features

### Multi-Currency Support

If you have income or expenses in multiple currencies:

```rust
use std::time::Duration as StdDuration;

let converter = CurrencyConverter::new(StdDuration::from_secs(3600));
let generator = ReportGenerator::new(&ledger).with_converter(converter);

// Reports will normalize all amounts to USD
let report = generator
    .income_expense_report(start, end, TimePeriod::Monthly, Some(Currency::usd()), None)
    .await?;
```

### Trend Analysis

```rust
async fn spending_trends(ledger: &LedgerManager) -> BeansResult<()> {
    let now = Utc::now();
    let generator = ReportGenerator::new(ledger);
    
    // Compare last 3 months
    for month in 0..3 {
        let end = now - Duration::days(30 * month);
        let start = end - Duration::days(30);
        
        let report = generator
            .income_expense_report(start, end, TimePeriod::Monthly, None, None)
            .await?;
        
        println!("Month {} ago: ${:.2} expenses", month, report.summary.expenses);
    }
    
    Ok(())
}
```

### Savings Goals

```rust
fn track_savings_goal(ledger: &LedgerManager, goal: Decimal) -> BeansResult<()> {
    let now = Utc::now();
    let start_of_year = now - Duration::days(365);
    
    let filter = EntryFilter {
        tags: vec!["savings".to_string()],
        entry_type: Some(EntryType::Expense),  // Savings are "expenses"
        start_date: Some(start_of_year),
        end_date: Some(now),
        ..Default::default()
    };
    
    let savings_entries = ledger.list_entries(&filter)?;
    let total_saved: Decimal = savings_entries.iter().map(|e| e.amount()).sum();
    
    let progress = (total_saved / goal) * dec!(100);
    
    println!("\n=== Savings Goal Progress ===");
    println!("Goal: ${:.2}", goal);
    println!("Saved: ${:.2}", total_saved);
    println!("Progress: {:.1}%", progress);
    
    if progress >= dec!(100) {
        println!("🎉 Congratulations! Goal achieved!");
    } else {
        let remaining = goal - total_saved;
        println!("Remaining: ${:.2}", remaining);
    }
    
    Ok(())
}
```

## Next Steps

- Explore [Custom Reports Guide](CUSTOM_REPORTS_GUIDE.md) for advanced analytics
- Check [Multi-Currency Guide](MULTI_CURRENCY_GUIDE.md) for international finances
- Review [Error Handling Guide](ERROR_HANDLING.md) for robust applications
- See [API Reference](API_REFERENCE.md) for complete API documentation

## Troubleshooting

**Q: My expenses don't sum correctly**  
A: Make sure you're using `Decimal` types, not `f64`, to avoid floating-point errors.

**Q: Tags aren't matching**  
A: Remember tags are case-sensitive in filters. Always use lowercase.

**Q: Reports show zero**  
A: Check your date ranges - entries might be outside the specified period.

**Q: How do I handle split expenses?**  
A: Create separate entries for your portion, or use descriptions to note the split.

Happy budgeting! 💰

