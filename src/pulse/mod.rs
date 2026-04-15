//! Pulse: a unified content stream that the drift screensaver cycles through.
//! Items come from multiple sources (today's readings, history, Big Book,
//! prayers, Steps + Principles) and the mixer interleaves them.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PulseKind {
    TodayReading,
    HistoricalReading,
    BigBookQuote,
    Prayer,
    StepText,
    Principle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulseItem {
    pub kind: PulseKind,
    /// 1..=12 if the item is tied to a specific Step; None otherwise.
    pub step: Option<u8>,
    /// Short header line shown above the body (e.g. "Step 3 · AA.org" or
    /// "Big Book p. 62" or "The Serenity Prayer").
    pub label: String,
    /// One paragraph of plain text, no markdown.
    pub body: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pulse_item_round_trips_json() {
        let item = PulseItem {
            kind: PulseKind::Prayer,
            step: None,
            label: "The Serenity Prayer".to_string(),
            body: "God, grant me the serenity to accept the things I cannot change...".to_string(),
        };
        let json = serde_json::to_string(&item).unwrap();
        let back: PulseItem = serde_json::from_str(&json).unwrap();
        assert_eq!(back.kind, PulseKind::Prayer);
        assert_eq!(back.label, "The Serenity Prayer");
    }

    #[test]
    fn pulse_kind_step_tagged_variants() {
        let item = PulseItem {
            kind: PulseKind::StepText,
            step: Some(3),
            label: "Step 3".to_string(),
            body: "Made a decision...".to_string(),
        };
        assert_eq!(item.step, Some(3));
    }
}
