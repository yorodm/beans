# Multi-Currency Guide

Beans provides robust support for working with multiple currencies, including automatic conversion and normalization for reports.

## Quick Start

```rust
use beans_lib::prelude::*;
use std::time::Duration as StdDuration;

#[tokio::main]
async fn main() -> BeansResult<()> {
    let ledger = LedgerManager::in_memory()?;
    
    // Add entries in different currencies
    let usd_entry = LedgerEntryBuilder::new()
        .name("US Income")
        .amount(dec!(1000.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Income)
        .build()?;
    ledger.add_entry(&usd_entry)?;
    
    let eur_entry = LedgerEntryBuilder::new()
        .name("EU Expense")
        .amount(dec!(500.00))
        .currency(Currency::eur())
        .entry_type(EntryType::Expense)
        .build()?;
    ledger.add_entry(&eur_entry)?;
    
    // Create converter
    let converter = CurrencyConverter::new(StdDuration::from_secs(3600));
    
    // Convert amount
    let eur_in_usd = converter
        .convert(dec!(500.00), &Currency::eur(), &Currency::usd())
        .await?;
    
    println!("€500 = ${}", eur_in_usd);
    
    Ok(())
}
```

## Supported Currencies

Beans supports all ISO 4217 currency codes via the `rusty-money` library. Common currencies include:

```rust
Currency::usd()  // United States Dollar
Currency::eur()  // Euro
Currency::gbp()  // British Pound
Currency::jpy()  // Japanese Yen
Currency::cad()  // Canadian Dollar
Currency::aud()  // Australian Dollar
Currency::chf()  // Swiss Franc
Currency::cny()  // Chinese Yuan

// Or use any ISO code:
Currency::from_code("SEK")?  // Swedish Krona
```

## Currency Conversion

### Basic Conversion

```rust
let converter = CurrencyConverter::new(StdDuration::from_secs(3600));

let amount_usd = converter
    .convert(
        dec!(1000.00),
        &Currency::eur(),
        &Currency::usd()
    )
    .await?;
```

### Caching Strategy

The converter caches exchange rates to improve performance and reduce API calls:

```rust
// Cache for 1 hour
let converter = CurrencyConverter::new(StdDuration::from_secs(3600));

// Cache for 24 hours
let converter = CurrencyConverter::new(StdDuration::from_secs(86400));

// First call fetches from API (~100ms)
let rate1 = converter.convert(dec!(100), &Currency::usd(), &Currency::eur()).await?;

// Subsequent calls use cache (~1μs)
let rate2 = converter.convert(dec!(200), &Currency::usd(), &Currency::eur()).await?;
```

### Batch Conversion

```rust
let amounts_and_currencies = vec![
    (dec!(100.00), Currency::usd()),
    (dec!(200.00), Currency::eur()),
    (dec!(150.00), Currency::gbp()),
];

let target = Currency::usd();
let mut total_usd = dec!(0);

for (amount, currency) in amounts_and_currencies {
    let converted = converter.convert(amount, &currency, &target).await?;
    total_usd += converted;
}

println!("Total in USD: ${}", total_usd);
```

## Multi-Currency Reports

### Normalized Reports

Generate reports with all amounts converted to a single currency:

```rust
let ledger = setup_multi_currency_ledger()?;
let converter = CurrencyConverter::new(StdDuration::from_secs(3600));

let generator = ReportGenerator::new(&ledger)
    .with_converter(converter);

// All amounts normalized to USD
let report = generator
    .income_expense_report(
        start_date,
        end_date,
        TimePeriod::Monthly,
        Some(Currency::usd()),  // Target currency
        None,
    )
    .await?;

println!("Total income (USD): ${}", report.summary.income);
println!("Total expenses (USD): ${}", report.summary.expenses);
```

### Mixed Currency Ledgers

```rust
fn add_international_expenses(ledger: &LedgerManager) -> BeansResult<()> {
    // US expenses
    ledger.add_entry(&LedgerEntryBuilder::new()
        .name("US Rent")
        .amount(dec!(1200.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Expense)
        .add_tag("housing")?
        .build()?)?;
    
    // European expenses
    ledger.add_entry(&LedgerEntryBuilder::new()
        .name("Paris Dinner")
        .amount(dec!(80.00))
        .currency(Currency::eur())
        .entry_type(EntryType::Expense)
        .add_tag("food")?
        .build()?)?;
    
    // UK expenses
    ledger.add_entry(&LedgerEntryBuilder::new()
        .name("London Transport")
        .amount(dec!(50.00))
        .currency(Currency::gbp())
        .entry_type(EntryType::Expense)
        .add_tag("transportation")?
        .build()?)?;
    
    Ok(())
}
```

## Exchange Rate API

Beans uses the free [fawazahmed0/currency-api](https://github.com/fawazahmed0/currency-api) for exchange rates.

### Requirements

- Internet connectivity
- No API key required
- Rate limits apply (use caching!)

### Error Handling

```rust
match converter.convert(amount, &from_currency, &to_currency).await {
    Ok(converted) => println!("Converted: {}", converted),
    Err(BeansError::CurrencyConversionError(msg)) => {
        eprintln!("Conversion failed: {}", msg);
        // Handle offline scenario or API issues
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### Fallback Strategy

```rust
async fn convert_with_fallback(
    converter: &CurrencyConverter,
    amount: Decimal,
    from: &Currency,
    to: &Currency,
    fallback_rate: Decimal,
) -> BeansResult<Decimal> {
    match converter.convert(amount, from, to).await {
        Ok(result) => Ok(result),
        Err(_) => {
            println!("Using fallback rate: {}", fallback_rate);
            Ok(amount * fallback_rate)
        }
    }
}
```

## Best Practices

### 1. Consistent Base Currency

Choose a base currency for reporting:

```rust
const BASE_CURRENCY: &str = "USD";

fn get_base_currency() -> BeansResult<Currency<'static>> {
    Currency::from_code(BASE_CURRENCY)
}
```

### 2. Cache Configuration

Balance freshness vs. performance:

```rust
// Real-time trading: short cache
let converter = CurrencyConverter::new(StdDuration::from_secs(300));  // 5 min

// Personal finance: longer cache
let converter = CurrencyConverter::new(StdDuration::from_secs(86400));  // 24 hours
```

### 3. Offline Support

Implement offline fallback:

```rust
struct OfflineRates {
    rates: HashMap<(String, String), Decimal>,
    last_updated: DateTime<Utc>,
}

impl OfflineRates {
    fn get_rate(&self, from: &str, to: &str) -> Option<Decimal> {
        self.rates.get(&(from.to_string(), to.to_string())).copied()
    }
}
```

### 4. Historical Rates

For accurate historical analysis, consider storing conversion rates with entries:

```rust
#[derive(Debug, Clone)]
struct EnrichedEntry {
    entry: LedgerEntry,
    exchange_rate_to_usd: Option<Decimal>,
    converted_amount_usd: Option<Decimal>,
}
```

## Advanced Examples

### Travel Expense Tracker

```rust
async fn track_travel_expenses(ledger: &LedgerManager) -> BeansResult<()> {
    let converter = CurrencyConverter::new(StdDuration::from_secs(3600));
    
    // Add expenses in local currencies
    let expenses = vec![
        ("Paris Hotel", dec!(150.00), Currency::eur()),
        ("London Train", dec!(80.00), Currency::gbp()),
        ("Tokyo Restaurant", dec!(5000.00), Currency::jpy()),
    ];
    
    let mut total_usd = dec!(0);
    
    for (name, amount, currency) in expenses {
        let entry = LedgerEntryBuilder::new()
            .name(name)
            .amount(amount)
            .currency(currency.clone())
            .entry_type(EntryType::Expense)
            .add_tag("travel")?
            .build()?;
        
        ledger.add_entry(&entry)?;
        
        // Convert to USD for total
        let usd_amount = converter
            .convert(amount, &currency, &Currency::usd())
            .await?;
        
        total_usd += usd_amount;
        
        println!("{}: {} {} = ${:.2}", name, amount, currency.code(), usd_amount);
    }
    
    println!("\nTotal travel expenses: ${:.2}", total_usd);
    
    Ok(())
}
```

### Currency Portfolio Tracker

```rust
async fn currency_portfolio(ledger: &LedgerManager) -> BeansResult<()> {
    let converter = CurrencyConverter::new(StdDuration::from_secs(3600));
    
    // Holdings in different currencies
    let holdings = vec![
        (dec!(1000.00), Currency::usd()),
        (dec!(800.00), Currency::eur()),
        (dec!(600.00), Currency::gbp()),
    ];
    
    let base = Currency::usd();
    let mut total = dec!(0);
    
    println!("Currency Portfolio (Base: USD)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    for (amount, currency) in holdings {
        let value_in_base = if currency.code() == base.code() {
            amount
        } else {
            converter.convert(amount, &currency, &base).await?
        };
        
        total += value_in_base;
        
        let percentage = if total > dec!(0) {
            (value_in_base / total) * dec!(100)
        } else {
            dec!(0)
        };
        
        println!("{:>10} {} = ${:>10.2} ({:.1}%)",
            amount, currency.code(), value_in_base, percentage
        );
    }
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Total:     ${:>10.2}", total);
    
    Ok(())
}
```

## Troubleshooting

**Q: Conversion fails with network error**  
A: Check internet connectivity. The API requires network access. Implement fallback rates for offline scenarios.

**Q: Rates seem inaccurate**  
A: Exchange rates are indicative and may differ from your bank's rates. For exact conversions, use your institution's rates.

**Q: How to handle cryptocurrency?**  
A: The current API focuses on fiat currencies. For crypto, you'll need a different exchange rate source.

**Q: Can I use custom exchange rates?**  
A: Currently not supported directly. You can manually convert amounts before adding entries.

## See Also

- [Getting Started Guide](GETTING_STARTED.md)
- [Personal Budget Tutorial](PERSONAL_BUDGET_TUTORIAL.md)
- [API Reference](API_REFERENCE.md)
- [Currency Conversion Example](../beans-lib/examples/currency_conversion.rs)

