use std::fs;
use std::path::Path;

use crate::config::PretendConfig;
use crate::error::Result;

pub fn load_config(path: &Path) -> Result<PretendConfig> {
    if \!path.exists() {
        return Ok(PretendConfig::default());
    }

    let content = fs::read_to_string(path)?;
    Ok(toml::from_str(&content).unwrap_or_default())
}
