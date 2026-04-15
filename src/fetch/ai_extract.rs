//! AI-driven reading extraction. For sources whose live HTML structure no
//! longer matches a stable CSS selector (or whose live site is dead and we're
//! fetching from Wayback), we send the page text to the AI gateway and ask
//! it to return only the daily reading body.

use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};

use crate::config::Config;
use crate::fetch::ai::{post_chat, ChatOpts};
use crate::models::RawReading;

const SYSTEM_PROMPT: &str =
    "You extract the most recent daily AA meditation, reflection, or reading from web pages. \
     Respond with ONLY the body text of the reading itself — no title, no date, no source attribution, \
     no JSON, no commentary. If the page contains no daily AA reading, respond with the literal word: NONE.";

const MAX_PAGE_CHARS: usize = 4000;

pub async fn extract_reading(
    client: &Client,
    config: &Config,
    html: &str,
    source: &str,
    title: &str,
    url: &str,
) -> Result<RawReading> {
    let body_text = strip_html_to_text(html);
    let truncated: String = body_text.chars().take(MAX_PAGE_CHARS).collect();

    let opts = ChatOpts {
        max_tokens: Some(1024),
        temperature: Some(0.3),
        json_mode: false,
    };
    let raw = post_chat(client, config, SYSTEM_PROMPT, &truncated, opts).await?;
    let raw_text = raw.trim();

    if raw_text.is_empty() || raw_text == "NONE" {
        anyhow::bail!("AI returned no reading for {source}");
    }

    Ok(RawReading {
        source: source.to_string(),
        title: title.to_string(),
        text: raw_text.to_string(),
        url: url.to_string(),
    })
}

/// Collapse HTML to plain visible text. Keeps whitespace minimal.
/// Skips content inside <script> and <style> tags.
pub fn strip_html_to_text(html: &str) -> String {
    use scraper::node::Node;

    let doc = Html::parse_document(html);
    let body_sel = Selector::parse("body").expect("body selector valid");
    let script_sel = Selector::parse("script, style").expect("script/style selector valid");
    let mut out = String::new();

    for body in doc.select(&body_sel) {
        let skip_ids: std::collections::HashSet<_> = body
            .select(&script_sel)
            .flat_map(|el| {
                std::iter::once(el.id()).chain(el.descendants().map(|d| d.id()))
            })
            .collect();

        for node_ref in body.descendants() {
            if skip_ids.contains(&node_ref.id()) {
                continue;
            }
            if let Node::Text(text) = node_ref.value() {
                let t = text.trim();
                if !t.is_empty() {
                    if !out.is_empty() {
                        out.push(' ');
                    }
                    out.push_str(t);
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_html_to_text_extracts_visible_text_only() {
        let html = r#"<html><head><title>nope</title></head><body><h1>Reading</h1><p>Be still and know.</p><script>console.log('x')</script></body></html>"#;
        let text = strip_html_to_text(html);
        assert!(text.contains("Reading"));
        assert!(text.contains("Be still and know."));
        assert!(!text.contains("console.log"));
    }

    #[test]
    fn strip_html_to_text_collapses_whitespace() {
        let html = "<html><body><p>a</p><p>b</p><p>c</p></body></html>";
        let text = strip_html_to_text(html);
        assert_eq!(text, "a b c");
    }

    #[test]
    fn strip_html_to_text_handles_empty_body() {
        let html = "<html><body></body></html>";
        let text = strip_html_to_text(html);
        assert!(text.is_empty());
    }
}
