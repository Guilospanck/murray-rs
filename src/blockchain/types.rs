use serde::{Deserialize, Serialize};

pub struct GetBlockParams {
  pub hash: Option<String>,
  pub height: Option<u32>,
}

pub struct GetAddressParams {
  pub address: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Pool {
  pub id: u64,
  pub name: String,
  pub slug: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Extras {
  pub avg_fee: Option<f64>,
  pub avg_fee_rate: Option<f64>,
  pub avg_tx_size: Option<f64>,
  pub coinbase_address: Option<String>,
  pub coinbase_raw: Option<String>,
  pub coinbase_signature: Option<String>,
  pub coinbase_signature_ascii: Option<String>,
  pub expected_fees: Option<u64>,
  pub expected_weight: Option<u64>,
  pub fee_percentiles: Option<Vec<f64>>,
  pub fee_range: Option<Vec<f64>>,
  pub header: Option<String>,
  pub match_rate: Option<f64>,
  pub median_fee: Option<f64>,
  pub median_fee_amt: Option<f64>,
  pub orphans: Option<Vec<Block>>,
  pub pool: Option<Pool>,
  pub reward: Option<u64>,
  pub segwit_total_size: Option<u64>,
  pub segwit_total_txs: Option<u64>,
  pub segwit_total_weight: Option<u64>,
  pub similarity: Option<f64>,
  pub total_fees: Option<u64>,
  pub total_input_amt: Option<f64>,
  pub total_inputs: Option<u64>,
  pub total_output_amt: Option<u64>,
  pub total_outputs: Option<u64>,
  pub utxo_set_change: Option<f64>,
  pub utxo_set_size: Option<f64>,
  pub virtual_size: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Block {
  pub id: String,
  pub height: u32,
  pub version: u32,
  pub timestamp: u32,
  pub bits: u32,
  pub nonce: u32,
  pub difficulty: f64,
  pub merkle_root: String,
  pub tx_count: u16,
  pub size: u32,
  pub weight: u32,
  pub previousblockhash: String,
  pub mediantime: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetBlockResponse {
  #[serde(flatten)]
  pub block: Block,
  pub extras: Extras,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetBlock2TimeResponse {
  pub timestamp: u64,
  pub height: u32,
  pub in_future: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFeesRecommendedResponse {
  pub fastest_fee: u64,
  pub half_hour_fee: u64,
  pub hour_fee: u64,
  pub economy_fee: u64,
  pub minimum_fee: u64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetFeesMempoolBlocksResponse {
  pub block_size: u32,
  pub block_v_size: f32,
  pub n_tx: u32,
  pub total_fees: u64,
  pub median_fee: f64,
  pub fee_range: Vec<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Stats {
  pub funded_txo_count: u64,
  pub funded_txo_sum: u64,
  pub spent_txo_count: u64,
  pub spent_txo_sum: u64,
  pub tx_count: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAddressDetailsResponse {
  pub address: String,
  pub chain_stats: Stats,
  pub mempool_stats: Stats,
}

#[derive(Deserialize, Serialize)]
pub struct GetBlockResponseJsonData {
  pub data: GetBlockResponse,
}

#[derive(Deserialize, Serialize)]
pub struct GetBlock2TimeResponseJsonData {
  pub data: GetBlock2TimeResponse,
}

#[derive(Deserialize, Serialize)]
pub struct GetFeesRecommendedResponseJsonData {
  pub data: GetFeesRecommendedResponse,
}

#[derive(Deserialize, Serialize)]
pub struct GetFeesMempoolBlocksResponseJsonData {
  pub data: Vec<GetFeesMempoolBlocksResponse>,
}

#[derive(Deserialize, Serialize)]
pub struct GetAddressDetailsResponseJsonData {
  pub data: GetAddressDetailsResponse,
}
