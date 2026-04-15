use crate::models::ClassifiedReading;
use crate::pulse::{PulseItem, PulseKind, PulseSource};

pub struct TodayReadings {
    items: Vec<PulseItem>,
}

impl TodayReadings {
    pub fn from_readings(readings: &[ClassifiedReading]) -> Self {
        let items = readings
            .iter()
            .map(|r| PulseItem {
                kind: PulseKind::TodayReading,
                step: Some(r.step),
                label: format!("Step {} · {}", r.step, r.source),
                body: r.text.clone(),
            })
            .collect();
        Self { items }
    }
}

impl PulseSource for TodayReadings {
    fn name(&self) -> &'static str { "today" }
    fn items(&self) -> &[PulseItem] { &self.items }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(step: u8, src: &str, text: &str) -> ClassifiedReading {
        ClassifiedReading {
            step,
            reason: "test".to_string(),
            source: src.to_string(),
            title: "t".to_string(),
            text: text.to_string(),
            url: "https://example.com".to_string(),
        }
    }

    #[test]
    fn today_maps_each_reading_to_one_item() {
        let src = TodayReadings::from_readings(&[r(3, "AA.org", "made a decision")]);
        assert_eq!(src.items().len(), 1);
        assert_eq!(src.items()[0].kind, PulseKind::TodayReading);
        assert_eq!(src.items()[0].step, Some(3));
        assert_eq!(src.items()[0].label, "Step 3 · AA.org");
        assert_eq!(src.items()[0].body, "made a decision");
    }

    #[test]
    fn today_preserves_order() {
        let src = TodayReadings::from_readings(&[
            r(1, "A", "alpha"),
            r(7, "B", "bravo"),
        ]);
        assert_eq!(src.items()[0].body, "alpha");
        assert_eq!(src.items()[1].body, "bravo");
    }

    #[test]
    fn today_empty_yields_no_items() {
        let src = TodayReadings::from_readings(&[]);
        assert!(src.items().is_empty());
    }
}
