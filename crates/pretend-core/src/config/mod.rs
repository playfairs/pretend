pub mod load;
pub mod save;

use serde::{
  Deserialize,
  Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PretendConfig {
  #[serde(default)]
  pub default_application: Option<String>,
  #[serde(default)]
  pub last_application_id: Option<String>,
}

pub use load::load_config;
pub use save::save_config;
