use anyhow::Result;
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
=    Fmt,
=    Test,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Fmt => run("fmt", &[]),
        Commands::Test => run("test", &[]),
    }
}

fn run(cmd: &str, args: &[&str]) -> Result<()> {
    let status = Command::new("cargo").arg(cmd).args(args).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow\!("cargo {} failed", cmd))
    }
}
