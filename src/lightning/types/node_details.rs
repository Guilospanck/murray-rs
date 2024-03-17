use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct Feature {
  pub bit: i32,
  pub name: String,
  pub is_required: bool,
  pub is_known: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NodeCountry {
  pub de: String,
  pub en: String,
  pub es: String,
  pub fr: String,
  pub ja: String,
  #[serde(rename = "pt-BR")]
  pub pt_br: String,
  pub ru: String,
  #[serde(rename = "zh-CN")]
  pub zh_cn: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChannelNode {
  pub alias: String,
  pub public_key: String,
  pub channels: i32,
  pub capacity: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Channel {
  pub status: i32,
  pub closing_reason: Option<String>,
  pub closing_date: Option<String>,
  pub capacity: u64,
  pub short_id: String,
  #[serde(deserialize_with = "deserialize_id")]
  pub id: String,
  pub fee_rate: i32,
  pub node: ChannelNode,
}

fn deserialize_id<'de, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  // Deserialize the id as a dynamic type, which can be either a String or a number
  let id_value: serde_json::Value = Deserialize::deserialize(deserializer)?;

  // Convert the id_value to a String, regardless of its original type
  Ok(id_value.to_string())
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NodeData {
  pub public_key: String,
  pub alias: String,
  pub first_seen: u64,
  pub updated_at: u64,
  pub color: String,
  pub sockets: String,
  pub as_number: i32,
  pub city_id: Option<i32>,
  pub country_id: i32,
  pub subdivision_id: Option<i32>,
  pub longitude: f64,
  pub latitude: f64,
  pub iso_code: String,
  pub as_organization: String,
  pub city: Option<String>,
  pub country: NodeCountry,
  pub subdivision: Option<String>,
  pub features: Vec<Feature>,
  #[serde(rename = "featuresBits")]
  pub features_bits: String,
  pub active_channel_count: i32,
  pub capacity: String,
  pub opened_channel_count: i32,
  pub closed_channel_count: i32,
  pub custom_records: HashMap<String, serde_json::Value>,
  pub channels: Vec<Channel>,
}
