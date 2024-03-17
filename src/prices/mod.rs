pub mod types;
use std::result;

use reqwest::{self, Client};

use self::types::{
  ConvertCurrencyJsonData, ConvertCurrencyParams, ConvertCurrencyReturn, GetTickerJsonData,
  GetTickerParams, GetTickerReturn, GetTickersJsonData, GetTickersReturn,
};

/// [`Price`] error
#[derive(thiserror::Error, Debug)]
pub enum PriceError {
  #[error("Invalid URL params")]
  InvalidURLParams,
  #[error("Bad request")]
  BadRequest,
}

type Result<T> = result::Result<T, PriceError>;

pub struct Prices {
  base_url: String,
  client: Client,
}

impl Prices {
  pub fn new(url: String) -> Self {
    let client = reqwest::Client::new();
    Self {
      base_url: url,
      client,
    }
  }

  pub fn set_base_url(&mut self, base_url: String) {
    self.base_url = base_url;
  }

  #[tokio::main]
  pub async fn convert_currency(
    &self,
    ConvertCurrencyParams { currency, value }: ConvertCurrencyParams,
  ) -> Result<ConvertCurrencyReturn> {
    let url = format!("{}/convert", self.base_url);
    let params = [
      ("currency", currency.to_string()),
      ("value", value.to_string()),
    ];

    let url = match reqwest::Url::parse_with_params(&url, &params) {
      Ok(url) => url,
      Err(_) => return Err(PriceError::InvalidURLParams),
    };

    let client = self.client.get(url).header("Accept", "application/json");

    let resp = match client.send().await {
      Ok(resp) => resp.json::<ConvertCurrencyJsonData>().await.unwrap(),
      Err(_) => return Err(PriceError::BadRequest),
    };

    Ok(resp.data)
  }

  #[tokio::main]
  pub async fn get_ticker(
    &self,
    GetTickerParams { symbol }: GetTickerParams,
  ) -> Result<GetTickerReturn> {
    let url = format!("{}/ticker", self.base_url);
    let params = [("symbol", symbol.to_string())];

    let url = match reqwest::Url::parse_with_params(&url, &params) {
      Ok(url) => url,
      Err(_) => return Err(PriceError::InvalidURLParams),
    };

    let client = self.client.get(url).header("Accept", "application/json");

    let resp = match client.send().await {
      Ok(resp) => resp.json::<GetTickerJsonData>().await.unwrap(),
      Err(_) => return Err(PriceError::BadRequest),
    };

    Ok(resp.data)
  }

  #[tokio::main]
  pub async fn get_tickers(
    &self,
    GetTickerParams { symbol }: GetTickerParams,
  ) -> Result<GetTickersReturn> {
    let url = format!("{}/tickers", self.base_url);
    let params = [("symbol", symbol.to_string())];

    let url = match reqwest::Url::parse_with_params(&url, &params) {
      Ok(url) => url,
      Err(_) => return Err(PriceError::InvalidURLParams),
    };

    let client = self.client.get(url).header("Accept", "application/json");

    let resp = match client.send().await {
      Ok(resp) => resp.json::<GetTickersJsonData>().await.unwrap(),
      Err(_) => return Err(PriceError::BadRequest),
    };

    Ok(resp.data)
  }
}
