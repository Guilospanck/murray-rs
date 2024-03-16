use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PreviousOutput {
  scriptpubkey: String,
  scriptpubkey_asm: String,
  scriptpubkey_type: String,
  scriptpubkey_address: String,
  value: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionInput {
  txid: String,
  vout: u32,
  prevout: PreviousOutput,
  scriptsig: String,
  scriptsig_asm: String,
  witness: Option<Vec<String>>,
  is_coinbase: bool,
  sequence: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionStatus {
  confirmed: bool,
  block_height: u32,
  block_hash: String,
  block_time: u64,
}

#[derive(Deserialize, Serialize, Debug)]
struct TransactionOutput {
  scriptpubkey: String,
  scriptpubkey_asm: String,
  scriptpubkey_type: String,
  scriptpubkey_address: String,
  value: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transaction {
  txid: String,
  version: u32,
  locktime: u32,
  vin: Vec<TransactionInput>, 
  vout: Vec<TransactionOutput>, 
  size: u32,
  weight: u32,
  sigops: u32,
  fee: u32,
  status: TransactionStatus,
}
