use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DifficultyEntry {
  pub time: u64,
  pub height: u64,
  pub difficulty: f64,
  pub adjustment: f64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HashratesEntry {
  pub timestamp: u64,
  pub avg_hashrate: f64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HashrateData {
  pub progress_percent: f64,
  pub difficulty_change: f64,
  pub estimated_retarget_date: u64,
  pub remaining_blocks: u64,
  pub remaining_time: u64,
  pub previous_retarget: f64,
  pub previous_time: u64,
  pub next_retarget_height: u64,
  pub time_avg: f64,
  pub adjusted_time_avg: f64,
  pub time_offset: f64,
  pub expected_blocks: f64,
  pub hashrates: Vec<HashratesEntry>,
  pub difficulty: Vec<DifficultyEntry>,
  pub current_hashrate: f64,
  pub current_difficulty: f64,
}
