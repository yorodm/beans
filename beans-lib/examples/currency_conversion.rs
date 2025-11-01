//! Currency Conversion Example
//!
//! This example demonstrates currency conversion capabilities:
//! - Converting amounts between different currencies
//! - Using the exchange rate cache for performance
//! - Handling conversion errors
//! - Working with multiple currencies in a ledger
//!
//! Note: This example requires internet connectivity to fetch live exchange rates.
//!
//! Run with: `cargo run --example currency_conversion`

use beans_lib::prelude::*;
use rust_decimal_macros::dec;
use std::time::Duration as StdDuration;

#[tokio::main]
async fn main() -> BeansResult<()> {
    println!("=== Beans Ledger: Currency Conversion Example ===\n");

    // === CREATE CURRENCY CONVERTER ===
    println!("--- Setting Up Currency Converter ---");
    
    // Create a converter with 1-hour cache TTL
    let cache_ttl = StdDuration::from_secs(3600);
    let converter = CurrencyConverter::new(cache_ttl);
    println!("✓ Created currency converter with 1-hour cache\n");

    // === BASIC CONVERSION ===
    println!("--- Basic Currency Conversion ---");
    
    let usd_amount = dec!(100.00);
    let from_currency = Currency::usd();
    let to_currency = Currency::eur();
    
    match converter.convert(usd_amount, &from_currency, &to_currency).await {
        Ok(converted) => {
            println!("${} USD = €{} EUR", usd_amount, converted);
        }
        Err(e) => {
            println!("⚠️  Conversion failed: {}", e);
            println!("   (This might happen if you're offline or the API is unavailable)");
        }
    }
    println!();

    // === MULTIPLE CONVERSIONS ===
    println!("--- Converting to Multiple Currencies ---");
    
    let base_amount = dec!(1000.00);
    let base_currency = Currency::usd();
    
    let target_currencies = vec![
        ("EUR", Currency::eur()),
        ("GBP", Currency::gbp()),
        ("JPY", Currency::jpy()),
        ("CAD", Currency::cad()),
        ("AUD", Currency::aud()),
    ];
    
    for (code, currency) in target_currencies {
        match converter.convert(base_amount, &base_currency, &currency).await {
            Ok(converted) => {
                println!("${} USD = {} {} {}", 
                    base_amount, 
                    currency.symbol().unwrap_or(""), 
                    converted,
                    code
                );
            }
            Err(e) => {
                println!("⚠️  Failed to convert to {}: {}", code, e);
            }
        }
    }
    println!();

    // === WORKING WITH MULTI-CURRENCY LEDGER ===
    println!("--- Multi-Currency Ledger Example ---");
    
    let ledger = LedgerManager::in_memory()?;
    
    // Add entries in different currencies
    let usd_salary = LedgerEntryBuilder::new()
        .name("US Salary")
        .amount(dec!(5000.00))
        .currency(Currency::usd())
        .entry_type(EntryType::Income)
        .add_tag("salary")?
        .build()?;
    ledger.add_entry(&usd_salary)?;
    
    let eur_rent = LedgerEntryBuilder::new()
        .name("Paris Rent")
        .amount(dec!(1200.00))
        .currency(Currency::eur())
        .entry_type(EntryType::Expense)
        .add_tag("housing")?
        .build()?;
    ledger.add_entry(&eur_rent)?;
    
    let gbp_groceries = LedgerEntryBuilder::new()
        .name("London Groceries")
        .amount(dec!(80.00))
        .currency(Currency::gbp())
        .entry_type(EntryType::Expense)
        .add_tag("food")?
        .build()?;
    ledger.add_entry(&gbp_groceries)?;
    
    println!("Added entries in multiple currencies:");
    let all_entries = ledger.list_entries(&EntryFilter::default())?;
    for entry in &all_entries {
        println!("  {} - {} {}", 
            entry.name(), 
            entry.amount(), 
            entry.currency_code()
        );
    }
    println!();
    
    // === NORMALIZING TO SINGLE CURRENCY ===
    println!("--- Normalizing to USD ---");
    
    let target_currency = Currency::usd();
    let mut total_usd = dec!(0);
    
    for entry in &all_entries {
        let entry_currency = Currency::from_code(entry.currency_code())?;
        
        // Convert to USD if needed
        let amount_usd = if entry.currency_code() == target_currency.code() {
            entry.amount()
        } else {
            match converter.convert(entry.amount(), &entry_currency, &target_currency).await {
                Ok(converted) => converted,
                Err(e) => {
                    println!("⚠️  Failed to convert {}: {}", entry.name(), e);
                    continue;
                }
            }
        };
        
        let sign = match entry.entry_type() {
            EntryType::Income => 1,
            EntryType::Expense => -1,
        };
        
        total_usd += amount_usd * Decimal::from(sign);
        
        println!("  {} {} {} = ${} USD", 
            entry.amount(), 
            entry.currency_code(),
            entry.name(),
            amount_usd
        );
    }
    
    println!("\nNet total in USD: ${}", total_usd);
    println!();

    // === CACHING DEMONSTRATION ===
    println!("--- Currency Cache Performance ---");
    
    // First conversion (will fetch from API)
    let start = std::time::Instant::now();
    let _ = converter.convert(dec!(100), &Currency::usd(), &Currency::eur()).await;
    let first_duration = start.elapsed();
    println!("First conversion (API fetch): {:?}", first_duration);
    
    // Second conversion (will use cache)
    let start = std::time::Instant::now();
    let _ = converter.convert(dec!(200), &Currency::usd(), &Currency::eur()).await;
    let second_duration = start.elapsed();
    println!("Second conversion (cached): {:?}", second_duration);
    println!("Speed improvement: {}x faster", 
        first_duration.as_micros() / second_duration.as_micros().max(1)
    );
    println!();

    // === ERROR HANDLING ===
    println!("--- Error Handling Examples ---");
    
    // Try converting with invalid currency
    println!("Attempting conversion with same source and target:");
    match converter.convert(dec!(100), &Currency::usd(), &Currency::usd()).await {
        Ok(result) => println!("  Result: ${}", result),
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    println!("=== Example Complete ===");
    println!("\nNote: Exchange rates are fetched from a public API.");
    println!("Rates are cached for performance and to reduce API calls.");
    
    Ok(())
}

