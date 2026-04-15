use reqwest::Client;
use scraper::{Html, Selector};

use crate::models::RawReading;

type ParseFn = fn(&str) -> Option<RawReading>;
type Source = (&'static str, &'static str, ParseFn);

/// Build the Wayback "latest snapshot" URL for the given live URL.
/// `web.archive.org/web/2/<url>` redirects to the latest snapshot.
pub fn wayback_url(live: &str) -> String {
    format!("https://web.archive.org/web/2/{live}")
}

/// Trim trailing/embedded boilerplate that often comes with scraped content
/// (copyright lines, "all rights reserved" notices, etc.).
fn trim_boilerplate(text: &str) -> String {
    let markers = [
        "All rights reserved",
        "registered trademarks",
        "Copyright (c)",
        "© ",
        "is the official Website",
        "Alcoholics Anonymous and the",
    ];
    let mut out = text.to_string();
    for m in markers {
        if let Some(idx) = out.find(m) {
            // Cut at the start of the line containing the marker.
            let line_start = out[..idx].rfind('\n').map(|p| p + 1).unwrap_or(0);
            out.truncate(line_start);
        }
    }
    out.trim().to_string()
}

/// Try the live URL first; on failure or empty body, retry via Wayback.
/// Returns the HTML body that successfully fetched (may still be empty).
pub async fn fetch_with_wayback_fallback(
    client: &reqwest::Client,
    live_url: &str,
) -> Option<String> {
    let try_one = async |url: &str| -> Option<String> {
        let resp = client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (compatible; iwiywi/0.1)")
            .send()
            .await
            .ok()?;
        if !resp.status().is_success() {
            return None;
        }
        resp.text().await.ok()
    };
    if let Some(body) = try_one(live_url).await {
        if !body.trim().is_empty() {
            return Some(body);
        }
    }
    try_one(&wayback_url(live_url)).await
}

pub async fn scrape_all(client: &Client, config: &crate::config::Config) -> Vec<RawReading> {
    let sources: Vec<Source> = vec![
        (
            "aa_org",
            "https://www.aa.org/daily-reflections",
            parse_aa_org,
        ),
        (
            "hazeldon",
            "https://www.hazeldenbettyford.org/thought-for-the-day",
            parse_hazeldon,
        ),
        (
            "happy_hour",
            "https://www.aahappyhour.com/aa-daily-readings/",
            parse_happy_hour,
        ),
        ("silkworth", "https://silkworth.net", parse_silkworth),
        (
            "aa_online_meeting",
            "https://www.aaonlinemeeting.net",
            parse_aa_online_meeting,
        ),
        (
            "aa_big_book",
            "https://www.aabigbook.com",
            parse_aa_big_book,
        ),
    ];

    let mut results = Vec::new();
    for (key, url, parse_fn) in &sources {
        match fetch_with_wayback_fallback(client, url).await {
            Some(html) => {
                if let Some(reading) = parse_fn(&html) {
                    results.push(reading);
                } else {
                    eprintln!("warn: no reading found at {key} (live + wayback)");
                }
            }
            None => eprintln!("warn: fetch failed for {key} (live + wayback)"),
        }
    }

    // AI-extracted sources: dead-DNS sites whose Wayback snapshots can be
    // sent to the AI gateway for reading extraction. Empty for now —
    // recoveringcourage/joeancharlie were never archived; odat's only
    // snapshot is a 2016 redirect with no daily content. Add new entries
    // here when a usable archive shows up.
    let ai_sources: &[(&str, &str, &str, &str)] = &[];
    for (key, live_url, source_label, title_label) in ai_sources {
        let wayback = wayback_url(live_url);
        let resp = match client
            .get(&wayback)
            .header("User-Agent", "Mozilla/5.0 (compatible; iwiywi/0.1)")
            .send()
            .await
        {
            Ok(r) if r.status().is_success() => r.text().await.ok(),
            _ => None,
        };
        let html = match resp {
            Some(h) if !h.trim().is_empty() => h,
            _ => { eprintln!("warn: wayback empty for {key}"); continue; }
        };
        match crate::fetch::ai_extract::extract_reading(
            client, config, &html, source_label, title_label, live_url,
        ).await {
            Ok(r) => results.push(r),
            Err(e) => eprintln!("warn: AI extract failed for {key}: {e}"),
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
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();
    if text.is_empty() {
        return None;
    }
    Some(RawReading {
        source: "AA.org".to_string(),
        title: "Daily Reflections".to_string(),
        text: trim_boilerplate(&text),
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
        text: trim_boilerplate(&text),
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
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();
    if text.is_empty() {
        return None;
    }
    Some(RawReading {
        source: "AA Happy Hour".to_string(),
        title: "AA Daily Readings".to_string(),
        text: trim_boilerplate(&text),
        url: "https://www.aahappyhour.com/aa-daily-readings/".to_string(),
    })
}

// Shared helper: returns first non-empty paragraph text from a selector
fn first_nonempty_paragraph(document: &Html, sel: &Selector) -> Option<String> {
    document
        .select(sel)
        .map(|e| e.text().collect::<String>().trim().to_string())
        .find(|s| !s.is_empty())
}

pub fn parse_silkworth(html: &str) -> Option<RawReading> {
    // https://silkworth.net — verify selector on live site
    let document = Html::parse_document(html);
    let sel = Selector::parse(".content p").ok()?;
    let text = first_nonempty_paragraph(&document, &sel)?;
    Some(RawReading {
        source: "Silkworth.net".to_string(),
        title: "Daily Reading".to_string(),
        text: trim_boilerplate(&text),
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
        text: trim_boilerplate(&text),
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
        text: trim_boilerplate(&text),
        url: "https://www.aabigbook.com".to_string(),
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wayback_url_uses_latest_snapshot() {
        let url = wayback_url("https://www.aa.org/daily-reflections");
        assert!(url.starts_with("https://web.archive.org/web/"));
        assert!(url.ends_with("https://www.aa.org/daily-reflections"));
    }

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

    #[test]
    fn parse_silkworth_extracts_text() {
        let html = r#"<html><body><div class="content"><p>We are not cured of alcoholism.</p></div></body></html>"#;
        let result = parse_silkworth(html);
        assert!(result.is_some());
        assert!(result.unwrap().text.contains("alcoholism"));
    }

    #[test]
    fn parse_aa_online_meeting_extracts_text() {
        let html = r#"<html><body><div class="reading-text"><p>Step one text here.</p></div></body></html>"#;
        let result = parse_aa_online_meeting(html);
        assert!(result.is_some());
        assert!(result.unwrap().text.contains("Step"));
    }

    #[test]
    fn parse_aa_big_book_extracts_text() {
        let html = r#"<html><body><div class="post-content"><p>The Big Book text.</p></div></body></html>"#;
        let result = parse_aa_big_book(html);
        assert!(result.is_some());
        assert!(result.unwrap().text.contains("Big Book"));
    }

    #[test]
    fn trim_boilerplate_removes_trademark_footer() {
        let raw = "The actual reading text here.\nAll rights reserved. © 2026 AA World Services.";
        let cleaned = trim_boilerplate(raw);
        assert_eq!(cleaned, "The actual reading text here.");
    }

    #[test]
    fn trim_boilerplate_handles_no_markers() {
        let raw = "Just a normal reading with no boilerplate.";
        assert_eq!(trim_boilerplate(raw), raw);
    }

    #[test]
    fn trim_boilerplate_cuts_at_official_website_line() {
        let raw = "Reading body.\nThis is the official Website of the General Service Office.";
        assert_eq!(trim_boilerplate(raw), "Reading body.");
    }

}
