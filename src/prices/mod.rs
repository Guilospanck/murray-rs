pub mod types;
use std::result;

use reqwest;

use self::types::{
  ConvertCurrencyJsonData, ConvertCurrencyParams, ConvertCurrencyReturn, GetTickerJsonData,
  GetTickerParams, GetTickerReturn, GetTickersJsonData, GetTickersReturn,
};

const BASE_URL: &str = "http://prices.murrayrothbot.com";

/// [`Price`] error
#[derive(thiserror::Error, Debug)]
pub enum PriceError {
  #[error("Invalid URL params")]
  InvalidURLParams,
  #[error("Bad request")]
  BadRequest,
}

type Result<T> = result::Result<T, PriceError>;

#[tokio::main]
pub async fn convert_currency(
  ConvertCurrencyParams { currency, value }: ConvertCurrencyParams,
) -> Result<ConvertCurrencyReturn> {
  let url = format!("{}/convert", BASE_URL);
  let params = [
    ("currency", currency.to_string()),
    ("value", value.to_string()),
  ];

  let url = match reqwest::Url::parse_with_params(&url, &params) {
    Ok(url) => url,
    Err(_) => return Err(PriceError::InvalidURLParams),
  };

  let client = reqwest::Client::new()
    .get(url)
    .header("Accept", "application/json");

  let resp = match client.send().await {
    Ok(resp) => resp.json::<ConvertCurrencyJsonData>().await.unwrap(),
    Err(_) => return Err(PriceError::BadRequest),
  };

  Ok(resp.data)
}

#[tokio::main]
pub async fn get_ticker(GetTickerParams { symbol }: GetTickerParams) -> Result<GetTickerReturn> {
  let url = format!("{}/ticker", BASE_URL);
  let params = [("symbol", symbol.to_string())];

  let url = match reqwest::Url::parse_with_params(&url, &params) {
    Ok(url) => url,
    Err(_) => return Err(PriceError::InvalidURLParams),
  };

  let client = reqwest::Client::new()
    .get(url)
    .header("Accept", "application/json");

  let resp = match client.send().await {
    Ok(resp) => resp.json::<GetTickerJsonData>().await.unwrap(),
    Err(_) => return Err(PriceError::BadRequest),
  };

  Ok(resp.data)
}

#[tokio::main]
pub async fn get_tickers(GetTickerParams { symbol }: GetTickerParams) -> Result<GetTickersReturn> {
  let url = format!("{}/tickers", BASE_URL);
  let params = [("symbol", symbol.to_string())];

  let url = match reqwest::Url::parse_with_params(&url, &params) {
    Ok(url) => url,
    Err(_) => return Err(PriceError::InvalidURLParams),
  };

  let client = reqwest::Client::new()
    .get(url)
    .header("Accept", "application/json");

  let resp = match client.send().await {
    Ok(resp) => resp.json::<GetTickersJsonData>().await.unwrap(),
    Err(_) => return Err(PriceError::BadRequest),
  };

  Ok(resp.data)
}
