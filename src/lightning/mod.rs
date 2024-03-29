pub mod types;
use std::result;

use reqwest::Client;

use crate::{GetHealthResponse, GetHealthResponseJsonData};

use self::types::{
  node_details::NodeData, statistics::Statistics, top_nodes::TopData, GetNodeDetailsParams,
  NodeResponseJsonData, StatisticsJsonData, TopJsonData, LightningError,
};

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

  /// Change the base url on the fly for the [`Lightning`] calls.
  /// 
  pub fn set_base_url(&mut self, base_url: String) {
    self.base_url = base_url;
  }

  /// Get information regarding a lightning
  /// node of a specific [`public_key`](self::types::GetNodeDetailsParams).
  /// 
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

  /// Get network-wide statistics from the lightning network,
  /// such as total number of channels and nodes, total capacity,
  /// average/median fee figures and more.
  /// 
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

  /// Get two lists of the top nodes:
  /// one ordered by liquidity (aggregate channel capacity)
  /// and the other ordered by connectivity (number of open channels).
  /// 
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

  /// Get simple information regarding the health
  /// of the lightning service.
  /// 
  /// More info at [service-lightning](https://github.com/murray-rothbot/service-lightning).
  /// 
  #[tokio::main]
  pub async fn get_health(&self) -> Result<GetHealthResponse> {
    let url = format!("{}/health", self.base_url);

    let client = self.client.get(url).header("Accept", "application/json");

    let server_response = match client.send().await {
      Ok(resp) => resp.error_for_status(),
      Err(e) => return Err(LightningError::BadRequest(e.to_string())),
    };

    let data = match server_response {
      Ok(resp) => match resp.json::<GetHealthResponseJsonData>().await {
        Ok(r) => r.data,
        Err(e) => return Err(LightningError::JSONParseError(e.to_string())),
      },
      Err(e) => return Err(LightningError::APIError(e.to_string())),
    };

    Ok(data)
  }
}
