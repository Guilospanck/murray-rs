pub mod types;
use std::result;

use reqwest::Client;

use crate::{GetHealthResponse, GetHealthResponseJsonData};

use self::types::{
  ConvertCurrencyJsonData, ConvertCurrencyParams, ConvertCurrencyReturn, GetTickerJsonData,
  GetTickerParams, GetTickerReturn, GetTickersJsonData, GetTickersReturn, PriceError
};

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
      Err(e) => return Err(PriceError::InvalidURLParams(e.to_string())),
    };

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<ConvertCurrencyJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
    };

    Ok(data)
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
      Err(e) => return Err(PriceError::InvalidURLParams(e.to_string())),
    };

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetTickerJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
    };

    Ok(data)
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
      Err(e) => return Err(PriceError::InvalidURLParams(e.to_string())),
    };

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetTickersJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_health(&self) -> Result<GetHealthResponse> {
    let url = format!("{}/health", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetHealthResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
    };

    Ok(data)
  }
}
