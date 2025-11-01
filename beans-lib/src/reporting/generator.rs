//! Report generation for ledger data.

use crate::currency::CurrencyConverter;
use crate::database::EntryFilter;
use crate::error::{BeansError, BeansResult};
use crate::ledger::LedgerManager;
use crate::models::{Currency, EntryType};
use crate::reporting::types::{
    ExportFormat, IncomeExpenseReport, PeriodSummary, TaggedReport, TimePeriod, TimeSeriesData,
    TimeSeriesPoint,
};
use chrono::{DateTime, Datelike, Duration, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Generates reports from ledger data.
#[derive(Debug, Clone)]
pub struct ReportGenerator<'a> {
    ledger: &'a LedgerManager,
    converter: Option<CurrencyConverter>,
}

impl<'a> ReportGenerator<'a> {
    /// Creates a new report generator for the given ledger.
    pub fn new(ledger: &'a LedgerManager) -> Self {
        Self {
            ledger,
            converter: None,
        }
    }

    /// Sets a currency converter for multi-currency reports.
    pub fn with_converter(mut self, converter: CurrencyConverter) -> Self {
        self.converter = Some(converter);
        self
    }

    /// Generates an income vs expense report for the given period.
    pub async fn income_expense_report(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        period: TimePeriod,
        target_currency: Option<Currency<'_>>,
        tags: Option<Vec<String>>,
    ) -> BeansResult<IncomeExpenseReport> {
        // Validate date range
        if start_date > end_date {
            return Err(BeansError::InvalidDateRange);
        }

        // Create filters for income and expenses
        let income_filter = EntryFilter {
            start_date: Some(start_date),
            end_date: Some(end_date),
            entry_type: Some(EntryType::Income),
            tags: tags.clone().unwrap_or_default(),
            ..Default::default()
        };

        let expense_filter = EntryFilter {
            start_date: Some(start_date),
            end_date: Some(end_date),
            entry_type: Some(EntryType::Expense),
            tags: tags.unwrap_or_default(),
            ..Default::default()
        };

        // Get all entries
        let income_entries = self.ledger.list_entries(&income_filter)?;
        let expense_entries = self.ledger.list_entries(&expense_filter)?;

        // Generate time series data
        let income_series = self
            .generate_time_series(
                "Income",
                &income_entries,
                start_date,
                end_date,
                period,
                target_currency.as_ref(),
            )
            .await?;

        let expense_series = self
            .generate_time_series(
                "Expenses",
                &expense_entries,
                start_date,
                end_date,
                period,
                target_currency.as_ref(),
            )
            .await?;

        // Calculate overall summary
        let total_income: Decimal = income_series.points.iter().map(|p| p.value).sum();
        let total_expenses: Decimal = expense_series.points.iter().map(|p| p.value).sum();

        let summary = PeriodSummary {
            income: total_income,
            expenses: total_expenses,
            net: total_income - total_expenses,
        };

        Ok(IncomeExpenseReport {
            income_series,
            expense_series,
            summary,
        })
    }

    /// Calculates a summary for the given period.
    pub async fn period_summary(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        target_currency: Option<Currency<'_>>,
        tags: Option<Vec<String>>,
    ) -> BeansResult<PeriodSummary> {
        // Validate date range
        if start_date > end_date {
            return Err(BeansError::InvalidDateRange);
        }

        // Create filters
        let filter = EntryFilter {
            start_date: Some(start_date),
            end_date: Some(end_date),
            tags: tags.unwrap_or_default(),
            ..Default::default()
        };

        // Get all entries
        let entries = self.ledger.list_entries(&filter)?;

        // Calculate totals with currency conversion if needed
        let mut total_income = Decimal::ZERO;
        let mut total_expenses = Decimal::ZERO;

        for entry in entries {
            let amount = if let Some(ref target_curr) = target_currency {
                self.convert_amount(&entry.currency()?, target_curr).await?
            } else {
                entry.amount()
            };

            match entry.entry_type() {
                EntryType::Income => total_income += amount,
                EntryType::Expense => total_expenses += amount,
            }
        }

        Ok(PeriodSummary {
            income: total_income,
            expenses: total_expenses,
            net: total_income - total_expenses,
        })
    }

    /// Generates a report grouped by tags.
    pub async fn tagged_report(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        target_currency: Option<Currency<'_>>,
    ) -> BeansResult<TaggedReport> {
        // Validate date range
        if start_date > end_date {
            return Err(BeansError::InvalidDateRange);
        }

        // Create filter
        let filter = EntryFilter {
            start_date: Some(start_date),
            end_date: Some(end_date),
            ..Default::default()
        };

        // Get all entries
        let entries = self.ledger.list_entries(&filter)?;

        // Group by tags
        let mut income_by_tag: HashMap<String, Decimal> = HashMap::new();
        let mut expenses_by_tag: HashMap<String, Decimal> = HashMap::new();
        let mut total_income = Decimal::ZERO;
        let mut total_expenses = Decimal::ZERO;

        for entry in entries {
            let amount = if let Some(ref target_curr) = target_currency {
                self.convert_amount(&entry.currency()?, target_curr).await?
            } else {
                entry.amount()
            };

            match entry.entry_type() {
                EntryType::Income => total_income += amount,
                EntryType::Expense => total_expenses += amount,
            }

            // If entry has no tags, use "Untagged"
            let tags: Vec<String> = if entry.tags().is_empty() {
                vec!["Untagged".to_string()]
            } else {
                entry.tags().iter().map(|t| t.name().to_string()).collect()
            };

            for tag in tags {
                match entry.entry_type() {
                    EntryType::Income => {
                        *income_by_tag.entry(tag.clone()).or_insert(Decimal::ZERO) += amount;
                    }
                    EntryType::Expense => {
                        *expenses_by_tag.entry(tag.clone()).or_insert(Decimal::ZERO) += amount;
                    }
                }
            }
        }

        // Calculate net by tag
        let mut net_by_tag: HashMap<String, Decimal> = HashMap::new();
        let all_tags: std::collections::HashSet<String> = income_by_tag
            .keys()
            .chain(expenses_by_tag.keys())
            .cloned()
            .collect();

        for tag in all_tags {
            let income = income_by_tag.get(&tag).copied().unwrap_or(Decimal::ZERO);
            let expenses = expenses_by_tag.get(&tag).copied().unwrap_or(Decimal::ZERO);
            net_by_tag.insert(tag, income - expenses);
        }

        Ok(TaggedReport {
            income_by_tag,
            expenses_by_tag,
            net_by_tag,
            summary: PeriodSummary {
                income: total_income,
                expenses: total_expenses,
                net: total_income - total_expenses,
            },
        })
    }

    /// Exports an income/expense report to the specified format.
    pub fn export_income_expense_report(
        &self,
        report: &IncomeExpenseReport,
        format: ExportFormat,
    ) -> BeansResult<String> {
        match format {
            ExportFormat::Json => self.export_to_json(report),
            ExportFormat::Csv => self.export_income_expense_to_csv(report),
        }
    }

    /// Exports a tagged report to the specified format.
    pub fn export_tagged_report(
        &self,
        report: &TaggedReport,
        format: ExportFormat,
    ) -> BeansResult<String> {
        match format {
            ExportFormat::Json => self.export_to_json(report),
            ExportFormat::Csv => self.export_tagged_to_csv(report),
        }
    }

    // Private helper methods

    /// Generates time series data from entries.
    async fn generate_time_series(
        &self,
        name: &str,
        entries: &[crate::models::LedgerEntry],
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        period: TimePeriod,
        target_currency: Option<&Currency<'_>>,
    ) -> BeansResult<TimeSeriesData> {
        // Generate all time buckets
        let buckets = self.generate_time_buckets(start_date, end_date, period);

        // Aggregate entries into buckets
        let mut bucket_values: HashMap<DateTime<Utc>, Decimal> = HashMap::new();

        for entry in entries {
            let bucket = self.get_bucket_for_date(entry.date(), period);
            let amount = if let Some(target_curr) = target_currency {
                self.convert_amount(&entry.currency()?, target_curr).await?
            } else {
                entry.amount()
            };

            *bucket_values.entry(bucket).or_insert(Decimal::ZERO) += amount;
        }

        // Create time series points
        let mut points: Vec<TimeSeriesPoint> = buckets
            .into_iter()
            .map(|timestamp| TimeSeriesPoint {
                timestamp,
                value: bucket_values
                    .get(&timestamp)
                    .copied()
                    .unwrap_or(Decimal::ZERO),
            })
            .collect();

        // Sort by timestamp
        points.sort_by_key(|p| p.timestamp);

        Ok(TimeSeriesData {
            name: name.to_string(),
            points,
        })
    }

    /// Generates time buckets for the given period.
    fn generate_time_buckets(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        period: TimePeriod,
    ) -> Vec<DateTime<Utc>> {
        let mut buckets = Vec::new();
        let mut current = self.get_bucket_for_date(start_date, period);
        let end_bucket = self.get_bucket_for_date(end_date, period);

        while current <= end_bucket {
            buckets.push(current);
            current = self.next_bucket(current, period);
        }

        buckets
    }

    /// Gets the bucket (normalized timestamp) for a given date.
    fn get_bucket_for_date(&self, date: DateTime<Utc>, period: TimePeriod) -> DateTime<Utc> {
        match period {
            TimePeriod::Daily => {
                // Start of day
                date.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc()
            }
            TimePeriod::Weekly => {
                // Start of week (Monday)
                let days_from_monday = date.weekday().num_days_from_monday();
                let start_of_week = date
                    .date_naive()
                    .checked_sub_signed(Duration::days(days_from_monday as i64))
                    .unwrap();
                start_of_week.and_hms_opt(0, 0, 0).unwrap().and_utc()
            }
            TimePeriod::Monthly => {
                // Start of month
                date.date_naive()
                    .with_day(1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
            }
            TimePeriod::Yearly => {
                // Start of year
                date.date_naive()
                    .with_month(1)
                    .and_then(|d| d.with_day(1))
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
            }
        }
    }

    /// Gets the next bucket after the current one.
    fn next_bucket(&self, current: DateTime<Utc>, period: TimePeriod) -> DateTime<Utc> {
        match period {
            TimePeriod::Daily => current + Duration::days(1),
            TimePeriod::Weekly => current + Duration::weeks(1),
            TimePeriod::Monthly => {
                // Add one month
                let month = current.month();
                let year = current.year();
                let (next_month, next_year) = if month == 12 {
                    (1, year + 1)
                } else {
                    (month + 1, year)
                };
                current
                    .date_naive()
                    .with_year(next_year)
                    .and_then(|d| d.with_month(next_month))
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
            }
            TimePeriod::Yearly => {
                // Add one year
                current
                    .date_naive()
                    .with_year(current.year() + 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_utc()
            }
        }
    }

    /// Converts an amount from one currency to another.
    async fn convert_amount(
        &self,
        from_currency: &Currency<'_>,
        to_currency: &Currency<'_>,
    ) -> BeansResult<Decimal> {
        // If currencies are the same, no conversion needed
        if from_currency.code() == to_currency.code() {
            return Ok(*from_currency.amount());
        }

        // Use converter if available
        if let Some(ref converter) = self.converter {
            let converted = converter.convert_amount(from_currency, to_currency).await?;
            Ok(*converted.amount())
        } else {
            // No converter available
            Err(BeansError::currency(format!(
                "Currency converter not available for conversion from {} to {}",
                from_currency.code(),
                to_currency.code()
            )))
        }
    }

    /// Exports data to JSON format.
    fn export_to_json<T: serde::Serialize>(&self, data: &T) -> BeansResult<String> {
        serde_json::to_string_pretty(data)
            .map_err(|e| BeansError::Json(format!("Failed to serialize to JSON: {}", e)))
    }

    /// Exports income/expense report to CSV format.
    fn export_income_expense_to_csv(&self, report: &IncomeExpenseReport) -> BeansResult<String> {
        let mut csv = String::from("Timestamp,Income,Expenses\n");

        // Combine both series by timestamp
        let mut all_timestamps: Vec<DateTime<Utc>> = report
            .income_series
            .points
            .iter()
            .chain(report.expense_series.points.iter())
            .map(|p| p.timestamp)
            .collect();
        all_timestamps.sort();
        all_timestamps.dedup();

        for timestamp in all_timestamps {
            let income = report
                .income_series
                .points
                .iter()
                .find(|p| p.timestamp == timestamp)
                .map(|p| p.value)
                .unwrap_or(Decimal::ZERO);

            let expenses = report
                .expense_series
                .points
                .iter()
                .find(|p| p.timestamp == timestamp)
                .map(|p| p.value)
                .unwrap_or(Decimal::ZERO);

            csv.push_str(&format!(
                "{},{},{}\n",
                timestamp.to_rfc3339(),
                income,
                expenses
            ));
        }

        // Add summary
        csv.push_str(&format!("\nSummary\n"));
        csv.push_str(&format!("Total Income,{}\n", report.summary.income));
        csv.push_str(&format!("Total Expenses,{}\n", report.summary.expenses));
        csv.push_str(&format!("Net,{}\n", report.summary.net));

        Ok(csv)
    }

    /// Exports tagged report to CSV format.
    fn export_tagged_to_csv(&self, report: &TaggedReport) -> BeansResult<String> {
        let mut csv = String::from("Tag,Income,Expenses,Net\n");

        // Get all tags
        let mut all_tags: Vec<String> = report
            .income_by_tag
            .keys()
            .chain(report.expenses_by_tag.keys())
            .cloned()
            .collect();
        all_tags.sort();
        all_tags.dedup();

        for tag in all_tags {
            let income = report
                .income_by_tag
                .get(&tag)
                .copied()
                .unwrap_or(Decimal::ZERO);
            let expenses = report
                .expenses_by_tag
                .get(&tag)
                .copied()
                .unwrap_or(Decimal::ZERO);
            let net = report
                .net_by_tag
                .get(&tag)
                .copied()
                .unwrap_or(Decimal::ZERO);

            csv.push_str(&format!("{},{},{},{}\n", tag, income, expenses, net));
        }

        // Add summary
        csv.push_str(&format!("\nSummary\n"));
        csv.push_str(&format!("Total Income,{}\n", report.summary.income));
        csv.push_str(&format!("Total Expenses,{}\n", report.summary.expenses));
        csv.push_str(&format!("Net,{}\n", report.summary.net));

        Ok(csv)
    }
}
