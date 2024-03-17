use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::node_details::NodeCountry;

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeInfo {
  #[serde(rename = "publicKey")]
  pub public_key: String,
  pub alias: String,
  pub capacity: Option<i64>, // Present in topByCapacity
  pub channels: Option<i64>, // Present in topByChannels
  pub city: Option<HashMap<String, String>>,
  pub country: Option<NodeCountry>,
  pub iso_code: Option<String>,
  pub subdivision: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopData {
  pub top_by_capacity: Vec<NodeInfo>,
  pub top_by_channels: Vec<NodeInfo>,
}
