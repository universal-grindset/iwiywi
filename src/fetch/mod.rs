pub mod classify;
pub mod deploy;
pub mod html;
pub mod scraper;

use anyhow::{Context, Result};
use reqwest::Client;
use std::fs;
use std::path::PathBuf;

use crate::config::{config_dir, load_env, Config};
use crate::storage::write_readings;

pub async fn run(config: &Config) -> Result<()> {
    // 1. Pull latest env vars from Vercel
    let env_path = config_dir().join(".env");
    println!("Pulling env vars...");
    deploy::env_pull(&env_path).context("vercel env pull")?;
    load_env().context("loading .env after pull")?;

    // 2. Scrape all sources concurrently
    println!("Scraping sources...");
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;
    let raw_readings = scraper::scrape_all(&client).await;
    println!("Got {} raw readings", raw_readings.len());

    if raw_readings.is_empty() {
        anyhow::bail!("no readings scraped — all sources failed");
    }

    // 3. Classify each reading via Vercel AI Gateway (concurrently)
    println!("Classifying readings...");
    let classify_tasks: Vec<_> = raw_readings
        .into_iter()
        .map(|r| {
            let client = client.clone();
            let config = crate::config::Config {
                ai: crate::config::AiConfig {
                    model: config.ai.model.clone(),
                    gateway_url: config.ai.gateway_url.clone(),
                },
                vercel: crate::config::VercelConfig {
                    project_url: config.vercel.project_url.clone(),
                },
            };
            tokio::spawn(async move { classify::classify(&client, &config, r).await })
        })
        .collect();

    let mut classified = Vec::new();
    for task in classify_tasks {
        match task.await {
            Ok(Ok(r)) => classified.push(r),
            Ok(Err(e)) => eprintln!("warn: classification failed: {e}"),
            Err(e) => eprintln!("warn: classify task panicked: {e}"),
        }
    }
    println!("Classified {} readings", classified.len());

    // 4. Save to ~/.iwiywi/readings-YYYY-MM-DD.json
    write_readings(&classified).context("writing readings to disk")?;
    println!("Saved readings to {}", crate::storage::readings_path().display());

    // 5. Render mobile HTML
    let html = html::render(&classified, &config.vercel.project_url);
    let dist_dir = PathBuf::from("/tmp/iwiywi-dist");
    fs::create_dir_all(&dist_dir)?;
    fs::write(dist_dir.join("index.html"), html)?;

    // 6. Deploy to Vercel
    println!("Deploying to Vercel...");
    deploy::deploy(&dist_dir).context("vercel deploy")?;
    println!("Done.");

    Ok(())
}
