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
}
