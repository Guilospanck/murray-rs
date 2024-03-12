pub mod types;
use std::result;

use reqwest::{self, Response};

use self::types::{ConvertCurrencyJsonData, ConvertCurrencyParams, ConvertCurrencyReturn};

const BASE_URL: &str = "http://prices.murrayrothbot.com";

/// [`Event`] error
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
  let params = [("currency", currency.to_string()), ("value", value.to_string())];

  let url = match reqwest::Url::parse_with_params(&url, &params) {
    Ok(url) => url,
    Err(_) => return Err(PriceError::InvalidURLParams),
  };

  let resp = match reqwest::get(url).await {
    Ok(resp) => resp.json::<ConvertCurrencyJsonData>().await.unwrap(),
    Err(_) => return Err(PriceError::BadRequest),
  };

  Ok(resp.data)
}
