mod support;

use beans_lib::currency::CurrencyConverter;
use beans_lib::error::BeansResult;
use beans_lib::models::Currency;
use rust_decimal_macros::dec;
use std::time::Duration;
use support::*;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_same_currency_conversion() -> BeansResult<()> {
    let converter = CurrencyConverter::default();
    let usd = Currency::new(dec!(100.00), usd())?;

    let result = converter.convert_amount(dec!(100.00), &usd, &usd).await?;

    assert_eq!(result, dec!(100.00));
    Ok(())
}

#[tokio::test]
async fn test_exchange_rate_from_api() -> BeansResult<()> {
    // Start a mock server
    let mock_server = MockServer::start().await;

    // Create a sample API response for USD to EUR
    let response_body = r#"{
        "date": "2025-10-31",
        "usd": {
            "eur": 0.85
        }
    }"#;

    // Mock the API endpoint
    Mock::given(method("GET"))
        .and(path("/v1/currencies/usd.json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    // Create a converter with the mock server URL
    let mut converter = CurrencyConverter::new(Duration::from_secs(24 * 60 * 60));
    converter.set_base_url(format!("{}/v1",mock_server.uri()));

    // Create test currencies
    let usd = Currency::new(dec!(100.00), usd())?;
    let eur = Currency::new(dec!(0.00), eur())?;

    // Get the exchange rate
    let rate = converter.get_exchange_rate(&usd, &eur).await?;

    // Verify the rate matches our mock
    assert_eq!(rate, 0.85);

    // Test conversion
    let result = converter.convert_amount(dec!(100.00), &usd, &eur).await?;

    // 100 USD * 0.85 = 85 EUR
    assert_eq!(result, dec!(85.00));

    Ok(())
}

#[tokio::test]
async fn test_cache_functionality() -> BeansResult<()> {
    // Start a mock server
    let mock_server = MockServer::start().await;

    // Create a sample API response for USD to EUR
    let response_body = r#"{
        "date": "2025-10-31",
        "usd": {
            "eur": 0.85,
            "gbp": 0.75,
            "jpy": 110.0
        }
    }"#;

    // Mock the API endpoint - we'll count how many times it's called
    Mock::given(method("GET"))
        .and(path("/v1/currencies/usd.json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .expect(1) // We expect this to be called exactly once
        .mount(&mock_server)
        .await;

    // Create a converter with the mock server URL
    let mut converter = CurrencyConverter::new(Duration::from_secs(24 * 60 * 60));
    converter.set_base_url(format!("{}/v1",mock_server.uri()));

    // Create test currencies
    let usd = Currency::new(dec!(100.00), usd())?;
    let eur = Currency::new(dec!(0.00), eur())?;
    let gbp = Currency::new(dec!(0.00), "GBP")?;

    // First call should hit the API
    let rate1 = converter.get_exchange_rate(&usd, &eur).await?;
    assert_eq!(rate1, 0.85);

    // Second call should use the cache
    let rate2 = converter.get_exchange_rate(&usd, &eur).await?;
    assert_eq!(rate2, 0.85);

    // Call for a different currency pair that was in the same response
    let rate3 = converter.get_exchange_rate(&usd, &gbp).await?;
    assert_eq!(rate3, 0.75);

    Ok(())
}

#[tokio::test]
async fn test_api_error_handling() -> BeansResult<()> {
    // Start a mock server
    let mock_server = MockServer::start().await;

    // Mock a server error
    Mock::given(method("GET"))
        .and(path("/v1/currencies/usd.json"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    // Create a converter with the mock server URL
    let mut converter = CurrencyConverter::new(Duration::from_secs(24 * 60 * 60));
    converter.set_base_url(format!("{}/v1",mock_server.uri()));

    // Create test currencies
    let usd = Currency::new(dec!(100.00), usd())?;
    let eur = Currency::new(dec!(0.00), eur())?;

    // Attempt to get the exchange rate
    let result = converter.get_exchange_rate(&usd, &eur).await;

    // Verify we get an error
      assert!(result.is_err_and(|e| match e {
        beans_lib::BeansError::Other(_) => true,
        _ => false
    }));

    Ok(())
}

#[tokio::test]
async fn test_invalid_json_handling() -> BeansResult<()> {
    // Start a mock server
    let mock_server = MockServer::start().await;

    // Create an invalid JSON response
    let response_body = r#"{
        "date": "2025-10-31",
        "usd": {
            "eur": 0.85,
    }"#; // Missing closing brace

    // Mock the API endpoint
    Mock::given(method("GET"))
        .and(path("/v1/currencies/usd.json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    // Create a converter with the mock server URL
    let mut converter = CurrencyConverter::new(Duration::from_secs(24 * 60 * 60));
    converter.set_base_url(format!("{}/v1",mock_server.uri()));

    // Create test currencies
    let usd = Currency::new(dec!(100.00), usd())?;
    let eur = Currency::new(dec!(0.00), eur())?;

    // Attempt to get the exchange rate
    let result = converter.get_exchange_rate(&usd, &eur).await;
    // Verify we get an error
    assert!(result.is_err_and(|e| match e {
        beans_lib::BeansError::Json(_) => true,
        _ => false
    }));

    Ok(())
}

#[tokio::test]
async fn test_missing_rate_handling() -> BeansResult<()> {
    // Start a mock server
    let mock_server = MockServer::start().await;

    // Create a response missing the requested currency
    let response_body = r#"{
        "date": "2025-10-31",
        "usd": {
            "gbp": 0.75,
            "jpy": 110.0
        }
    }"#;

    // Mock the API endpoint
    Mock::given(method("GET"))
        .and(path("/v1/currencies/usd.json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&mock_server)
        .await;

    // Create a converter with the mock server URL
    let mut converter = CurrencyConverter::new(Duration::from_secs(24 * 60 * 60));
    converter.set_base_url(format!("{}/v1",mock_server.uri()));

    // Create test currencies
    let usd = Currency::new(dec!(100.00), usd())?;
    let eur = Currency::new(dec!(0.00), eur())?;

    // Attempt to get the exchange rate
    let result = converter.get_exchange_rate(&usd, &eur).await;

    // Verify we get an error
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(err.to_string().contains("Exchange rate not available"));
    }

    Ok(())
}

#[tokio::test]
async fn test_fallback_url() -> BeansResult<()> {
    // Start two mock servers (primary and fallback)
    let primary_server = MockServer::start().await;
    let fallback_server = MockServer::start().await;

    // Primary server returns an error
    Mock::given(method("GET"))
        .and(path("/v1/currencies/usd.json"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&primary_server)
        .await;

    // Fallback server returns a valid response
    let response_body = r#"{
        "date": "2025-10-31",
        "usd": {
            "eur": 0.85
        }
    }"#;

    Mock::given(method("GET"))
        .and(path("/v1/currencies/usd.json"))
        .respond_with(ResponseTemplate::new(200).set_body_string(response_body))
        .mount(&fallback_server)
        .await;

    // Create a converter with both URLs
    let mut converter = CurrencyConverter::new(Duration::from_secs(24 * 60 * 60));
    converter.set_base_url(format!("{}/v1",primary_server.uri()));
    converter.set_base_url(format!("{}/v1",fallback_server.uri()));

    // Create test currencies
    let usd = Currency::new(dec!(100.00), usd())?;
    let eur = Currency::new(dec!(0.00), eur())?;

    // Get the exchange rate - should use fallback
    let rate = converter.get_exchange_rate(&usd, &eur).await?;

    // Verify the rate matches our fallback mock
    assert_eq!(rate, 0.85);

    Ok(())
}
