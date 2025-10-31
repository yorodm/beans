use beans_lib::models::Currency;
use currency_rs::CurrencyOpts;

#[test]
fn test_currency_creation() {
    let usd = Currency::new_float(42.50, Some(CurrencyOpts::new().set_symbol("USD")));
    assert_eq!(usd.to_string(), "42.50");
    
    let eur = Currency::new_float(100.00, Some(CurrencyOpts::new().set_symbol("EUR")));
    assert_eq!(eur.to_string(), "100.00");
}

#[test]
fn test_currency_from_string() {
    let gbp = Currency::new_string("75.25", Some(CurrencyOpts::new().set_symbol("GBP"))).unwrap();
    assert_eq!(gbp.to_string(), "75.25");
}

#[test]
fn test_currency_precision() {
    let jpy = Currency::new_float(1234.56, Some(CurrencyOpts::new().set_symbol("JPY").set_precision(0)));
    assert_eq!(jpy.to_string(), "1235");
    
    let chf = Currency::new_float(42.4567, Some(CurrencyOpts::new().set_symbol("CHF").set_precision(3)));
    assert_eq!(chf.to_string(), "42.457");
}

#[test]
fn test_currency_arithmetic() {
    let usd = Currency::new_float(42.50, Some(CurrencyOpts::new().set_symbol("USD")));
    
    // Addition
    let sum = usd.clone().add(10.0);
    assert_eq!(sum.to_string(), "52.50");
    
    // Subtraction
    let diff = usd.clone().subtract(10.0);
    assert_eq!(diff.to_string(), "32.50");
    
    // Multiplication
    let product = usd.clone().multiply(2.0);
    assert_eq!(product.to_string(), "85.00");
    
    // Division
    let quotient = usd.clone().divide(2.0);
    assert_eq!(quotient.to_string(), "21.25");
}
