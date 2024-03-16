pub mod types;
use std::result;

use lazy_static;
use reqwest::{self, Client};
use std::sync::{Arc, RwLock};

use self::types::{
  GetAddressDetailsResponse, GetAddressDetailsResponseJsonData, GetAddressParams,
  GetAddressTransactionsResponse, GetAddressTransactionsResponseJsonData, GetAddressUTXOResponse,
  GetAddressUTXOResponseJsonData, GetBlock2TimeResponse, GetBlock2TimeResponseJsonData,
  GetBlockParams, GetBlockResponse, GetBlockResponseJsonData, GetFeesMempoolBlocksResponse,
  GetFeesMempoolBlocksResponseJsonData, GetFeesRecommendedResponse,
  GetFeesRecommendedResponseJsonData, GetHashrateResponse, GetHashrateResponseJsonData,
  GetHealthResponse, GetHealthResponseJsonData, GetMempoolResponse, GetMempoolResponseJsonData,
  GetTransactionParams, GetTransactionResponse, GetTransactionResponseJsonData,
  PostTransactionParams, PostTransactionResponse, PostTransactionResponseJsonData,
};

const BASE_URL: &str = "http://blockchain.murrayrothbot.com";

lazy_static::lazy_static! {
  static ref BLOCKCHAIN: Arc<RwLock<Blockchain>> = Arc::new(RwLock::new(Blockchain::new(BASE_URL.to_string())));
}

/// [`Price`] error
#[derive(thiserror::Error, Debug)]
pub enum PriceError {
  #[error("Invalid URL params: `{0}`")]
  InvalidURLParams(String),
  #[error("Bad request: `{0}`")]
  BadRequest(String),
  #[error("API error: `{0}`")]
  APIError(String),
  #[error("JSON parse error: `{0}`")]
  JSONParseError(String),
}

type Result<T> = result::Result<T, PriceError>;

struct Blockchain {
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
      Err(e) => return Err(PriceError::InvalidURLParams(e.to_string())),
    };

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetBlockResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
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
      Err(e) => return Err(PriceError::InvalidURLParams(e.to_string())),
    };

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetBlock2TimeResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_fees_recommended(&self) -> Result<GetFeesRecommendedResponse> {
    let url = format!("{}/fees/recommended", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetFeesRecommendedResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_fees_mempool_blocks(&self) -> Result<Vec<GetFeesMempoolBlocksResponse>> {
    let url = format!("{}/fees/mempool-blocks", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetFeesMempoolBlocksResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
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
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetAddressDetailsResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
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
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetAddressTransactionsResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
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
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetAddressUTXOResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_hashrate(&self) -> Result<GetHashrateResponse> {
    let url = format!("{}/hashrate", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetHashrateResponseJsonData>().await {
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

  #[tokio::main]
  pub async fn get_mempool(&self) -> Result<GetMempoolResponse> {
    let url = format!("{}/mempool", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetMempoolResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
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
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetTransactionResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
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
      Err(e) => return Err(PriceError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(res) => match res.json::<PostTransactionResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(PriceError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(PriceError::APIError(e.to_string())),
    };

    Ok(data)
  }
}

pub fn set_base_url(url: String) {
  let mut blockchain = BLOCKCHAIN.write().unwrap();
  blockchain.set_base_url(url);
}

pub fn get_block(params: GetBlockParams) -> Result<GetBlockResponse> {
  let blockchain = BLOCKCHAIN.read().unwrap();
  blockchain.get_block(params)
}

pub fn get_block2time(params: GetBlockParams) -> Result<GetBlock2TimeResponse> {
  let blockchain = BLOCKCHAIN.read().unwrap();
  blockchain.get_block2time(params)
}

pub fn get_fees_recommended() -> Result<GetFeesRecommendedResponse> {
  let blockchain = BLOCKCHAIN.read().unwrap();
  blockchain.get_fees_recommended()
}

pub fn get_fees_mempool_blocks() -> Result<Vec<GetFeesMempoolBlocksResponse>> {
  let blockchain = BLOCKCHAIN.read().unwrap();
  blockchain.get_fees_mempool_blocks()
}

pub fn get_address_details(params: GetAddressParams) -> Result<GetAddressDetailsResponse> {
  let blockchain = BLOCKCHAIN.read().unwrap();
  blockchain.get_address_details(params)
}

pub fn get_address_transactions(
  params: GetAddressParams,
) -> Result<Vec<GetAddressTransactionsResponse>> {
  let blockchain: std::sync::RwLockReadGuard<'_, Blockchain> = BLOCKCHAIN.read().unwrap();
  blockchain.get_address_transactions(params)
}

pub fn get_address_utxos(params: GetAddressParams) -> Result<Vec<GetAddressUTXOResponse>> {
  let blockchain = BLOCKCHAIN.read().unwrap();
  blockchain.get_address_utxos(params)
}

pub fn get_hashrate() -> Result<GetHashrateResponse> {
  let blockchain = BLOCKCHAIN.read().unwrap();
  blockchain.get_hashrate()
}

pub fn get_health() -> Result<GetHealthResponse> {
  let blockchain = BLOCKCHAIN.read().unwrap();
  blockchain.get_health()
}

pub fn get_mempool() -> Result<GetMempoolResponse> {
  let blockchain = BLOCKCHAIN.read().unwrap();
  blockchain.get_mempool()
}

pub fn get_transaction(params: GetTransactionParams) -> Result<GetTransactionResponse> {
  let blockchain = BLOCKCHAIN.read().unwrap();
  blockchain.get_transaction(params)
}

pub fn post_transaction(params: PostTransactionParams) -> Result<PostTransactionResponse> {
  let blockchain = BLOCKCHAIN.read().unwrap();
  blockchain.post_transaction(params)
}

#[cfg(test)]
mod tests {

  use crate::blockchain::types::GetBlockParams;
  use httpmock::{prelude::*, Method, Mock};
  use serde::Serialize;
  use serde_json::json;

  use super::{
    types::block::{self, Block},
    Blockchain,
  };

  struct Sut {
    server: MockServer,
  }

  impl Sut {
    fn new() -> Self {
      let server = MockServer::start();
      Sut { server }
    }

    fn from(&self, path: &str, status: u16, method: Method, body: &str) -> (Mock, Blockchain) {
      let url = self.server.base_url();

      // Create a mock on the server.
      let mock = self.server.mock(|when, then| {
        when.method(method).path(path);
        then
          .status(status)
          .header("content-type", "application/json")
          .body(body);
      });

      let blockchain = Blockchain::new(url);

      return (mock, blockchain);
    }
  }

  /// GET BLOCK
  #[test]
  fn get_block_should_return_successfully() {
    // arrange
    let expected_response = json!({
      "id": "00000000000000000007566f8f035a1dc38b351e6f54778b311fe6dbabd79b46",
      "height": 736941,
      "version": 536870916,
      "timestamp": 1652891466,
      "bits": 386466234,
      "nonce": 3514220842u32,
      "difficulty": 31251101365711.12,
      "merkle_root": "4a3072f98f60cbb639bb7f46180b8843d17c7502627ffb633db0ed86610cdd71",
      "tx_count": 2381,
      "size": 1709571,
      "weight": 3997770,
      "previousblockhash": "00000000000000000005ef14db0b4befcbbe1e9b8676eec67fcf810a899c4d5e",
      "extras": { "reward": 638307429, "coinbaseTx": { "vin": [ { "scriptsig": "03ad3e0b2cfabe6d6df8fb5429a5de5fc2bd1bafffbc90d33c77eb73307d51931d247f21d7bccde51710000000f09f909f092f4632506f6f6c2f6b0000000000000000000000000000000000000000000000000000000000000000000000050086411100" } ], "vout": [ { "scriptpubkey_address": "1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY", "value": 638307429 } ] }, "coinbaseRaw": "03ad3e0b2cfabe6d6df8fb5429a5de5fc2bd1bafffbc90d33c77eb73307d51931d247f21d7bccde51710000000f09f909f092f4632506f6f6c2f6b0000000000000000000000000000000000000000000000000000000000000000000000050086411100", "medianFee": 10, "feeRange": [ 1, 8, 9, 10, 15, 21, 348 ], "totalFees": 13307429, "avgFee": 5591, "avgFeeRate": 13, "pool": { "id": 36, "name": "F2Pool", "slug": "f2pool" }, "matchRate": 93 }
    });
    let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
    let sut = Sut::new();
    let (mock, blockchain) = sut.from("/block", 200, Method::GET, &body);

    // act
    let response = blockchain
      .get_block(GetBlockParams {
        hash: Some("some_value".to_string()),
        height: Some(50000),
      })
      .unwrap();

    // assert
    mock.assert();
    assert_eq!(response.block.id, expected_response["id"]);
  }

  #[test]
  fn get_block_should_return_successfully_when_no_params() {
    // arrange
    let expected_response = json!({
      "id": "00000000000000000007566f8f035a1dc38b351e6f54778b311fe6dbabd79b46",
      "height": 736941,
      "version": 536870916,
      "timestamp": 1652891466,
      "bits": 386466234,
      "nonce": 3514220842u32,
      "difficulty": 31251101365711.12,
      "merkle_root": "4a3072f98f60cbb639bb7f46180b8843d17c7502627ffb633db0ed86610cdd71",
      "tx_count": 2381,
      "size": 1709571,
      "weight": 3997770,
      "previousblockhash": "00000000000000000005ef14db0b4befcbbe1e9b8676eec67fcf810a899c4d5e",
      "extras": { "reward": 638307429, "coinbaseTx": { "vin": [ { "scriptsig": "03ad3e0b2cfabe6d6df8fb5429a5de5fc2bd1bafffbc90d33c77eb73307d51931d247f21d7bccde51710000000f09f909f092f4632506f6f6c2f6b0000000000000000000000000000000000000000000000000000000000000000000000050086411100" } ], "vout": [ { "scriptpubkey_address": "1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY", "value": 638307429 } ] }, "coinbaseRaw": "03ad3e0b2cfabe6d6df8fb5429a5de5fc2bd1bafffbc90d33c77eb73307d51931d247f21d7bccde51710000000f09f909f092f4632506f6f6c2f6b0000000000000000000000000000000000000000000000000000000000000000000000050086411100", "medianFee": 10, "feeRange": [ 1, 8, 9, 10, 15, 21, 348 ], "totalFees": 13307429, "avgFee": 5591, "avgFeeRate": 13, "pool": { "id": 36, "name": "F2Pool", "slug": "f2pool" }, "matchRate": 93 }
    });
    let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
    let sut = Sut::new();
    let (mock, blockchain) = sut.from("/block", 200, Method::GET, &body);

    // act
    let response = blockchain
      .get_block(GetBlockParams {
        hash: None,
        height: None,
      })
      .unwrap();

    // assert
    mock.assert();
    assert_eq!(response.block.id, expected_response["id"]);
  }

  #[test]
  #[should_panic]
  fn get_block_should_return_error_when_problem_with_server() {
    // arrange
    let body = "".to_string();
    let sut = Sut::new();
    let (_mock, blockchain) = sut.from("/block", 400, Method::GET, &body);

    // act
    let _response = blockchain
      .get_block(GetBlockParams {
        hash: None,
        height: None,
      })
      .unwrap();
  }

  #[test]
  #[should_panic]
  fn get_block_should_return_error_when_body_returns_wrong_json() {
    // arrange
    let body = "wrong-return".to_string();
    let sut = Sut::new();
    let (_mock, blockchain) = sut.from("/block", 200, Method::GET, &body);

    // act
    let _response = blockchain
      .get_block(GetBlockParams {
        hash: None,
        height: None,
      })
      .unwrap();
  }

  /// GET BLOCK2TIME
  #[test]
  fn get_block2time_should_return_successfully() {
    // arrange
    let expected_response = json!({
      "height": 736941,
      "timestamp": 1652891466,
      "in_future": false,
    });
    let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
    let sut = Sut::new();
    let (mock, blockchain) = sut.from("/block2time", 200, Method::GET, &body);

    // act
    let response = blockchain
      .get_block2time(GetBlockParams {
        hash: Some("some_value".to_string()),
        height: Some(50000),
      })
      .unwrap();

    // assert
    mock.assert();
    assert_eq!(response.block2time.height, expected_response["height"]);
  }

  #[test]
  fn get_block2time_should_return_successfully_when_no_params() {
    // arrange
    let expected_response = json!({
      "height": 736941,
      "timestamp": 1652891466,
      "in_future": false,
    });
    let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
    let sut = Sut::new();
    let (mock, blockchain) = sut.from("/block2time", 200, Method::GET, &body);

    // act
    let response = blockchain
      .get_block2time(GetBlockParams {
        hash: None,
        height: None,
      })
      .unwrap();

    // assert
    mock.assert();
    assert_eq!(response.block2time.height, expected_response["height"]);
  }

  #[test]
  #[should_panic]
  fn get_block2time_should_return_error_when_problem_with_server() {
    // arrange
    let body = "".to_string();
    let sut = Sut::new();
    let (_mock, blockchain) = sut.from("/block2time", 400, Method::GET, &body);

    // act
    let _response = blockchain
      .get_block2time(GetBlockParams {
        hash: None,
        height: None,
      })
      .unwrap();
  }

  #[test]
  #[should_panic]
  fn get_block2time_should_return_error_when_body_returns_wrong_json() {
    // arrange
    let body = "wrong-return".to_string();
    let sut = Sut::new();
    let (_mock, blockchain) = sut.from("/block2time", 200, Method::GET, &body);

    // act
    let _response = blockchain
      .get_block2time(GetBlockParams {
        hash: None,
        height: None,
      })
      .unwrap();
  }

  /// GET FEES RECOMMENDED
  #[test]
  fn get_fees_recommended_should_return_successfully() {
    // arrange
    let expected_response = json!({
      "fastestFee": 1,
      "halfHourFee": 2,
      "hourFee": 3,
      "economyFee": 4,
      "minimumFee": 5
    });
    let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
    let sut = Sut::new();
    let (mock, blockchain) = sut.from("/fees/recommended", 200, Method::GET, &body);

    // act
    let response = blockchain
      .get_fees_recommended()
      .unwrap();

    // assert
    mock.assert();
    assert_eq!(response.fees_recommended.fastest_fee, expected_response["fastestFee"]);
  }

  #[test]
  fn get_fees_recommended_should_return_successfully_when_no_params() {
    // arrange
    let expected_response = json!({
      "fastestFee": 1,
      "halfHourFee": 2,
      "hourFee": 3,
      "economyFee": 4,
      "minimumFee": 5
    });
    let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
    let sut = Sut::new();
    let (mock, blockchain) = sut.from("/fees/recommended", 200, Method::GET, &body);

    // act
    let response = blockchain
      .get_fees_recommended()
      .unwrap();

    // assert
    mock.assert();
    assert_eq!(response.fees_recommended.fastest_fee, expected_response["fastestFee"]);
  }

  #[test]
  #[should_panic]
  fn get_fees_recommended_should_return_error_when_problem_with_server() {
    // arrange
    let body = "".to_string();
    let sut = Sut::new();
    let (_mock, blockchain) = sut.from("/fees/recommended", 400, Method::GET, &body);

    // act
    let _response = blockchain
      .get_fees_recommended()
      .unwrap();
  }

  #[test]
  #[should_panic]
  fn get_fees_recommended_should_return_error_when_body_returns_wrong_json() {
    // arrange
    let body = "wrong-return".to_string();
    let sut = Sut::new();
    let (_mock, blockchain) = sut.from("/fees/recommended", 200, Method::GET, &body);

    // act
    let _response = blockchain
      .get_fees_recommended()
      .unwrap();
  }


}
