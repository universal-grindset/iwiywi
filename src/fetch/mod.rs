pub mod classify;
pub mod gist;
pub mod markdown;
pub mod scraper;

use anyhow::{Context, Result};
use reqwest::Client;

use crate::config::{load_env, save_config, Config};
use crate::storage::write_readings;

pub async fn run(config: &Config) -> Result<()> {
    load_env().context("loading ~/.iwiywi/.env")?;

    println!("Scraping sources...");
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;
    let raw_readings = scraper::scrape_all(&client).await;
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
    println!(
        "Saved readings to {}",
        crate::storage::readings_path().display()
    );

    let md = markdown::render(&classified);
    let gist_id =
        gist::publish(&md, config.mobile.gist_id.as_deref()).context("publishing gist")?;

    if config.mobile.gist_id.as_deref() != Some(gist_id.as_str()) {
        let mut updated = config.clone();
        updated.mobile.gist_id = Some(gist_id.clone());
        save_config(&updated).context("saving gist_id to config.toml")?;
        println!("Created gist https://gist.github.com/{gist_id}");
    } else {
        println!("Updated gist https://gist.github.com/{gist_id}");
    }

    Ok(())
}
