//! Pulse: a unified content stream that the drift screensaver cycles through.
//! Items come from multiple sources (today's readings, history, Big Book,
//! prayers, Steps + Principles) and the mixer interleaves them.

pub mod bundled;
pub mod grapevine;
pub mod historical;
pub mod today;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PulseKind {
    TodayReading,
    HistoricalReading,
    BigBookQuote,
    Prayer,
    StepText,
    Principle,
    Tradition,
    Concept,
    Slogan,
    Grapevine,
}

impl PulseKind {
    pub fn display_label(&self) -> &'static str {
        match self {
            PulseKind::TodayReading      => "Today",
            PulseKind::HistoricalReading => "From the archive",
            PulseKind::BigBookQuote      => "Big Book",
            PulseKind::Prayer            => "Prayer",
            PulseKind::StepText          => "Step",
            PulseKind::Principle         => "Principle",
            PulseKind::Tradition         => "Tradition",
            PulseKind::Concept           => "Concept",
            PulseKind::Slogan            => "Slogan",
            PulseKind::Grapevine         => "Grapevine",
        }
    }
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

use sha2::{Digest, Sha256};

pub trait PulseSource {
    fn name(&self) -> &str;
    fn items(&self) -> &[PulseItem];
}

pub struct PulseMixer {
    items: Vec<PulseItem>,
    cursor: usize,
}

impl PulseMixer {
    pub fn from_sources(sources: &[Box<dyn PulseSource>], filter_step: Option<u8>) -> Self {
        let mut items: Vec<PulseItem> = Vec::new();
        let mut seen: std::collections::HashSet<[u8; 32]> = std::collections::HashSet::new();
        for src in sources {
            for item in src.items() {
                if let Some(want) = filter_step {
                    if item.step != Some(want) {
                        continue;
                    }
                }
                let mut hasher = Sha256::new();
                hasher.update(src.name().as_bytes());
                hasher.update([0u8]);
                hasher.update(item.body.as_bytes());
                let digest: [u8; 32] = hasher.finalize().into();
                if seen.insert(digest) {
                    items.push(item.clone());
                }
            }
        }
        PulseMixer { items, cursor: 0 }
    }

    // `len`, `cursor`, and `all` are part of the public mixer surface — used
    // today only by tests, but kept available for future callers (debug UI,
    // logging, alternate cyclers).
    #[allow(dead_code)]
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
    #[allow(dead_code)]
    pub fn cursor(&self) -> usize { self.cursor }
    #[allow(dead_code)]
    pub fn all(&self) -> &[PulseItem] { &self.items }

    pub fn current(&self) -> Option<&PulseItem> {
        self.items.get(self.cursor)
    }

    pub fn advance(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.cursor = (self.cursor + 1) % self.items.len();
    }

    /// Jump to a deterministic random index from `seed`. Stays in-bounds.
    /// On a 1-item mixer it's a no-op.
    pub fn random_jump(&mut self, seed: u32) {
        if self.items.len() < 2 {
            return;
        }
        // Same xorshift-mult hash as drift::pseudo_rand for determinism.
        let mut x = seed.wrapping_mul(2_654_435_761).wrapping_add(self.cursor as u32);
        x ^= x >> 13;
        x = x.wrapping_mul(0x5bd1e995);
        x ^= x >> 15;
        let mut next = (x as usize) % self.items.len();
        if next == self.cursor {
            next = (next + 1) % self.items.len();
        }
        self.cursor = next;
    }
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

    struct StubSource {
        name: &'static str,
        items: Vec<PulseItem>,
    }
    impl PulseSource for StubSource {
        fn name(&self) -> &str { self.name }
        fn items(&self) -> &[PulseItem] { &self.items }
    }

    fn item(kind: PulseKind, step: Option<u8>, body: &str) -> PulseItem {
        PulseItem { kind, step, label: "label".to_string(), body: body.to_string() }
    }

    #[test]
    fn mixer_collects_items_from_all_sources() {
        let s1: Box<dyn PulseSource> = Box::new(StubSource {
            name: "s1",
            items: vec![item(PulseKind::TodayReading, Some(1), "a"), item(PulseKind::TodayReading, Some(2), "b")],
        });
        let s2: Box<dyn PulseSource> = Box::new(StubSource {
            name: "s2",
            items: vec![item(PulseKind::Prayer, None, "c")],
        });
        let mixer = PulseMixer::from_sources(&[s1, s2], None);
        assert_eq!(mixer.len(), 3);
    }

    #[test]
    fn mixer_advance_walks_cursor() {
        let s: Box<dyn PulseSource> = Box::new(StubSource {
            name: "s",
            items: vec![
                item(PulseKind::TodayReading, None, "x"),
                item(PulseKind::TodayReading, None, "y"),
                item(PulseKind::TodayReading, None, "z"),
            ],
        });
        let mut mixer = PulseMixer::from_sources(&[s], None);
        let first = mixer.current().unwrap().body.clone();
        mixer.advance();
        let second = mixer.current().unwrap().body.clone();
        mixer.advance();
        let third = mixer.current().unwrap().body.clone();
        mixer.advance();
        let wrapped = mixer.current().unwrap().body.clone();
        assert_eq!([first.as_str(), second.as_str(), third.as_str(), wrapped.as_str()],
                   ["x", "y", "z", "x"]);
    }

    #[test]
    fn mixer_filter_step_keeps_only_matching_items_and_unscoped_principles() {
        let s: Box<dyn PulseSource> = Box::new(StubSource {
            name: "s",
            items: vec![
                item(PulseKind::TodayReading, Some(1), "step1"),
                item(PulseKind::TodayReading, Some(3), "step3a"),
                item(PulseKind::TodayReading, Some(3), "step3b"),
                item(PulseKind::Prayer, None, "prayer"),
            ],
        });
        let mixer = PulseMixer::from_sources(&[s], Some(3));
        assert_eq!(mixer.len(), 2);
        for item in mixer.all() {
            assert_eq!(item.step, Some(3));
        }
    }

    #[test]
    fn mixer_dedupes_by_source_and_text_hash() {
        let s: Box<dyn PulseSource> = Box::new(StubSource {
            name: "dup",
            items: vec![
                item(PulseKind::HistoricalReading, None, "same body"),
                item(PulseKind::HistoricalReading, None, "same body"),
                item(PulseKind::HistoricalReading, None, "different body"),
            ],
        });
        let mixer = PulseMixer::from_sources(&[s], None);
        assert_eq!(mixer.len(), 2);
    }

    #[test]
    fn mixer_random_jump_changes_cursor_position() {
        let s: Box<dyn PulseSource> = Box::new(StubSource {
            name: "s",
            items: (0..50).map(|n| item(PulseKind::TodayReading, None, &n.to_string())).collect(),
        });
        let mut mixer = PulseMixer::from_sources(&[s], None);
        let start = mixer.cursor();
        mixer.random_jump(0xdead_beef);
        // With 50 items and a fixed seed, the new cursor should differ.
        assert_ne!(mixer.cursor(), start);
    }

    #[test]
    fn mixer_empty_returns_none() {
        let mixer = PulseMixer::from_sources(&[], None);
        assert_eq!(mixer.len(), 0);
        assert!(mixer.current().is_none());
    }

    #[test]
    fn pulse_kind_display_labels_are_user_friendly() {
        assert_eq!(PulseKind::TodayReading.display_label(), "Today");
        assert_eq!(PulseKind::HistoricalReading.display_label(), "From the archive");
        assert_eq!(PulseKind::BigBookQuote.display_label(), "Big Book");
        assert_eq!(PulseKind::Prayer.display_label(), "Prayer");
        assert_eq!(PulseKind::StepText.display_label(), "Step");
        assert_eq!(PulseKind::Principle.display_label(), "Principle");
    }
}
