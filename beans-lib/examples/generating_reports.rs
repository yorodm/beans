//! Generating Reports Example
//!
//! This example demonstrates the reporting and analytics capabilities:
//! - Generating income vs expense reports
//! - Creating time-series data for different periods
//! - Filtering reports by tags
//! - Multi-currency reporting with normalization
//! - Exporting reports to different formats
//!
//! Run with: `cargo run --example generating_reports`

use beans_lib::prelude::*;
use chrono::{Duration, Utc};
use rust_decimal_macros::dec;
use std::time::Duration as StdDuration;

#[tokio::main]
async fn main() -> BeansResult<()> {
    println!("=== Beans Ledger: Generating Reports Example ===\n");

    // Create a ledger and populate it with sample data
    let ledger = setup_sample_ledger()?;
    println!("✓ Created ledger with sample transactions\n");

    // === BASIC INCOME/EXPENSE REPORT ===
    println!("--- Basic Income vs Expense Report ---");
    
    let now = Utc::now();
    let start_date = now - Duration::days(90); // Last 90 days
    let end_date = now;
    
    // Create a report generator
    let generator = ReportGenerator::new(&ledger);
    
    // Generate monthly report
    let monthly_report = generator
        .income_expense_report(
            start_date,
            end_date,
            TimePeriod::Monthly,
            None, // No currency conversion
            None, // No tag filtering
        )
        .await?;
    
    println!("Period: {} to {}", 
        start_date.format("%Y-%m-%d"),
        end_date.format("%Y-%m-%d")
    );
    println!("Total Income: {}", monthly_report.summary.income);
    println!("Total Expenses: {}", monthly_report.summary.expenses);
    println!("Net: {}", monthly_report.summary.net);
    println!();

    // === TIME SERIES DATA ===
    println!("--- Monthly Time Series ---");
    
    println!("Income by month:");
    for point in &monthly_report.income_series.points {
        println!("  {} - {}", 
            point.timestamp.format("%Y-%m-%d"),
            point.value
        );
    }
    println!();
    
    println!("Expenses by month:");
    for point in &monthly_report.expense_series.points {
        println!("  {} - {}", 
            point.timestamp.format("%Y-%m-%d"),
            point.value
        );
    }
    println!();

    // === WEEKLY REPORT ===
    println!("--- Weekly Report ---");
    
    let weekly_start = now - Duration::days(28); // Last 4 weeks
    let weekly_report = generator
        .income_expense_report(
            weekly_start,
            end_date,
            TimePeriod::Weekly,
            None,
            None,
        )
        .await?;
    
    println!("Period: {} to {}", 
        weekly_start.format("%Y-%m-%d"),
        end_date.format("%Y-%m-%d")
    );
    println!("Number of periods: {}", weekly_report.income_series.points.len());
    let avg_income = if weekly_report.income_series.points.len() > 0 {
        weekly_report.summary.income / Decimal::from(weekly_report.income_series.points.len())
    } else {
        dec!(0)
    };
    println!("Average income per week: {}", avg_income);
    println!();

    // === DAILY REPORT ===
    println!("--- Daily Report (Last 7 Days) ---");
    
    let daily_start = now - Duration::days(7);
    let daily_report = generator
        .income_expense_report(
            daily_start,
            end_date,
            TimePeriod::Daily,
            None,
            None,
        )
        .await?;
    
    println!("Daily breakdown:");
    for (income_pt, expense_pt) in daily_report.income_series.points.iter()
        .zip(daily_report.expense_series.points.iter()) {
        let net = income_pt.value - expense_pt.value;
        println!("  {} | Income: {:>8} | Expenses: {:>8} | Net: {:>8}", 
            income_pt.timestamp.format("%Y-%m-%d"),
            format!("{:.2}", income_pt.value),
            format!("{:.2}", expense_pt.value),
            format!("{:.2}", net)
        );
    }
    println!();

    // === FILTERED REPORT (BY TAGS) ===
    println!("--- Report Filtered by Tags ---");
    
    // Report for only "monthly" tagged expenses
    let monthly_expenses_report = generator
        .income_expense_report(
            start_date,
            end_date,
            TimePeriod::Monthly,
            None,
            Some(vec!["monthly".to_string()]),
        )
        .await?;
    
    println!("Monthly recurring expenses:");
    println!("  Total: {}", monthly_expenses_report.summary.expenses);
    let avg_monthly = if monthly_expenses_report.expense_series.points.len() > 0 {
        monthly_expenses_report.summary.expenses / Decimal::from(monthly_expenses_report.expense_series.points.len())
    } else {
        dec!(0)
    };
    println!("  Average per month: {}", avg_monthly);
    println!();

    // Report for food-related expenses
    let food_report = generator
        .income_expense_report(
            start_date,
            end_date,
            TimePeriod::Monthly,
            None,
            Some(vec!["food".to_string()]),
        )
        .await?;
    
    println!("Food expenses:");
    println!("  Total: {}", food_report.summary.expenses);
    let avg_food = if food_report.expense_series.points.len() > 0 {
        food_report.summary.expenses / Decimal::from(food_report.expense_series.points.len())
    } else {
        dec!(0)
    };
    println!("  Average per month: {}", avg_food);
    println!();

    // === MULTI-CURRENCY REPORT ===
    println!("--- Multi-Currency Report ---");
    
    // Create a ledger with multi-currency entries
    let multi_ledger = setup_multi_currency_ledger()?;
    
    // Create converter for normalization
    let converter = CurrencyConverter::new(StdDuration::from_secs(3600));
    let multi_generator = ReportGenerator::new(&multi_ledger)
        .with_converter(converter);
    
    // Generate report normalized to USD
    println!("Generating report with currency normalization to USD...");
    match multi_generator
        .income_expense_report(
            start_date,
            end_date,
            TimePeriod::Monthly,
            Some(Currency::usd()),
            None,
        )
        .await {
            Ok(report) => {
                println!("✓ Multi-currency report generated");
                println!("  Total Income (USD): ${}", report.summary.income);
                println!("  Total Expenses (USD): ${}", report.summary.expenses);
                println!("  Net (USD): ${}", report.summary.net);
            }
            Err(e) => {
                println!("⚠️  Currency conversion failed: {}", e);
                println!("   (This requires internet connectivity)");
            }
        }
    println!();

    // === TAGGED REPORT ===
    println!("--- Report Grouped by Tags ---");
    
    let tagged_report = generator
        .tagged_report(start_date, end_date, None)
        .await?;
    
    println!("Expenses by category:");
    let mut expenses_vec: Vec<_> = tagged_report.expenses_by_tag.iter().collect();
    expenses_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    for (tag, amount) in expenses_vec {
        println!("  {}: ${:.2}", tag, amount);
    }
    println!();

    // === SAVINGS RATE CALCULATION ===
    println!("--- Savings Rate Analysis ---");
    
    let total_income = monthly_report.summary.income;
    let total_expenses = monthly_report.summary.expenses;
    let savings = total_income - total_expenses;
    let savings_rate = if total_income > dec!(0) {
        (savings / total_income) * dec!(100)
    } else {
        dec!(0)
    };
    
    println!("Total Income: ${:.2}", total_income);
    println!("Total Expenses: ${:.2}", total_expenses);
    println!("Total Savings: ${:.2}", savings);
    println!("Savings Rate: {:.1}%", savings_rate);
    println!();

    println!("=== Example Complete ===");
    println!("\nTip: Reports can be exported to JSON or CSV for further analysis.");
    println!("See the API documentation for export methods.");
    
    Ok(())
}

/// Helper function to set up a ledger with sample data spanning multiple months
fn setup_sample_ledger() -> BeansResult<LedgerManager> {
    let ledger = LedgerManager::in_memory()?;
    let now = Utc::now();
    
    // Add monthly recurring entries for last 3 months
    for month in 0..3 {
        let date = now - Duration::days(30 * month);
        
        // Income
        let salary = LedgerEntryBuilder::new()
            .name("Monthly Salary")
            .amount(dec!(5000.00))
            .currency(Currency::usd())
            .entry_type(EntryType::Income)
            .date(date)
            .add_tag("salary")?
            .add_tag("monthly")?
            .build()?;
        ledger.add_entry(&salary)?;
        
        // Fixed expenses
        let rent = LedgerEntryBuilder::new()
            .name("Rent")
            .amount(dec!(1200.00))
            .currency(Currency::usd())
            .entry_type(EntryType::Expense)
            .date(date)
            .add_tag("housing")?
            .add_tag("monthly")?
            .build()?;
        ledger.add_entry(&rent)?;
        
        let utilities = LedgerEntryBuilder::new()
            .name("Utilities")
            .amount(dec!(200.00))
            .currency(Currency::usd())
            .entry_type(EntryType::Expense)
            .date(date)
            .add_tag("utilities")?
            .add_tag("monthly")?
            .build()?;
        ledger.add_entry(&utilities)?;
        
        // Variable expenses
        let groceries = LedgerEntryBuilder::new()
            .name("Groceries")
            .amount(dec!(400.00 + (month as f64 * 50.0)))
            .currency(Currency::usd())
            .entry_type(EntryType::Expense)
            .date(date)
            .add_tag("food")?
            .add_tag("groceries")?
            .build()?;
        ledger.add_entry(&groceries)?;
    }
    
    // Add some weekly entries for the current month
    for week in 0..4 {
        let date = now - Duration::days(7 * week);
        
        let dining = LedgerEntryBuilder::new()
            .name("Dining Out")
            .amount(dec!(75.00 + (week as f64 * 10.0)))
            .currency(Currency::usd())
            .entry_type(EntryType::Expense)
            .date(date)
            .add_tag("food")?
            .add_tag("dining")?
            .build()?;
        ledger.add_entry(&dining)?;
    }
    
    Ok(ledger)
}

/// Helper function to set up a multi-currency ledger
fn setup_multi_currency_ledger() -> BeansResult<LedgerManager> {
    let ledger = LedgerManager::in_memory()?;
    
    let entry1 = LedgerEntryBuilder::new()
        .name("US Income")
        .amount(dec!(5000.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Income)
        .build()?;
    ledger.add_entry(&entry1)?;
    
    let entry2 = LedgerEntryBuilder::new()
        .name("EU Expense")
        .amount(dec!(1200.00))
        .currency(Currency::eur())
        .entry_type(EntryType::Expense)
        .build()?;
    ledger.add_entry(&entry2)?;
    
    let entry3 = LedgerEntryBuilder::new()
        .name("UK Expense")
        .amount(dec!(500.00))
        .currency(Currency::gbp())
        .entry_type(EntryType::Expense)
        .build()?;
    ledger.add_entry(&entry3)?;
    
    Ok(ledger)
}
