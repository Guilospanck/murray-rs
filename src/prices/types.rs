

use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, strum::Display)]
#[strum(ascii_case_insensitive)]
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