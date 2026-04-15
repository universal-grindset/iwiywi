mod config;
mod fetch;
mod install;
mod models;
mod pulse;
mod storage;
mod tui;
mod web;

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
    /// Serve the pulse as a web page you can open from any browser
    Serve {
        /// Address to bind (default 0.0.0.0 — reachable from LAN / VPS)
        #[arg(long, default_value = "0.0.0.0")]
        bind: String,
        /// TCP port to listen on
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    config::load_env().ok();

    match cli.command {
        Some(Commands::Fetch) => {
            let cfg = config::load_config()?;
            fetch::run(&cfg).await?;
        }
        Some(Commands::Install) => {
            install::run()?;
        }
        Some(Commands::Serve { bind, port }) => {
            let cfg = config::load_config()?;
            // Match the TUI's "open it with no data and it fetches" behavior
            // — otherwise a fresh VPS would serve an empty pulse until 6am.
            if storage::read_readings()?.is_empty() {
                eprintln!("No readings for today — fetching before serve...");
                if let Err(e) = fetch::run(&cfg).await {
                    eprintln!("warn: initial fetch failed: {e}");
                }
            }
            let grapevine_html = fetch_grapevine_html().await;
            web::run(&bind, port, grapevine_html).await?;
        }
        None => {
            let cfg = config::load_config()?;
            if storage::read_readings()?.is_empty() {
                println!("No readings for today — fetching...");
                fetch::run(&cfg).await?;
            }
            // Grapevine + Reddit are quick HTTP fetches with 5s timeouts;
            // run them in parallel and move on. Bill / Community / Summary
            // are AI calls — defer them into tui::run so the TUI appears
            // immediately and the AI content streams in as it resolves.
            let (grapevine_html, reddit_json) = tokio::join!(
                fetch_grapevine_html(),
                fetch::reddit::fetch_community_json(),
            );
            let _ = &cfg; // cfg is passed to tui::run for the background AI.
            crate::tui::run(grapevine_html, reddit_json, cfg).await?;
        }
    }
    Ok(())
}

/// Best-effort fetch of the Grapevine Quote of the Day page. Returns `None`
/// on any failure.
async fn fetch_grapevine_html() -> Option<String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .ok()?;
    let resp = client
        .get(pulse::grapevine::Grapevine::live_url())
        .header("User-Agent", "Mozilla/5.0 (compatible; iwiywi/0.6)")
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.text().await.ok()
}
