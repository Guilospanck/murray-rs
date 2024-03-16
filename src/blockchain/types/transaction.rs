use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PreviousOutput {
  pub scriptpubkey: String,
  pub scriptpubkey_asm: String,
  pub scriptpubkey_type: String,
  pub scriptpubkey_address: String,
  pub value: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionInput {
  pub txid: String,
  pub vout: u32,
  pub prevout: PreviousOutput,
  pub scriptsig: String,
  pub scriptsig_asm: String,
  pub witness: Option<Vec<String>>,
  pub is_coinbase: bool,
  pub sequence: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionStatus {
  pub confirmed: bool,
  pub block_height: Option<u32>, // These fields can be Optional because the tx may have not been confirmed yet
  pub block_hash: Option<String>,
  pub block_time: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionOutput {
  pub scriptpubkey: String,
  pub scriptpubkey_asm: String,
  pub scriptpubkey_type: String,
  pub scriptpubkey_address: String,
  pub value: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
  pub txid: String,
  pub version: u32,
  pub locktime: u32,
  pub vin: Vec<TransactionInput>,
  pub vout: Vec<TransactionOutput>,
  pub size: u32,
  pub weight: u32,
  pub sigops: u32,
  pub fee: u32,
  pub status: TransactionStatus,
}
