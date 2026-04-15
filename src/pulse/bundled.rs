use crate::pulse::{PulseItem, PulseKind, PulseSource};

const PRAYERS_JSON: &str = include_str!("data/prayers.json");

#[derive(serde::Deserialize)]
struct PrayerEntry {
    label: String,
    body: String,
}

pub struct Prayers {
    items: Vec<PulseItem>,
}

impl Prayers {
    pub fn load() -> Self {
        let entries: Vec<PrayerEntry> =
            serde_json::from_str(PRAYERS_JSON).expect("prayers.json malformed");
        let items = entries
            .into_iter()
            .map(|e| PulseItem {
                kind: PulseKind::Prayer,
                step: None,
                label: e.label,
                body: e.body,
            })
            .collect();
        Prayers { items }
    }
}

impl PulseSource for Prayers {
    fn name(&self) -> &str { "prayers" }
    fn items(&self) -> &[PulseItem] { &self.items }
}

const STEPS_JSON: &str = include_str!("data/step_explainers.json");

#[derive(serde::Deserialize)]
struct StepEntry {
    step: u8,
    step_text: String,
    principle: String,
    principle_body: String,
}

pub struct StepExplainers {
    items: Vec<PulseItem>,
}

impl StepExplainers {
    pub fn load() -> Self {
        let entries: Vec<StepEntry> =
            serde_json::from_str(STEPS_JSON).expect("step_explainers.json malformed");
        let mut items = Vec::with_capacity(entries.len() * 2);
        for e in entries {
            items.push(PulseItem {
                kind: PulseKind::StepText,
                step: Some(e.step),
                label: format!("Step {}", e.step),
                body: e.step_text,
            });
            items.push(PulseItem {
                kind: PulseKind::Principle,
                step: Some(e.step),
                label: format!("Principle of Step {} · {}", e.step, e.principle),
                body: e.principle_body,
            });
        }
        StepExplainers { items }
    }
}

impl PulseSource for StepExplainers {
    fn name(&self) -> &str { "step_explainers" }
    fn items(&self) -> &[PulseItem] { &self.items }
}

const BIG_BOOK_JSON: &str = include_str!("data/big_book.json");

#[derive(serde::Deserialize)]
struct BigBookEntry {
    page: u16,
    step: Option<u8>,
    body: String,
}

pub struct BigBookQuotes {
    items: Vec<PulseItem>,
}

impl BigBookQuotes {
    pub fn load() -> Self {
        let entries: Vec<BigBookEntry> =
            serde_json::from_str(BIG_BOOK_JSON).expect("big_book.json malformed");
        let items = entries
            .into_iter()
            .map(|e| PulseItem {
                kind: PulseKind::BigBookQuote,
                step: e.step,
                label: format!("Big Book p. {}", e.page),
                body: e.body,
            })
            .collect();
        BigBookQuotes { items }
    }
}

impl PulseSource for BigBookQuotes {
    fn name(&self) -> &str { "big_book" }
    fn items(&self) -> &[PulseItem] { &self.items }
}

const TRADITIONS_JSON: &str = include_str!("data/traditions.json");

#[derive(serde::Deserialize)]
struct TraditionEntry {
    n: u8,
    body: String,
}

pub struct Traditions {
    items: Vec<PulseItem>,
}

impl Traditions {
    pub fn load() -> Self {
        let entries: Vec<TraditionEntry> =
            serde_json::from_str(TRADITIONS_JSON).expect("traditions.json malformed");
        let items = entries
            .into_iter()
            .map(|e| PulseItem {
                kind: PulseKind::Tradition,
                step: None,
                label: format!("Tradition {}", e.n),
                body: e.body,
            })
            .collect();
        Traditions { items }
    }
}

impl PulseSource for Traditions {
    fn name(&self) -> &str { "traditions" }
    fn items(&self) -> &[PulseItem] { &self.items }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prayers_load_yields_seven_items() {
        let p = Prayers::load();
        assert_eq!(p.items().len(), 7);
        assert_eq!(p.name(), "prayers");
    }

    #[test]
    fn prayers_all_tagged_as_prayer_kind_and_no_step() {
        let p = Prayers::load();
        for item in p.items() {
            assert_eq!(item.kind, PulseKind::Prayer);
            assert!(item.step.is_none());
        }
    }

    #[test]
    fn serenity_prayer_is_first() {
        let p = Prayers::load();
        assert_eq!(p.items()[0].label, "The Serenity Prayer");
    }

    #[test]
    fn step_explainers_load_yields_24_items() {
        // 12 Step texts + 12 Principles = 24 items.
        let s = StepExplainers::load();
        assert_eq!(s.items().len(), 24);
    }

    #[test]
    fn step_explainers_each_step_has_text_and_principle() {
        let s = StepExplainers::load();
        for n in 1u8..=12 {
            let for_step: Vec<_> = s.items().iter().filter(|i| i.step == Some(n)).collect();
            assert_eq!(for_step.len(), 2, "step {n} should yield exactly 2 items");
            assert!(for_step.iter().any(|i| i.kind == PulseKind::StepText));
            assert!(for_step.iter().any(|i| i.kind == PulseKind::Principle));
        }
    }

    #[test]
    fn step_one_principle_is_honesty() {
        let s = StepExplainers::load();
        let principle = s.items().iter().find(|i| i.step == Some(1) && i.kind == PulseKind::Principle).unwrap();
        assert!(principle.label.contains("Honesty"));
    }

    #[test]
    fn big_book_loads_fifteen_quotes() {
        let bb = BigBookQuotes::load();
        assert_eq!(bb.items().len(), 15);
        assert_eq!(bb.name(), "big_book");
    }

    #[test]
    fn big_book_step_tags_preserved() {
        let bb = BigBookQuotes::load();
        let step3_count = bb.items().iter().filter(|i| i.step == Some(3)).count();
        assert!(step3_count >= 3, "expected at least 3 Step-3 quotes");
    }

    #[test]
    fn big_book_label_includes_page_number() {
        let bb = BigBookQuotes::load();
        assert!(bb.items()[0].label.contains("p."));
    }

    #[test]
    fn traditions_load_yields_twelve() {
        let t = Traditions::load();
        assert_eq!(t.items().len(), 12);
        assert_eq!(t.name(), "traditions");
    }

    #[test]
    fn traditions_all_tradition_kind_no_step() {
        let t = Traditions::load();
        for item in t.items() {
            assert_eq!(item.kind, PulseKind::Tradition);
            assert!(item.step.is_none());
            assert!(item.label.starts_with("Tradition "));
        }
    }

    #[test]
    fn tradition_one_starts_with_common_welfare() {
        let t = Traditions::load();
        assert!(t.items()[0].body.starts_with("Our common welfare"));
    }

    #[test]
    fn pulse_kind_new_variants_have_display_labels() {
        assert_eq!(PulseKind::Tradition.display_label(), "Tradition");
        assert_eq!(PulseKind::Concept.display_label(), "Concept");
        assert_eq!(PulseKind::Slogan.display_label(), "Slogan");
        assert_eq!(PulseKind::Grapevine.display_label(), "Grapevine");
    }
}
