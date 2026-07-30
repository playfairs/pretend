use serde::{
  Deserialize,
  Serialize,
};

use super::{
  Assets,
  Button,
  Timestamps,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Activity {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub details: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub state: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub timestamps: Option<Timestamps>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub assets: Option<Assets>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub buttons: Option<Vec<Button>>,
}
