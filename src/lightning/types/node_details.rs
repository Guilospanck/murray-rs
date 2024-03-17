use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct Feature {
  bit: i32,
  name: String,
  is_required: bool,
  is_known: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NodeCountry {
  de: String,
  en: String,
  es: String,
  fr: String,
  ja: String,
  #[serde(rename = "pt-BR")]
  pt_br: String,
  ru: String,
  #[serde(rename = "zh-CN")]
  zh_cn: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChannelNode {
  alias: String,
  public_key: String,
  channels: i32,
  capacity: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Channel {
  status: i32,
  closing_reason: Option<String>,
  closing_date: Option<String>,
  capacity: i32,
  short_id: String,
  #[serde(deserialize_with = "deserialize_id")]
  id: String,
  fee_rate: i32,
  node: ChannelNode,
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
  pub first_seen: i32,
  pub updated_at: i32,
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
