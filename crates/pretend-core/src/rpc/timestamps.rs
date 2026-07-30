use serde::{
  Deserialize,
  Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Timestamps {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub start: Option<u64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub end: Option<u64>,
}
