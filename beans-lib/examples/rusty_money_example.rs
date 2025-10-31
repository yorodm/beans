use beans_lib::models::currency::{Currency, usd_with_amount};
use rust_decimal_macros::dec;

fn main() {
    // Create a USD currency with value 42.50
    let usd = usd_with_amount(dec!(42.50));
    println!("USD: {}", usd);
    
    // Create a custom currency
    let eur = Currency::new(dec!(100.00), "EUR").unwrap();
    println!("EUR: {}", eur);
    
    // Arithmetic operations
    let usd2 = usd_with_amount(dec!(10.00));
    let sum = usd.add(&usd2).unwrap();
    println!("Sum: {}", sum);
    
    let diff = usd.subtract(&usd2).unwrap();
    println!("Difference: {}", diff);
    
    let product = usd.multiply(dec!(2));
    println!("Product: {}", product);
    
    let quotient = usd.divide(dec!(2)).unwrap();
    println!("Quotient: {}", quotient);
    
    // Check if a currency is positive, negative, or zero
    println!("Is positive: {}", usd.is_positive());
    println!("Is negative: {}", usd.is_negative());
    println!("Is zero: {}", usd.is_zero());
}

