use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),
  #[error("json error: {0}")]
  Json(#[from] serde_json::Error),
  #[error("toml decode error: {0}")]
  Toml(#[from] toml::de::Error),
  #[error("toml encode error: {0}")]
  TomlSer(#[from] toml::ser::Error),
  #[error("application not found: {0}")]
  ApplicationNotFound(String),
  #[error("rpc error: {0}")]
  Rpc(String),
  #[error("config error: {0}")]
  Config(String),
}
