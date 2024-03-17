use serde::{Deserialize, Serialize};

pub mod node_details;
pub mod statistics;
use self::{node_details::NodeData, statistics::Statistics};


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