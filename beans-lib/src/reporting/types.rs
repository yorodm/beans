//! Types for reporting and analytics.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Time period granularity for reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimePeriod {
    /// Daily granularity.
    Daily,
    /// Weekly granularity.
    Weekly,
    /// Monthly granularity.
    Monthly,
    /// Yearly granularity.
    Yearly,
}

/// A single data point in a time series.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    /// Timestamp for the data point.
    pub timestamp: DateTime<Utc>,
    /// Value for the data point.
    pub value: Decimal,
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
}

