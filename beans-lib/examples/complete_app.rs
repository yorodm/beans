//! Complete Application Example
//!
//! This example demonstrates a more complete application that combines
//! multiple features of the Beans library:
//! - Setting up a persistent ledger file
//! - Managing entries over time
//! - Filtering and querying data
//! - Generating reports and analytics
//! - Working with tags for organization
//! - Multi-currency support
//!
//! This serves as a template for building a real-world application.
//!
//! Run with: `cargo run --example complete_app`

use beans_lib::prelude::*;
use chrono::{Duration, Utc};
use rust_decimal_macros::dec;
use std::time::Duration as StdDuration;

#[tokio::main]
async fn main() -> BeansResult<()> {
    println!("╔════════════════════════════════════════════════════╗");
    println!("║        Beans Personal Finance Manager             ║");
    println!("║           Complete Application Example            ║");
    println!("╚════════════════════════════════════════════════════╝\n");

    // === SETUP ===
    // In a real application, you'd use a persistent file:
    // let ledger = LedgerManager::open("./my_ledger.bean")?;
    let ledger = LedgerManager::in_memory()?;
    println!("✓ Ledger initialized\n");

    // === SIMULATE ADDING TRANSACTIONS OVER TIME ===
    println!("📝 Adding transactions...\n");
    
    simulate_monthly_transactions(&ledger)?;
    
    let total_entries = ledger.count_entries(&EntryFilter::default())?;
    println!("✓ Added {} transactions\n", total_entries);

    // === DISPLAY RECENT TRANSACTIONS ===
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 RECENT TRANSACTIONS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    let recent_filter = EntryFilter {
        limit: Some(10),
        ..Default::default()
    };
    
    let recent_entries = ledger.list_entries(&recent_filter)?;
    
    for entry in &recent_entries {
        let symbol = match entry.entry_type() {
            EntryType::Income => "💰",
            EntryType::Expense => "💸",
        };
        
        println!("{} {} {} {}",
            symbol,
            entry.date().format("%Y-%m-%d"),
            format_entry_line(entry),
            format_tags(entry.tags())
        );
    }
    println!();

    // === MONTHLY SUMMARY ===
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📈 MONTHLY SUMMARY");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    let now = Utc::now();
    let start_of_month = now - Duration::days(30);
    
    let generator = ReportGenerator::new(&ledger);
    
    let monthly_report = generator
        .income_expense_report(
            start_of_month,
            now,
            TimePeriod::Monthly,
            None,
            None,
        )
        .await?;
    
    println!("Period: {} to {}", 
        start_of_month.format("%Y-%m-%d"),
        now.format("%Y-%m-%d")
    );
    println!();
    println!("  Income:    ${:>10.2}", monthly_report.summary.income);
    println!("  Expenses:  ${:>10.2}", monthly_report.summary.expenses);
    println!("  ─────────────────────────");
    println!("  Net:       ${:>10.2}", monthly_report.summary.net);
    println!();
    
    // Calculate savings rate
    let savings_rate = if monthly_report.summary.income > dec!(0) {
        (monthly_report.summary.net / monthly_report.summary.income) * dec!(100)
    } else {
        dec!(0)
    };
    
    println!("  Savings Rate: {:.1}%", savings_rate);
    println!();

    // === SPENDING BY CATEGORY ===
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🏷️  SPENDING BY CATEGORY");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    let tagged_report = generator
        .tagged_report(start_of_month, now, None)
        .await?;
    
    let mut tags_vec: Vec<_> = tagged_report.expenses_by_tag.iter().collect();
    tags_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    for (tag, amount) in tags_vec.iter().take(10) {
        let percentage = if monthly_report.summary.expenses > dec!(0) {
            (*amount / monthly_report.summary.expenses) * dec!(100)
        } else {
            dec!(0)
        };
        
        println!("  {:.<20} ${:>8.2} ({:>5.1}%)", 
            tag,
            amount,
            percentage
        );
    }
    println!();

    // === CATEGORY BREAKDOWN ===
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 CATEGORY DETAILS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    display_category_detail(&ledger, "food", "Food & Dining", start_of_month, now)?;
    display_category_detail(&ledger, "housing", "Housing", start_of_month, now)?;
    display_category_detail(&ledger, "utilities", "Utilities", start_of_month, now)?;

    // === WEEKLY TRENDS ===
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📉 WEEKLY SPENDING TREND");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    let weekly_report = generator
        .income_expense_report(
            now - Duration::days(28),
            now,
            TimePeriod::Weekly,
            None,
            None,
        )
        .await?;
    
    for (i, point) in weekly_report.expense_series.points.iter().enumerate() {
        let bar_length = (point.value.to_f64().unwrap_or(0.0) / 50.0) as usize;
        let bar = "█".repeat(bar_length);
        println!("  Week {} {:.<15} ${:>7.2}", 
            i + 1,
            bar,
            point.value
        );
    }
    println!("\n  (Each █ ≈ $50)\n");

    // === COMPARISON WITH PREVIOUS MONTH ===
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🔄 MONTH-OVER-MONTH COMPARISON");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    let prev_month_start = start_of_month - Duration::days(30);
    let prev_month_end = start_of_month;
    
    let prev_report = generator
        .income_expense_report(
            prev_month_start,
            prev_month_end,
            TimePeriod::Monthly,
            None,
            None,
        )
        .await?;
    
    let income_change = monthly_report.summary.income - prev_report.summary.income;
    let expense_change = monthly_report.summary.expenses - prev_report.summary.expenses;
    
    println!("  Income:   {} ${:.2}", 
        if income_change >= dec!(0) { "📈" } else { "📉" },
        income_change.abs()
    );
    println!("  Expenses: {} ${:.2}", 
        if expense_change <= dec!(0) { "📈" } else { "📉" },
        expense_change.abs()
    );
    println!();

    // === INSIGHTS AND RECOMMENDATIONS ===
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💡 INSIGHTS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // Savings rate analysis
    if savings_rate < dec!(10) {
        println!("  ⚠️  Savings rate is low ({:.1}%). Consider reducing expenses.", savings_rate);
    } else if savings_rate >= dec!(20) {
        println!("  ✓ Great savings rate ({:.1}%)! Keep it up!", savings_rate);
    } else {
        println!("  ✓ Savings rate is {:.1}%. Room for improvement.", savings_rate);
    }
    
    // Spending trend
    if expense_change > dec!(0) {
        println!("  ⚠️  Expenses increased by ${:.2} from last month.", expense_change);
    } else if expense_change < dec!(0) {
        println!("  ✓ Expenses decreased by ${:.2} from last month!", expense_change.abs());
    }
    
    println!();

    // === MULTI-CURRENCY DEMO ===
    if let Ok(converter) = try_currency_demo(&ledger).await {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("💱 MULTI-CURRENCY SUPPORT");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        println!("  ✓ Currency conversion available");
        println!("  ✓ Exchange rates cached for performance");
        println!();
    }

    println!("╔════════════════════════════════════════════════════╗");
    println!("║                 Report Complete                    ║");
    println!("╚════════════════════════════════════════════════════╝\n");

    Ok(())
}

/// Simulate adding monthly transactions
fn simulate_monthly_transactions(ledger: &LedgerManager) -> BeansResult<()> {
    let now = Utc::now();
    
    // Monthly income
    let salary = LedgerEntryBuilder::new()
        .name("Monthly Salary")
        .amount(dec!(5000.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Income)
        .date(now - Duration::days(1))
        .add_tag("salary")?
        .add_tag("monthly")?
        .build()?;
    ledger.add_entry(&salary)?;
    
    // Fixed monthly expenses
    let fixed_expenses = vec![
        ("Rent", dec!(1200.00), vec!["housing", "monthly", "fixed"]),
        ("Internet", dec!(60.00), vec!["utilities", "monthly", "fixed"]),
        ("Phone", dec!(50.00), vec!["utilities", "monthly", "fixed"]),
        ("Insurance", dec!(150.00), vec!["insurance", "monthly", "fixed"]),
        ("Gym", dec!(40.00), vec!["health", "monthly", "fixed"]),
    ];
    
    for (name, amount, tags) in fixed_expenses {
        let mut builder = LedgerEntryBuilder::new()
            .name(name)
            .amount(amount)
            .currency(Currency::usd())
            .entry_type(EntryType::Expense)
            .date(now - Duration::days(5));
        
        for tag in tags {
            builder = builder.add_tag(tag)?;
        }
        
        ledger.add_entry(&builder.build()?)?;
    }
    
    // Variable weekly expenses
    for week in 0..4 {
        let week_date = now - Duration::days(7 * week);
        
        let variable_expenses = vec![
            ("Groceries", dec!(100.00 + week as f64 * 10.0), vec!["food", "groceries"]),
            ("Gas", dec!(45.00), vec!["transportation"]),
            ("Dining Out", dec!(60.00 + week as f64 * 5.0), vec!["food", "dining"]),
        ];
        
        for (name, amount, tags) in variable_expenses {
            let mut builder = LedgerEntryBuilder::new()
                .name(name)
                .amount(amount)
                .currency(Currency::usd())
                .entry_type(EntryType::Expense)
                .date(week_date);
            
            for tag in tags {
                builder = builder.add_tag(tag)?;
            }
            
            ledger.add_entry(&builder.build()?)?;
        }
    }
    
    // Random one-off expenses
    let oneoff = vec![
        ("Movie Tickets", dec!(30.00), vec!["entertainment"], 10),
        ("Book Purchase", dec!(25.00), vec!["education"], 15),
        ("Coffee Shop", dec!(15.00), vec!["food", "dining"], 3),
        ("Haircut", dec!(35.00), vec!["personal-care"], 12),
    ];
    
    for (name, amount, tags, days_ago) in oneoff {
        let mut builder = LedgerEntryBuilder::new()
            .name(name)
            .amount(amount)
            .currency(Currency::usd())
            .entry_type(EntryType::Expense)
            .date(now - Duration::days(days_ago));
        
        for tag in tags {
            builder = builder.add_tag(tag)?;
        }
        
        ledger.add_entry(&builder.build()?)?;
    }
    
    Ok(())
}

/// Display detailed breakdown for a category
fn display_category_detail(
    ledger: &LedgerManager,
    tag: &str,
    display_name: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> BeansResult<()> {
    let filter = EntryFilter {
        tags: vec![tag.to_string()],
        entry_type: Some(EntryType::Expense),
        start_date: Some(start),
        end_date: Some(end),
        ..Default::default()
    };
    
    let entries = ledger.list_entries(&filter)?;
    
    if entries.is_empty() {
        return Ok(());
    }
    
    let total: Decimal = entries.iter().map(|e| e.amount()).sum();
    
    println!("{} ({})", display_name, tag);
    println!("  Entries: {} | Total: ${:.2}", entries.len(), total);
    
    for entry in entries.iter().take(3) {
        println!("    • {} - ${:.2}", entry.name(), entry.amount());
    }
    
    if entries.len() > 3 {
        println!("    ... and {} more", entries.len() - 3);
    }
    
    println!();
    
    Ok(())
}

/// Try to demonstrate currency conversion if internet is available
async fn try_currency_demo(ledger: &LedgerManager) -> BeansResult<CurrencyConverter> {
    let converter = CurrencyConverter::new(StdDuration::from_secs(3600));
    
    // Try a simple conversion to test connectivity
    converter.convert(dec!(100), &Currency::usd(), &Currency::eur()).await?;
    
    Ok(converter)
}

/// Format an entry line for display
fn format_entry_line(entry: &LedgerEntry) -> String {
    format!("{:<30} ${:>8.2}",
        truncate(entry.name(), 30),
        entry.amount()
    )
}

/// Format tags for display
fn format_tags(tags: &std::collections::HashSet<Tag>) -> String {
    let tag_names: Vec<_> = tags.iter().map(|t| t.name()).take(3).collect();
    if tag_names.is_empty() {
        String::new()
    } else {
        format!("[{}]", tag_names.join(", "))
    }
}

/// Truncate a string to a maximum length
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len-3])
    }
}
