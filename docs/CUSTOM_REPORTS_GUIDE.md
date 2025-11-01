# Custom Reports and Analytics Guide

This guide shows you how to create custom reports and analytics beyond the built-in reporting features.

## Built-in Reports

Beans provides these standard reports:

1. **Income/Expense Report** - Time-series comparison
2. **Tagged Report** - Spending by category
3. **Period Summary** - Totals for a time range

## Custom Report Patterns

### Pattern 1: Top Expenses

```rust
fn top_expenses(ledger: &LedgerManager, limit: usize) -> BeansResult<Vec<LedgerEntry>> {
    let entries = ledger.list_entries(&EntryFilter {
        entry_type: Some(EntryType::Expense),
        ..Default::default()
    })?;
    
    let mut sorted = entries;
    sorted.sort_by(|a, b| b.amount().cmp(&a.amount()));
    sorted.truncate(limit);
    
    Ok(sorted)
}
```

### Pattern 2: Category Comparison

```rust
async fn compare_categories(
    ledger: &LedgerManager,
    categories: Vec<&str>,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> BeansResult<HashMap<String, Decimal>> {
    let mut results = HashMap::new();
    
    for category in categories {
        let filter = EntryFilter {
            tags: vec![category.to_string()],
            entry_type: Some(EntryType::Expense),
            start_date: Some(start),
            end_date: Some(end),
            ..Default::default()
        };
        
        let entries = ledger.list_entries(&filter)?;
        let total: Decimal = entries.iter().map(|e| e.amount()).sum();
        results.insert(category.to_string(), total);
    }
    
    Ok(results)
}
```

### Pattern 3: Monthly Trends

```rust
async fn monthly_trends(ledger: &LedgerManager, months: usize) -> BeansResult<Vec<PeriodSummary>> {
    let now = Utc::now();
    let generator = ReportGenerator::new(ledger);
    let mut trends = Vec::new();
    
    for month in 0..months {
        let end = now - Duration::days(30 * month as i64);
        let start = end - Duration::days(30);
        
        let summary = generator
            .period_summary(start, end, None, None)
            .await?;
        
        trends.push(summary);
    }
    
    Ok(trends)
}
```

### Pattern 4: Spending Velocity

```rust
fn calculate_daily_average(
    ledger: &LedgerManager,
    category: Option<&str>,
) -> BeansResult<Decimal> {
    let mut filter = EntryFilter {
        entry_type: Some(EntryType::Expense),
        ..Default::default()
    };
    
    if let Some(cat) = category {
        filter.tags = vec![cat.to_string()];
    }
    
    let entries = ledger.list_entries(&filter)?;
    
    if entries.is_empty() {
        return Ok(dec!(0));
    }
    
    // Find date range
    let dates: Vec<_> = entries.iter().map(|e| e.date()).collect();
    let min_date = dates.iter().min().unwrap();
    let max_date = dates.iter().max().unwrap();
    let days = (max_date.signed_duration_since(*min_date).num_days() + 1) as i64;
    
    let total: Decimal = entries.iter().map(|e| e.amount()).sum();
    let daily_avg = total / Decimal::from(days);
    
    Ok(daily_avg)
}
```

### Pattern 5: Savings Rate Over Time

```rust
async fn savings_rate_trend(
    ledger: &LedgerManager,
    periods: usize,
) -> BeansResult<Vec<(String, Decimal)>> {
    let now = Utc::now();
    let generator = ReportGenerator::new(ledger);
    let mut rates = Vec::new();
    
    for period in 0..periods {
        let end = now - Duration::days(30 * period as i64);
        let start = end - Duration::days(30);
        
        let summary = generator.period_summary(start, end, None, None).await?;
        
        let rate = if summary.income > dec!(0) {
            (summary.net / summary.income) * dec!(100)
        } else {
            dec!(0)
        };
        
        let label = format!("{}", start.format("%Y-%m"));
        rates.push((label, rate));
    }
    
    Ok(rates)
}
```

## Export and Visualization

### CSV Export

```rust
use std::fs::File;
use std::io::Write;

fn export_to_csv(entries: &[LedgerEntry], path: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    
    // Header
    writeln!(file, "Date,Name,Amount,Currency,Type,Description,Tags")?;
    
    // Data
    for entry in entries {
        let tags = entry.tags()
            .iter()
            .map(|t| t.name())
            .collect::<Vec<_>>()
            .join(";");
        
        writeln!(file, "{},{},{},{},{},{},{}",
            entry.date().format("%Y-%m-%d"),
            entry.name(),
            entry.amount(),
            entry.currency_code(),
            entry.entry_type(),
            entry.description().unwrap_or(""),
            tags
        )?;
    }
    
    Ok(())
}
```

### JSON Export

```rust
use serde_json;

fn export_report_to_json(
    report: &IncomeExpenseReport,
    path: &str,
) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(report)?;
    std::fs::write(path, json)?;
    Ok(())
}
```

## Advanced Analytics

### Running Balance

```rust
fn calculate_running_balance(entries: &mut [LedgerEntry]) -> Vec<(DateTime<Utc>, Decimal)> {
    // Sort by date
    entries.sort_by_key(|e| e.date());
    
    let mut balance = dec!(0);
    let mut balances = Vec::new();
    
    for entry in entries {
        match entry.entry_type() {
            EntryType::Income => balance += entry.amount(),
            EntryType::Expense => balance -= entry.amount(),
        }
        balances.push((entry.date(), balance));
    }
    
    balances
}
```

### Statistical Analysis

```rust
fn expense_statistics(entries: &[LedgerEntry]) -> (Decimal, Decimal, Decimal) {
    let amounts: Vec<Decimal> = entries
        .iter()
        .filter(|e| matches!(e.entry_type(), EntryType::Expense))
        .map(|e| e.amount())
        .collect();
    
    if amounts.is_empty() {
        return (dec!(0), dec!(0), dec!(0));
    }
    
    // Mean
    let mean: Decimal = amounts.iter().sum::<Decimal>() / Decimal::from(amounts.len());
    
    // Median
    let mut sorted = amounts.clone();
    sorted.sort();
    let median = sorted[sorted.len() / 2];
    
    // Max
    let max = *sorted.last().unwrap();
    
    (mean, median, max)
}
```

## Creating Custom Report Types

```rust
#[derive(Debug, Clone)]
pub struct CustomSpendingReport {
    pub category: String,
    pub total_spent: Decimal,
    pub average_transaction: Decimal,
    pub transaction_count: usize,
    pub top_merchant: Option<String>,
    pub trend: Vec<(String, Decimal)>,
}

impl CustomSpendingReport {
    pub async fn generate(
        ledger: &LedgerManager,
        category: &str,
        months: usize,
    ) -> BeansResult<Self> {
        let now = Utc::now();
        let start = now - Duration::days((months * 30) as i64);
        
        let filter = EntryFilter {
            tags: vec![category.to_string()],
            entry_type: Some(EntryType::Expense),
            start_date: Some(start),
            end_date: Some(now),
            ..Default::default()
        };
        
        let entries = ledger.list_entries(&filter)?;
        
        let total_spent: Decimal = entries.iter().map(|e| e.amount()).sum();
        let transaction_count = entries.len();
        let average_transaction = if transaction_count > 0 {
            total_spent / Decimal::from(transaction_count)
        } else {
            dec!(0)
        };
        
        // Find top merchant
        let mut merchants: HashMap<String, Decimal> = HashMap::new();
        for entry in &entries {
            *merchants.entry(entry.name().to_string()).or_insert(dec!(0)) += entry.amount();
        }
        let top_merchant = merchants.iter()
            .max_by_key(|(_, amount)| *amount)
            .map(|(name, _)| name.clone());
        
        // Calculate trend
        let mut trend = Vec::new();
        for month in 0..months {
            let month_end = now - Duration::days((month * 30) as i64);
            let month_start = month_end - Duration::days(30);
            
            let month_filter = EntryFilter {
                tags: vec![category.to_string()],
                entry_type: Some(EntryType::Expense),
                start_date: Some(month_start),
                end_date: Some(month_end),
                ..Default::default()
            };
            
            let month_entries = ledger.list_entries(&month_filter)?;
            let month_total: Decimal = month_entries.iter().map(|e| e.amount()).sum();
            
            trend.push((month_start.format("%Y-%m").to_string(), month_total));
        }
        
        Ok(Self {
            category: category.to_string(),
            total_spent,
            average_transaction,
            transaction_count,
            top_merchant,
            trend,
        })
    }
    
    pub fn display(&self) {
        println!("\n=== {} Report ===", self.category);
        println!("Total Spent: ${:.2}", self.total_spent);
        println!("Transactions: {}", self.transaction_count);
        println!("Average: ${:.2}", self.average_transaction);
        if let Some(ref merchant) = self.top_merchant {
            println!("Top Merchant: {}", merchant);
        }
        println!("\nMonthly Trend:");
        for (month, amount) in &self.trend {
            println!("  {}: ${:.2}", month, amount);
        }
    }
}
```

## Best Practices

1. **Cache Results**: Store complex reports for reuse
2. **Incremental Updates**: Update reports incrementally rather than recalculating
3. **Background Processing**: Run heavy analytics in background threads
4. **Pagination**: For large datasets, paginate results
5. **Visualization**: Export to tools like Excel, Grafana, or custom dashboards

## See Also

- [Getting Started Guide](GETTING_STARTED.md)
- [API Reference](API_REFERENCE.md)
- [Generating Reports Example](../beans-lib/examples/generating_reports.rs)

