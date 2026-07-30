use std::path::PathBuf;

use crate::config::{
  PretendConfig,
  load_config,
  save_config,
};
use crate::database::{
  AppDatabase,
  CacheStore,
};
use crate::error::Result;
use crate::models::{
  App,
  Presence,
};
use crate::rpc::{
  Activity,
  RpcClient,
};
use crate::util::paths;

pub struct PretendService {
  database: AppDatabase,
  config: PretendConfig,
  cache: CacheStore,
  config_path: PathBuf,
}

impl PretendService {
  pub fn new() -> Result<Self> {
    let config_dir = paths::config_dir()?;
    let config_path = config_dir.join("config.toml");
    let config = load_config(&config_path)?;
    let database = AppDatabase::load()?;
    let cache = CacheStore::new(&config_dir);
    Ok(Self {
      database,
      config,
      cache,
      config_path,
    })
  }

  pub fn start(
    &self,
    application: &str,
    details: Option<String>,
    state: Option<String>,
  ) -> Result<Presence> {
    let app = self.database.find(application)?;
    let presence = Presence::new(app.application_id.clone(), details, state);
    let mut rpc = RpcClient::new();
    rpc.set_activity(&app.application_id, presence.to_activity())?;
    self.cache.record(&app)?;
    let mut config = self.config.clone();
    config.default_application = Some(app.name.clone());
    config.last_application_id = Some(app.application_id.clone());
    save_config(&self.config_path, &config)?;
    Ok(presence)
  }

  pub fn stop(&self) -> Result<()> {
    let Some(application_id) = self.config.last_application_id.clone() else {
      return Err(crate::error::Error::Rpc("no active activity".into()));
    };
    let mut rpc = RpcClient::new();
    rpc.clear_activity(&application_id)?;
    let mut config = self.config.clone();
    config.last_application_id = None;
    save_config(&self.config_path, &config)?;
    Ok(())
  }

  pub fn list(&self) -> Vec<App> {
    self.database.all().to_vec()
  }

  pub fn search(&self, query: &str) -> Vec<App> {
    self.database.search(query)
  }

  pub fn config_summary(&self) -> String {
    let mut lines = Vec::new();
    lines.push(format!("config_path: {}", self.config_path.display()));
    lines.push(format!(
      "default_application: {}",
      self.config.default_application.as_deref().unwrap_or("none")
    ));
    lines.push(format!(
      "last_application_id: {}",
      self.config.last_application_id.as_deref().unwrap_or("none")
    ));
    lines.join("\n")
  }
}
