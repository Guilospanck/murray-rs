use serde::{Deserialize, Serialize};

pub mod node_details;
pub mod statistics;
pub mod top_nodes;
use self::{node_details::NodeData, statistics::Statistics, top_nodes::TopData};

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

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetNodeDetailsParams {
  pub public_key: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NodeResponseJsonData {
  pub data: NodeData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StatisticsJsonData {
  pub data: Statistics,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TopJsonData {
  pub data: TopData,
}
