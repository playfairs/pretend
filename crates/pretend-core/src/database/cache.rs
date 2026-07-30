use std::fs;
use std::path::Path;

use crate::error::Result;
use crate::models::App;

#[derive(Debug)]
pub struct CacheStore {
  path: std::path::PathBuf,
}

impl CacheStore {
  pub fn new(base_dir: &Path) -> Self {
    let path = base_dir.join("cache");
    Self { path }
  }

  pub fn record(&self, app: &App) -> Result<()> {
    if let Some(parent) = self.path.parent() {
      fs::create_dir_all(parent)?;
    }
    fs::create_dir_all(&self.path)?;
    let recent = self.path.join("recent.json");
    let mut entries = Vec::new();
    if recent.exists() {
      let content = fs::read_to_string(&recent)?;
      entries = serde_json::from_str(&content).unwrap_or_default();
    }
    if !entries.contains(&app.name) {
      entries.push(app.name.clone());
    }
    fs::write(recent, serde_json::to_string(&entries)?)?;
    Ok(())
  }
}
