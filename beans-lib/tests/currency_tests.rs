use beans_lib::prelude::Currency;
use beans_lib::prelude::Decimal;
use rust_decimal_macros::dec;

// Helper functions for creating common currencies with zero amount
pub fn usd<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "USD").unwrap()
}

// Helper function for creating USD currency with a specific amount
pub fn usd_with_amount<'a>(amount: Decimal) -> Currency<'a> {
    Currency::new(amount, "USD").unwrap()
}

pub fn eur<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "EUR").unwrap()
}

pub fn gbp<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "GBP").unwrap()
}

pub fn jpy<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "JPY").unwrap()
}

pub fn cny<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "CNY").unwrap()
}

pub fn cad<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "CAD").unwrap()
}

pub fn aud<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "AUD").unwrap()
}

pub fn chf<'a>() -> Currency<'a> {
    Currency::new(Decimal::ZERO, "CHF").unwrap()
}

#[test]
fn test_currency_creation() {
    let usd = Currency::new(dec!(42.50), "USD").unwrap();
    assert_eq!(usd.code(), "USD");
    assert_eq!(usd.amount(), dec!(42.50));

    let zero_usd = Currency::zero("USD").unwrap();
    assert_eq!(zero_usd.code(), "USD");
    assert_eq!(zero_usd.amount(), dec!(0));
    assert!(zero_usd.is_zero());
}

#[test]
fn test_currency_invalid_code() {
    let result = Currency::new(dec!(42.50), "INVALID");
    assert!(result.is_err());
}

#[test]
fn test_currency_arithmetic() {
    let usd1 = Currency::new(dec!(10), "USD").unwrap();
    let usd2 = Currency::new(dec!(20), "USD").unwrap();

    // Addition
    let sum = usd1.add(&usd2).unwrap();
    assert_eq!(sum.amount(), dec!(30));
    assert_eq!(sum.code(), "USD");

    // Subtraction
    let diff = usd2.subtract(&usd1).unwrap();
    assert_eq!(diff.amount(), dec!(10));
    assert_eq!(diff.code(), "USD");

    // Multiplication
    let product = usd1.multiply(dec!(3));
    assert_eq!(product.amount(), dec!(30));
    assert_eq!(product.code(), "USD");

    // Division
    let quotient = usd2.divide(dec!(2)).unwrap();
    assert_eq!(quotient.amount(), dec!(10));
    assert_eq!(quotient.code(), "USD");
}

#[test]
fn test_currency_arithmetic_errors() {
    let usd = Currency::new(dec!(10), "USD").unwrap();
    let eur = Currency::new(dec!(10), "EUR").unwrap();

    // Different currencies
    assert!(usd.add(&eur).is_err());
    assert!(usd.subtract(&eur).is_err());

    // Division by zero
    assert!(usd.divide(dec!(0)).is_err());
}

#[test]
fn test_helper_functions() {
    assert_eq!(usd().code(), "USD");
    assert_eq!(eur().code(), "EUR");
    assert_eq!(gbp().code(), "GBP");
    assert_eq!(jpy().code(), "JPY");
    assert_eq!(cny().code(), "CNY");
    assert_eq!(cad().code(), "CAD");
    assert_eq!(aud().code(), "AUD");
    assert_eq!(chf().code(), "CHF");
}

#[test]
fn test_positive_negative_predicates() {
    let positive = Currency::new(dec!(10), "USD").unwrap();
    let negative = Currency::new(dec!(-10), "USD").unwrap();
    let zero = Currency::zero("USD").unwrap();

    assert!(positive.is_positive());
    assert!(!positive.is_negative());
    assert!(!positive.is_zero());

    assert!(!negative.is_positive());
    assert!(negative.is_negative());
    assert!(!negative.is_zero());

    assert!(!zero.is_positive());
    assert!(!zero.is_negative());
    assert!(zero.is_zero());
}
