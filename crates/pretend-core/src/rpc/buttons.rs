use serde::{
  Deserialize,
  Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
  pub label: String,
  pub url: String,
}
