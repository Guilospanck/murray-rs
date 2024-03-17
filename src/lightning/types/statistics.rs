use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct StatisticsData {
  pub id: u64,
  pub added: String, // ISO 8601 date format
  pub channel_count: u64,
  pub node_count: u64,
  pub total_capacity: u64,
  pub tor_nodes: u64,
  pub clearnet_nodes: u64,
  pub unannounced_nodes: u64,
  pub avg_capacity: u64,
  pub avg_fee_rate: u64,
  pub avg_base_fee_mtokens: u64,
  pub med_capacity: u64,
  pub med_fee_rate: u64,
  pub med_base_fee_mtokens: u64,
  pub clearnet_tor_nodes: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Statistics {
  pub latest: StatisticsData,
  pub previous: StatisticsData,
}
