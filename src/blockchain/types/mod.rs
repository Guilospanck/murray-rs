use serde::{Deserialize, Serialize};

pub mod block;
pub mod fee;
pub mod transaction;
pub mod address;
pub mod mining;

use self::{address::AddressDetails, block::{Block, Block2Time, Extras}, fee::{FeesMempoolBlocks, FeesRecommended}, mining::HashrateData, transaction::{Transaction, TransactionStatus}};

pub struct GetBlockParams {
  pub hash: Option<String>,
  pub height: Option<u32>,
}

pub struct GetAddressParams {
  pub address: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetBlockResponse {
  #[serde(flatten)]
  pub block: Block,
  pub extras: Extras,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetBlock2TimeResponse {
  #[serde(flatten)]
  pub block2time: Block2Time
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetFeesRecommendedResponse {
  #[serde(flatten)]
  pub fees_recommended: FeesRecommended
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetFeesMempoolBlocksResponse {
  #[serde(flatten)]
  pub fees_mempool_blocks: FeesMempoolBlocks
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAddressDetailsResponse {
  #[serde(flatten)]
  pub address_details: AddressDetails
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAddressTransactionsResponse {
  #[serde(flatten)]
  pub address_transactions: Transaction
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAddressUTXOResponse {
  pub txid: String,
  pub vout: u32,
  pub status: TransactionStatus,
  pub value: u64
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetHashrateResponse {
  #[serde(flatten)]
  pub hashrate: HashrateData
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

#[derive(Deserialize, Serialize)]
pub struct GetAddressTransactionsResponseJsonData {
  pub data: Vec<GetAddressTransactionsResponse>,
}

#[derive(Deserialize, Serialize)]
pub struct GetAddressUTXOResponseJsonData {
  pub data: Vec<GetAddressUTXOResponse>,
}

#[derive(Deserialize, Serialize)]
pub struct GetHashrateResponseJsonData {
  pub data: GetHashrateResponse,
}