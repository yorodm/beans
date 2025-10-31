//! Reporting and analytics module.

mod generator;
mod types;

pub use generator::ReportGenerator;
pub use types::{IncomeExpenseReport, PeriodSummary, TimeSeriesData, TimeSeriesPoint, TimePeriod};

