use serde::{Deserialize, Serialize};

pub mod node_details;
pub mod statistics;
pub mod top_nodes;
use self::{node_details::NodeData, statistics::Statistics, top_nodes::TopData};

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
