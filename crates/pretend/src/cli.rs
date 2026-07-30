use anyhow::Result;
use clap::{
  Args,
  Parser,
  Subcommand,
};
use pretend_core::PretendService;

use crate::commands;

#[derive(Parser)]
#[command(name = "pretend", version, about = "Discord Rich Presence manager")]
pub struct Cli {
  #[command(subcommand)]
  command: Command,
}

#[derive(Subcommand)]
enum Command {
  Start(StartArgs),
  Stop,
  List,
  Search { query: String },
  Config,
}

#[derive(Args)]
struct StartArgs {
  application: String,
  #[arg(long)]
  details: Option<String>,
  #[arg(long)]
  state: Option<String>,
}

impl Cli {
  pub fn run() -> Result<()> {
    let cli = Cli::parse();
    let service = PretendService::new()?;

    match cli.command {
      Command::Start(args) => commands::start::run(
        &service,
        &args.application,
        args.details.as_deref(),
        args.state.as_deref(),
      )?,
      Command::Stop => commands::stop::run(&service)?,
      Command::List => commands::list::run(&service)?,
      Command::Search { query } => commands::search::run(&service, &query)?,
      Command::Config => commands::config::run(&service)?,
    }

    Ok(())
  }
}
