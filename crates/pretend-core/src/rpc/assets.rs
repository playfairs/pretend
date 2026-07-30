use serde::{
  Deserialize,
  Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Assets {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub large_image: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub large_text: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub small_image: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub small_text: Option<String>,
}
