//! Bill W. daily reflection source. AI-generated each day in the style of
//! a recovering alcoholic elder — not a reproduction of any copyrighted
//! AAWS text ("As Bill Sees It" remains under copyright). Labeled on
//! every render as `Bill W. — AI reflection` so the attribution is honest.
//!
//! Cache: `<config_dir>/bill/YYYY-MM-DD.json`. On cache miss we fire a
//! best-effort gateway call; on any failure today simply has no Bill item
//! (no crash, no bundled fallback — keeps it honest).

use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::fetch::ai::{post_chat, ChatOpts};
use crate::pulse::{PulseItem, PulseKind, PulseSource};

const SYSTEM_PROMPT: &str =
    "You are writing a short daily meditation in the voice of a recovering alcoholic elder — \
     plain-spoken, first-person, contemplative, never preachy. Length: 100 to 140 words. \
     Return only the meditation prose; no title, no date, no attribution, no headings. \
     Forbidden: (1) any direct quotation of a copyrighted AA text, (2) enumerating the 12 steps as a list, \
     (3) modern slang or pop-culture references that break the timeless voice, \
     (4) platitudes like 'one day at a time' — if you use the phrase, earn it.";

#[derive(Serialize, Deserialize)]
struct CacheEntry {
    date: String,
    step: u8,
    text: String,
}

pub struct BillReflection {
    items: Vec<PulseItem>,
}

impl BillReflection {
    /// Build an empty source. Used when cache read fails and gateway is
    /// unavailable. `items()` returns an empty slice so the mixer skips it.
    pub fn empty() -> Self {
        Self { items: Vec::new() }
    }

    /// Load today's reflection from cache if present, else produce an empty
    /// source. Synchronous — reads JSON off disk only. Used in tests and
    /// as a future hook for no-network startup paths.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn load_from(cache_dir: &Path, today: NaiveDate) -> Self {
        match read_cached(cache_dir, today) {
            Some(entry) => Self {
                items: vec![build_item(&entry)],
            },
            None => Self::empty(),
        }
    }

    /// Read cache; on miss, call the gateway and write a fresh entry. Any
    /// network/IO failure short-circuits to `empty()`.
    pub async fn load_or_generate(
        cache_dir: &Path,
        client: &Client,
        config: &Config,
        today: NaiveDate,
    ) -> Self {
        if let Some(entry) = read_cached(cache_dir, today) {
            return Self {
                items: vec![build_item(&entry)],
            };
        }
        let step = step_of_day(today);
        match generate(client, config, today, step).await {
            Ok(text) => {
                let entry = CacheEntry {
                    date: today.to_string(),
                    step,
                    text,
                };
                let _ = write_cache(cache_dir, today, &entry);
                Self {
                    items: vec![build_item(&entry)],
                }
            }
            Err(_) => Self::empty(),
        }
    }
}

impl PulseSource for BillReflection {
    fn name(&self) -> &'static str {
        "bill"
    }
    fn items(&self) -> &[PulseItem] {
        &self.items
    }
}

fn cache_path(cache_dir: &Path, today: NaiveDate) -> PathBuf {
    cache_dir.join(format!("{today}.json"))
}

fn read_cached(cache_dir: &Path, today: NaiveDate) -> Option<CacheEntry> {
    let path = cache_path(cache_dir, today);
    let raw = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&raw).ok()
}

fn write_cache(cache_dir: &Path, today: NaiveDate, entry: &CacheEntry) -> Result<()> {
    std::fs::create_dir_all(cache_dir)?;
    let path = cache_path(cache_dir, today);
    let json = serde_json::to_string_pretty(entry)?;
    std::fs::write(path, json)?;
    Ok(())
}

fn step_of_day(today: NaiveDate) -> u8 {
    // Simple month+day rotation through 1..=12 so the Bill reflection drifts
    // across the program over the year. Doesn't match any particular AA
    // calendar — just provides variety.
    ((today.day() as u8).wrapping_sub(1) % 12) + 1
}

fn build_item(entry: &CacheEntry) -> PulseItem {
    PulseItem {
        kind: PulseKind::BillReflection,
        step: Some(entry.step),
        label: format!("Bill W. — AI reflection · Step {}", entry.step),
        body: entry.text.clone(),
    }
}

async fn generate(client: &Client, config: &Config, today: NaiveDate, step: u8) -> Result<String> {
    let user = format!(
        "Write today's reflection. Today's date is {today}. \
         Center it on the theme of Step {step}, but do not name the step or quote step texts. \
         Let the meditation feel earned, not assigned.",
    );
    let opts = ChatOpts {
        max_tokens: Some(400),
        temperature: Some(0.7),
        json_mode: false,
    };
    let raw = post_chat(client, config, SYSTEM_PROMPT, &user, opts).await?;
    Ok(raw.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn empty_source_has_no_items() {
        let s = BillReflection::empty();
        assert!(s.items().is_empty());
        assert_eq!(s.name(), "bill");
    }

    #[test]
    fn cache_miss_returns_empty_source() {
        let dir = tempdir().unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 4, 15).unwrap();
        let s = BillReflection::load_from(dir.path(), today);
        assert!(s.items().is_empty());
    }

    #[test]
    fn cache_hit_populates_single_item() {
        let dir = tempdir().unwrap();
        let today = NaiveDate::from_ymd_opt(2026, 4, 15).unwrap();
        let entry = CacheEntry {
            date: today.to_string(),
            step: 3,
            text: "A quiet morning. I remember where surrender led me.".to_string(),
        };
        write_cache(dir.path(), today, &entry).unwrap();

        let s = BillReflection::load_from(dir.path(), today);
        assert_eq!(s.items().len(), 1);
        let item = &s.items()[0];
        assert_eq!(item.kind, PulseKind::BillReflection);
        assert_eq!(item.step, Some(3));
        assert!(item.body.contains("surrender"));
        assert!(item.label.contains("Bill W."));
    }

    #[test]
    fn step_of_day_stays_in_range() {
        for d in 1..=31 {
            if let Some(date) = NaiveDate::from_ymd_opt(2026, 1, d) {
                let s = step_of_day(date);
                assert!((1..=12).contains(&s));
            }
        }
    }
}
