//! Integration tests for reporting and analytics module.

mod support;

use beans_lib::error::BeansResult;
use beans_lib::ledger::LedgerManager;
use beans_lib::models::{EntryType, LedgerEntryBuilder, Tag};
use beans_lib::reporting::{ExportFormat, ReportGenerator, TimePeriod};
use chrono::{Duration, TimeZone, Utc};
use rust_decimal_macros::dec;

/// Creates a ledger with sample entries for testing.
async fn create_test_ledger_with_entries() -> BeansResult<LedgerManager> {
    let ledger = LedgerManager::in_memory()?;

    // Create entries over a 3-month period
    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

    // January entries
    let income1 = LedgerEntryBuilder::new()
        .name("Salary January")
        .currency(support::usd().to_string())
        .amount(dec!(5000.00))
        .entry_type(EntryType::Income)
        .date(start)
        .tag(Tag::new("salary")?)
        .build()?;
    ledger.add_entry(&income1)?;

    let expense1 = LedgerEntryBuilder::new()
        .name("Rent January")
        .currency(support::usd().to_string())
        .amount(dec!(1500.00))
        .entry_type(EntryType::Expense)
        .date(start + Duration::days(5))
        .tag(Tag::new("rent")?)
        .build()?;
    ledger.add_entry(&expense1)?;

    let expense2 = LedgerEntryBuilder::new()
        .name("Groceries January")
        .currency(support::usd().to_string())
        .amount(dec!(300.00))
        .entry_type(EntryType::Expense)
        .date(start + Duration::days(10))
        .tag(Tag::new("groceries")?)
        .build()?;
    ledger.add_entry(&expense2)?;

    // February entries
    let income2 = LedgerEntryBuilder::new()
        .name("Salary February")
        .currency(support::usd().to_string())
        .amount(dec!(5000.00))
        .entry_type(EntryType::Income)
        .date(start + Duration::days(31))
        .tag(Tag::new("salary")?)
        .build()?;
    ledger.add_entry(&income2)?;

    let expense3 = LedgerEntryBuilder::new()
        .name("Rent February")
        .currency(support::usd().to_string())
        .amount(dec!(1500.00))
        .entry_type(EntryType::Expense)
        .date(start + Duration::days(36))
        .tag(Tag::new("rent")?)
        .build()?;
    ledger.add_entry(&expense3)?;

    let expense4 = LedgerEntryBuilder::new()
        .name("Utilities February")
        .currency(support::usd().to_string())
        .amount(dec!(200.00))
        .entry_type(EntryType::Expense)
        .date(start + Duration::days(40))
        .tag(Tag::new("utilities")?)
        .build()?;
    ledger.add_entry(&expense4)?;

    // March entries
    let income3 = LedgerEntryBuilder::new()
        .name("Salary March")
        .currency(support::usd().to_string())
        .amount(dec!(5000.00))
        .entry_type(EntryType::Income)
        .date(start + Duration::days(60))
        .tag(Tag::new("salary")?)
        .build()?;
    ledger.add_entry(&income3)?;

    let expense5 = LedgerEntryBuilder::new()
        .name("Rent March")
        .currency(support::usd().to_string())
        .amount(dec!(1500.00))
        .entry_type(EntryType::Expense)
        .date(start + Duration::days(65))
        .tag(Tag::new("rent")?)
        .build()?;
    ledger.add_entry(&expense5)?;

    Ok(ledger)
}

#[tokio::test]
async fn test_income_expense_report_monthly() -> BeansResult<()> {
    let ledger = create_test_ledger_with_entries().await?;
    let generator = ReportGenerator::new(&ledger);

    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 3, 31, 23, 59, 59).unwrap();

    let report = generator
        .income_expense_report(start, end, TimePeriod::Monthly, None, None)
        .await?;

    // Verify summary
    assert_eq!(report.summary.income, dec!(15000.00)); // 5000 * 3 months
    assert_eq!(report.summary.expenses, dec!(5000.00)); // 1500 + 300 + 1500 + 200 + 1500
    assert_eq!(report.summary.net, dec!(10000.00)); // 15000 - 5000

    // Verify income series has 3 data points (one per month)
    assert_eq!(report.income_series.points.len(), 3);
    assert_eq!(report.income_series.points[0].value, dec!(5000.00));
    assert_eq!(report.income_series.points[1].value, dec!(5000.00));
    assert_eq!(report.income_series.points[2].value, dec!(5000.00));

    // Verify expense series
    assert_eq!(report.expense_series.points.len(), 3);
    assert_eq!(report.expense_series.points[0].value, dec!(1800.00)); // 1500 + 300
    assert_eq!(report.expense_series.points[1].value, dec!(1700.00)); // 1500 + 200
    assert_eq!(report.expense_series.points[2].value, dec!(1500.00)); // 1500

    Ok(())
}

#[tokio::test]
async fn test_income_expense_report_daily() -> BeansResult<()> {
    let ledger = LedgerManager::in_memory()?;

    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

    // Add entries on specific days
    let income1 = LedgerEntryBuilder::new()
        .name("Income Day 1")
        .currency(support::usd().to_string())
        .amount(dec!(100.00))
        .entry_type(EntryType::Income)
        .date(start)
        .build()?;
    ledger.add_entry(&income1)?;

    let expense1 = LedgerEntryBuilder::new()
        .name("Expense Day 2")
        .currency(support::usd().to_string())
        .amount(dec!(50.00))
        .entry_type(EntryType::Expense)
        .date(start + Duration::days(1))
        .build()?;
    ledger.add_entry(&expense1)?;

    let generator = ReportGenerator::new(&ledger);
    let end = Utc.with_ymd_and_hms(2024, 1, 3, 23, 59, 59).unwrap();

    let report = generator
        .income_expense_report(start, end, TimePeriod::Daily, None, None)
        .await?;

    // Verify we have daily data points
    assert!(report.income_series.points.len() >= 3);
    assert!(report.expense_series.points.len() >= 3);

    // Verify summary
    assert_eq!(report.summary.income, dec!(100.00));
    assert_eq!(report.summary.expenses, dec!(50.00));
    assert_eq!(report.summary.net, dec!(50.00));

    Ok(())
}

#[tokio::test]
async fn test_income_expense_report_weekly() -> BeansResult<()> {
    let ledger = LedgerManager::in_memory()?;

    // Start on a Monday
    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap(); // Jan 1, 2024 is a Monday

    // Week 1
    let income1 = LedgerEntryBuilder::new()
        .name("Income Week 1")
        .currency(support::usd().to_string())
        .amount(dec!(500.00))
        .entry_type(EntryType::Income)
        .date(start)
        .build()?;
    ledger.add_entry(&income1)?;

    // Week 2
    let income2 = LedgerEntryBuilder::new()
        .name("Income Week 2")
        .currency(support::usd().to_string())
        .amount(dec!(600.00))
        .entry_type(EntryType::Income)
        .date(start + Duration::days(7))
        .build()?;
    ledger.add_entry(&income2)?;

    let generator = ReportGenerator::new(&ledger);
    let end = Utc.with_ymd_and_hms(2024, 1, 14, 23, 59, 59).unwrap();

    let report = generator
        .income_expense_report(start, end, TimePeriod::Weekly, None, None)
        .await?;

    // Verify we have weekly data
    assert!(report.income_series.points.len() >= 2);
    assert_eq!(report.summary.income, dec!(1100.00));

    Ok(())
}

#[tokio::test]
async fn test_income_expense_report_yearly() -> BeansResult<()> {
    let ledger = LedgerManager::in_memory()?;

    let start = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();

    // 2023 entries
    let income1 = LedgerEntryBuilder::new()
        .name("Income 2023")
        .currency(support::usd().to_string())
        .amount(dec!(50000.00))
        .entry_type(EntryType::Income)
        .date(start)
        .build()?;
    ledger.add_entry(&income1)?;

    // 2024 entries
    let income2 = LedgerEntryBuilder::new()
        .name("Income 2024")
        .currency(support::usd().to_string())
        .amount(dec!(60000.00))
        .entry_type(EntryType::Income)
        .date(start + Duration::days(365))
        .build()?;
    ledger.add_entry(&income2)?;

    let generator = ReportGenerator::new(&ledger);
    let end = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();

    let report = generator
        .income_expense_report(start, end, TimePeriod::Yearly, None, None)
        .await?;

    // Verify yearly aggregation
    assert_eq!(report.income_series.points.len(), 2);
    assert_eq!(report.summary.income, dec!(110000.00));

    Ok(())
}

#[tokio::test]
async fn test_period_summary() -> BeansResult<()> {
    let ledger = create_test_ledger_with_entries().await?;
    let generator = ReportGenerator::new(&ledger);

    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 3, 31, 23, 59, 59).unwrap();

    let summary = generator.period_summary(start, end, None, None).await?;

    assert_eq!(summary.income, dec!(15000.00));
    assert_eq!(summary.expenses, dec!(5000.00));
    assert_eq!(summary.net, dec!(10000.00));

    Ok(())
}

#[tokio::test]
async fn test_period_summary_with_tag_filter() -> BeansResult<()> {
    let ledger = create_test_ledger_with_entries().await?;
    let generator = ReportGenerator::new(&ledger);

    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 3, 31, 23, 59, 59).unwrap();

    // Filter by salary tag
    let summary = generator
        .period_summary(start, end, None, Some(vec!["salary".to_string()]))
        .await?;

    assert_eq!(summary.income, dec!(15000.00)); // 3 salary payments
    assert_eq!(summary.expenses, dec!(0.00)); // No expenses with salary tag
    assert_eq!(summary.net, dec!(15000.00));

    Ok(())
}

#[tokio::test]
async fn test_tagged_report() -> BeansResult<()> {
    let ledger = create_test_ledger_with_entries().await?;
    let generator = ReportGenerator::new(&ledger);

    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 3, 31, 23, 59, 59).unwrap();

    let report = generator.tagged_report(start, end, None).await?;

    // Verify income by tag
    assert_eq!(
        *report.income_by_tag.get("salary").unwrap(),
        dec!(15000.00)
    );

    // Verify expenses by tag
    assert_eq!(*report.expenses_by_tag.get("rent").unwrap(), dec!(4500.00));
    assert_eq!(
        *report.expenses_by_tag.get("groceries").unwrap(),
        dec!(300.00)
    );
    assert_eq!(
        *report.expenses_by_tag.get("utilities").unwrap(),
        dec!(200.00)
    );

    // Verify net by tag
    assert_eq!(*report.net_by_tag.get("salary").unwrap(), dec!(15000.00));
    assert_eq!(*report.net_by_tag.get("rent").unwrap(), dec!(-4500.00));

    // Verify overall summary
    assert_eq!(report.summary.income, dec!(15000.00));
    assert_eq!(report.summary.expenses, dec!(5000.00));
    assert_eq!(report.summary.net, dec!(10000.00));

    Ok(())
}

#[tokio::test]
async fn test_tagged_report_with_untagged_entries() -> BeansResult<()> {
    let ledger = LedgerManager::in_memory()?;

    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

    // Entry without tags
    let income1 = LedgerEntryBuilder::new()
        .name("Untagged Income")
        .currency(support::usd().to_string())
        .amount(dec!(1000.00))
        .entry_type(EntryType::Income)
        .date(start)
        .build()?;
    ledger.add_entry(&income1)?;

    let generator = ReportGenerator::new(&ledger);
    let end = Utc.with_ymd_and_hms(2024, 1, 31, 23, 59, 59).unwrap();

    let report = generator.tagged_report(start, end, None).await?;

    // Verify untagged entries are grouped under "Untagged"
    assert_eq!(
        *report.income_by_tag.get("Untagged").unwrap(),
        dec!(1000.00)
    );

    Ok(())
}

#[tokio::test]
async fn test_export_income_expense_report_json() -> BeansResult<()> {
    let ledger = create_test_ledger_with_entries().await?;
    let generator = ReportGenerator::new(&ledger);

    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 3, 31, 23, 59, 59).unwrap();

    let report = generator
        .income_expense_report(start, end, TimePeriod::Monthly, None, None)
        .await?;

    let json = generator.export_income_expense_report(&report, ExportFormat::Json)?;

    // Verify it's valid JSON
    assert!(json.contains("income_series"));
    assert!(json.contains("expense_series"));
    assert!(json.contains("summary"));

    // Verify it can be parsed back
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(parsed["summary"]["income"].is_number());

    Ok(())
}

#[tokio::test]
async fn test_export_income_expense_report_csv() -> BeansResult<()> {
    let ledger = create_test_ledger_with_entries().await?;
    let generator = ReportGenerator::new(&ledger);

    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 3, 31, 23, 59, 59).unwrap();

    let report = generator
        .income_expense_report(start, end, TimePeriod::Monthly, None, None)
        .await?;

    let csv = generator.export_income_expense_report(&report, ExportFormat::Csv)?;

    // Verify CSV format
    assert!(csv.contains("Timestamp,Income,Expenses"));
    assert!(csv.contains("Summary"));
    assert!(csv.contains("Total Income"));
    assert!(csv.contains("Total Expenses"));
    assert!(csv.contains("Net"));

    // Verify data is present
    assert!(csv.contains("15000"));
    assert!(csv.contains("5000"));

    Ok(())
}

#[tokio::test]
async fn test_export_tagged_report_json() -> BeansResult<()> {
    let ledger = create_test_ledger_with_entries().await?;
    let generator = ReportGenerator::new(&ledger);

    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 3, 31, 23, 59, 59).unwrap();

    let report = generator.tagged_report(start, end, None).await?;

    let json = generator.export_tagged_report(&report, ExportFormat::Json)?;

    // Verify it's valid JSON
    assert!(json.contains("income_by_tag"));
    assert!(json.contains("expenses_by_tag"));
    assert!(json.contains("net_by_tag"));
    assert!(json.contains("summary"));

    Ok(())
}

#[tokio::test]
async fn test_export_tagged_report_csv() -> BeansResult<()> {
    let ledger = create_test_ledger_with_entries().await?;
    let generator = ReportGenerator::new(&ledger);

    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 3, 31, 23, 59, 59).unwrap();

    let report = generator.tagged_report(start, end, None).await?;

    let csv = generator.export_tagged_report(&report, ExportFormat::Csv)?;

    // Verify CSV format
    assert!(csv.contains("Tag,Income,Expenses,Net"));
    assert!(csv.contains("Summary"));
    assert!(csv.contains("salary"));
    assert!(csv.contains("rent"));

    Ok(())
}

#[tokio::test]
async fn test_invalid_date_range() -> BeansResult<()> {
    let ledger = LedgerManager::in_memory()?;
    let generator = ReportGenerator::new(&ledger);

    let start = Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

    // Should fail because start > end
    let result = generator
        .income_expense_report(start, end, TimePeriod::Monthly, None, None)
        .await;

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        beans_lib::error::BeansError::InvalidDateRange
    ));

    Ok(())
}

#[tokio::test]
async fn test_report_with_empty_ledger() -> BeansResult<()> {
    let ledger = LedgerManager::in_memory()?;
    let generator = ReportGenerator::new(&ledger);

    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 3, 31, 23, 59, 59).unwrap();

    let report = generator
        .income_expense_report(start, end, TimePeriod::Monthly, None, None)
        .await?;

    // Empty ledger should return zero values
    assert_eq!(report.summary.income, dec!(0.00));
    assert_eq!(report.summary.expenses, dec!(0.00));
    assert_eq!(report.summary.net, dec!(0.00));

    // Should still have time buckets (just with zero values)
    assert_eq!(report.income_series.points.len(), 3);
    assert_eq!(report.expense_series.points.len(), 3);

    Ok(())
}

#[tokio::test]
async fn test_timezone_handling() -> BeansResult<()> {
    let ledger = LedgerManager::in_memory()?;

    // Entry at 11 PM UTC on Jan 31
    let date_utc = Utc.with_ymd_and_hms(2024, 1, 31, 23, 0, 0).unwrap();

    let income = LedgerEntryBuilder::new()
        .name("Late Night Income")
        .currency(support::usd().to_string())
        .amount(dec!(100.00))
        .entry_type(EntryType::Income)
        .date(date_utc)
        .build()?;
    ledger.add_entry(&income)?;

    let generator = ReportGenerator::new(&ledger);
    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2024, 2, 29, 23, 59, 59).unwrap();

    let report = generator
        .income_expense_report(start, end, TimePeriod::Monthly, None, None)
        .await?;

    // Income should be in January bucket (UTC)
    assert_eq!(report.income_series.points[0].value, dec!(100.00));
    assert_eq!(report.income_series.points[1].value, dec!(0.00)); // February should be empty

    Ok(())
}

#[tokio::test]
async fn test_multiple_tags_per_entry() -> BeansResult<()> {
    let ledger = LedgerManager::in_memory()?;

    let start = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

    // Entry with multiple tags
    let income = LedgerEntryBuilder::new()
        .name("Freelance Work")
        .currency(support::usd().to_string())
        .amount(dec!(1000.00))
        .entry_type(EntryType::Income)
        .date(start)
        .tag(Tag::new("freelance")?)
        .tag(Tag::new("income")?)
        .tag(Tag::new("project-a")?)
        .build()?;
    ledger.add_entry(&income)?;

    let generator = ReportGenerator::new(&ledger);
    let end = Utc.with_ymd_and_hms(2024, 1, 31, 23, 59, 59).unwrap();

    let report = generator.tagged_report(start, end, None).await?;

    // Entry should appear in all tag groups
    assert_eq!(
        *report.income_by_tag.get("freelance").unwrap(),
        dec!(1000.00)
    );
    assert_eq!(*report.income_by_tag.get("income").unwrap(), dec!(1000.00));
    assert_eq!(
        *report.income_by_tag.get("project-a").unwrap(),
        dec!(1000.00)
    );

    // Total should still be 1000 (entry counted once in summary)
    assert_eq!(report.summary.income, dec!(1000.00));

    Ok(())
}

