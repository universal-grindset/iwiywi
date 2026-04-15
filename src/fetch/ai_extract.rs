//! AI-driven reading extraction. For sources whose live HTML structure no
//! longer matches a stable CSS selector (or whose live site is dead and we're
//! fetching from Wayback), we send the page text to the AI gateway and ask
//! it to return only the daily reading body.

use anyhow::{Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::models::RawReading;

const SYSTEM_PROMPT: &str =
    "You extract the most recent daily AA meditation, reflection, or reading from web pages. \
     Respond with ONLY the body text of the reading itself — no title, no date, no source attribution, \
     no JSON, no commentary. If the page contains no daily AA reading, respond with the literal word: NONE.";

const MAX_PAGE_CHARS: usize = 4000;

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: String,
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

    let request = ChatRequest {
        model: &config.ai.model,
        messages: vec![
            Message { role: "system", content: SYSTEM_PROMPT.to_string() },
            Message { role: "user", content: truncated },
        ],
    };

    let endpoint = match &config.ai.api_version {
        Some(v) => format!("{}/chat/completions?api-version={v}", config.ai.gateway_url),
        None => format!("{}/chat/completions", config.ai.gateway_url),
    };
    let req = client.post(&endpoint).json(&request);
    let req = match &config.ai.api_version {
        Some(_) => req.header(
            "api-key",
            std::env::var("AZURE_OPENAI_API_KEY").context("AZURE_OPENAI_API_KEY not set")?,
        ),
        None => req.bearer_auth(
            std::env::var("VERCEL_AI_GATEWAY_TOKEN").context("VERCEL_AI_GATEWAY_TOKEN not set")?,
        ),
    };
    let resp = req.send().await.context("calling AI gateway for extraction")?;
    let chat: ChatResponse = resp.json().await.context("parsing AI extraction response")?;
    let raw_text = chat.choices.first().map(|c| c.message.content.trim()).unwrap_or("");

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
        // Collect node IDs of script/style subtrees to skip
        let skip_ids: std::collections::HashSet<_> = body
            .select(&script_sel)
            .flat_map(|el| {
                // Collect the element itself and all its descendants
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
