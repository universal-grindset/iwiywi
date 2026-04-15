//! Community pulse: insightful posts from /r/stopdrinking and
//! /r/alcoholicsanonymous, curated and paraphrased by the AI gateway into
//! short reflections. Real user posts are never republished verbatim; the
//! gateway paraphrases them and strips usernames. Labeled in the TUI as
//! "From the rooms (paraphrased)" so the provenance is honest.
//!
//! Cache: `<config_dir>/community/YYYY-MM-DD.json`. Best-effort; on any
//! failure the source simply contributes nothing today.

use anyhow::Result;
use chrono::NaiveDate;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::fetch::ai::{post_chat, ChatOpts};
use crate::pulse::{PulseItem, PulseKind, PulseSource};

const SYSTEM_PROMPT: &str =
    "You curate anonymous community reflections from a recovery subreddit. \
     From the posts provided, pick up to 3 that offer insight on sobriety, the AA program, \
     surrender, service, or the daily work of recovery. Avoid crisis posts, relapse details, \
     promotional content, questions without insight, and anything that names specific people. \
     Paraphrase each in plain language — do not quote user text verbatim. \
     Each body 40 to 80 words. Return JSON only, this exact shape: \
     {\"items\": [{\"title\": \"...\", \"body\": \"...\", \"source_sub\": \"stopdrinking\"}]}. \
     If no posts are suitable, return {\"items\": []}.";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CuratedItem {
    title: String,
    body: String,
    source_sub: String,
}

#[derive(Serialize, Deserialize)]
struct CuratedPayload {
    items: Vec<CuratedItem>,
}

#[derive(Serialize, Deserialize)]
struct CacheFile {
    date: String,
    items: Vec<CuratedItem>,
}

pub struct CommunityPulse {
    items: Vec<PulseItem>,
}

impl CommunityPulse {
    pub fn empty() -> Self { Self { items: Vec::new() } }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn load_from(cache_dir: &Path, today: NaiveDate) -> Self {
        match read_cached(cache_dir, today) {
            Some(file) => Self { items: file.items.iter().map(build_item).collect() },
            None => Self::empty(),
        }
    }

    /// Given raw Reddit JSON (already fetched best-effort) and a configured
    /// AI gateway, curate into pulse items. Cache on success. Any failure
    /// falls back to `empty()`.
    pub async fn load_or_curate(
        cache_dir: &Path,
        client: &Client,
        config: &Config,
        today: NaiveDate,
        raw_json: Option<&str>,
    ) -> Self {
        if let Some(file) = read_cached(cache_dir, today) {
            return Self { items: file.items.iter().map(build_item).collect() };
        }
        let Some(raw) = raw_json else { return Self::empty(); };
        let excerpts = extract_post_excerpts(raw);
        if excerpts.is_empty() { return Self::empty(); }
        match curate(client, config, &excerpts).await {
            Ok(items) => {
                let file = CacheFile { date: today.to_string(), items: items.clone() };
                let _ = write_cache(cache_dir, today, &file);
                Self { items: items.iter().map(build_item).collect() }
            }
            Err(_) => Self::empty(),
        }
    }
}

impl PulseSource for CommunityPulse {
    fn name(&self) -> &'static str { "community" }
    fn items(&self) -> &[PulseItem] { &self.items }
}

fn build_item(c: &CuratedItem) -> PulseItem {
    PulseItem {
        kind: PulseKind::Community,
        step: None,
        label: format!("/r/{} — {}", c.source_sub, c.title),
        body: c.body.clone(),
    }
}

fn cache_path(cache_dir: &Path, today: NaiveDate) -> PathBuf {
    cache_dir.join(format!("{today}.json"))
}

fn read_cached(cache_dir: &Path, today: NaiveDate) -> Option<CacheFile> {
    let raw = std::fs::read_to_string(cache_path(cache_dir, today)).ok()?;
    serde_json::from_str(&raw).ok()
}

fn write_cache(cache_dir: &Path, today: NaiveDate, file: &CacheFile) -> Result<()> {
    std::fs::create_dir_all(cache_dir)?;
    let json = serde_json::to_string_pretty(file)?;
    std::fs::write(cache_path(cache_dir, today), json)?;
    Ok(())
}

/// Pull `title` + `selftext` from a Reddit listing JSON. Truncates per-post
/// so the curation prompt stays under a reasonable token count.
pub fn extract_post_excerpts(raw: &str) -> Vec<(String, String, String)> {
    let v: serde_json::Value = match serde_json::from_str(raw) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let children = v
        .pointer("/data/children")
        .and_then(|c| c.as_array())
        .cloned()
        .unwrap_or_default();
    let mut out = Vec::new();
    for child in children.iter().take(25) {
        let d = &child["data"];
        let sub = d["subreddit"].as_str().unwrap_or("").to_string();
        let title = d["title"].as_str().unwrap_or("").to_string();
        let selftext = d["selftext"].as_str().unwrap_or("").to_string();
        if title.is_empty() { continue; }
        if selftext.trim().is_empty() { continue; }
        let truncated: String = selftext.chars().take(600).collect();
        out.push((sub, title, truncated));
    }
    out
}

async fn curate(
    client: &Client,
    config: &Config,
    excerpts: &[(String, String, String)],
) -> Result<Vec<CuratedItem>> {
    let mut user = String::from("Posts:\n\n");
    for (sub, title, body) in excerpts {
        user.push_str(&format!("[/r/{sub}] {title}\n{body}\n\n"));
    }
    let opts = ChatOpts {
        max_tokens: Some(1200),
        temperature: Some(0.3),
        json_mode: true,
    };
    let raw = post_chat(client, config, SYSTEM_PROMPT, &user, opts).await?;
    let payload: CuratedPayload = serde_json::from_str(&raw)?;
    Ok(payload.items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn empty_source_has_no_items() {
        let s = CommunityPulse::empty();
        assert!(s.items().is_empty());
        assert_eq!(s.name(), "community");
    }

    #[test]
    fn cache_miss_returns_empty() {
        let dir = tempdir().unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 4, 15).unwrap();
        let s = CommunityPulse::load_from(dir.path(), today);
        assert!(s.items().is_empty());
    }

    #[test]
    fn cache_hit_builds_items() {
        let dir = tempdir().unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 4, 15).unwrap();
        let file = CacheFile {
            date: today.to_string(),
            items: vec![CuratedItem {
                title: "On patience".to_string(),
                body: "Someone shared...".to_string(),
                source_sub: "stopdrinking".to_string(),
            }],
        };
        write_cache(dir.path(), today, &file).unwrap();
        let s = CommunityPulse::load_from(dir.path(), today);
        assert_eq!(s.items().len(), 1);
        assert!(s.items()[0].label.contains("/r/stopdrinking"));
        assert_eq!(s.items()[0].kind, PulseKind::Community);
    }

    #[test]
    fn extract_post_excerpts_parses_reddit_json() {
        let raw = r#"{
          "data": { "children": [
            {"data": {"subreddit": "stopdrinking", "title": "Day 30", "selftext": "Quiet milestone."}},
            {"data": {"subreddit": "alcoholicsanonymous", "title": "No body", "selftext": ""}}
          ]}
        }"#;
        let out = extract_post_excerpts(raw);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].0, "stopdrinking");
        assert_eq!(out[0].1, "Day 30");
        assert_eq!(out[0].2, "Quiet milestone.");
    }

    #[test]
    fn extract_post_excerpts_handles_garbage() {
        assert!(extract_post_excerpts("not json").is_empty());
        assert!(extract_post_excerpts("{}").is_empty());
    }
}
