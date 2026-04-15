//! Grapevine source: tries to scrape today's "Quote of the Day" from
//! grapevine.org. If the live fetch or parse fails, falls back to a bundled
//! corpus of free-tier ToTD quotes so the source always has at least one item.

use crate::pulse::{PulseItem, PulseKind, PulseSource};
use scraper::{Html, Selector};

const FALLBACK_JSON: &str = include_str!("data/grapevine_fallback.json");
const LIVE_URL: &str = "https://www.aagrapevine.org/quote-day";

pub struct Grapevine { items: Vec<PulseItem> }

impl Grapevine {
    /// Construct from a pre-fetched HTML body. Caller is responsible for the
    /// HTTP fetch (so this is testable without networking). Pass `None` to
    /// use only the bundled fallback.
    pub fn from_html(html: Option<&str>) -> Self {
        let mut items = Vec::new();
        if let Some(body) = html {
            if let Some(quote) = parse_quote(body) {
                items.push(PulseItem {
                    kind: PulseKind::Grapevine,
                    step: None,
                    label: "Grapevine — Quote of the Day".to_string(),
                    body: quote,
                });
            }
        }
        // Always include the fallback so the source is never empty.
        let fallback: Vec<String> =
            serde_json::from_str(FALLBACK_JSON).expect("grapevine_fallback.json malformed");
        for s in fallback {
            items.push(PulseItem {
                kind: PulseKind::Grapevine,
                step: None,
                label: "Grapevine".to_string(),
                body: s,
            });
        }
        Self { items }
    }

    pub fn live_url() -> &'static str { LIVE_URL }
}

impl PulseSource for Grapevine {
    fn name(&self) -> &'static str { "grapevine" }
    fn items(&self) -> &[PulseItem] { &self.items }
}

fn parse_quote(html: &str) -> Option<String> {
    let doc = Html::parse_document(html);
    // Try a few likely selectors for the QotD page; drop bylines.
    let selectors = [".quote-of-the-day p", "blockquote p", "blockquote"];
    for sel in &selectors {
        if let Ok(s) = Selector::parse(sel) {
            for el in doc.select(&s) {
                let text: String = el.text().collect::<String>().trim().to_string();
                if text.len() >= 20 && text.len() <= 600 {
                    return Some(text);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grapevine_with_no_html_yields_fallback_only() {
        let g = Grapevine::from_html(None);
        assert_eq!(g.items().len(), 15);
        assert_eq!(g.name(), "grapevine");
    }

    #[test]
    fn grapevine_with_parsable_html_prepends_live_quote() {
        let html = r#"<html><body><div class="quote-of-the-day"><p>Live quote that is long enough.</p></div></body></html>"#;
        let g = Grapevine::from_html(Some(html));
        assert_eq!(g.items().len(), 16);
        assert_eq!(g.items()[0].body, "Live quote that is long enough.");
        assert_eq!(g.items()[0].label, "Grapevine — Quote of the Day");
    }

    #[test]
    fn grapevine_with_unparseable_html_still_yields_fallback() {
        let html = "<html><body>nothing useful</body></html>";
        let g = Grapevine::from_html(Some(html));
        assert_eq!(g.items().len(), 15);
    }

    #[test]
    fn grapevine_all_items_grapevine_kind() {
        let g = Grapevine::from_html(None);
        for item in g.items() {
            assert_eq!(item.kind, PulseKind::Grapevine);
            assert!(item.step.is_none());
        }
    }
}
