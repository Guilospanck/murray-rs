use std::fs;

use httpmock::{prelude::*, Method, Mock};
use murray_rs::{GetBlockParams, Murray};
use serde_json::Value;

struct Sut {
  server: MockServer,
}

impl Sut {
  fn new() -> Self {
    let server = MockServer::start();
    Sut { server }
  }

  fn from(&self, path: &str, status: u16, method: Method, body: &str) -> (Mock, Murray) {
    // Create a mock on the server.
    let mock = self.server.mock(|when, then| {
      when.method(method).path(path);
      then
        .status(status)
        .header("content-type", "application/json")
        .body(body);
    });

    let mut murray = Murray::new();
    murray.blockchain.set_base_url(self.server.base_url());

    return (mock, murray);
  }
}

/// GET BLOCK
#[test]
fn get_block_should_return_successfully() {
  // arrange
  let expected_response = fs::read_to_string("tests/mocks/block-response.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let sut = Sut::new();
  let (mock, murray) = sut.from("/block", 200, Method::GET, &body);

  // act
  let response = murray.blockchain.get_block(GetBlockParams {
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
  let expected_response = fs::read_to_string("tests/mocks/block-response.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let sut = Sut::new();
  let (mock, murray) = sut.from("/block", 200, Method::GET, &body);

  // act
  let response = murray.blockchain.get_block(GetBlockParams {
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
  let (_mock, murray) = sut.from("/block", 400, Method::GET, &body);

  // act
  let _response = murray.blockchain.get_block(GetBlockParams {
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
  let (_mock, murray) = sut.from("/block", 200, Method::GET, &body);

  // act
  let _response = murray.blockchain.get_block(GetBlockParams {
    hash: None,
    height: None,
  })
  .unwrap();
}

/// GET BLOCK2TIME
#[test]
fn get_block2time_should_return_successfully() {
  // arrange
  let expected_response = fs::read_to_string("tests/mocks/block2time-response.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let sut = Sut::new();
  let (mock, murray) = sut.from("/block2time", 200, Method::GET, &body);

  // act
  let response = murray.blockchain.get_block2time(GetBlockParams {
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
  let expected_response = fs::read_to_string("tests/mocks/block2time-response.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let sut = Sut::new();
  let (mock, murray) = sut.from("/block2time", 200, Method::GET, &body);

  // act
  let response = murray.blockchain.get_block2time(GetBlockParams {
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
  let (_mock, murray) = sut.from("/block2time", 400, Method::GET, &body);

  // act
  let _response = murray.blockchain.get_block2time(GetBlockParams {
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
  let (_mock, murray) = sut.from("/block2time", 200, Method::GET, &body);

  // act
  let _response = murray.blockchain.get_block2time(GetBlockParams {
    hash: None,
    height: None,
  })
  .unwrap();
}

/// GET FEES RECOMMENDED
#[test]
fn get_fees_recommended_should_return_successfully() {
  // arrange
  let expected_response = fs::read_to_string("tests/mocks/fees-recommended.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let sut = Sut::new();
  let (mock, murray) = sut.from("/fees/recommended", 200, Method::GET, &body);

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
fn get_fees_recommended_should_return_successfully_when_no_params() {
  // arrange
  let expected_response = fs::read_to_string("tests/mocks/fees-recommended.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let sut = Sut::new();
  let (mock, murray) = sut.from("/fees/recommended", 200, Method::GET, &body);

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
  let (_mock, murray) = sut.from("/fees/recommended", 400, Method::GET, &body);

  // act
  let _response = murray.blockchain.get_fees_recommended().unwrap();
}

#[test]
#[should_panic]
fn get_fees_recommended_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/fees/recommended", 200, Method::GET, &body);

  // act
  let _response = murray.blockchain.get_fees_recommended().unwrap();
}

/// GET FEES MEMPOOL BLOCKS
#[test]
fn get_fees_mempool_blocks_should_return_successfully() {
  // arrange
  let expected_response = fs::read_to_string("tests/mocks/fees-mempool-blocks.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let sut = Sut::new();
  let (mock, murray) = sut.from("/fees/mempool-blocks", 200, Method::GET, &body);

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
fn get_fees_mempool_blocks_should_return_successfully_when_no_params() {
  // arrange
  let expected_response = fs::read_to_string("tests/mocks/fees-mempool-blocks.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let sut = Sut::new();
  let (mock, murray) = sut.from("/fees/mempool-blocks", 200, Method::GET, &body);

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
  let (_mock, murray) = sut.from("/fees/mempool-blocks", 400, Method::GET, &body);

  // act
  let _response = murray.blockchain.get_fees_mempool_blocks().unwrap();
}

#[test]
#[should_panic]
fn get_fees_mempool_blocks_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/fees/mempool-blocks", 200, Method::GET, &body);

  // act
  let _response = murray.blockchain.get_fees_mempool_blocks().unwrap();
}
