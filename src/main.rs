mod config;
mod fetch;
mod models;
mod storage;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "iwiywi", about = "It Works If You Work It — daily AA readings", version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch today's AA readings, classify, and deploy to Vercel
    Fetch,
    /// Install launchd job to run fetch at 6am daily
    Install,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    config::load_env().ok(); // load .env if present, ignore if missing

    match cli.command {
        Some(Commands::Fetch) => {
            let cfg = config::load_config()?;
            fetch::run(&cfg).await?;
        }
        Some(Commands::Install) => {
            // Implemented in Sub-Plan 2
            eprintln!("install not yet implemented — coming in next release");
        }
        None => {
            // TUI — implemented in Sub-Plan 2
            eprintln!("TUI not yet implemented — run `iwiywi fetch` to populate readings");
        }
    }
    Ok(())
}
