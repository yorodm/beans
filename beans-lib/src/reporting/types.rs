//! Types for reporting and analytics.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Time period granularity for reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimePeriod {
    /// Daily granularity.
    Daily,
    /// Weekly granularity.
    Weekly,
    /// Monthly granularity.
    Monthly,
    /// Quarterly granularity.
    Quarterly,
    /// Yearly granularity.
    Yearly,
}

impl TimePeriod {
    /// Returns a string representation of the time period.
    pub fn as_str(&self) -> &'static str {
        match self {
            TimePeriod::Daily => "daily",
            TimePeriod::Weekly => "weekly",
            TimePeriod::Monthly => "monthly",
            TimePeriod::Quarterly => "quarterly",
            TimePeriod::Yearly => "yearly",
        }
    }
    
    /// Returns the format string for formatting dates according to this period.
    pub fn date_format(&self) -> &'static str {
        match self {
            TimePeriod::Daily => "%Y-%m-%d",
            TimePeriod::Weekly => "%Y-W%W",
            TimePeriod::Monthly => "%Y-%m",
            TimePeriod::Quarterly => "%Y-Q%q",
            TimePeriod::Yearly => "%Y",
        }
    }
}

/// A single data point in a time series.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    /// Timestamp for the data point.
    pub timestamp: DateTime<Utc>,
    /// Value for the data point.
    pub value: Decimal,
    /// Label for the data point (e.g., "2023-01" for a monthly point).
    pub label: String,
}

/// A named series of data points.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesData {
    /// Name of the series.
    pub name: String,
    /// Data points in the series.
    pub points: Vec<TimeSeriesPoint>,
}

/// Summary of income and expenses for a period.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeriodSummary {
    /// Total income for the period.
    pub income: Decimal,
    /// Total expenses for the period.
    pub expenses: Decimal,
    /// Net income (income - expenses) for the period.
    pub net: Decimal,
    /// Currency code for the summary amounts.
    pub currency: String,
}

/// Income and expense report with time series data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncomeExpenseReport {
    /// Income time series.
    pub income_series: TimeSeriesData,
    /// Expense time series.
    pub expense_series: TimeSeriesData,
    /// Overall summary for the entire period.
    pub summary: PeriodSummary,
    /// Time period granularity used for the report.
    pub period: TimePeriod,
    /// Start date of the report period.
    pub start_date: DateTime<Utc>,
    /// End date of the report period.
    pub end_date: DateTime<Utc>,
    /// Tags used for filtering the report, if any.
    pub tags: Option<Vec<String>>,
    /// Currency used for the report.
    pub currency: String,
}

/// Grouping criteria for reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GroupBy {
    /// Group by time period.
    TimePeriod,
    /// Group by tag.
    Tag,
    /// Group by currency.
    Currency,
    /// Group by entry type.
    EntryType,
}

/// Tag-based summary for reporting.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagSummary {
    /// Tag name.
    pub tag: String,
    /// Total amount for the tag.
    pub amount: Decimal,
    /// Number of entries with this tag.
    pub count: usize,
    /// Percentage of total amount.
    pub percentage: f64,
}

/// Tag-based report with summaries by tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagReport {
    /// Summaries by tag.
    pub tag_summaries: Vec<TagSummary>,
    /// Overall summary for the entire period.
    pub summary: PeriodSummary,
    /// Start date of the report period.
    pub start_date: DateTime<Utc>,
    /// End date of the report period.
    pub end_date: DateTime<Utc>,
    /// Currency used for the report.
    pub currency: String,
}

/// Export format for reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExportFormat {
    /// CSV format.
    Csv,
    /// JSON format.
    Json,
}

impl ExportFormat {
    /// Returns the file extension for this format.
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Csv => "csv",
            ExportFormat::Json => "json",
        }
    }
    
    /// Returns the content type for this format.
    pub fn content_type(&self) -> &'static str {
        match self {
            ExportFormat::Csv => "text/csv",
            ExportFormat::Json => "application/json",
        }
    }
}
