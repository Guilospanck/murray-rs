mod blockchain;
mod lightning;
mod prices;

pub use blockchain::types::*;
pub use prices::types::*;

const BASE_BLOCKCHAIN_URL: &str = "http://blockchain.murrayrothbot.com";
const BASE_PRICES_URL: &str = "http://prices.murrayrothbot.com";

pub struct Murray {
  pub blockchain: blockchain::Blockchain,
  pub prices: prices::Prices,
}

impl Murray {
  pub fn new() -> Self {
    let blockchain = blockchain::Blockchain::new(BASE_BLOCKCHAIN_URL.to_string());
    let prices = prices::Prices::new(BASE_PRICES_URL.to_string());

    Self { blockchain, prices }
  }
}
