//! Daily summary: one short "theme for today" line, AI-generated once per
//! day at startup and shown as a toast when the TUI launches. Cached under
//! `<config_dir>/ai_cache/summary/YYYY-MM-DD.txt`. Best-effort; absent on
//! failure.

use anyhow::Result;
use chrono::NaiveDate;
use reqwest::Client;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::fetch::ai::{post_chat, ChatOpts};

const SYSTEM_PROMPT: &str =
    "You write the single-line theme for today's AA meditation app. \
     Twelve words maximum. Start with the literal word 'Today' followed by a colon. \
     Plain, grounded, actionable. No emoji, no quotes, no trailing period. \
     Example shape: 'Today: notice where willingness already lives.'";

pub async fn load_or_generate(
    cache_dir: &Path,
    client: &Client,
    config: &Config,
    today: NaiveDate,
    step: u8,
) -> Option<String> {
    let path = cache_path(cache_dir, today);
    if let Ok(cached) = std::fs::read_to_string(&path) {
        let trimmed = cached.trim().to_string();
        if !trimmed.is_empty() { return Some(trimmed); }
    }
    match generate(client, config, today, step).await {
        Ok(line) => {
            let _ = write_cache(&path, &line);
            Some(line)
        }
        Err(_) => None,
    }
}

fn cache_path(cache_dir: &Path, today: NaiveDate) -> PathBuf {
    cache_dir.join(format!("{today}.txt"))
}

fn write_cache(path: &Path, line: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, line)?;
    Ok(())
}

async fn generate(
    client: &Client,
    config: &Config,
    today: NaiveDate,
    step: u8,
) -> Result<String> {
    let user = format!(
        "Today is {today}. The day's step-theme is Step {step}. \
         Produce the one-line theme per your instructions.",
    );
    let opts = ChatOpts {
        max_tokens: Some(40),
        temperature: Some(0.6),
        json_mode: false,
    };
    let raw = post_chat(client, config, SYSTEM_PROMPT, &user, opts).await?;
    Ok(raw.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn cache_hit_is_returned_without_network() {
        let dir = tempdir().unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 4, 15).unwrap();
        let path = cache_path(dir.path(), today);
        if let Some(parent) = path.parent() { std::fs::create_dir_all(parent).unwrap(); }
        std::fs::write(&path, "Today: quiet steps forward.").unwrap();

        // Use a bogus config; cache hit should short-circuit before any call.
        let client = Client::new();
        let config = Config {
            ai: crate::config::AiConfig {
                model: "x".to_string(),
                gateway_url: "http://127.0.0.1:1".to_string(),
                api_version: None,
            },
        };
        let out = load_or_generate(dir.path(), &client, &config, today, 3).await;
        assert_eq!(out.as_deref(), Some("Today: quiet steps forward."));
    }
}
