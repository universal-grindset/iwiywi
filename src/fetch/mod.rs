pub mod ai_extract;
pub mod classify;
pub mod scraper;

use anyhow::{Context, Result};
use reqwest::Client;

use crate::config::{load_env, Config};
use crate::storage::write_readings;

pub async fn run(config: &Config) -> Result<()> {
    load_env().context("loading ~/.iwiywi/.env")?;

    println!("Scraping sources...");
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;
    let raw_readings = scraper::scrape_all(&client, config).await;
    println!("Got {} raw readings", raw_readings.len());

    if raw_readings.is_empty() {
        anyhow::bail!("no readings scraped — all sources failed");
    }

    println!("Classifying readings...");
    let classify_tasks: Vec<_> = raw_readings
        .into_iter()
        .map(|r| {
            let client = client.clone();
            let config = config.clone();
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

    if classified.is_empty() {
        anyhow::bail!("all readings failed classification");
    }

    write_readings(&classified).context("writing readings to disk")?;
    println!("Saved readings to {}", crate::storage::readings_path().display());

    Ok(())
}
