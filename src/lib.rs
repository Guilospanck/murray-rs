mod blockchain;
mod lightning;
mod prices;

pub use blockchain::types::*;

const BASE_BLOCKCHAIN_URL: &str = "http://blockchain.murrayrothbot.com";

pub struct Murray {
  pub blockchain: blockchain::Blockchain,
}

impl Murray {
  pub fn new() -> Self {
    let blockchain = blockchain::Blockchain::new(BASE_BLOCKCHAIN_URL.to_string());

    Self {
      blockchain
    }
  }
}