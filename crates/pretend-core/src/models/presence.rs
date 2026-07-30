use serde::{
  Deserialize,
  Serialize,
};

use crate::rpc::{
  Activity,
  Assets,
  Button,
  Timestamps,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Presence {
  pub application_id: String,
  pub details: Option<String>,
  pub state: Option<String>,
  #[serde(default)]
  pub timestamps: Option<Timestamps>,
  #[serde(default)]
  pub assets: Option<Assets>,
  #[serde(default)]
  pub buttons: Vec<Button>,
}

impl Presence {
  pub fn new(application_id: String, details: Option<String>, state: Option<String>) -> Self {
    Self {
      application_id,
      details,
      state,
      timestamps: None,
      assets: None,
      buttons: Vec::new(),
    }
  }

  pub fn to_activity(&self) -> Activity {
    Activity {
      details: self.details.clone(),
      state: self.state.clone(),
      timestamps: self.timestamps.clone(),
      assets: self.assets.clone(),
      buttons: if self.buttons.is_empty() {
        None
      } else {
        Some(self.buttons.clone())
      },
    }
  }
}
