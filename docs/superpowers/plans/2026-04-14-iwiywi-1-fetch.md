# iwiywi — Sub-Plan 1: Foundation + Fetch Pipeline

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement `iwiywi fetch` — scrapes 12 AA sources, classifies each into Step 1–12 via Vercel AI Gateway, saves to `~/.iwiywi/readings-YYYY-MM-DD.json`, generates mobile HTML, and deploys to Vercel.

**Architecture:** Single Rust binary with clap subcommands. Fetch pipeline: concurrent scraping (tokio) → concurrent AI classification → JSON storage → HTML render → `vercel deploy` subprocess. All errors use `anyhow::Result` with `.context()` for traceability; per-source scrape failures are caught and skipped, not fatal.

**Tech Stack:** Rust, tokio, reqwest, scraper (HTML parsing), serde_json, anyhow, clap, toml, chrono, dotenvy, mockito (tests)

**Spec:** `docs/superpowers/specs/2026-04-14-iwiywi-design.md`

---

## File Map

| File | Responsibility |
|------|---------------|
| `Cargo.toml` | Dependencies |
| `src/main.rs` | clap entrypoint, dispatch to `fetch::run()` / TUI stub / install stub |
| `src/models.rs` | `RawReading`, `ClassifiedReading` — shared types |
| `src/config.rs` | `Config` struct, `load_config()`, `load_env()` |
| `src/storage.rs` | `write_readings()`, `read_readings()`, `readings_path()` |
| `src/fetch/mod.rs` | `run()` — orchestrates scrape → classify → store → html → deploy |
| `src/fetch/scraper.rs` | One async fn per source, all return `Result<RawReading>` |
| `src/fetch/classify.rs` | `classify()` — Vercel AI Gateway HTTP call |
| `src/fetch/html.rs` | `render()` — builds mobile `index.html` string |
| `src/fetch/deploy.rs` | `env_pull()`, `deploy()` — shells out to `vercel` CLI |

---

## Task 1: Project Setup

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`

- [ ] **Step 1: Initialize the project**

```bash
cd /Users/jdf/Documents/iwiywi
cargo init --name iwiywi
```

Expected: `src/main.rs` and `Cargo.toml` created.

- [ ] **Step 2: Replace Cargo.toml**

```toml
[package]
name = "iwiywi"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "iwiywi"
path = "src/main.rs"

[dependencies]
anyhow = "1"
chrono = { version = "0.4", features = ["serde"] }
clap = { version = "4", features = ["derive"] }
dotenvy = "0.15"
reqwest = { version = "0.12", features = ["json"] }
scraper = "0.20"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
toml = "0.8"
ratatui = "0.28"
crossterm = "0.28"
qrcode = "0.14"

[dev-dependencies]
mockito = "1"
```

- [ ] **Step 3: Verify it compiles**

```bash
cargo check
```

Expected: no errors.

- [ ] **Step 4: Commit**

```bash
git init
git add Cargo.toml src/main.rs
git commit -m "chore: initialize iwiywi project"
```

---

## Task 2: Shared Types (models.rs)

**Files:**
- Create: `src/models.rs`

- [ ] **Step 1: Write failing test**

Add to `src/models.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawReading {
    pub source: String,
    pub title: String,
    pub text: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassifiedReading {
    pub step: u8,
    pub reason: String,
    pub source: String,
    pub title: String,
    pub text: String,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classified_reading_round_trips_json() {
        let r = ClassifiedReading {
            step: 3,
            reason: "Surrender".to_string(),
            source: "AA.org".to_string(),
            title: "Daily Reflections".to_string(),
            text: "Made a decision...".to_string(),
            url: "https://www.aa.org/daily-reflections".to_string(),
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: ClassifiedReading = serde_json::from_str(&json).unwrap();
        assert_eq!(back.step, 3);
        assert_eq!(back.source, "AA.org");
    }
}
```

- [ ] **Step 2: Add mod to main.rs**

Replace `src/main.rs` content:

```rust
mod models;

fn main() {
    println!("iwiywi");
}
```

- [ ] **Step 3: Run test — expect PASS**

```bash
cargo test models
```

Expected: `classified_reading_round_trips_json ... ok`

- [ ] **Step 4: Commit**

```bash
git add src/models.rs src/main.rs
git commit -m "feat: add shared Reading types"
```

---

## Task 3: Config Module

**Files:**
- Create: `src/config.rs`

- [ ] **Step 1: Write failing tests**

Create `src/config.rs`:

```rust
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub ai: AiConfig,
    pub vercel: VercelConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiConfig {
    pub model: String,
    pub gateway_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VercelConfig {
    pub project_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ai: AiConfig {
                model: "anthropic/claude-haiku-4-5".to_string(),
                gateway_url: "https://ai-gateway.vercel.sh/v1".to_string(),
            },
            vercel: VercelConfig {
                project_url: String::new(),
            },
        }
    }
}

pub fn config_dir() -> PathBuf {
    dirs::home_dir()
        .expect("could not find home directory")
        .join(".iwiywi")
}

pub fn load_config() -> Result<Config> {
    let path = config_dir().join("config.toml");
    if !path.exists() {
        return Ok(Config::default());
    }
    let s = fs::read_to_string(&path).context("reading config.toml")?;
    toml::from_str(&s).context("parsing config.toml")
}

pub fn load_env() -> Result<()> {
    let env_path = config_dir().join(".env");
    if env_path.exists() {
        dotenvy::from_path(&env_path).context("loading .env")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_expected_model() {
        let c = Config::default();
        assert_eq!(c.ai.model, "anthropic/claude-haiku-4-5");
    }

    #[test]
    fn config_round_trips_toml() {
        let c = Config::default();
        let s = toml::to_string(&c).unwrap();
        let back: Config = toml::from_str(&s).unwrap();
        assert_eq!(back.ai.model, c.ai.model);
        assert_eq!(back.ai.gateway_url, c.ai.gateway_url);
    }
}
```

- [ ] **Step 2: Add `dirs` dependency to Cargo.toml**

Add under `[dependencies]`:

```toml
dirs = "5"
```

- [ ] **Step 3: Add mod to main.rs**

```rust
mod config;
mod models;

fn main() {
    println!("iwiywi");
}
```

- [ ] **Step 4: Run tests — expect PASS**

```bash
cargo test config
```

Expected: `default_config_has_expected_model ... ok`, `config_round_trips_toml ... ok`

- [ ] **Step 5: Commit**

```bash
git add src/config.rs src/main.rs Cargo.toml
git commit -m "feat: add config module"
```

---

## Task 4: Storage Module

**Files:**
- Create: `src/storage.rs`

- [ ] **Step 1: Write failing tests**

Create `src/storage.rs`:

```rust
use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::path::PathBuf;

use crate::config::config_dir;
use crate::models::ClassifiedReading;

pub fn readings_path() -> PathBuf {
    let date = Local::now().format("%Y-%m-%d").to_string();
    config_dir().join(format!("readings-{date}.json"))
}

pub fn write_readings(readings: &[ClassifiedReading]) -> Result<()> {
    let dir = config_dir();
    fs::create_dir_all(&dir).context("creating ~/.iwiywi")?;
    let json = serde_json::to_string_pretty(readings).context("serializing readings")?;
    fs::write(readings_path(), json).context("writing readings JSON")?;
    Ok(())
}

pub fn read_readings() -> Result<Vec<ClassifiedReading>> {
    let path = readings_path();
    if !path.exists() {
        return Ok(vec![]);
    }
    let s = fs::read_to_string(&path).context("reading readings JSON")?;
    serde_json::from_str(&s).context("parsing readings JSON")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ClassifiedReading;
    use std::env;

    fn fixture() -> ClassifiedReading {
        ClassifiedReading {
            step: 7,
            reason: "Humility".to_string(),
            source: "Test".to_string(),
            title: "Test Reading".to_string(),
            text: "Humbly asked...".to_string(),
            url: "https://example.com".to_string(),
        }
    }

    #[test]
    fn write_then_read_round_trips() {
        // Use a temp dir to avoid clobbering real data
        let tmp = env::temp_dir().join("iwiywi_test");
        fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("readings-test.json");
        let readings = vec![fixture()];
        let json = serde_json::to_string_pretty(&readings).unwrap();
        fs::write(&path, &json).unwrap();
        let back: Vec<ClassifiedReading> = serde_json::from_str(&json).unwrap();
        assert_eq!(back.len(), 1);
        assert_eq!(back[0].step, 7);
    }
}
```

- [ ] **Step 2: Add mod to main.rs**

```rust
mod config;
mod models;
mod storage;

fn main() {
    println!("iwiywi");
}
```

- [ ] **Step 3: Run tests — expect PASS**

```bash
cargo test storage
```

Expected: `write_then_read_round_trips ... ok`

- [ ] **Step 4: Commit**

```bash
git add src/storage.rs src/main.rs
git commit -m "feat: add storage module"
```

---

## Task 5: Scraper Module — Pattern + AA.org

**Files:**
- Create: `src/fetch/mod.rs`
- Create: `src/fetch/scraper.rs`

- [ ] **Step 1: Create fetch module directory and stubs**

```bash
mkdir -p src/fetch
```

Create `src/fetch/mod.rs`:

```rust
pub mod scraper;
pub mod classify;
pub mod html;
pub mod deploy;
```

- [ ] **Step 2: Write failing test for aa.org parser**

Create `src/fetch/scraper.rs`:

```rust
use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};

use crate::models::RawReading;

pub async fn scrape_all(client: &Client) -> Vec<RawReading> {
    let scrapers: Vec<(&str, fn(&str) -> Option<RawReading>)> = vec![
        ("aa_org", parse_aa_org),
        ("hazeldon", parse_hazeldon),
        ("happy_hour", parse_happy_hour),
        // remaining sources added in Task 6
    ];

    let urls: Vec<(&str, &str)> = vec![
        ("aa_org", "https://www.aa.org/daily-reflections"),
        ("hazeldon", "https://www.hazeldenbettyford.org/thought-for-the-day"),
        ("happy_hour", "https://www.aahappyhour.com/aa-daily-readings/"),
    ];

    let mut results = Vec::new();
    for (key, url) in &urls {
        match client.get(*url).send().await {
            Ok(resp) => match resp.text().await {
                Ok(html) => {
                    let parser = scrapers.iter().find(|(k, _)| k == key).map(|(_, f)| f);
                    if let Some(parse_fn) = parser {
                        if let Some(reading) = parse_fn(&html) {
                            results.push(reading);
                        } else {
                            eprintln!("warn: parser returned None for {key}");
                        }
                    }
                }
                Err(e) => eprintln!("warn: failed to read body from {url}: {e}"),
            },
            Err(e) => eprintln!("warn: failed to fetch {url}: {e}"),
        }
    }
    results
}

pub fn parse_aa_org(html: &str) -> Option<RawReading> {
    // IMPLEMENTATION NOTE: Visit https://www.aa.org/daily-reflections during
    // implementation and inspect the HTML to confirm selector below.
    // Common pattern: the reading text is in a <div class="field-item"> or similar.
    let document = Html::parse_document(html);
    let sel = Selector::parse(".field--name-body p").ok()?;
    let text: String = document
        .select(&sel)
        .map(|e| e.text().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();
    if text.is_empty() {
        return None;
    }
    Some(RawReading {
        source: "AA.org".to_string(),
        title: "Daily Reflections".to_string(),
        text,
        url: "https://www.aa.org/daily-reflections".to_string(),
    })
}

pub fn parse_hazeldon(html: &str) -> Option<RawReading> {
    // IMPLEMENTATION NOTE: Visit https://www.hazeldenbettyford.org/thought-for-the-day
    // and inspect HTML to confirm selector.
    let document = Html::parse_document(html);
    let sel = Selector::parse(".thought-body p").ok()?;
    let text: String = document
        .select(&sel)
        .map(|e| e.text().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();
    if text.is_empty() {
        return None;
    }
    Some(RawReading {
        source: "Hazeldon Betty Ford".to_string(),
        title: "Thought for the Day".to_string(),
        text,
        url: "https://www.hazeldenbettyford.org/thought-for-the-day".to_string(),
    })
}

pub fn parse_happy_hour(html: &str) -> Option<RawReading> {
    // IMPLEMENTATION NOTE: Visit https://www.aahappyhour.com/aa-daily-readings/
    // and inspect HTML to confirm selector.
    let document = Html::parse_document(html);
    let sel = Selector::parse(".entry-content p").ok()?;
    let text: String = document
        .select(&sel)
        .map(|e| e.text().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();
    if text.is_empty() {
        return None;
    }
    Some(RawReading {
        source: "AA Happy Hour".to_string(),
        title: "AA Daily Readings".to_string(),
        text,
        url: "https://www.aahappyhour.com/aa-daily-readings/".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_aa_org_extracts_text() {
        // Minimal fixture HTML matching the selector
        let html = r#"
            <html><body>
              <div class="field--name-body">
                <p>Made a decision to turn our will and our lives over to the care of God.</p>
              </div>
            </body></html>
        "#;
        let result = parse_aa_org(html);
        assert!(result.is_some());
        let r = result.unwrap();
        assert!(r.text.contains("Made a decision"));
        assert_eq!(r.source, "AA.org");
    }

    #[test]
    fn parse_aa_org_returns_none_for_missing_content() {
        let html = "<html><body><div>no reading here</div></body></html>";
        assert!(parse_aa_org(html).is_none());
    }

    #[test]
    fn parse_hazeldon_extracts_text() {
        let html = r#"
            <html><body>
              <div class="thought-body">
                <p>Humbly asked Him to remove our shortcomings.</p>
              </div>
            </body></html>
        "#;
        let result = parse_hazeldon(html);
        assert!(result.is_some());
        assert!(result.unwrap().text.contains("Humbly"));
    }

    #[test]
    fn parse_happy_hour_extracts_text() {
        let html = r#"
            <html><body>
              <div class="entry-content">
                <p>We admitted we were powerless over alcohol.</p>
              </div>
            </body></html>
        "#;
        let result = parse_happy_hour(html);
        assert!(result.is_some());
        assert!(result.unwrap().text.contains("powerless"));
    }
}
```

- [ ] **Step 3: Add fetch mod to main.rs**

```rust
mod config;
mod fetch;
mod models;
mod storage;

fn main() {
    println!("iwiywi");
}
```

Also create stub files for the other fetch submodules so it compiles:

`src/fetch/classify.rs`:
```rust
// stub — implemented in Task 7
```

`src/fetch/html.rs`:
```rust
// stub — implemented in Task 8
```

`src/fetch/deploy.rs`:
```rust
// stub — implemented in Task 9
```

- [ ] **Step 4: Run tests — expect PASS**

```bash
cargo test fetch::scraper
```

Expected: 4 tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/fetch/ src/main.rs
git commit -m "feat: add scraper module with first 3 sources"
```

---

## Task 6: Remaining 9 Scrapers

**Files:**
- Modify: `src/fetch/scraper.rs`

**IMPORTANT — selector verification:** Before implementing each `parse_*` function, fetch the live URL in a browser and inspect the HTML to find the correct CSS selector for the reading text. The selectors below are best-guess starting points; update them to match actual site structure.

- [ ] **Step 1: Write failing tests for all 9 remaining sources**

Add to the `#[cfg(test)]` block in `src/fetch/scraper.rs`:

```rust
    #[test]
    fn parse_silkworth_extracts_text() {
        let html = r#"<html><body><div class="content"><p>We are not cured of alcoholism.</p></div></body></html>"#;
        let result = parse_silkworth(html);
        assert!(result.is_some());
        assert!(result.unwrap().text.contains("alcoholism"));
    }

    #[test]
    fn parse_google_extracts_text() {
        // Google featured snippet fixture
        let html = r#"<html><body><div class="BNeawe"><span>God grant me the serenity.</span></div></body></html>"#;
        let result = parse_google_snippet(html);
        assert!(result.is_some());
    }

    #[test]
    fn parse_reddit_extracts_text() {
        let html = r#"<html><body><div data-testid="post-content"><p>Today's reading discussion.</p></div></body></html>"#;
        let result = parse_reddit(html);
        assert!(result.is_some());
    }
```

- [ ] **Step 2: Implement the remaining parse functions**

Add to `src/fetch/scraper.rs` (after `parse_happy_hour`):

```rust
pub fn parse_silkworth(html: &str) -> Option<RawReading> {
    // https://silkworth.net — verify selector on live site
    let document = Html::parse_document(html);
    let sel = Selector::parse(".content p").ok()?;
    let text = first_nonempty_paragraph(&document, &sel)?;
    Some(RawReading {
        source: "Silkworth.net".to_string(),
        title: "Daily Reading".to_string(),
        text,
        url: "https://silkworth.net".to_string(),
    })
}

pub fn parse_aa_online_meeting(html: &str) -> Option<RawReading> {
    // https://www.aaonlinemeeting.net — verify selector on live site
    let document = Html::parse_document(html);
    let sel = Selector::parse(".reading-text p").ok()?;
    let text = first_nonempty_paragraph(&document, &sel)?;
    Some(RawReading {
        source: "AA Online Meeting".to_string(),
        title: "Daily Reading".to_string(),
        text,
        url: "https://www.aaonlinemeeting.net".to_string(),
    })
}

pub fn parse_aa_big_book(html: &str) -> Option<RawReading> {
    // https://www.aabigbook.com — verify selector on live site
    let document = Html::parse_document(html);
    let sel = Selector::parse(".post-content p").ok()?;
    let text = first_nonempty_paragraph(&document, &sel)?;
    Some(RawReading {
        source: "AA Big Book".to_string(),
        title: "Daily Reading".to_string(),
        text,
        url: "https://www.aabigbook.com".to_string(),
    })
}

pub fn parse_recovering_courage(html: &str) -> Option<RawReading> {
    // https://www.recoveringcourage.com — verify selector on live site
    let document = Html::parse_document(html);
    let sel = Selector::parse("article p").ok()?;
    let text = first_nonempty_paragraph(&document, &sel)?;
    Some(RawReading {
        source: "Recovering Courage".to_string(),
        title: "Daily Reading".to_string(),
        text,
        url: "https://www.recoveringcourage.com".to_string(),
    })
}

pub fn parse_odat(html: &str) -> Option<RawReading> {
    // https://odat.us — One Day At A Time — verify selector on live site
    let document = Html::parse_document(html);
    let sel = Selector::parse(".daily-reading p").ok()?;
    let text = first_nonempty_paragraph(&document, &sel)?;
    Some(RawReading {
        source: "One Day At A Time".to_string(),
        title: "Daily Reading".to_string(),
        text,
        url: "https://odat.us".to_string(),
    })
}

pub fn parse_joe_and_charlie(html: &str) -> Option<RawReading> {
    // https://joeancharlie.com — A Program for You — verify selector on live site
    let document = Html::parse_document(html);
    let sel = Selector::parse(".entry-content p").ok()?;
    let text = first_nonempty_paragraph(&document, &sel)?;
    Some(RawReading {
        source: "Joe and Charlie".to_string(),
        title: "A Program for You".to_string(),
        text,
        url: "https://joeancharlie.com".to_string(),
    })
}

pub fn parse_google_snippet(html: &str) -> Option<RawReading> {
    // Google featured snippet — selector may change; verify on live search result
    let document = Html::parse_document(html);
    // Try multiple known Google snippet selectors
    for selector_str in &[".BNeawe", ".hgKElc", "[data-attrid='wa:/description']"] {
        if let Ok(sel) = Selector::parse(selector_str) {
            let text: String = document
                .select(&sel)
                .next()
                .map(|e| e.text().collect::<String>())?
                .trim()
                .to_string();
            if !text.is_empty() {
                return Some(RawReading {
                    source: "Google".to_string(),
                    title: "AA Thought for the Day".to_string(),
                    text,
                    url: "https://www.google.com/search?q=aa+thought+for+the+day".to_string(),
                });
            }
        }
    }
    None
}

pub fn parse_reddit(html: &str) -> Option<RawReading> {
    // r/alcoholicsanonymous — daily thread or top post
    let document = Html::parse_document(html);
    let sel = Selector::parse("[data-testid='post-content'] p").ok()?;
    let text = first_nonempty_paragraph(&document, &sel)?;
    Some(RawReading {
        source: "Reddit r/alcoholicsanonymous".to_string(),
        title: "Daily Thread".to_string(),
        text,
        url: "https://www.reddit.com/r/alcoholicsanonymous/".to_string(),
    })
}

// Shared helper: returns first non-empty paragraph text from a selector
fn first_nonempty_paragraph(document: &Html, sel: &Selector) -> Option<String> {
    document
        .select(sel)
        .map(|e| e.text().collect::<String>().trim().to_string())
        .find(|s| !s.is_empty())
}
```

- [ ] **Step 3: Update `scrape_all` to include all 12 sources**

Replace the `scrapers` and `urls` vecs in `scrape_all`:

```rust
pub async fn scrape_all(client: &Client) -> Vec<RawReading> {
    let sources: Vec<(&str, &str, fn(&str) -> Option<RawReading>)> = vec![
        ("aa_org",              "https://www.aa.org/daily-reflections",                                           parse_aa_org),
        ("hazeldon",            "https://www.hazeldenbettyford.org/thought-for-the-day",                          parse_hazeldon),
        ("happy_hour",          "https://www.aahappyhour.com/aa-daily-readings/",                                  parse_happy_hour),
        ("silkworth",           "https://silkworth.net",                                                           parse_silkworth),
        ("aa_online_meeting",   "https://www.aaonlinemeeting.net",                                                 parse_aa_online_meeting),
        ("aa_big_book",         "https://www.aabigbook.com",                                                       parse_aa_big_book),
        ("recovering_courage",  "https://www.recoveringcourage.com",                                               parse_recovering_courage),
        ("odat",                "https://odat.us",                                                                 parse_odat),
        ("joe_and_charlie",     "https://joeancharlie.com",                                                        parse_joe_and_charlie),
        ("google",              "https://www.google.com/search?q=aa+thought+for+the+day",                         parse_google_snippet),
        ("reddit",              "https://www.reddit.com/r/alcoholicsanonymous/search/?q=daily+reading&sort=new",  parse_reddit),
        // Source 12: identify and add during implementation if any above are unreachable
    ];

    let mut results = Vec::new();
    for (key, url, parse_fn) in &sources {
        match client.get(*url)
            .header("User-Agent", "Mozilla/5.0 (compatible; iwiywi/0.1)")
            .send().await
        {
            Ok(resp) => match resp.text().await {
                Ok(html) => {
                    if let Some(reading) = parse_fn(&html) {
                        results.push(reading);
                    } else {
                        eprintln!("warn: no reading found at {key}");
                    }
                }
                Err(e) => eprintln!("warn: bad body from {key}: {e}"),
            },
            Err(e) => eprintln!("warn: fetch failed for {key}: {e}"),
        }
    }
    results
}
```

- [ ] **Step 4: Run tests — expect PASS**

```bash
cargo test fetch::scraper
```

Expected: all scraper tests pass.

- [ ] **Step 5: Commit**

```bash
git add src/fetch/scraper.rs
git commit -m "feat: add all 12 source scrapers"
```

---

## Task 7: AI Classification Module

**Files:**
- Modify: `src/fetch/classify.rs`

- [ ] **Step 1: Write failing test using mockito**

Replace `src/fetch/classify.rs`:

```rust
use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::models::{ClassifiedReading, RawReading};

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    response_format: ResponseFormat,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: String,
}

#[derive(Serialize)]
struct ResponseFormat {
    r#type: &'static str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: String,
}

#[derive(Deserialize)]
struct StepResult {
    step: u8,
    reason: String,
}

const SYSTEM_PROMPT: &str =
    "You are an AA step classifier. Given a daily reading excerpt, return the \
     single most relevant AA step number (1-12) and a one-sentence reason. \
     Respond with JSON only: {\"step\": 3, \"reason\": \"...\"}";

pub async fn classify(
    client: &Client,
    config: &Config,
    reading: RawReading,
) -> Result<ClassifiedReading> {
    let token = std::env::var("VERCEL_AI_GATEWAY_TOKEN")
        .context("VERCEL_AI_GATEWAY_TOKEN not set")?;

    let request = ChatRequest {
        model: &config.ai.model,
        messages: vec![
            Message { role: "system", content: SYSTEM_PROMPT.to_string() },
            Message { role: "user", content: reading.text.clone() },
        ],
        response_format: ResponseFormat { r#type: "json_object" },
    };

    let url = format!("{}/chat/completions", config.ai.gateway_url);
    let resp = client
        .post(&url)
        .bearer_auth(&token)
        .json(&request)
        .send()
        .await
        .context("calling Vercel AI Gateway")?;

    let chat: ChatResponse = resp.json().await.context("parsing AI response")?;
    let content = &chat.choices[0].message.content;
    let result: StepResult = serde_json::from_str(content)
        .context("parsing step JSON from AI response")?;

    // Clamp step to valid range
    let step = result.step.clamp(1, 12);

    Ok(ClassifiedReading {
        step,
        reason: result.reason,
        source: reading.source,
        title: reading.title,
        text: reading.text,
        url: reading.url,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn classify_parses_step_from_gateway_response() {
        let mut server = Server::new_async().await;
        let mock = server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "choices": [{
                    "message": {
                        "content": "{\"step\": 3, \"reason\": \"Surrender to a Higher Power\"}"
                    }
                }]
            }"#)
            .create_async()
            .await;

        std::env::set_var("VERCEL_AI_GATEWAY_TOKEN", "test-token");

        let client = Client::new();
        let config = crate::config::Config {
            ai: crate::config::AiConfig {
                model: "anthropic/claude-haiku-4-5".to_string(),
                gateway_url: server.url(),
            },
            vercel: crate::config::VercelConfig {
                project_url: String::new(),
            },
        };

        let raw = RawReading {
            source: "AA.org".to_string(),
            title: "Daily Reflections".to_string(),
            text: "Made a decision to turn our will...".to_string(),
            url: "https://www.aa.org/daily-reflections".to_string(),
        };

        let result = classify(&client, &config, raw).await.unwrap();
        assert_eq!(result.step, 3);
        assert_eq!(result.reason, "Surrender to a Higher Power");
        assert_eq!(result.source, "AA.org");
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn classify_clamps_step_to_valid_range() {
        let mut server = Server::new_async().await;
        server
            .mock("POST", "/chat/completions")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"choices":[{"message":{"content":"{\"step\":99,\"reason\":\"out of range\"}"}}]}"#)
            .create_async()
            .await;

        std::env::set_var("VERCEL_AI_GATEWAY_TOKEN", "test-token");
        let client = Client::new();
        let config = crate::config::Config {
            ai: crate::config::AiConfig {
                model: "test".to_string(),
                gateway_url: server.url(),
            },
            vercel: crate::config::VercelConfig { project_url: String::new() },
        };
        let raw = RawReading {
            source: "Test".to_string(), title: "Test".to_string(),
            text: "text".to_string(), url: "http://test".to_string(),
        };
        let result = classify(&client, &config, raw).await.unwrap();
        assert_eq!(result.step, 12); // clamped from 99
    }
}
```

- [ ] **Step 2: Run tests — expect PASS**

```bash
cargo test fetch::classify
```

Expected: both tests pass.

- [ ] **Step 3: Commit**

```bash
git add src/fetch/classify.rs
git commit -m "feat: add AI classification via Vercel AI Gateway"
```

---

## Task 8: HTML Generation

**Files:**
- Modify: `src/fetch/html.rs`

- [ ] **Step 1: Write failing test**

Replace `src/fetch/html.rs`:

```rust
use crate::models::ClassifiedReading;

// Map step number to a hex color matching TUI palette
fn step_color(step: u8) -> &'static str {
    match step {
        1  => "#ff6b6b",
        2  => "#ffd93d",
        3  => "#6bcbff",
        4  => "#c678dd",
        5  => "#56b6c2",
        6  => "#98c379",
        7  => "#e06c75",
        8  => "#e5c07b",
        9  => "#61afef",
        10 => "#be5af7",
        11 => "#4ec9b0",
        12 => "#b5f0a5",
        _  => "#ffffff",
    }
}

pub fn render(readings: &[ClassifiedReading], vercel_url: &str) -> String {
    let cards: String = readings.iter().map(|r| {
        let color = step_color(r.step);
        let text_escaped = html_escape(&r.text);
        let source_escaped = html_escape(&r.source);
        format!(r#"
        <div class="card">
          <div class="card-header" style="color:{color}">
            <span class="step">Step {step}</span>
            <span class="source">{source}</span>
          </div>
          <p class="text">{text}</p>
        </div>
        "#,
            color = color,
            step = r.step,
            source = source_escaped,
            text = text_escaped,
        )
    }).collect();

    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>iwiywi — Daily AA Readings</title>
<style>
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{ background: #0d1117; color: #e6edf3; font-family: -apple-system, sans-serif; padding: 16px; }}
  h1 {{ color: #58a6ff; font-size: 16px; letter-spacing: 2px; text-transform: uppercase; margin-bottom: 20px; padding-bottom: 10px; border-bottom: 1px solid #21262d; }}
  .card {{ margin-bottom: 20px; }}
  .card-header {{ display: flex; justify-content: space-between; font-size: 11px; font-weight: bold; letter-spacing: 1px; text-transform: uppercase; margin-bottom: 8px; }}
  .text {{ font-size: 15px; line-height: 1.7; color: #c9d1d9; padding-left: 12px; border-left: 3px solid currentColor; }}
  .divider {{ border: none; border-top: 1px solid #21262d; margin: 20px 0; }}
</style>
</head>
<body>
<h1>iwiywi — {date}</h1>
{cards}
</body>
</html>"#,
        date = chrono::Local::now().format("%B %-d, %Y"),
        cards = cards,
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_reading(step: u8) -> ClassifiedReading {
        ClassifiedReading {
            step,
            reason: "test".to_string(),
            source: "AA.org".to_string(),
            title: "Daily Reflections".to_string(),
            text: "Made a <decision> & more".to_string(),
            url: "https://www.aa.org/daily-reflections".to_string(),
        }
    }

    #[test]
    fn render_produces_valid_html_structure() {
        let readings = vec![fixture_reading(3), fixture_reading(7)];
        let html = render(&readings, "https://iwiywi.vercel.app");
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Step 3"));
        assert!(html.contains("Step 7"));
    }

    #[test]
    fn render_escapes_html_in_text() {
        let readings = vec![fixture_reading(1)];
        let html = render(&readings, "https://iwiywi.vercel.app");
        assert!(html.contains("&lt;decision&gt;"));
        assert!(html.contains("&amp;"));
        assert!(!html.contains("<decision>"));
    }

    #[test]
    fn render_uses_step_color() {
        let readings = vec![fixture_reading(3)];
        let html = render(&readings, "https://iwiywi.vercel.app");
        assert!(html.contains("#6bcbff")); // Step 3 color
    }
}
```

- [ ] **Step 2: Run tests — expect PASS**

```bash
cargo test fetch::html
```

Expected: 3 tests pass.

- [ ] **Step 3: Commit**

```bash
git add src/fetch/html.rs
git commit -m "feat: add mobile HTML renderer"
```

---

## Task 9: Vercel Deploy Module

**Files:**
- Modify: `src/fetch/deploy.rs`

- [ ] **Step 1: Implement deploy functions**

Replace `src/fetch/deploy.rs`:

```rust
use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

/// Refreshes ~/.iwiywi/.env by running `vercel env pull`
pub fn env_pull(env_path: &Path) -> Result<()> {
    let status = Command::new("vercel")
        .args(["env", "pull", "--yes"])
        .arg(env_path)
        .status()
        .context("running `vercel env pull` — is the vercel CLI installed?")?;

    if !status.success() {
        bail!("`vercel env pull` exited with {}", status);
    }
    Ok(())
}

/// Writes index.html to dist_dir and deploys via `vercel deploy --prod`
pub fn deploy(dist_dir: &Path) -> Result<()> {
    let output = Command::new("vercel")
        .args(["deploy", "--prod"])
        .arg(dist_dir)
        .output()
        .context("running `vercel deploy` — is the vercel CLI installed and authenticated?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("`vercel deploy` failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    eprintln!("Deployed: {stdout}");
    Ok(())
}
```

- [ ] **Step 2: Verify it compiles**

```bash
cargo build 2>&1 | grep -v "warning"
```

Expected: no errors.

- [ ] **Step 3: Commit**

```bash
git add src/fetch/deploy.rs
git commit -m "feat: add Vercel CLI deploy module"
```

---

## Task 10: Fetch Orchestration

**Files:**
- Modify: `src/fetch/mod.rs`

- [ ] **Step 1: Implement run()**

Replace `src/fetch/mod.rs`:

```rust
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
```

- [ ] **Step 2: Verify it compiles**

```bash
cargo build 2>&1 | grep "^error"
```

Expected: no output (no errors).

- [ ] **Step 3: Commit**

```bash
git add src/fetch/mod.rs
git commit -m "feat: fetch orchestration — scrape, classify, store, deploy"
```

---

## Task 11: Main CLI Entry Point

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Implement clap CLI**

Replace `src/main.rs`:

```rust
mod config;
mod fetch;
mod models;
mod storage;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "iwiywi", about = "It Works If You Work It — daily AA readings")]
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
            // Implemented in Sub-Plan 2
            eprintln!("install not yet implemented — coming in next release");
        }
        None => {
            // TUI — implemented in Sub-Plan 2
            eprintln!("TUI not yet implemented — run `iwiywi fetch` to populate readings");
        }
    }
    Ok(())
}
```

- [ ] **Step 2: Build and smoke-test**

```bash
cargo build --release
./target/release/iwiywi --help
```

Expected output:
```
It Works If You Work It — daily AA readings

Usage: iwiywi [COMMAND]

Commands:
  fetch    Fetch today's AA readings, classify, and deploy to Vercel
  install  Install launchd job to run fetch at 6am daily
  help     Print this message or the help of the given subcommand(s)
```

- [ ] **Step 3: Commit**

```bash
git add src/main.rs
git commit -m "feat: clap CLI with fetch and install subcommands"
```

---

## Task 12: End-to-End Integration Test

- [ ] **Step 1: Set up Vercel project (one-time)**

```bash
# Authenticate if needed
vercel login

# Create project
vercel projects create iwiywi --yes

# Store your AI Gateway token
vercel env add VERCEL_AI_GATEWAY_TOKEN production
# (enter your token when prompted)
```

- [ ] **Step 2: Initialize config**

```bash
mkdir -p ~/.iwiywi
cat > ~/.iwiywi/config.toml << 'EOF'
[ai]
model = "anthropic/claude-haiku-4-5"
gateway_url = "https://ai-gateway.vercel.sh/v1"

[vercel]
project_url = "https://iwiywi.vercel.app"
EOF
```

**NOTE:** Verify the Vercel AI Gateway URL against the current Vercel docs — the URL in the config above is from the spec and should be confirmed before running.

- [ ] **Step 3: Run fetch manually**

```bash
./target/release/iwiywi fetch
```

Expected: output like:
```
Pulling env vars...
Scraping sources...
Got 10 raw readings
Classifying readings...
Classified 10 readings
Saved readings to /Users/<you>/.iwiywi/readings-2026-04-14.json
Deploying to Vercel...
Deployed: https://iwiywi.vercel.app
Done.
```

- [ ] **Step 4: Verify JSON**

```bash
cat ~/.iwiywi/readings-$(date +%F).json | python3 -m json.tool | head -40
```

Expected: pretty-printed JSON with 10+ entries, each containing `step` (1–12), `reason`, `source`, `text`, `url`.

- [ ] **Step 5: Verify Vercel deployment**

Open the Vercel URL in a browser. Expected: dark mobile page with AA readings, each showing step number and source.

- [ ] **Step 6: Fix any scraper selectors that returned no data**

For each source that returned `warn: no reading found at <key>`, visit that URL in a browser, inspect the HTML, and update the CSS selector in `src/fetch/scraper.rs`. Then repeat Steps 3–5.

- [ ] **Step 7: Final commit**

```bash
git add -A
git commit -m "chore: end-to-end fetch pipeline verified"
```

---

## Done Signal

Sub-Plan 1 is complete when:
- `cargo test` — all tests pass
- `~/.iwiywi/readings-$(date +%F).json` exists with 10+ classified readings
- Vercel URL loads the mobile page with today's readings
- `iwiywi --help` shows the CLI correctly
