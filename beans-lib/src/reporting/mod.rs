//! Reporting and analytics module.

mod generator;
mod types;

pub use generator::ReportGenerator;
pub use types::{
    ExportFormat, IncomeExpenseReport, PeriodSummary, TaggedReport, TimePeriod, TimeSeriesData,
    TimeSeriesPoint,
};
