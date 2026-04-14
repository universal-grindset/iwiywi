use reqwest::Client;
use scraper::{Html, Selector};

use crate::models::RawReading;

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
    fn parse_recovering_courage_extracts_text() {
        let html = r#"<html><body><article><p>Courage in recovery.</p></article></body></html>"#;
        let result = parse_recovering_courage(html);
        assert!(result.is_some());
        assert!(result.unwrap().text.contains("Courage"));
    }

    #[test]
    fn parse_odat_extracts_text() {
        let html = r#"<html><body><div class="daily-reading"><p>One day at a time.</p></div></body></html>"#;
        let result = parse_odat(html);
        assert!(result.is_some());
        assert!(result.unwrap().text.contains("One day"));
    }

    #[test]
    fn parse_joe_and_charlie_extracts_text() {
        let html = r#"<html><body><div class="entry-content"><p>A program for you text.</p></div></body></html>"#;
        let result = parse_joe_and_charlie(html);
        assert!(result.is_some());
        assert!(result.unwrap().text.contains("program"));
    }

    #[test]
    fn parse_google_snippet_extracts_text() {
        // Google featured snippet fixture
        let html = r#"<html><body><div class="BNeawe"><span>God grant me the serenity.</span></div></body></html>"#;
        let result = parse_google_snippet(html);
        assert!(result.is_some());
        assert!(result.unwrap().text.contains("serenity"));
    }

    #[test]
    fn parse_reddit_extracts_text() {
        let html = r#"<html><body><div data-testid="post-content"><p>Today's reading discussion.</p></div></body></html>"#;
        let result = parse_reddit(html);
        assert!(result.is_some());
        assert!(result.unwrap().text.contains("reading"));
    }
}
