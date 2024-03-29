use serde::{Deserialize, Serialize};

pub mod block;
pub mod fee;
pub mod transaction;
pub mod address;
pub mod mining;

use self::{address::AddressDetails, block::{Block, Block2Time, Extras}, fee::{FeesMempoolBlocks, FeesRecommended, MempoolData}, mining::HashrateData, transaction::{Transaction, TransactionStatus}};

/// [`Blockchain`] error
#[derive(thiserror::Error, Debug)]
pub enum BlockchainError {
  #[error("Invalid URL params: `{0}`")]
  InvalidURLParams(String),
  #[error("Bad request: `{0}`")]
  BadRequest(String),
  #[error("API error: `{0}`")]
  APIError(String),
  #[error("JSON parse error: `{0}`")]
  JSONParseError(String),
}

pub struct GetBlockParams {
  pub hash: Option<String>,
  pub height: Option<u32>,
}

pub struct GetAddressParams {
  pub address: String,
}

pub struct GetTransactionParams {
  pub txid: String,
}

pub struct PostTransactionParams {
  pub tx_hex: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetBlockResponse {
  #[serde(flatten)]
  pub block: Block,
  pub extras: Option<Extras>,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct GetMempoolResponse {
  #[serde(flatten)]
  pub mempool_response: MempoolData
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetTransactionResponse {
  #[serde(flatten)]
  pub transaction: Transaction
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PostTransactionResponse {
  pub txid: String
}

#[derive(Deserialize, Serialize)]
pub(super) struct GetBlockResponseJsonData {
  pub data: GetBlockResponse,
}

#[derive(Deserialize, Serialize)]
pub(super) struct GetBlock2TimeResponseJsonData {
  pub data: GetBlock2TimeResponse,
}

#[derive(Deserialize, Serialize)]
pub(super) struct GetFeesRecommendedResponseJsonData {
  pub data: GetFeesRecommendedResponse,
}

#[derive(Deserialize, Serialize)]
pub(super) struct GetFeesMempoolBlocksResponseJsonData {
  pub data: Vec<GetFeesMempoolBlocksResponse>,
}

#[derive(Deserialize, Serialize)]
pub(super) struct GetAddressDetailsResponseJsonData {
  pub data: GetAddressDetailsResponse,
}

#[derive(Deserialize, Serialize)]
pub(super) struct GetAddressTransactionsResponseJsonData {
  pub data: Vec<GetAddressTransactionsResponse>,
}

#[derive(Deserialize, Serialize)]
pub(super) struct GetAddressUTXOResponseJsonData {
  pub data: Vec<GetAddressUTXOResponse>,
}

#[derive(Deserialize, Serialize)]
pub(super) struct GetHashrateResponseJsonData {
  pub data: GetHashrateResponse,
}

#[derive(Deserialize, Serialize)]
pub(super) struct GetMempoolResponseJsonData {
  pub data: GetMempoolResponse,
}

#[derive(Deserialize, Serialize)]
pub(super) struct GetTransactionResponseJsonData {
  pub data: GetTransactionResponse,
}

#[derive(Deserialize, Serialize)]
pub(super) struct PostTransactionResponseJsonData {
  pub data: PostTransactionResponse,
}
