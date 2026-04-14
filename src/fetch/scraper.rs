use reqwest::Client;

pub struct Source {
    pub name: &'static str,
    pub url: &'static str,
}

const SOURCES: &[Source] = &[
    Source {
        name: "aa_org",
        url: "https://www.aa.org/the-twelve-steps/step-1",
    },
    Source {
        name: "hazeldon",
        url: "https://www.hazelden.org/",
    },
    Source {
        name: "happy_hour",
        url: "https://www.happyhoursobriety.com/",
    },
];

pub async fn scrape_all() -> Vec<(String, Option<String>)> {
    let client = Client::new();
    let mut results = Vec::new();

    for source in SOURCES {
        match client.get(source.url).send().await {
            Ok(response) => match response.text().await {
                Ok(html) => {
                    let content = match source.name {
                        "aa_org" => parse_aa_org(&html),
                        "hazeldon" => parse_hazeldon(&html),
                        "happy_hour" => parse_happy_hour(&html),
                        _ => None,
                    };
                    results.push((source.name.to_string(), content));
                }
                Err(e) => {
                    eprintln!("Failed to read response body from {}: {}", source.url, e);
                    results.push((source.name.to_string(), None));
                }
            },
            Err(e) => {
                eprintln!("Failed to fetch {}: {}", source.url, e);
                results.push((source.name.to_string(), None));
            }
        }
    }

    results
}

pub fn parse_aa_org(html: &str) -> Option<String> {
    // Look for content div or main content area
    if let Some(start) = html.find("<div") {
        if let Some(end) = html[start..].find("</div>") {
            let content = &html[start..start + end + 6];
            // Extract text by removing HTML tags
            let text = remove_html_tags(content);
            if !text.trim().is_empty() {
                return Some(text);
            }
        }
    }
    None
}

pub fn parse_hazeldon(html: &str) -> Option<String> {
    // Look for main content area
    if let Some(start) = html.find("<main") {
        if let Some(end) = html[start..].find("</main>") {
            let content = &html[start..start + end + 7];
            let text = remove_html_tags(content);
            if !text.trim().is_empty() {
                return Some(text);
            }
        }
    }
    None
}

pub fn parse_happy_hour(html: &str) -> Option<String> {
    // Look for article content
    if let Some(start) = html.find("<article") {
        if let Some(end) = html[start..].find("</article>") {
            let content = &html[start..start + end + 10];
            let text = remove_html_tags(content);
            if !text.trim().is_empty() {
                return Some(text);
            }
        }
    }
    None
}

fn remove_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }

    result
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_aa_org_extracts_text() {
        let html = r#"
            <html>
            <body>
            <div class="content">Step 1: We admitted we were powerless</div>
            </body>
            </html>
        "#;
        let result = parse_aa_org(html);
        assert!(result.is_some());
        let text = result.unwrap();
        assert!(text.contains("Step 1"));
        assert!(text.contains("powerless"));
    }

    #[test]
    fn parse_aa_org_returns_none_for_missing_content() {
        let html = "<html><body></body></html>";
        let result = parse_aa_org(html);
        assert!(result.is_none());
    }

    #[test]
    fn parse_hazeldon_extracts_text() {
        let html = r#"
            <html>
            <body>
            <main class="content">Recovery is possible with support</main>
            </body>
            </html>
        "#;
        let result = parse_hazeldon(html);
        assert!(result.is_some());
        let text = result.unwrap();
        assert!(text.contains("Recovery"));
        assert!(text.contains("possible"));
    }

    #[test]
    fn parse_happy_hour_extracts_text() {
        let html = r#"
            <html>
            <body>
            <article class="post">One day at a time, we find our way</article>
            </body>
            </html>
        "#;
        let result = parse_happy_hour(html);
        assert!(result.is_some());
        let text = result.unwrap();
        assert!(text.contains("One"));
        assert!(text.contains("way"));
    }
}
