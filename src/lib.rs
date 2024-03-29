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

/// The [`Murray`] struct is the entrypoint
/// in order to make calls regarding
/// prices, blockchain and lightning information.
/// 
/// You can call it using either `default` method,
/// in which all endpoints for those services will
/// use a default URL, or with `new`, where you can
/// define what are your endpoints for them.
/// 
pub struct Murray {
  pub blockchain: blockchain::Blockchain,
  pub prices: prices::Prices,
  pub lightning: lightning::Lightning,
}

/// Holds the values for blockchain, prices and
/// lightning endpoints when creating a new instance
/// of [`Murray`].
pub struct BaseEndpointsParams {
  pub blockchain_endpoint: Option<String>,
  pub prices_endpoint: Option<String>,
  pub lightning_endpoint: Option<String>,
}

impl Default for BaseEndpointsParams {
  fn default() -> Self {
    Self {
      blockchain_endpoint: Some(BASE_BLOCKCHAIN_URL.to_string()),
      prices_endpoint: Some(BASE_PRICES_URL.to_string()),
      lightning_endpoint: Some(BASE_LIGHTNING_URL.to_string()),
    }
  }
}

impl Murray {
  pub fn new(
    BaseEndpointsParams {
      blockchain_endpoint,
      prices_endpoint,
      lightning_endpoint,
    }: BaseEndpointsParams,
  ) -> Self {
    let mut blockchain_url = BASE_BLOCKCHAIN_URL.to_string();
    if let Some(url) = blockchain_endpoint {
      blockchain_url = url
    }

    let mut prices_url = BASE_PRICES_URL.to_string();
    if let Some(url) = prices_endpoint {
      prices_url = url
    }

    let mut lightning_url = BASE_LIGHTNING_URL.to_string();
    if let Some(url) = lightning_endpoint {
      lightning_url = url
    }

    let blockchain = blockchain::Blockchain::new(blockchain_url);
    let prices = prices::Prices::new(prices_url);
    let lightning = lightning::Lightning::new(lightning_url);

    Self {
      blockchain,
      prices,
      lightning,
    }
  }
}

impl Default for Murray {
  /// Creates a default instance of Murray
  /// with default endpoints
  /// - http://blockchain.murrayrothbot.com for blockchain calls;
  /// - http://prices.murrayrothbot.com for prices calls;
  /// - http://lightning.murrayrothbot.com for lightning calls.
  /// 
  fn default() -> Self {
    Self::new(BaseEndpointsParams::default())
  }
}
