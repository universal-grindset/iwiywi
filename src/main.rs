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
            //
            // Grapevine + Reddit run in parallel since both are HTTP fetches
            // to unrelated origins. Then Bill / Community / Summary run in
            // parallel since all three hit the AI gateway and are independent
            // — if they all miss cache, running them sequentially would add
            // ~10s to startup on a cold day.
            let (grapevine_html, reddit_json) = tokio::join!(
                fetch_grapevine_html(),
                fetch::reddit::fetch_community_json(),
            );

            let client = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(12))
                .build()
                .ok();
            let today = chrono::Local::now().date_naive();
            let step_of_day = ((today.day() as u8).wrapping_sub(1) % 12) + 1;

            let bill_dir = config::config_dir().join("bill");
            let community_dir = config::config_dir().join("community");
            let summary_dir = config::config_dir().join("ai_cache").join("summary");
            let (bill, community, summary) = match client.as_ref() {
                Some(c) => tokio::join!(
                    pulse::bill::BillReflection::load_or_generate(&bill_dir, c, &cfg, today),
                    pulse::community::CommunityPulse::load_or_curate(
                        &community_dir, c, &cfg, today, reddit_json.as_deref(),
                    ),
                    pulse::summary::load_or_generate(&summary_dir, c, &cfg, today, step_of_day),
                ),
                None => (
                    pulse::bill::BillReflection::empty(),
                    pulse::community::CommunityPulse::empty(),
                    None,
                ),
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
