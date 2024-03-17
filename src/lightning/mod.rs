pub mod types;
use std::result;

use reqwest::{self, Client};

use self::types::{
  node_details::NodeData, statistics::Statistics, top_nodes::TopData, GetNodeDetailsParams,
  NodeResponseJsonData, StatisticsJsonData, TopJsonData,
};

/// [`Lightning`] error
#[derive(thiserror::Error, Debug)]
pub enum LightningError {
  #[error("Invalid URL params: `{0}`")]
  InvalidURLParams(String),
  #[error("Bad request: `{0}`")]
  BadRequest(String),
  #[error("API error: `{0}`")]
  APIError(String),
  #[error("JSON parse error: `{0}`")]
  JSONParseError(String),
}

type Result<T> = result::Result<T, LightningError>;

pub struct Lightning {
  base_url: String,
  client: Client,
}

impl Lightning {
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
  pub async fn get_node_details(
    &self,
    GetNodeDetailsParams { public_key }: GetNodeDetailsParams,
  ) -> Result<NodeData> {
    let url = format!("{}/node/{}", self.base_url, public_key);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(LightningError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<NodeResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(LightningError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(LightningError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_statistics(&self) -> Result<Statistics> {
    let url = format!("{}/statistics", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(LightningError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<StatisticsJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(LightningError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(LightningError::APIError(e.to_string())),
    };

    Ok(data)
  }

  #[tokio::main]
  pub async fn get_top_nodes(&self) -> Result<TopData> {
    let url = format!("{}/top", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(LightningError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<TopJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(LightningError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(LightningError::APIError(e.to_string())),
    };

    Ok(data)
  }
}
