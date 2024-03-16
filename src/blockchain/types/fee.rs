use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeesRecommended {
  pub fastest_fee: u64,
  pub half_hour_fee: u64,
  pub hour_fee: u64,
  pub economy_fee: u64,
  pub minimum_fee: u64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FeesMempoolBlocks {
  pub block_size: u32,
  pub block_v_size: f32,
  pub n_tx: u32,
  pub total_fees: u64,
  pub median_fee: f64,
  pub fee_range: Vec<f64>,
}