pub mod types;
use std::result;

use reqwest::Client;

use crate::{GetHealthResponse, GetHealthResponseJsonData};

use self::types::{
  GetAddressDetailsResponse, GetAddressDetailsResponseJsonData, GetAddressParams,
  GetAddressTransactionsResponse, GetAddressTransactionsResponseJsonData, GetAddressUTXOResponse,
  GetAddressUTXOResponseJsonData, GetBlock2TimeResponse, GetBlock2TimeResponseJsonData,
  GetBlockParams, GetBlockResponse, GetBlockResponseJsonData, GetFeesMempoolBlocksResponse,
  GetFeesMempoolBlocksResponseJsonData, GetFeesRecommendedResponse,
  GetFeesRecommendedResponseJsonData, GetHashrateResponse, GetHashrateResponseJsonData,
  GetMempoolResponse, GetMempoolResponseJsonData, GetTransactionParams, GetTransactionResponse,
  GetTransactionResponseJsonData, PostTransactionParams, PostTransactionResponse,
  PostTransactionResponseJsonData,
};

/// [`Blockchain`] error
#[derive(thiserror::Error, Debug)]
pub enum BlockchainError {
  #[error("Invalid URL params: `{0}`")]
  InvalidURLParams(String),
  #[error("Bad request: `{0}`")]
  BadRequest(String),
  #[error("API error: `{0}`")]
  APIError(String),
  #[error("JSON parse error: `{0}`")]
  JSONParseError(String),
}

type Result<T> = result::Result<T, BlockchainError>;

pub struct Blockchain {
  base_url: String,
  client: Client,
}

impl Blockchain {
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
  pub async fn get_block(
    &self,
    GetBlockParams { hash, height }: GetBlockParams,
  ) -> Result<GetBlockResponse> {
    let url = format!("{}/block", self.base_url);

    let mut params = vec![];

    if let Some(s) = hash {
      params.push(("hash", s))
    }

    if let Some(s) = height {
      params.push(("height", s.to_string()))
    }

    let url = match reqwest::Url::parse_with_params(&url, &params) {
      Ok(url) => url,
      Err(e) => return Err(BlockchainError::InvalidURLParams(e.to_string())),
    };

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetBlockResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(BlockchainError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(BlockchainError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_block2time(
    &self,
    GetBlockParams { hash, height }: GetBlockParams,
  ) -> Result<GetBlock2TimeResponse> {
    let url = format!("{}/block2time", self.base_url);

    let mut params = vec![];

    if let Some(s) = hash {
      params.push(("hash", s))
    }

    if let Some(s) = height {
      params.push(("height", s.to_string()))
    }

    let url = match reqwest::Url::parse_with_params(&url, &params) {
      Ok(url) => url,
      Err(e) => return Err(BlockchainError::InvalidURLParams(e.to_string())),
    };

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetBlock2TimeResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(BlockchainError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(BlockchainError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_fees_recommended(&self) -> Result<GetFeesRecommendedResponse> {
    let url = format!("{}/fees/recommended", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetFeesRecommendedResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(BlockchainError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(BlockchainError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_fees_mempool_blocks(&self) -> Result<Vec<GetFeesMempoolBlocksResponse>> {
    let url = format!("{}/fees/mempool-blocks", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetFeesMempoolBlocksResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(BlockchainError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(BlockchainError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_address_details(
    &self,
    GetAddressParams { address }: GetAddressParams,
  ) -> Result<GetAddressDetailsResponse> {
    let url = format!("{}/address/{}", self.base_url, address);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetAddressDetailsResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(BlockchainError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(BlockchainError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_address_transactions(
    &self,
    GetAddressParams { address }: GetAddressParams,
  ) -> Result<Vec<GetAddressTransactionsResponse>> {
    let url = format!("{}/address/{}/txs", self.base_url, address);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetAddressTransactionsResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(BlockchainError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(BlockchainError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_address_utxos(
    &self,
    GetAddressParams { address }: GetAddressParams,
  ) -> Result<Vec<GetAddressUTXOResponse>> {
    let url = format!("{}/address/{}/txs/utxo", self.base_url, address);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetAddressUTXOResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(BlockchainError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_hashrate(&self) -> Result<GetHashrateResponse> {
    let url = format!("{}/hashrate", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetHashrateResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(BlockchainError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(BlockchainError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_health(&self) -> Result<GetHealthResponse> {
    let url = format!("{}/health", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetHealthResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(BlockchainError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(BlockchainError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_mempool(&self) -> Result<GetMempoolResponse> {
    let url = format!("{}/mempool", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetMempoolResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(BlockchainError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(BlockchainError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_transaction(
    &self,
    GetTransactionParams { txid }: GetTransactionParams,
  ) -> Result<GetTransactionResponse> {
    let url = format!("{}/tx/{}", self.base_url, txid);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetTransactionResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(BlockchainError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(BlockchainError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn post_transaction(
    &self,
    PostTransactionParams { tx_hex }: PostTransactionParams,
  ) -> Result<PostTransactionResponse> {
    let url = format!("{}/tx", self.base_url);

    let json_data = format!(r#"{{"txHex":"{}"}}"#, tx_hex);

    let client = self
      .client
      .post(url)
      .header("Accept", "application/json")
      .body(json_data);

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(BlockchainError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(res) => match res.json::<PostTransactionResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(BlockchainError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(BlockchainError::APIError(e.to_string())),
    };

    Ok(data)
  }
}
