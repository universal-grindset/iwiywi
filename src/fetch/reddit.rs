//! Best-effort Reddit fetcher. Uses the public `.json` endpoint (no OAuth
//! needed for read-only access), with a 5s timeout and a polite UA. Merges
//! posts from the configured subreddits into a single JSON envelope with
//! `data.children[]`, which `community::extract_post_excerpts` then parses.

use reqwest::Client;
use serde_json::{json, Value};

const SUBS: [&str; 2] = ["stopdrinking", "alcoholicsanonymous"];

/// Fetch top-of-day posts from each sub and fold into one synthetic listing
/// (same shape as a single `.json` response). Returns `None` if every
/// subreddit fails.
pub async fn fetch_community_json() -> Option<String> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .ok()?;

    let mut children: Vec<Value> = Vec::new();
    for sub in SUBS {
        if let Some(v) = fetch_one(&client, sub).await {
            if let Some(arr) = v.pointer("/data/children").and_then(Value::as_array) {
                children.extend(arr.iter().cloned());
            }
        }
    }
    if children.is_empty() {
        return None;
    }

    let envelope = json!({ "data": { "children": children } });
    serde_json::to_string(&envelope).ok()
}

async fn fetch_one(client: &Client, sub: &str) -> Option<Value> {
    let url = format!("https://old.reddit.com/r/{sub}/top.json?t=day&limit=25");
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (compatible; iwiywi/0.6)")
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    resp.json::<Value>().await.ok()
}
