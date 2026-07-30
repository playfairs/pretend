use std::fs;
use std::path::Path;

use crate::config::PretendConfig;
use crate::error::Result;

pub fn save_config(path: &Path, config: &PretendConfig) -> Result<()> {
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }
  let content = toml::to_string(config)?;
  fs::write(path, content)?;
  Ok(())
}
