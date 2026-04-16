//! Ambient weather anchor. Fetches a one-line summary from wttr.in at
//! startup (best-effort, 5s timeout) and renders it in the top-left of
//! the viewport — paired with the moon/sober anchor in the top-right for
//! the "dedicated monitor" aesthetic.
//!
//! Location defaults to wttr.in's IP-based geo; set `IWIYWI_WEATHER_LOC`
//! to override (e.g. `seattle`, `94102`, `SEA`). Respects `NO_COLOR` by
//! emitting no ANSI escapes itself — ratatui styles handle that.

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct WeatherSnapshot {
    /// Fully-formatted one-liner from wttr.in, e.g. `"Seattle: ☀ +54°F"`.
    pub text: String,
}

/// Best-effort fetch. Returns `None` on any failure (DNS, timeout,
/// non-2xx, empty body, "Unknown location"). Cached content doesn't
/// exist — wttr.in is cheap enough to hit once per iwiywi launch, and
/// caching-to-disk would need cache-invalidation logic we don't need.
pub async fn fetch() -> Option<WeatherSnapshot> {
    let loc = std::env::var("IWIYWI_WEATHER_LOC").unwrap_or_default();
    let url = format!("https://wttr.in/{loc}?format=3");
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .ok()?;
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (compatible; iwiywi/0.6)")
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let text = resp.text().await.ok()?.trim().to_string();
    // wttr.in quirks: empty body, an error page, or an "Unknown location"
    // message when it can't resolve — treat all as a fetch failure.
    if text.is_empty()
        || text.contains("Unknown location")
        || text.contains("ERROR")
        || text.len() > 80
    {
        return None;
    }
    Some(WeatherSnapshot { text })
}
