use serde::{Deserialize, Serialize};

mod blockchain;
mod lightning;
mod prices;

pub use blockchain::types::*;
pub use lightning::types::*;
pub use prices::types::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct GetHealthResponse {
  pub message: String,
}

#[derive(Deserialize, Serialize)]
struct GetHealthResponseJsonData {
  pub data: GetHealthResponse,
}

const BASE_BLOCKCHAIN_URL: &str = "http://blockchain.murrayrothbot.com";
const BASE_PRICES_URL: &str = "http://prices.murrayrothbot.com";
const BASE_LIGHTNING_URL: &str = "http://lightning.murrayrothbot.com";

pub struct Murray {
  pub blockchain: blockchain::Blockchain,
  pub prices: prices::Prices,
  pub lightning: lightning::Lightning,
}

impl Murray {
  pub fn new() -> Self {
    let blockchain = blockchain::Blockchain::new(BASE_BLOCKCHAIN_URL.to_string());
    let prices = prices::Prices::new(BASE_PRICES_URL.to_string());
    let lightning = lightning::Lightning::new(BASE_LIGHTNING_URL.to_string());

    Self {
      blockchain,
      prices,
      lightning,
    }
  }
}

impl Default for Murray {
  fn default() -> Self {
    Self::new()
  }
}
