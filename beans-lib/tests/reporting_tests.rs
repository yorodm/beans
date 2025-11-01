mod support;

use beans_lib::currency::CurrencyConverter;
use beans_lib::error::BeansResult;
use beans_lib::ledger::LedgerManager;
use beans_lib::models::{Currency, EntryType, LedgerEntry, LedgerEntryBuilder, Tag};
use beans_lib::reporting::{
    ExportFormat, IncomeExpenseReport, PeriodSummary, ReportGenerator, TagReport, TimePeriod,
};
use chrono::{Duration, TimeZone, Utc};
use rust_decimal_macros::dec;
use std::io::Cursor;
use support::*;

// Helper function to create a test ledger with sample entries
async fn create_test_ledger() -> BeansResult<LedgerManager> {
    let ledger = LedgerManager::in_memory()?;
    
    // Create sample entries
    
    // January entries
    let jan_1 = Utc.with_ymd_and_hms(2023, 1, 5, 12, 0, 0).unwrap();
    let jan_2 = Utc.with_ymd_and_hms(2023, 1, 15, 12, 0, 0).unwrap();
    let jan_3 = Utc.with_ymd_and_hms(2023, 1, 25, 12, 0, 0).unwrap();
    
    // February entries
    let feb_1 = Utc.with_ymd_and_hms(2023, 2, 5, 12, 0, 0).unwrap();
    let feb_2 = Utc.with_ymd_and_hms(2023, 2, 15, 12, 0, 0).unwrap();
    
    // March entries
    let mar_1 = Utc.with_ymd_and_hms(2023, 3, 5, 12, 0, 0).unwrap();
    let mar_2 = Utc.with_ymd_and_hms(2023, 3, 15, 12, 0, 0).unwrap();
    
    // Create tags
    let food_tag = Tag::new("food")?;
    let rent_tag = Tag::new("rent")?;
    let salary_tag = Tag::new("salary")?;
    let bonus_tag = Tag::new("bonus")?;
    let entertainment_tag = Tag::new("entertainment")?;
    
    // Income entries
    let salary_jan = LedgerEntryBuilder::new()
        .date(jan_1)
        .name("January Salary")
        .currency(usd().to_string())
        .amount(dec!(5000.00))
        .tag(salary_tag.clone())
        .entry_type(EntryType::Income)
        .build()?;
    
    let bonus_feb = LedgerEntryBuilder::new()
        .date(feb_1)
        .name("February Bonus")
        .currency(usd().to_string())
        .amount(dec!(1000.00))
        .tag(bonus_tag.clone())
        .entry_type(EntryType::Income)
        .build()?;
    
    let salary_feb = LedgerEntryBuilder::new()
        .date(feb_2)
        .name("February Salary")
        .currency(usd().to_string())
        .amount(dec!(5000.00))
        .tag(salary_tag.clone())
        .entry_type(EntryType::Income)
        .build()?;
    
    let salary_mar = LedgerEntryBuilder::new()
        .date(mar_1)
        .name("March Salary")
        .currency(usd().to_string())
        .amount(dec!(5000.00))
        .tag(salary_tag.clone())
        .entry_type(EntryType::Income)
        .build()?;
    
    // Expense entries
    let rent_jan = LedgerEntryBuilder::new()
        .date(jan_2)
        .name("January Rent")
        .currency(usd().to_string())
        .amount(dec!(1500.00))
        .tag(rent_tag.clone())
        .entry_type(EntryType::Expense)
        .build()?;
    
    let food_jan = LedgerEntryBuilder::new()
        .date(jan_3)
        .name("January Groceries")
        .currency(usd().to_string())
        .amount(dec!(300.00))
        .tag(food_tag.clone())
        .entry_type(EntryType::Expense)
        .build()?;
    
    let rent_feb = LedgerEntryBuilder::new()
        .date(feb_2)
        .name("February Rent")
        .currency(usd().to_string())
        .amount(dec!(1500.00))
        .tag(rent_tag.clone())
        .entry_type(EntryType::Expense)
        .build()?;
    
    let rent_mar = LedgerEntryBuilder::new()
        .date(mar_2)
        .name("March Rent")
        .currency(usd().to_string())
        .amount(dec!(1500.00))
        .tag(rent_tag.clone())
        .entry_type(EntryType::Expense)
        .build()?;
    
    let entertainment_mar = LedgerEntryBuilder::new()
        .date(mar_2)
        .name("Movie Night")
        .currency(eur().to_string())
        .amount(dec!(50.00))
        .tag(entertainment_tag.clone())
        .entry_type(EntryType::Expense)
        .build()?;
    
    // Add entries to ledger
    ledger.add_entry(&salary_jan)?;
    ledger.add_entry(&bonus_feb)?;
    ledger.add_entry(&salary_feb)?;
    ledger.add_entry(&salary_mar)?;
    ledger.add_entry(&rent_jan)?;
    ledger.add_entry(&food_jan)?;
    ledger.add_entry(&rent_feb)?;
    ledger.add_entry(&rent_mar)?;
    ledger.add_entry(&entertainment_mar)?;
    
    Ok(ledger)
}

#[tokio::test]
async fn test_period_summary() -> BeansResult<()> {
    // Create test ledger
    let ledger = create_test_ledger().await?;
    
    // Create report generator
    let generator = ReportGenerator::new(&ledger);
    
    // Define date range for January
    let start_date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2023, 1, 31, 23, 59, 59).unwrap();
    
    // Generate summary
    let summary = generator.period_summary(start_date, end_date, None, None).await?;
    
    // Verify summary
    assert_eq!(summary.income, dec!(5000.00));
    assert_eq!(summary.expenses, dec!(1800.00));
    assert_eq!(summary.net, dec!(3200.00));
    assert_eq!(summary.currency, "USD");
    
    Ok(())
}

#[tokio::test]
async fn test_income_expense_report_monthly() -> BeansResult<()> {
    // Create test ledger
    let ledger = create_test_ledger().await?;
    
    // Create report generator
    let generator = ReportGenerator::new(&ledger);
    
    // Define date range for Q1
    let start_date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2023, 3, 31, 23, 59, 59).unwrap();
    
    // Generate report with monthly granularity
    let report = generator
        .income_expense_report(start_date, end_date, TimePeriod::Monthly, None, None)
        .await?;
    
    // Verify report
    assert_eq!(report.income_series.points.len(), 3); // 3 months
    assert_eq!(report.expense_series.points.len(), 3); // 3 months
    
    // Verify January data
    let jan_income = &report.income_series.points[0];
    let jan_expenses = &report.expense_series.points[0];
    assert_eq!(jan_income.value, dec!(5000.00));
    assert_eq!(jan_expenses.value, dec!(1800.00));
    
    // Verify February data
    let feb_income = &report.income_series.points[1];
    let feb_expenses = &report.expense_series.points[1];
    assert_eq!(feb_income.value, dec!(6000.00)); // Salary + Bonus
    assert_eq!(feb_expenses.value, dec!(1500.00));
    
    // Verify March data
    let mar_income = &report.income_series.points[2];
    let mar_expenses = &report.expense_series.points[2];
    assert_eq!(mar_income.value, dec!(5000.00));
    assert!(mar_expenses.value > dec!(1500.00)); // Rent + EUR entertainment
    
    // Verify summary
    assert_eq!(report.summary.income, dec!(16000.00));
    assert!(report.summary.expenses > dec!(4800.00)); // Total expenses including EUR
    assert!(report.summary.net > dec!(11000.00)); // Net income
    
    Ok(())
}

#[tokio::test]
async fn test_tag_report() -> BeansResult<()> {
    // Create test ledger
    let ledger = create_test_ledger().await?;
    
    // Create report generator
    let generator = ReportGenerator::new(&ledger);
    
    // Define date range for Q1
    let start_date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2023, 3, 31, 23, 59, 59).unwrap();
    
    // Generate tag report for expenses
    let report = generator
        .tag_report(start_date, end_date, None, Some(EntryType::Expense))
        .await?;
    
    // Verify report
    assert!(report.tag_summaries.len() >= 3); // At least 3 tags (rent, food, entertainment)
    
    // Verify rent tag (should be the highest)
    let rent_summary = report.tag_summaries.iter().find(|s| s.tag == "rent").unwrap();
    assert_eq!(rent_summary.amount, dec!(4500.00)); // 3 months of rent
    assert_eq!(rent_summary.count, 3); // 3 rent entries
    
    // Verify food tag
    let food_summary = report.tag_summaries.iter().find(|s| s.tag == "food").unwrap();
    assert_eq!(food_summary.amount, dec!(300.00));
    assert_eq!(food_summary.count, 1);
    
    Ok(())
}

#[tokio::test]
async fn test_export_report_json() -> BeansResult<()> {
    // Create test ledger
    let ledger = create_test_ledger().await?;
    
    // Create report generator
    let generator = ReportGenerator::new(&ledger);
    
    // Define date range for January
    let start_date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2023, 1, 31, 23, 59, 59).unwrap();
    
    // Generate report
    let report = generator
        .income_expense_report(start_date, end_date, TimePeriod::Monthly, None, None)
        .await?;
    
    // Export to JSON
    let mut buffer = Cursor::new(Vec::new());
    generator.export_report(&report, ExportFormat::Json, &mut buffer).await?;
    
    // Verify JSON
    let json = String::from_utf8(buffer.into_inner()).unwrap();
    assert!(json.contains("\"income_series\""));
    assert!(json.contains("\"expense_series\""));
    assert!(json.contains("\"summary\""));
    
    Ok(())
}

#[tokio::test]
async fn test_export_report_csv() -> BeansResult<()> {
    // Create test ledger
    let ledger = create_test_ledger().await?;
    
    // Create report generator
    let generator = ReportGenerator::new(&ledger);
    
    // Define date range for January
    let start_date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2023, 1, 31, 23, 59, 59).unwrap();
    
    // Generate report
    let report = generator
        .income_expense_report(start_date, end_date, TimePeriod::Monthly, None, None)
        .await?;
    
    // Export to CSV
    let mut buffer = Cursor::new(Vec::new());
    generator.export_report(&report, ExportFormat::Csv, &mut buffer).await?;
    
    // Verify CSV
    let csv = String::from_utf8(buffer.into_inner()).unwrap();
    assert!(csv.contains("Period,Date,Income,Expenses,Net"));
    assert!(csv.contains("monthly,2023-01"));
    
    Ok(())
}

#[tokio::test]
async fn test_currency_conversion() -> BeansResult<()> {
    // Create test ledger
    let ledger = create_test_ledger().await?;
    
    // Create mock converter
    let converter = CurrencyConverter::default();
    
    // Create report generator with converter
    let generator = ReportGenerator::new(&ledger).with_converter(converter);
    
    // Define date range for March (has EUR expense)
    let start_date = Utc.with_ymd_and_hms(2023, 3, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2023, 3, 31, 23, 59, 59).unwrap();
    
    // Create target currency (USD)
    let usd_currency = Currency::new(dec!(0.00), usd())?;
    
    // This test will be skipped if the exchange rate API is not available
    // as we're not mocking the API here
    let result = generator
        .income_expense_report(start_date, end_date, TimePeriod::Monthly, Some(usd_currency), None)
        .await;
    
    // If the API is available, verify the report
    if result.is_ok() {
        let report = result.unwrap();
        assert_eq!(report.income_series.points.len(), 1); // 1 month
        assert_eq!(report.expense_series.points.len(), 1); // 1 month
        assert_eq!(report.currency, "USD");
    }
    
    Ok(())
}

#[tokio::test]
async fn test_filtering_by_tags() -> BeansResult<()> {
    // Create test ledger
    let ledger = create_test_ledger().await?;
    
    // Create report generator
    let generator = ReportGenerator::new(&ledger);
    
    // Define date range for Q1
    let start_date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2023, 3, 31, 23, 59, 59).unwrap();
    
    // Generate report filtered by "rent" tag
    let report = generator
        .income_expense_report(
            start_date,
            end_date,
            TimePeriod::Monthly,
            None,
            Some(vec!["rent".to_string()]),
        )
        .await?;
    
    // Verify report only includes rent expenses
    assert_eq!(report.income_series.points.len(), 3); // 3 months
    assert_eq!(report.expense_series.points.len(), 3); // 3 months
    
    // Verify January data (only rent)
    let jan_expenses = &report.expense_series.points[0];
    assert_eq!(jan_expenses.value, dec!(1500.00));
    
    // Verify February data (only rent)
    let feb_expenses = &report.expense_series.points[1];
    assert_eq!(feb_expenses.value, dec!(1500.00));
    
    // Verify March data (only rent)
    let mar_expenses = &report.expense_series.points[2];
    assert_eq!(mar_expenses.value, dec!(1500.00));
    
    // Verify summary
    assert_eq!(report.summary.expenses, dec!(4500.00)); // Total rent expenses
    
    Ok(())
}

#[tokio::test]
async fn test_different_time_periods() -> BeansResult<()> {
    // Create test ledger
    let ledger = create_test_ledger().await?;
    
    // Create report generator
    let generator = ReportGenerator::new(&ledger);
    
    // Define date range for Q1
    let start_date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2023, 3, 31, 23, 59, 59).unwrap();
    
    // Test quarterly report
    let quarterly_report = generator
        .income_expense_report(start_date, end_date, TimePeriod::Quarterly, None, None)
        .await?;
    
    // Verify quarterly report
    assert_eq!(quarterly_report.income_series.points.len(), 1); // 1 quarter
    assert_eq!(quarterly_report.expense_series.points.len(), 1); // 1 quarter
    
    // Test yearly report
    let yearly_report = generator
        .income_expense_report(start_date, end_date, TimePeriod::Yearly, None, None)
        .await?;
    
    // Verify yearly report
    assert_eq!(yearly_report.income_series.points.len(), 1); // 1 year
    assert_eq!(yearly_report.expense_series.points.len(), 1); // 1 year
    
    // Test weekly report (should have more points)
    let weekly_report = generator
        .income_expense_report(start_date, end_date, TimePeriod::Weekly, None, None)
        .await?;
    
    // Verify weekly report (Q1 has ~13 weeks)
    assert!(weekly_report.income_series.points.len() > 10);
    assert!(weekly_report.expense_series.points.len() > 10);
    
    Ok(())
}

#[tokio::test]
async fn test_empty_report() -> BeansResult<()> {
    // Create empty ledger
    let ledger = LedgerManager::in_memory()?;
    
    // Create report generator
    let generator = ReportGenerator::new(&ledger);
    
    // Define date range
    let start_date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2023, 3, 31, 23, 59, 59).unwrap();
    
    // Generate report
    let report = generator
        .income_expense_report(start_date, end_date, TimePeriod::Monthly, None, None)
        .await?;
    
    // Verify empty report
    assert_eq!(report.income_series.points.len(), 0);
    assert_eq!(report.expense_series.points.len(), 0);
    assert_eq!(report.summary.income, dec!(0));
    assert_eq!(report.summary.expenses, dec!(0));
    assert_eq!(report.summary.net, dec!(0));
    
    Ok(())
}

#[tokio::test]
async fn test_invalid_date_range() -> BeansResult<()> {
    // Create test ledger
    let ledger = create_test_ledger().await?;
    
    // Create report generator
    let generator = ReportGenerator::new(&ledger);
    
    // Define invalid date range (end before start)
    let start_date = Utc.with_ymd_and_hms(2023, 3, 1, 0, 0, 0).unwrap();
    let end_date = Utc.with_ymd_and_hms(2023, 1, 31, 23, 59, 59).unwrap();
    
    // Generate report (should fail)
    let result = generator
        .income_expense_report(start_date, end_date, TimePeriod::Monthly, None, None)
        .await;
    
    // Verify error
    assert!(result.is_err());
    
    Ok(())
}

