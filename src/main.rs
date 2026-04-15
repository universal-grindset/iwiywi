mod config;
mod fetch;
mod install;
mod models;
mod pulse;
mod storage;
mod tui;

use anyhow::Result;
use clap::{Parser, Subcommand};
use chrono::Datelike;

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
    config::load_env().ok();

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
            // Best-effort background fetches. Each returns Option<...>; any
            // failure just drops the contribution for today — nothing blocks
            // startup, nothing fails the TUI.
            let grapevine_html = fetch_grapevine_html().await;
            let reddit_json = fetch::reddit::fetch_community_json().await;

            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(8))
                .build()
                .ok();
            let today = chrono::Local::now().date_naive();
            let step_of_day = ((today.day() as u8).wrapping_sub(1) % 12) + 1;

            let bill = match client.as_ref() {
                Some(c) => pulse::bill::BillReflection::load_or_generate(
                    &config::config_dir().join("bill"),
                    c,
                    &cfg,
                    today,
                ).await,
                None => pulse::bill::BillReflection::empty(),
            };
            let community = match client.as_ref() {
                Some(c) => pulse::community::CommunityPulse::load_or_curate(
                    &config::config_dir().join("community"),
                    c,
                    &cfg,
                    today,
                    reddit_json.as_deref(),
                ).await,
                None => pulse::community::CommunityPulse::empty(),
            };
            let summary = match client.as_ref() {
                Some(c) => pulse::summary::load_or_generate(
                    &config::config_dir().join("ai_cache").join("summary"),
                    c,
                    &cfg,
                    today,
                    step_of_day,
                ).await,
                None => None,
            };

            crate::tui::run(grapevine_html, bill, community, summary)?;
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
