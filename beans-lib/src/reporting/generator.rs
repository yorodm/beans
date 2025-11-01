//! Report generation for ledger data.

use crate::currency::CurrencyConverter;
use crate::database::EntryFilter;
use crate::error::{BeansError, BeansResult};
use crate::ledger::LedgerManager;
use crate::models::{Currency, EntryType, LedgerEntry};
use crate::reporting::types::{
    ExportFormat, GroupBy, IncomeExpenseReport, PeriodSummary, TagReport, TagSummary, TimePeriod,
    TimeSeriesData, TimeSeriesPoint,
};
use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc, Weekday};
use rust_decimal::Decimal;
use serde_json;
use std::collections::{HashMap, HashSet};
use std::io;

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
        // Validate dates
        if start_date > end_date {
            return Err(BeansError::InvalidDateRange);
        }

        // Get entries for the period
        let entries = self.get_filtered_entries(start_date, end_date, tags.clone())?;
        
        // If no entries, return empty report
        if entries.is_empty() {
            let currency_code = match &target_currency {
                Some(c) => c.code().to_string(),
                None => "USD".to_string(), // Default currency if none specified
            };
            
            return Ok(IncomeExpenseReport {
                income_series: TimeSeriesData {
                    name: "Income".to_string(),
                    points: Vec::new(),
                },
                expense_series: TimeSeriesData {
                    name: "Expenses".to_string(),
                    points: Vec::new(),
                },
                summary: PeriodSummary {
                    income: Decimal::ZERO,
                    expenses: Decimal::ZERO,
                    net: Decimal::ZERO,
                    currency: currency_code.clone(),
                },
                period,
                start_date,
                end_date,
                tags,
                currency: currency_code,
            });
        }

        // Generate time series data
        let (income_series, expense_series, summary) = self
            .generate_time_series(entries, start_date, end_date, period, target_currency)
            .await?;

        Ok(IncomeExpenseReport {
            income_series,
            expense_series,
            summary,
            period,
            start_date,
            end_date,
            tags,
            currency: summary.currency.clone(),
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
        // Validate dates
        if start_date > end_date {
            return Err(BeansError::InvalidDateRange);
        }

        // Get entries for the period
        let entries = self.get_filtered_entries(start_date, end_date, tags)?;
        
        // Calculate summary
        self.calculate_summary(entries, target_currency).await
    }

    /// Generates a tag-based report for the given period.
    pub async fn tag_report(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        target_currency: Option<Currency<'_>>,
        entry_type: Option<EntryType>,
    ) -> BeansResult<TagReport> {
        // Validate dates
        if start_date > end_date {
            return Err(BeansError::InvalidDateRange);
        }

        // Get entries for the period
        let mut filter = EntryFilter::default();
        filter.start_date = Some(start_date);
        filter.end_date = Some(end_date);
        
        if let Some(et) = entry_type {
            filter.entry_type = Some(et);
        }
        
        let entries = self.ledger.list_entries(&filter)?;
        
        // If no entries, return empty report
        if entries.is_empty() {
            let currency_code = match &target_currency {
                Some(c) => c.code().to_string(),
                None => "USD".to_string(), // Default currency if none specified
            };
            
            return Ok(TagReport {
                tag_summaries: Vec::new(),
                summary: PeriodSummary {
                    income: Decimal::ZERO,
                    expenses: Decimal::ZERO,
                    net: Decimal::ZERO,
                    currency: currency_code.clone(),
                },
                start_date,
                end_date,
                currency: currency_code,
            });
        }
        
        // Calculate summary
        let summary = self.calculate_summary(entries.clone(), target_currency.clone()).await?;
        
        // Group by tags
        let mut tag_amounts: HashMap<String, (Decimal, usize)> = HashMap::new();
        let mut total_amount = Decimal::ZERO;
        
        for entry in &entries {
            let amount = match &target_currency {
                Some(target) => {
                    if let Some(converter) = &self.converter {
                        let entry_currency = entry.money()?;
                        converter.convert_amount(entry.amount(), &entry_currency, target).await?
                    } else {
                        // If no converter but currencies match, use the amount directly
                        if entry.currency() == target.code() {
                            entry.amount()
                        } else {
                            return Err(BeansError::ExchangeRateUnavailable {
                                from: entry.currency(),
                                to: target.code().to_string(),
                            });
                        }
                    }
                }
                None => entry.amount(),
            };
            
            total_amount += amount;
            
            // Add amount to each tag
            for tag in entry.tags() {
                let tag_name = tag.name().to_string();
                let entry = tag_amounts.entry(tag_name).or_insert((Decimal::ZERO, 0));
                entry.0 += amount;
                entry.1 += 1;
            }
        }
        
        // Create tag summaries
        let mut tag_summaries: Vec<TagSummary> = tag_amounts
            .into_iter()
            .map(|(tag, (amount, count))| {
                let percentage = if total_amount.is_zero() {
                    0.0
                } else {
                    (amount / total_amount * Decimal::from(100)).to_f64().unwrap_or(0.0)
                };
                
                TagSummary {
                    tag,
                    amount,
                    count,
                    percentage,
                }
            })
            .collect();
        
        // Sort by amount descending
        tag_summaries.sort_by(|a, b| b.amount.cmp(&a.amount));
        
        Ok(TagReport {
            tag_summaries,
            summary,
            start_date,
            end_date,
            currency: summary.currency,
        })
    }

    /// Exports a report to the specified format.
    pub async fn export_report<W: io::Write>(
        &self,
        report: &IncomeExpenseReport,
        format: ExportFormat,
        writer: &mut W,
    ) -> BeansResult<()> {
        match format {
            ExportFormat::Json => {
                serde_json::to_writer_pretty(writer, report)
                    .map_err(|e| BeansError::Other(format!("Failed to export to JSON: {}", e)))?;
            }
            ExportFormat::Csv => {
                // Write header
                writeln!(
                    writer,
                    "Period,Date,Income,Expenses,Net"
                ).map_err(|e| BeansError::Other(format!("Failed to write CSV header: {}", e)))?;
                
                // Combine income and expense data points by timestamp
                let mut combined_data: HashMap<DateTime<Utc>, (Decimal, Decimal)> = HashMap::new();
                
                for point in &report.income_series.points {
                    combined_data.insert(point.timestamp, (point.value, Decimal::ZERO));
                }
                
                for point in &report.expense_series.points {
                    if let Some((income, _)) = combined_data.get_mut(&point.timestamp) {
                        *income = *income;
                        combined_data.insert(point.timestamp, (*income, point.value));
                    } else {
                        combined_data.insert(point.timestamp, (Decimal::ZERO, point.value));
                    }
                }
                
                // Sort by timestamp
                let mut timestamps: Vec<DateTime<Utc>> = combined_data.keys().cloned().collect();
                timestamps.sort();
                
                // Write data rows
                for timestamp in timestamps {
                    let (income, expenses) = combined_data.get(&timestamp).unwrap();
                    let net = income - expenses;
                    
                    // Format date according to period
                    let date_str = timestamp.format(report.period.date_format()).to_string();
                    
                    writeln!(
                        writer,
                        "{},{},{},{},{}",
                        report.period.as_str(),
                        date_str,
                        income,
                        expenses,
                        net
                    ).map_err(|e| BeansError::Other(format!("Failed to write CSV row: {}", e)))?;
                }
            }
        }
        
        Ok(())
    }

    /// Exports a tag report to the specified format.
    pub async fn export_tag_report<W: io::Write>(
        &self,
        report: &TagReport,
        format: ExportFormat,
        writer: &mut W,
    ) -> BeansResult<()> {
        match format {
            ExportFormat::Json => {
                serde_json::to_writer_pretty(writer, report)
                    .map_err(|e| BeansError::Other(format!("Failed to export to JSON: {}", e)))?;
            }
            ExportFormat::Csv => {
                // Write header
                writeln!(
                    writer,
                    "Tag,Amount,Count,Percentage"
                ).map_err(|e| BeansError::Other(format!("Failed to write CSV header: {}", e)))?;
                
                // Write data rows
                for summary in &report.tag_summaries {
                    writeln!(
                        writer,
                        "{},{},{},{}",
                        summary.tag,
                        summary.amount,
                        summary.count,
                        summary.percentage
                    ).map_err(|e| BeansError::Other(format!("Failed to write CSV row: {}", e)))?;
                }
                
                // Write summary row
                writeln!(
                    writer,
                    "TOTAL,{},{},100.0",
                    report.summary.income + report.summary.expenses,
                    report.tag_summaries.iter().map(|s| s.count).sum::<usize>()
                ).map_err(|e| BeansError::Other(format!("Failed to write CSV summary: {}", e)))?;
            }
        }
        
        Ok(())
    }

    // Helper methods

    /// Gets entries filtered by date range and tags.
    fn get_filtered_entries(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        tags: Option<Vec<String>>,
    ) -> BeansResult<Vec<LedgerEntry>> {
        let mut filter = EntryFilter::default();
        filter.start_date = Some(start_date);
        filter.end_date = Some(end_date);
        
        if let Some(tag_list) = tags {
            filter.tags = tag_list;
        }
        
        self.ledger.list_entries(&filter)
    }

    /// Calculates a summary for the given entries.
    async fn calculate_summary(
        &self,
        entries: Vec<LedgerEntry>,
        target_currency: Option<Currency<'_>>,
    ) -> BeansResult<PeriodSummary> {
        let mut income = Decimal::ZERO;
        let mut expenses = Decimal::ZERO;
        
        let currency_code = match &target_currency {
            Some(c) => c.code().to_string(),
            None => {
                if !entries.is_empty() {
                    entries[0].currency()
                } else {
                    "USD".to_string() // Default currency if no entries
                }
            }
        };
        
        for entry in entries {
            let amount = match &target_currency {
                Some(target) => {
                    if let Some(converter) = &self.converter {
                        let entry_currency = entry.money()?;
                        converter.convert_amount(entry.amount(), &entry_currency, target).await?
                    } else {
                        // If no converter but currencies match, use the amount directly
                        if entry.currency() == target.code() {
                            entry.amount()
                        } else {
                            return Err(BeansError::ExchangeRateUnavailable {
                                from: entry.currency(),
                                to: target.code().to_string(),
                            });
                        }
                    }
                }
                None => entry.amount(),
            };
            
            match entry.entry_type() {
                EntryType::Income => income += amount,
                EntryType::Expense => expenses += amount,
            }
        }
        
        let net = income - expenses;
        
        Ok(PeriodSummary {
            income,
            expenses,
            net,
            currency: currency_code,
        })
    }

    /// Generates time series data for income and expenses.
    async fn generate_time_series(
        &self,
        entries: Vec<LedgerEntry>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        period: TimePeriod,
        target_currency: Option<Currency<'_>>,
    ) -> BeansResult<(TimeSeriesData, TimeSeriesData, PeriodSummary)> {
        // Group entries by period and type
        let mut income_by_period: HashMap<DateTime<Utc>, Decimal> = HashMap::new();
        let mut expense_by_period: HashMap<DateTime<Utc>, Decimal> = HashMap::new();
        
        let mut total_income = Decimal::ZERO;
        let mut total_expenses = Decimal::ZERO;
        
        let currency_code = match &target_currency {
            Some(c) => c.code().to_string(),
            None => {
                if !entries.is_empty() {
                    entries[0].currency()
                } else {
                    "USD".to_string() // Default currency if no entries
                }
            }
        };
        
        // Process each entry
        for entry in entries {
            // Convert amount if needed
            let amount = match &target_currency {
                Some(target) => {
                    if let Some(converter) = &self.converter {
                        let entry_currency = entry.money()?;
                        converter.convert_amount(entry.amount(), &entry_currency, target).await?
                    } else {
                        // If no converter but currencies match, use the amount directly
                        if entry.currency() == target.code() {
                            entry.amount()
                        } else {
                            return Err(BeansError::ExchangeRateUnavailable {
                                from: entry.currency(),
                                to: target.code().to_string(),
                            });
                        }
                    }
                }
                None => entry.amount(),
            };
            
            // Get period timestamp
            let period_timestamp = self.get_period_timestamp(entry.date(), period);
            
            // Add to appropriate map
            match entry.entry_type() {
                EntryType::Income => {
                    *income_by_period.entry(period_timestamp).or_insert(Decimal::ZERO) += amount;
                    total_income += amount;
                }
                EntryType::Expense => {
                    *expense_by_period.entry(period_timestamp).or_insert(Decimal::ZERO) += amount;
                    total_expenses += amount;
                }
            }
        }
        
        // Generate all period timestamps between start and end dates
        let period_timestamps = self.generate_period_timestamps(start_date, end_date, period);
        
        // Create time series points
        let mut income_points = Vec::new();
        let mut expense_points = Vec::new();
        
        for timestamp in period_timestamps {
            let income = *income_by_period.get(&timestamp).unwrap_or(&Decimal::ZERO);
            let expense = *expense_by_period.get(&timestamp).unwrap_or(&Decimal::ZERO);
            
            let label = timestamp.format(period.date_format()).to_string();
            
            income_points.push(TimeSeriesPoint {
                timestamp,
                value: income,
                label: label.clone(),
            });
            
            expense_points.push(TimeSeriesPoint {
                timestamp,
                value: expense,
                label,
            });
        }
        
        // Create time series data
        let income_series = TimeSeriesData {
            name: "Income".to_string(),
            points: income_points,
        };
        
        let expense_series = TimeSeriesData {
            name: "Expenses".to_string(),
            points: expense_points,
        };
        
        // Create summary
        let summary = PeriodSummary {
            income: total_income,
            expenses: total_expenses,
            net: total_income - total_expenses,
            currency: currency_code,
        };
        
        Ok((income_series, expense_series, summary))
    }

    /// Gets the timestamp for the start of the period containing the given date.
    fn get_period_timestamp(&self, date: DateTime<Utc>, period: TimePeriod) -> DateTime<Utc> {
        match period {
            TimePeriod::Daily => {
                // Start of day
                Utc.with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0).unwrap()
            }
            TimePeriod::Weekly => {
                // Start of week (Monday)
                let days_from_monday = date.weekday().num_days_from_monday();
                let start_of_week = date - Duration::days(days_from_monday as i64);
                Utc.with_ymd_and_hms(start_of_week.year(), start_of_week.month(), start_of_week.day(), 0, 0, 0).unwrap()
            }
            TimePeriod::Monthly => {
                // Start of month
                Utc.with_ymd_and_hms(date.year(), date.month(), 1, 0, 0, 0).unwrap()
            }
            TimePeriod::Quarterly => {
                // Start of quarter
                let quarter_month = ((date.month() - 1) / 3) * 3 + 1;
                Utc.with_ymd_and_hms(date.year(), quarter_month, 1, 0, 0, 0).unwrap()
            }
            TimePeriod::Yearly => {
                // Start of year
                Utc.with_ymd_and_hms(date.year(), 1, 1, 0, 0, 0).unwrap()
            }
        }
    }

    /// Generates all period timestamps between start and end dates.
    fn generate_period_timestamps(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        period: TimePeriod,
    ) -> Vec<DateTime<Utc>> {
        let mut timestamps = Vec::new();
        let mut current = self.get_period_timestamp(start_date, period);
        
        while current <= end_date {
            timestamps.push(current);
            
            // Move to next period
            current = match period {
                TimePeriod::Daily => current + Duration::days(1),
                TimePeriod::Weekly => current + Duration::days(7),
                TimePeriod::Monthly => {
                    let mut year = current.year();
                    let mut month = current.month() + 1;
                    
                    if month > 12 {
                        month = 1;
                        year += 1;
                    }
                    
                    Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap()
                }
                TimePeriod::Quarterly => {
                    let mut year = current.year();
                    let mut month = current.month() + 3;
                    
                    if month > 12 {
                        month = month - 12;
                        year += 1;
                    }
                    
                    Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap()
                }
                TimePeriod::Yearly => {
                    Utc.with_ymd_and_hms(current.year() + 1, 1, 1, 0, 0, 0).unwrap()
                }
            };
        }
        
        timestamps
    }
}
