mod cli;
mod commands;
mod output;

use anyhow::Result;
use cli::Cli;

fn main() -> Result<()> {
  tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .with_target(false)
    .init();
  Cli::run()
}
