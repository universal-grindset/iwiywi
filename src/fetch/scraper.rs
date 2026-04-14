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
