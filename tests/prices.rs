use std::fs;

use httpmock::{prelude::*, Method, Mock};
use murray_rs::{ConvertCurrencyParams, Murray};
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
    murray.prices.set_base_url(self.server.base_url());

    return (mock, murray);
  }
}

/// CONVERT CURRENCY
#[test]
fn convert_currency_should_return_successfully() {
  // arrange
  let expected_response =
    fs::read_to_string("tests/mocks/prices/convert-response.json").expect("Unable to read file");
  let expected_response: Value = serde_json::from_str(&expected_response).expect("Unable to parse");
  let body = format!(r#"{{"data":  {}}}"#, expected_response.to_string());
  let sut = Sut::new();
  let (mock, murray) = sut.from("/convert", 200, Method::GET, "", &body);

  // act
  let response = murray
    .prices
    .convert_currency(ConvertCurrencyParams {
      currency: murray_rs::Currency::BRL,
      value: 100,
    })
    .unwrap();

  // assert
  mock.assert();
  assert_eq!(response.btc, expected_response["btc"]);
}

#[test]
#[should_panic]
fn convert_currency_should_return_error_when_problem_with_server() {
  // arrange
  let body = "".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/convert", 400, Method::GET, "", &body);

  // act
  let _response = murray
    .prices
    .convert_currency(ConvertCurrencyParams {
      currency: murray_rs::Currency::BRL,
      value: 100,
    })
    .unwrap();
}

#[test]
#[should_panic]
fn convert_currency_should_return_error_when_body_returns_wrong_json() {
  // arrange
  let body = "wrong-return".to_string();
  let sut = Sut::new();
  let (_mock, murray) = sut.from("/convert", 200, Method::GET, "", &body);

  // act
  let _response = murray
    .prices
    .convert_currency(ConvertCurrencyParams {
      currency: murray_rs::Currency::BRL,
      value: 100,
    })
    .unwrap();
}
