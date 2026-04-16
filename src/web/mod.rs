//! HTTP server that exposes the pulse as a web page you can open from any
//! browser. `iwiywi serve` boots this; the CLI binary and the server share
//! the same data sources, storage layout, and env-var knobs as the TUI.
//!
//! Design: the server is effectively a single-page app. On startup we build
//! the same source list the TUI builds, serialize every `PulseItem` once,
//! and hand the whole bundle to the browser. Client-side JS then handles
//! ordering, focus, step filtering, auto-advance, and keyboard nav — matching
//! the TUI's behavior without the server having to track per-viewer state.
//!
//! This keeps the server stateless: multiple people can hit the same URL
//! and each gets an independent pulse. The daily fetch job is what changes
//! today's readings; the web process only re-reads them on request.

use anyhow::{Context, Result};
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::config;
use crate::pulse::{self, PulseItem, PulseSource};
use crate::storage::read_readings;

const INDEX_HTML: &str = include_str!("index.html");
const PULSE_CSS: &str = include_str!("pulse.css");
const PULSE_JS: &str = include_str!("pulse.js");
const MANIFEST_JSON: &str = include_str!("manifest.json");

/// Snapshot of everything the browser needs for one pulse session. Built
/// fresh on each `/api/items` hit so today's readings (written by the 6am
/// fetch job) are picked up without restarting the server.
#[derive(Serialize)]
struct ItemsResponse {
    items: Vec<PulseItem>,
    sobriety_days: Option<i64>,
    pulse_secs: Option<u64>,
    /// Local date the snapshot was taken, as `YYYY-MM-DD`. Browsers display
    /// this and also poll for a change to know when to reload fresh readings.
    date: String,
}

struct AppState {
    /// Grapevine HTML captured once at server start. Same trade-off as the
    /// TUI: a stale quote-of-the-day is fine; restarting the process gets a
    /// fresh one. Keeps request handling off the public internet.
    grapevine_html: Option<String>,
}

pub async fn run(bind: &str, port: u16, grapevine_html: Option<String>) -> Result<()> {
    let state = Arc::new(AppState { grapevine_html });
    let app = Router::new()
        .route("/", get(index))
        .route("/pulse.css", get(css))
        .route("/pulse.js", get(js))
        .route("/manifest.json", get(manifest))
        .route("/api/items", get(items))
        .route("/healthz", get(healthz))
        .with_state(state);

    let addr: SocketAddr = format!("{bind}:{port}")
        .parse()
        .with_context(|| format!("parsing bind address {bind}:{port}"))?;
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("binding {addr}"))?;
    eprintln!("iwiywi serving on http://{addr}");
    axum::serve(listener, app)
        .await
        .context("running http server")?;
    Ok(())
}

async fn index() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        INDEX_HTML,
    )
}

async fn css() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        PULSE_CSS,
    )
}

async fn js() -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            "application/javascript; charset=utf-8",
        )],
        PULSE_JS,
    )
}

async fn manifest() -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            "application/manifest+json; charset=utf-8",
        )],
        MANIFEST_JSON,
    )
}

async fn healthz() -> &'static str {
    "ok"
}

async fn items(State(state): State<Arc<AppState>>) -> Response {
    match build_items(&state.grapevine_html) {
        Ok(body) => Json(body).into_response(),
        Err(e) => {
            let msg = format!("error building pulse: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
        }
    }
}

fn build_items(grapevine_html: &Option<String>) -> Result<ItemsResponse> {
    let readings = read_readings().context("reading today's readings")?;
    let today_basename = format!("readings-{}.json", chrono::Local::now().format("%Y-%m-%d"));
    let sources: Vec<Box<dyn PulseSource>> = vec![
        Box::new(pulse::today::TodayReadings::from_readings(&readings)),
        Box::new(pulse::historical::HistoricalReadings::load_from(
            &config::config_dir(),
            &today_basename,
        )),
        Box::new(pulse::bundled::BigBookQuotes::load()),
        Box::new(pulse::bundled::Prayers::load()),
        Box::new(pulse::bundled::StepExplainers::load()),
        Box::new(pulse::bundled::Traditions::load()),
        Box::new(pulse::bundled::Concepts::load()),
        Box::new(pulse::bundled::Slogans::load()),
        Box::new(pulse::grapevine::Grapevine::from_html(
            grapevine_html.as_deref(),
        )),
        Box::new(pulse::favorites::Favorites::load_from(
            config::config_dir().join("favorites.json"),
        )),
    ];

    // Flatten every source's items into one list with the same dedupe rule
    // the PulseMixer uses: (source name, body) hashed. The client does its
    // own ordering/filtering, so we don't pre-sort.
    use sha2::{Digest, Sha256};
    let mut items: Vec<PulseItem> = Vec::new();
    let mut seen: std::collections::HashSet<[u8; 32]> = std::collections::HashSet::new();
    for src in &sources {
        for item in src.items() {
            let mut hasher = Sha256::new();
            hasher.update(src.name().as_bytes());
            hasher.update([0u8]);
            hasher.update(item.body.as_bytes());
            let digest: [u8; 32] = hasher.finalize().into();
            if seen.insert(digest) {
                items.push(item.clone());
            }
        }
    }

    Ok(ItemsResponse {
        items,
        sobriety_days: config::sobriety_days(),
        pulse_secs: config::pulse_secs().map(|d| d.as_secs()),
        date: chrono::Local::now().format("%Y-%m-%d").to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_items_returns_a_non_empty_bundle() {
        // With only bundled sources available in a fresh test env, there
        // should still be prayers/steps/slogans/etc. Confirms the web mode
        // never serves an empty page even if today's readings haven't been
        // fetched yet.
        let resp = build_items(&None).expect("build items");
        assert!(!resp.items.is_empty(), "bundled sources should yield items");
        assert_eq!(resp.date.len(), 10, "date is YYYY-MM-DD");
    }
}
