mod config;
mod fetch;
mod install;
mod models;
mod pulse;
mod storage;
mod tui;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "iwiywi",
    about = "It Works If You Work It — daily AA readings",
    version = "0.1.0"
)]
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
            install::run()?;
        }
        None => {
            let cfg = config::load_config()?;
            if storage::read_readings()?.is_empty() {
                println!("No readings for today — fetching...");
                fetch::run(&cfg).await?;
            }
            // Fire-and-forget Grapevine fetch: best-effort, fall back to the
            // bundled corpus on any failure or timeout. Keeps startup quick.
            let grapevine_html = fetch_grapevine_html().await;
            crate::tui::run(grapevine_html)?;
        }
    }
    Ok(())
}

/// Best-effort fetch of the Grapevine Quote of the Day page. Returns `None`
/// on any failure (DNS, timeout, non-2xx, body read error). Caller treats
/// `None` as "use bundled fallback only."
async fn fetch_grapevine_html() -> Option<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .ok()?;
    let resp = client
        .get(pulse::grapevine::Grapevine::live_url())
        .header("User-Agent", "Mozilla/5.0 (compatible; iwiywi/0.5)")
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.text().await.ok()
}
