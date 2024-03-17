use serde::{Deserialize, Serialize};

/// [`Price`] error
#[derive(thiserror::Error, Debug)]
pub enum PriceError {
  #[error("Invalid URL params: `{0}`")]
  InvalidURLParams(String),
  #[error("Bad request: `{0}`")]
  BadRequest(String),
  #[error("API error: `{0}`")]
  APIError(String),
  #[error("JSON parse error: `{0}`")]
  JSONParseError(String),
}

#[derive(Debug, PartialEq, strum::Display)]
pub enum Currency {
  BTC,
  BRL,
  SATS,
  USD,
}

pub struct ConvertCurrencyParams {
  pub currency: Currency,
  pub value: i64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConvertCurrencyReturn {
  pub btc: String,
  pub usd: String,
  pub brl: String,
  pub sat: String,
}

#[derive(Debug, PartialEq, strum::Display)]
pub enum Symbol {
  BTCUSD,
  BTCBRL,
  BTCUSDT,
}

pub struct GetTickerParams {
  pub symbol: Symbol,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetTickerReturn {
  pub price: String,
  pub change24h: String,
  pub source: String,
  pub symbol: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tickers {
  pub price: String,
  pub change24h: Option<String>,
  pub source: String,
  pub symbol: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetTickersReturn {
  pub tickers: Vec<Tickers>,
}

#[derive(Deserialize, Serialize)]
pub(super) struct ConvertCurrencyJsonData {
  pub data: ConvertCurrencyReturn,
}

#[derive(Deserialize, Serialize)]
pub(super) struct GetTickerJsonData {
  pub data: GetTickerReturn,
}

#[derive(Deserialize, Serialize)]
pub(super) struct GetTickersJsonData {
  pub data: GetTickersReturn,
}
