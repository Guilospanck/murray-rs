

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, strum::Display)]
pub enum Currency {
  BTC,
  BRL,
  SATS,
  USD
}

pub struct ConvertCurrencyParams {
  pub currency: Currency,
  pub value: i64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConvertCurrencyReturn {
  pub btc: String,
  pub usd: String,
  pub brl: String,
  pub sat: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ConvertCurrencyJsonData {
  pub data: ConvertCurrencyReturn
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

#[derive(Deserialize, Serialize)]
pub(crate) struct GetTickerJsonData {
  pub data: GetTickerReturn
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
  pub tickers: Vec<Tickers>
}

#[derive(Deserialize, Serialize)]
pub(crate) struct GetTickersJsonData {
  pub data: GetTickersReturn
}