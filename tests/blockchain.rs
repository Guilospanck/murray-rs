use std::fs;

use httpmock::{prelude::*, Method, Mock};
use murray_rs::{
  BaseEndpoints, GetAddressParams, GetBlockParams, GetTransactionParams, Murray,
  PostTransactionParams,
};
use serde_json::Value;

struct Sut {
  server: MockServer,
}

impl Sut {
  fn new() -> Self {
    let server = MockServer::start();
    Sut { server }
  }

  fn from(
    &self,
    path: &str,
    status: u16,
    method: Method,
    req_body: &str,
    res_body: &str,
  ) -> (Mock, Murray) {
    // Create a mock on the server.
    let mock = match method {
      Method::GET
      | Method::HEAD
      | Method::DELETE
      | Method::OPTIONS
      | Method::TRACE
      | Method::CONNECT => self.server.mock(|when, then| {
        when.method(method).path(path);
        then
          .status(status)
          .header("content-type", "application/json")
          .body(res_body);
      }),
      Method::POST | Method::PATCH | Method::PUT => self.server.mock(|when, then| {
        when.method(method).path(path).body(req_body);
        then
          .status(status)
          .header("content-type", "application/json")
          .body(res_body);
      }),
    };

    let mut murray = Murray::default();
    murray.blockchain.set_base_url(self.server.base_url());

    (mock, murray)
  }
}

/// GET BLOCK
#[test]
fn get_block_should_return_successfully() {
  // arrange
  let expected_response =
    fs::read_to_string("tests/mocks/blockchain/block-response.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from("/block", 200, Method::GET, "", &body);

  // act
  let response = murray
    .blockchain
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
  let expected_response =
    fs::read_to_string("tests/mocks/blockchain/block-response.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from("/block", 200, Method::GET, "", &body);

  // act
  let response = murray
    .blockchain
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
  let (_mock, murray) = sut.from("/block", 400, Method::GET, "", &body);

  // act
  let _response = murray
    .blockchain
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
  let (_mock, murray) = sut.from("/block", 200, Method::GET, "", &body);

  // act
  let _response = murray
    .blockchain
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
  let expected_response = fs::read_to_string("tests/mocks/blockchain/block2time-response.json")
    .expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from("/block2time", 200, Method::GET, "", &body);

  // act
  let response = murray
    .blockchain
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
  let expected_response = fs::read_to_string("tests/mocks/blockchain/block2time-response.json")
    .expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from("/block2time", 200, Method::GET, "", &body);

  // act
  let response = murray
    .blockchain
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
  let (_mock, murray) = sut.from("/block2time", 400, Method::GET, "", &body);

  // act
  let _response = murray
    .blockchain
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
  let (_mock, murray) = sut.from("/block2time", 200, Method::GET, "", &body);

  // act
  let _response = murray
    .blockchain
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
  let expected_response = fs::read_to_string("tests/mocks/blockchain/fees-recommended.json")
    .expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from("/fees/recommended", 200, Method::GET, "", &body);

  // act
  let response = murray.blockchain.get_fees_recommended().unwrap();

  // assert
  mock.assert();
  assert_eq!(
    response.fees_recommended.fastest_fee,
    expected_response["fastestFee"]
  );
}

#[test]
#[should_panic]
fn get_fees_recommended_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/fees/recommended", 400, Method::GET, "", &body);

  // act
  let _response = murray.blockchain.get_fees_recommended().unwrap();
}

#[test]
#[should_panic]
fn get_fees_recommended_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/fees/recommended", 200, Method::GET, "", &body);

  // act
  let _response = murray.blockchain.get_fees_recommended().unwrap();
}

/// GET FEES MEMPOOL BLOCKS
#[test]
fn get_fees_mempool_blocks_should_return_successfully() {
  // arrange
  let expected_response = fs::read_to_string("tests/mocks/blockchain/fees-mempool-blocks.json")
    .expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from("/fees/mempool-blocks", 200, Method::GET, "", &body);

  // act
  let response = murray.blockchain.get_fees_mempool_blocks().unwrap();

  // assert
  mock.assert();
  assert_eq!(
    response[0].fees_mempool_blocks.total_fees,
    expected_response[0]["totalFees"]
  );
}

#[test]
#[should_panic]
fn get_fees_mempool_blocks_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/fees/mempool-blocks", 400, Method::GET, "", &body);

  // act
  let _response = murray.blockchain.get_fees_mempool_blocks().unwrap();
}

#[test]
#[should_panic]
fn get_fees_mempool_blocks_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/fees/mempool-blocks", 200, Method::GET, "", &body);

  // act
  let _response = murray.blockchain.get_fees_mempool_blocks().unwrap();
}

/// GET ADDRESS DETAILS
#[test]
fn get_address_details_should_return_successfully() {
  // arrange
  let expected_response = fs::read_to_string("tests/mocks/blockchain/get-address-details.json")
    .expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from("/address/some-address", 200, Method::GET, "", &body);

  // act
  let response = murray
    .blockchain
    .get_address_details(GetAddressParams {
      address: "some-address".to_string(),
    })
    .unwrap();

  // assert
  mock.assert();
  assert_eq!(
    response.address_details.mempool_stats.funded_txo_sum,
    expected_response["mempool_stats"]["funded_txo_sum"]
  );
}

#[test]
#[should_panic]
fn get_address_details_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/address/some-address", 400, Method::GET, "", &body);

  // act
  let _response = murray
    .blockchain
    .get_address_details(GetAddressParams {
      address: "some-address".to_string(),
    })
    .unwrap();
}

#[test]
#[should_panic]
fn get_address_details_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/address/some-address", 200, Method::GET, "", &body);

  // act
  let _response = murray
    .blockchain
    .get_address_details(GetAddressParams {
      address: "some-address".to_string(),
    })
    .unwrap();
}

/// GET ADDRESS TRANSACTIONS
#[test]
fn get_address_transactions_should_return_successfully() {
  // arrange
  let expected_response =
    fs::read_to_string("tests/mocks/blockchain/get-address-txs.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from("/address/some-address/txs", 200, Method::GET, "", &body);

  // act
  let response = murray
    .blockchain
    .get_address_transactions(GetAddressParams {
      address: "some-address".to_string(),
    })
    .unwrap();

  // assert
  mock.assert();
  assert_eq!(
    response[0].address_transactions.txid,
    expected_response[0]["txid"]
  );
}

#[test]
#[should_panic]
fn get_address_transactions_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/address/some-address/txs", 400, Method::GET, "", &body);

  // act
  let _response = murray
    .blockchain
    .get_address_transactions(GetAddressParams {
      address: "some-address".to_string(),
    })
    .unwrap();
}

#[test]
#[should_panic]
fn get_address_transactions_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/address/some-address/txs", 200, Method::GET, "", &body);

  // act
  let _response = murray
    .blockchain
    .get_address_transactions(GetAddressParams {
      address: "some-address".to_string(),
    })
    .unwrap();
}

/// GET ADDRESS UTXOS
#[test]
fn get_address_utxos_should_return_successfully() {
  // arrange
  let expected_response = fs::read_to_string("tests/mocks/blockchain/get-address-utxos.json")
    .expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from(
    "/address/some-address/txs/utxo",
    200,
    Method::GET,
    "",
    &body,
  );

  // act
  let response = murray
    .blockchain
    .get_address_utxos(GetAddressParams {
      address: "some-address".to_string(),
    })
    .unwrap();

  // assert
  mock.assert();
  assert_eq!(response[0].txid, expected_response[0]["txid"]);
}

#[test]
#[should_panic]
fn get_address_utxos_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from(
    "/address/some-address/txs/utxo",
    400,
    Method::GET,
    "",
    &body,
  );

  // act
  let _response = murray
    .blockchain
    .get_address_utxos(GetAddressParams {
      address: "some-address".to_string(),
    })
    .unwrap();
}

#[test]
#[should_panic]
fn get_address_utxos_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from(
    "/address/some-address/txs/utxo",
    200,
    Method::GET,
    "",
    &body,
  );

  // act
  let _response = murray
    .blockchain
    .get_address_utxos(GetAddressParams {
      address: "some-address".to_string(),
    })
    .unwrap();
}

/// GET HASHRATE
#[test]
fn get_hashrate_should_return_successfully() {
  // arrange
  let expected_response = fs::read_to_string("tests/mocks/blockchain/get-hashrate-response.json")
    .expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from("/hashrate", 200, Method::GET, "", &body);

  // act
  let response = murray.blockchain.get_hashrate().unwrap();

  // assert
  mock.assert();
  assert_eq!(
    response.hashrate.current_hashrate,
    expected_response["currentHashrate"]
  );
}

#[test]
#[should_panic]
fn get_hashrate_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/hashrate", 400, Method::GET, "", &body);

  // act
  let _response = murray.blockchain.get_hashrate().unwrap();
}

#[test]
#[should_panic]
fn get_hashrate_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/hashrate", 200, Method::GET, "", &body);

  // act
  let _response = murray.blockchain.get_hashrate().unwrap();
}

/// GET HEALTH
#[test]
fn get_health_should_return_successfully() {
  // arrange
  let expected_response =
    fs::read_to_string("tests/mocks/blockchain/get-health.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from("/health", 200, Method::GET, "", &body);

  // act
  let response = murray.blockchain.get_health().unwrap();

  // assert
  mock.assert();
  assert_eq!(response.message, expected_response["message"]);
}

#[test]
#[should_panic]
fn get_health_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/health", 400, Method::GET, "", &body);

  // act
  let _response = murray.blockchain.get_health().unwrap();
}

#[test]
#[should_panic]
fn get_health_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/health", 200, Method::GET, "", &body);

  // act
  let _response = murray.blockchain.get_health().unwrap();
}

/// GET MEMPOOL
#[test]
fn get_mempool_should_return_successfully() {
  // arrange
  let expected_response =
    fs::read_to_string("tests/mocks/blockchain/get-mempool.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from("/mempool", 200, Method::GET, "", &body);

  // act
  let response = murray.blockchain.get_mempool().unwrap();

  // assert
  mock.assert();
  assert_eq!(
    response.mempool_response.total_fee,
    expected_response["total_fee"]
  );
}

#[test]
#[should_panic]
fn get_mempool_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/mempool", 400, Method::GET, "", &body);

  // act
  let _response = murray.blockchain.get_mempool().unwrap();
}

#[test]
#[should_panic]
fn get_mempool_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/mempool", 200, Method::GET, "", &body);

  // act
  let _response = murray.blockchain.get_mempool().unwrap();
}

/// GET TRANSACTION
#[test]
fn get_transaction_should_return_successfully() {
  // arrange
  let expected_response =
    fs::read_to_string("tests/mocks/blockchain/get-transaction.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let (mock, murray) = sut.from("/tx/some-tx-id", 200, Method::GET, "", &body);

  // act
  let response = murray
    .blockchain
    .get_transaction(GetTransactionParams {
      txid: "some-tx-id".to_string(),
    })
    .unwrap();

  // assert
  mock.assert();
  assert_eq!(response.transaction.txid, expected_response["txid"]);
}

#[test]
#[should_panic]
fn get_transaction_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/tx/some-tx-id", 400, Method::GET, "", &body);

  // act
  let _response = murray
    .blockchain
    .get_transaction(GetTransactionParams {
      txid: "some-tx-id".to_string(),
    })
    .unwrap();
}

#[test]
#[should_panic]
fn get_transaction_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/tx/some-tx-id", 200, Method::GET, "", &body);

  // act
  let _response = murray
    .blockchain
    .get_transaction(GetTransactionParams {
      txid: "some-tx-id".to_string(),
    })
    .unwrap();
}

/// POST TRANSACTION
#[test]
fn post_transaction_should_return_successfully() {
  // arrange
  let expected_response = fs::read_to_string("tests/mocks/blockchain/post-transaction.json")
    .expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response);
  let sut = Sut::new();
  let tx_hex = "some-tx-hex";
  let req_body = format!(r#"{{"txHex":"{}"}}"#, tx_hex);
  let (mock, murray) = sut.from("/tx", 200, Method::POST, &req_body, &body);

  // act
  let response = murray
    .blockchain
    .post_transaction(PostTransactionParams {
      tx_hex: tx_hex.to_string(),
    })
    .unwrap();

  // assert
  mock.assert();
  assert_eq!(response.txid, expected_response["txid"]);
}

#[test]
#[should_panic]
fn post_transaction_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let tx_hex = "some-tx-hex";
  let req_body = format!(r#"{{"txHex":"{}"}}"#, tx_hex);
  let (_mock, murray) = sut.from("/tx", 400, Method::POST, &req_body, &body);

  // act
  let _response = murray
    .blockchain
    .post_transaction(PostTransactionParams {
      tx_hex: tx_hex.to_string(),
    })
    .unwrap();
}

#[test]
#[should_panic]
fn post_transaction_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let tx_hex = "some-tx-hex";
  let req_body = format!(r#"{{"txHex":"{}"}}"#, tx_hex);
  let (_mock, murray) = sut.from("/tx", 200, Method::POST, &req_body, &body);

  // act
  let _response = murray
    .blockchain
    .post_transaction(PostTransactionParams {
      tx_hex: tx_hex.to_string(),
    })
    .unwrap();
}
