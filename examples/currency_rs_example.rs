use currency_rs::{Currency, CurrencyOpts};

fn main() {
    // Create a USD currency with value 42.50
    let usd = Currency::new_float(42.50, CurrencyOpts::new().set_symbol("USD"));
    println!("USD: {}", usd);
    
    // Create a EUR currency with value 100.00
    let eur = Currency::new_float(100.00, CurrencyOpts::new().set_symbol("EUR"));
    println!("EUR: {}", eur);
    
    // Check if a currency code is valid
    println!("Is USD valid: {}", Currency::is_valid_currency_code("USD"));
    println!("Is XYZ valid: {}", Currency::is_valid_currency_code("XYZ"));
    
    // Get all available currency codes
    println!("Available currency codes: {:?}", Currency::get_available_currency_codes());
}
