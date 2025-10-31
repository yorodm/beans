use currency_rs::{Currency, CurrencyOpts};

fn main() {
    // Create a USD currency with value 42.50
    let usd = Currency::new_float(42.50, Some(CurrencyOpts::new().set_symbol("USD")));
    println!("USD: {}", usd);
    
    // Create a EUR currency with value 100.00
    let eur = Currency::new_float(100.00, Some(CurrencyOpts::new().set_symbol("EUR")));
    println!("EUR: {}", eur);
    
    // Create a currency from a string
    let gbp = Currency::new_string("75.25", Some(CurrencyOpts::new().set_symbol("GBP"))).unwrap();
    println!("GBP: {}", gbp);
    
    // Format with precision
    let jpy = Currency::new_float(1234.56, Some(CurrencyOpts::new().set_symbol("JPY").set_precision(0)));
    println!("JPY: {}", jpy);
    
    // Add currencies
    let sum = usd.add(50.0);
    println!("USD + 50.0: {}", sum);
    
    // Subtract
    let diff = eur.subtract(25.0);
    println!("EUR - 25.0: {}", diff);
}
