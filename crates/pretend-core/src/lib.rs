pub mod config;
pub mod database;
pub mod error;
pub mod models;
pub mod rpc;
pub mod service;
pub mod util;

pub use error::{
  Error,
  Result,
};
pub use models::{
  App,
  Presence,
};
pub use service::PretendService;
