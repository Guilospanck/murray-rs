use std::fs;

use httpmock::{prelude::*, Method, Mock};
use murray_rs::{GetNodeDetailsParams, Murray};
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

    let mut murray = Murray::new();
    murray.lightning.set_base_url(self.server.base_url());

    return (mock, murray);
  }
}

/// GET NODE DETAILS
#[test]
fn get_node_details_should_return_successfully() {
  // arrange
  let expected_response =
    fs::read_to_string("tests/mocks/lightning/node-details.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let pub_key_param = String::from("some-pubkey");
  let sut = Sut::new();
  let (mock, murray) = sut.from(
    format!("/node/{}", pub_key_param).as_ref(),
    200,
    Method::GET,
    "",
    &body,
  );

  // act
  let response = murray
    .lightning
    .get_node_details(GetNodeDetailsParams {
      public_key: pub_key_param,
    })
    .unwrap();

  // assert
  mock.assert();
  assert_eq!(response.capacity, expected_response["capacity"]);
}

#[test]
#[should_panic]
fn get_node_details_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let pub_key_param = String::from("some-pubkey");
  let sut = Sut::new();
  let (_mock, murray) = sut.from(
    format!("/node/{}", pub_key_param).as_ref(),
    400,
    Method::GET,
    "",
    &body,
  );

  // act
  let _response = murray
    .lightning
    .get_node_details(GetNodeDetailsParams {
      public_key: pub_key_param,
    })
    .unwrap();
}

/// GET HEALTH
#[test]
fn get_health_should_return_successfully() {
  // arrange
  let expected_response =
    fs::read_to_string("tests/mocks/lightning/get-health.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let sut = Sut::new();
  let (mock, murray) = sut.from("/health", 200, Method::GET, "", &body);

  // act
  let response = murray.lightning.get_health().unwrap();

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
  let _response = murray.lightning.get_health().unwrap();
}

#[test]
#[should_panic]
fn get_health_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/health", 200, Method::GET, "", &body);

  // act
  let _response = murray.lightning.get_health().unwrap();
}

/// GET STATISTICS
#[test]
fn get_statistics_should_return_successfully() {
  // arrange
  let expected_response =
    fs::read_to_string("tests/mocks/lightning/statistics.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let sut = Sut::new();
  let (mock, murray) = sut.from("/statistics", 200, Method::GET, "", &body);

  // act
  let response = murray.lightning.get_statistics().unwrap();

  // assert
  mock.assert();
  assert_eq!(response.latest.id, expected_response["latest"]["id"]);
}

#[test]
#[should_panic]
fn get_statistics_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/statistics", 400, Method::GET, "", &body);

  // act
  let _response = murray.lightning.get_statistics().unwrap();
}

#[test]
#[should_panic]
fn get_statistics_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/statistics", 200, Method::GET, "", &body);

  // act
  let _response = murray.lightning.get_statistics().unwrap();
}


/// GET TOP NODES
#[test]
fn get_top_nodes_should_return_successfully() {
  // arrange
  let expected_response =
    fs::read_to_string("tests/mocks/lightning/top-nodes.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let sut = Sut::new();
  let (mock, murray) = sut.from("/top", 200, Method::GET, "", &body);

  // act
  let response = murray.lightning.get_top_nodes().unwrap();

  // assert
  mock.assert();
  assert_eq!(response.top_by_channels[0].public_key, expected_response["topByChannels"][0]["publicKey"]);
}

#[test]
#[should_panic]
fn get_top_nodes_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/top", 400, Method::GET, "", &body);

  // act
  let _response = murray.lightning.get_top_nodes().unwrap();
}

#[test]
#[should_panic]
fn get_top_nodes_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/top", 200, Method::GET, "", &body);

  // act
  let _response = murray.lightning.get_top_nodes().unwrap();
}
