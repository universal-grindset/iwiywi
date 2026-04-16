//! Pulse: a unified content stream that the drift screensaver cycles through.
//! Items come from multiple sources (today's readings, history, Big Book,
//! prayers, Steps + Principles) and the mixer interleaves them.

pub mod bill;
pub mod bundled;
pub mod community;
pub mod favorites;
pub mod grapevine;
pub mod historical;
pub mod summary;
pub mod today;

// Helper hooks used by the settings menu to cycle through value rings.
pub fn cycle<T: Copy + PartialEq>(values: &[T], current: T, delta: i32) -> T {
    let idx = values.iter().position(|v| *v == current).unwrap_or(0);
    let len = values.len() as i32;
    let next = ((idx as i32 + delta).rem_euclid(len)) as usize;
    values[next]
}

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
    Favorite,
    BillReflection,
    Community,
}

impl PulseKind {
    pub fn display_label(&self) -> &'static str {
        match self {
            PulseKind::TodayReading => "Today",
            PulseKind::HistoricalReading => "From the archive",
            PulseKind::BigBookQuote => "Big Book",
            PulseKind::Prayer => "Prayer",
            PulseKind::StepText => "Step",
            PulseKind::Principle => "Principle",
            PulseKind::Tradition => "Tradition",
            PulseKind::Concept => "Concept",
            PulseKind::Slogan => "Slogan",
            PulseKind::Grapevine => "Grapevine",
            PulseKind::Favorite => "Favorite",
            PulseKind::BillReflection => "Bill W. — AI reflection",
            PulseKind::Community => "From the rooms (paraphrased)",
        }
    }

    /// Per-kind frame accent color (Frame / Corners / Rule patterns).
    /// Gives a subtle visual cue to the source type while the palette
    /// still drives body/accent. Chosen to harmonize with the default
    /// dark palette; users with very bright palettes may want to stay
    /// on the Drift/Wave/etc. patterns to avoid visual clash.
    pub fn frame_tint(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match self {
            PulseKind::TodayReading => Color::Rgb(0x88, 0xA8, 0xD0), // cool blue
            PulseKind::HistoricalReading => Color::Rgb(0x78, 0x70, 0x80), // archival grey
            PulseKind::BigBookQuote => Color::Rgb(0xC4, 0x9A, 0x47), // warm gold
            PulseKind::Prayer => Color::Rgb(0x7E, 0x95, 0xBF),       // dusty blue
            PulseKind::StepText => Color::Rgb(0x9C, 0x7A, 0xCC),     // step purple
            PulseKind::Principle => Color::Rgb(0x8A, 0xA3, 0x73),    // sage
            PulseKind::Tradition => Color::Rgb(0x7A, 0x9E, 0xA3),    // teal
            PulseKind::Concept => Color::Rgb(0xA0, 0x7A, 0x5E),      // sandstone
            PulseKind::Slogan => Color::Rgb(0xD9, 0x7F, 0x5A),       // coral
            PulseKind::Grapevine => Color::Rgb(0x7C, 0xB2, 0x72),    // magazine green
            PulseKind::Favorite => Color::Rgb(0xE6, 0xA1, 0xA1),     // soft pink
            PulseKind::BillReflection => Color::Rgb(0x96, 0x73, 0xB3), // amethyst
            PulseKind::Community => Color::Rgb(0x6A, 0xAC, 0x9E),    // seafoam
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Random,
    Sequential,
    ByStep,
    BySource,
}

impl Order {
    pub const ALL: [Order; 4] = [
        Order::Random,
        Order::Sequential,
        Order::ByStep,
        Order::BySource,
    ];

    pub fn parse(raw: Option<&str>) -> Order {
        match raw {
            Some("sequential") => Order::Sequential,
            Some("by-step") => Order::ByStep,
            Some("by-source") => Order::BySource,
            _ => Order::Random,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Order::Random => "random",
            Order::Sequential => "sequential",
            Order::ByStep => "by-step",
            Order::BySource => "by-source",
        }
    }
}

pub fn order_from_env() -> Order {
    Order::parse(std::env::var("IWIYWI_ORDER").ok().as_deref())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    // Source-based: admit only one source.
    All,
    Today,
    History,
    BigBook,
    Prayers,
    Steps,
    Principles,
    Grapevine,
    Traditions,
    Concepts,
    Slogans,
    Favorites,
    Bill,
    Community,
    // Content-based: admit all sources but post-filter individual items.
    Short,       // body < 220 chars (quick reads)
    Long,        // body > 420 chars (deeper meditations)
    Surrender,   // step 1–3
    Action,      // step 4–9
    Maintenance, // step 10–12
}

impl Focus {
    pub const ALL_VARIANTS: [Focus; 19] = [
        Focus::All,
        Focus::Today,
        Focus::History,
        Focus::BigBook,
        Focus::Prayers,
        Focus::Steps,
        Focus::Principles,
        Focus::Grapevine,
        Focus::Traditions,
        Focus::Concepts,
        Focus::Slogans,
        Focus::Favorites,
        Focus::Bill,
        Focus::Community,
        Focus::Short,
        Focus::Long,
        Focus::Surrender,
        Focus::Action,
        Focus::Maintenance,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            Focus::All => "all",
            Focus::Today => "today",
            Focus::History => "history",
            Focus::BigBook => "big_book",
            Focus::Prayers => "prayers",
            Focus::Steps => "steps",
            Focus::Principles => "principles",
            Focus::Grapevine => "grapevine",
            Focus::Traditions => "traditions",
            Focus::Concepts => "concepts",
            Focus::Slogans => "slogans",
            Focus::Favorites => "favorites",
            Focus::Bill => "bill",
            Focus::Community => "community",
            Focus::Short => "short",
            Focus::Long => "long",
            Focus::Surrender => "surrender",
            Focus::Action => "action",
            Focus::Maintenance => "maintenance",
        }
    }

    pub fn parse(raw: Option<&str>) -> Focus {
        match raw {
            Some("today") => Focus::Today,
            Some("history") => Focus::History,
            Some("big_book") => Focus::BigBook,
            Some("prayers") => Focus::Prayers,
            Some("steps") => Focus::Steps,
            Some("principles") => Focus::Principles,
            Some("grapevine") => Focus::Grapevine,
            Some("traditions") => Focus::Traditions,
            Some("concepts") => Focus::Concepts,
            Some("slogans") => Focus::Slogans,
            Some("favorites") => Focus::Favorites,
            Some("bill") => Focus::Bill,
            Some("community") => Focus::Community,
            Some("short") => Focus::Short,
            Some("long") => Focus::Long,
            Some("surrender") => Focus::Surrender,
            Some("action") => Focus::Action,
            Some("maintenance") => Focus::Maintenance,
            _ => Focus::All,
        }
    }

    /// True if the given source name passes this focus filter. For
    /// content-based focuses, all sources pass — filtering happens at the
    /// item level via `admits_item`.
    pub fn admits(&self, source_name: &str) -> bool {
        match self {
            Focus::All => true,
            Focus::Today => source_name == "today",
            Focus::History => source_name == "historical",
            Focus::BigBook => source_name == "big_book",
            Focus::Prayers => source_name == "prayers",
            Focus::Steps | Focus::Principles => source_name == "step_explainers",
            Focus::Grapevine => source_name == "grapevine",
            Focus::Traditions => source_name == "traditions",
            Focus::Concepts => source_name == "concepts",
            Focus::Slogans => source_name == "slogans",
            Focus::Favorites => source_name == "favorites",
            Focus::Bill => source_name == "bill",
            Focus::Community => source_name == "community",
            // Content-based focuses let every source through; the item
            // filter is the one that narrows the pool.
            Focus::Short | Focus::Long | Focus::Surrender | Focus::Action | Focus::Maintenance => {
                true
            }
        }
    }

    /// True if the given item should be kept. Only content-based focuses
    /// narrow further here — source-based focuses already filtered above.
    pub fn admits_item(&self, item: &PulseItem) -> bool {
        match self {
            Focus::Short => item.body.chars().count() < 220,
            Focus::Long => item.body.chars().count() > 420,
            Focus::Surrender => item.step.is_some_and(|s| (1..=3).contains(&s)),
            Focus::Action => item.step.is_some_and(|s| (4..=9).contains(&s)),
            Focus::Maintenance => item.step.is_some_and(|s| (10..=12).contains(&s)),
            _ => true,
        }
    }
}

pub fn focus_from_env() -> Focus {
    Focus::parse(std::env::var("IWIYWI_FOCUS").ok().as_deref())
}

pub trait PulseSource {
    fn name(&self) -> &'static str;
    fn items(&self) -> &[PulseItem];
}

pub struct PulseMixer {
    items: Vec<PulseItem>,
    cursor: usize,
}

impl PulseMixer {
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn from_sources(
        sources: &[Box<dyn PulseSource>],
        filter_step: Option<u8>,
        order: Order,
    ) -> Self {
        Self::from_sources_focused(sources, filter_step, order, Focus::All)
    }

    pub fn from_sources_focused(
        sources: &[Box<dyn PulseSource>],
        filter_step: Option<u8>,
        order: Order,
        focus: Focus,
    ) -> Self {
        let mut items: Vec<PulseItem> = Vec::new();
        let mut seen: std::collections::HashSet<[u8; 32]> = std::collections::HashSet::new();
        for src in sources.iter().filter(|s| focus.admits(s.name())) {
            for item in src.items() {
                if !focus.admits_item(item) {
                    continue;
                }
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
        match order {
            // Hardcoded: walk the 12 canonical Steps verbatim, 1 → 12. If the
            // user wanted other step-tagged content they'd pick a source via
            // Focus or stay on Sequential/Random. Principles and everything
            // else are dropped here.
            Order::ByStep => {
                items.retain(|i| i.kind == PulseKind::StepText);
                items.sort_by_key(|i| i.step.unwrap_or(255));
            }
            Order::BySource | Order::Random | Order::Sequential => { /* preserved as appended */ }
        }
        PulseMixer { items, cursor: 0 }
    }

    // `len`, `cursor`, and `all` are part of the public mixer surface — used
    // today only by tests, but kept available for future callers (debug UI,
    // logging, alternate cyclers).
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    pub fn cursor(&self) -> usize {
        self.cursor
    }
    #[allow(dead_code)]
    pub fn all(&self) -> &[PulseItem] {
        &self.items
    }

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
    /// Set the cursor to a specific index. Bounds-checked: out-of-range
    /// indices become a no-op. Used by `/`-search to jump to a match.
    pub fn jump_to(&mut self, index: usize) {
        if index < self.items.len() {
            self.cursor = index;
        }
    }

    pub fn random_jump(&mut self, seed: u32) {
        if self.items.len() < 2 {
            return;
        }
        // Same xorshift-mult hash as drift::pseudo_rand for determinism.
        let mut x = seed
            .wrapping_mul(2_654_435_761)
            .wrapping_add(self.cursor as u32);
        x ^= x >> 13;
        x = x.wrapping_mul(0x5bd1e995);
        x ^= x >> 15;
        let mut next = (x as usize) % self.items.len();
        if next == self.cursor {
            next = (next + 1) % self.items.len();
        }
        self.cursor = next;
    }

    /// Advance the cursor according to the given order. For `Random`, jumps
    /// to a deterministic random position from `seed`. For sequential variants,
    /// walks the items in the order produced by `from_sources`.
    pub fn advance_per_order(&mut self, order: Order, seed: u32) {
        match order {
            Order::Random => self.random_jump(seed),
            Order::Sequential | Order::ByStep | Order::BySource => self.advance(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_focus_keeps_only_brief_items() {
        let short = PulseItem {
            kind: PulseKind::Prayer,
            step: None,
            label: "L".to_string(),
            body: "Be still.".to_string(),
        };
        let long = PulseItem {
            kind: PulseKind::Prayer,
            step: None,
            label: "L".to_string(),
            body: "x".repeat(500),
        };
        assert!(Focus::Short.admits_item(&short));
        assert!(!Focus::Short.admits_item(&long));
    }

    #[test]
    fn long_focus_keeps_only_lengthy_items() {
        let short = PulseItem {
            kind: PulseKind::Prayer,
            step: None,
            label: "L".to_string(),
            body: "Be still.".to_string(),
        };
        let long = PulseItem {
            kind: PulseKind::Prayer,
            step: None,
            label: "L".to_string(),
            body: "x".repeat(500),
        };
        assert!(!Focus::Long.admits_item(&short));
        assert!(Focus::Long.admits_item(&long));
    }

    #[test]
    fn step_group_focuses_partition_steps_1_through_12() {
        for n in 1..=12u8 {
            let item = PulseItem {
                kind: PulseKind::StepText,
                step: Some(n),
                label: "L".to_string(),
                body: "b".to_string(),
            };
            let surrender = Focus::Surrender.admits_item(&item);
            let action = Focus::Action.admits_item(&item);
            let maintain = Focus::Maintenance.admits_item(&item);
            // Exactly one group must accept each step.
            let count = [surrender, action, maintain]
                .into_iter()
                .filter(|b| *b)
                .count();
            assert_eq!(count, 1, "step {n} matched {count} groups, want 1");
        }
    }

    #[test]
    fn content_focuses_reject_items_without_step_for_step_groups() {
        let nostep = PulseItem {
            kind: PulseKind::Prayer,
            step: None,
            label: "L".to_string(),
            body: "b".to_string(),
        };
        assert!(!Focus::Surrender.admits_item(&nostep));
        assert!(!Focus::Action.admits_item(&nostep));
        assert!(!Focus::Maintenance.admits_item(&nostep));
    }

    #[test]
    fn focus_all_variants_length_matches_enum() {
        assert_eq!(Focus::ALL_VARIANTS.len(), 19);
    }

    #[test]
    fn every_focus_label_round_trips() {
        for f in Focus::ALL_VARIANTS {
            assert_eq!(Focus::parse(Some(f.label())), f);
        }
    }

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
        fn name(&self) -> &'static str {
            self.name
        }
        fn items(&self) -> &[PulseItem] {
            &self.items
        }
    }

    fn item(kind: PulseKind, step: Option<u8>, body: &str) -> PulseItem {
        PulseItem {
            kind,
            step,
            label: "label".to_string(),
            body: body.to_string(),
        }
    }

    #[test]
    fn mixer_collects_items_from_all_sources() {
        let s1: Box<dyn PulseSource> = Box::new(StubSource {
            name: "s1",
            items: vec![
                item(PulseKind::TodayReading, Some(1), "a"),
                item(PulseKind::TodayReading, Some(2), "b"),
            ],
        });
        let s2: Box<dyn PulseSource> = Box::new(StubSource {
            name: "s2",
            items: vec![item(PulseKind::Prayer, None, "c")],
        });
        let mixer = PulseMixer::from_sources(&[s1, s2], None, Order::Random);
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
        let mut mixer = PulseMixer::from_sources(&[s], None, Order::Random);
        let first = mixer.current().unwrap().body.clone();
        mixer.advance();
        let second = mixer.current().unwrap().body.clone();
        mixer.advance();
        let third = mixer.current().unwrap().body.clone();
        mixer.advance();
        let wrapped = mixer.current().unwrap().body.clone();
        assert_eq!(
            [
                first.as_str(),
                second.as_str(),
                third.as_str(),
                wrapped.as_str()
            ],
            ["x", "y", "z", "x"]
        );
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
        let mixer = PulseMixer::from_sources(&[s], Some(3), Order::Random);
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
        let mixer = PulseMixer::from_sources(&[s], None, Order::Random);
        assert_eq!(mixer.len(), 2);
    }

    #[test]
    fn mixer_random_jump_changes_cursor_position() {
        let s: Box<dyn PulseSource> = Box::new(StubSource {
            name: "s",
            items: (0..50)
                .map(|n| item(PulseKind::TodayReading, None, &n.to_string()))
                .collect(),
        });
        let mut mixer = PulseMixer::from_sources(&[s], None, Order::Random);
        let start = mixer.cursor();
        mixer.random_jump(0xdead_beef);
        // With 50 items and a fixed seed, the new cursor should differ.
        assert_ne!(mixer.cursor(), start);
    }

    #[test]
    fn mixer_empty_returns_none() {
        let mixer = PulseMixer::from_sources(&[], None, Order::Random);
        assert_eq!(mixer.len(), 0);
        assert!(mixer.current().is_none());
    }

    #[test]
    fn pulse_kind_display_labels_are_user_friendly() {
        assert_eq!(PulseKind::TodayReading.display_label(), "Today");
        assert_eq!(
            PulseKind::HistoricalReading.display_label(),
            "From the archive"
        );
        assert_eq!(PulseKind::BigBookQuote.display_label(), "Big Book");
        assert_eq!(PulseKind::Prayer.display_label(), "Prayer");
        assert_eq!(PulseKind::StepText.display_label(), "Step");
        assert_eq!(PulseKind::Principle.display_label(), "Principle");
    }

    #[test]
    fn order_parse_defaults_to_random() {
        assert_eq!(Order::parse(None), Order::Random);
        assert_eq!(Order::parse(Some("garbage")), Order::Random);
    }

    #[test]
    fn order_parse_recognizes_all_variants() {
        assert_eq!(Order::parse(Some("sequential")), Order::Sequential);
        assert_eq!(Order::parse(Some("by-step")), Order::ByStep);
        assert_eq!(Order::parse(Some("by-source")), Order::BySource);
    }

    #[test]
    fn mixer_by_step_keeps_only_step_text_sorted() {
        // ByStep hardcodes the 12 canonical Step texts in order — anything
        // else (Today readings, Principles, Big Book quotes with step tags)
        // gets filtered out.
        let s: Box<dyn PulseSource> = Box::new(StubSource {
            name: "s",
            items: vec![
                item(PulseKind::TodayReading, Some(7), "today-7"),
                item(PulseKind::StepText, Some(3), "step-3-text"),
                item(PulseKind::StepText, Some(1), "step-1-text"),
                item(PulseKind::Principle, Some(1), "principle-1"),
            ],
        });
        let mixer = PulseMixer::from_sources(&[s], None, Order::ByStep);
        let bodies: Vec<&str> = mixer.all().iter().map(|i| i.body.as_str()).collect();
        assert_eq!(bodies, ["step-1-text", "step-3-text"]);
    }

    #[test]
    fn advance_per_order_random_uses_random_jump() {
        let s: Box<dyn PulseSource> = Box::new(StubSource {
            name: "s",
            items: (0..50)
                .map(|n| item(PulseKind::TodayReading, None, &n.to_string()))
                .collect(),
        });
        let mut mixer = PulseMixer::from_sources(&[s], None, Order::Random);
        let start = mixer.cursor();
        mixer.advance_per_order(Order::Random, 0xdead_beef);
        assert_ne!(mixer.cursor(), start);
    }

    #[test]
    fn focus_parse_defaults_to_all() {
        assert_eq!(Focus::parse(None), Focus::All);
        assert_eq!(Focus::parse(Some("garbage")), Focus::All);
    }

    #[test]
    fn focus_parse_recognizes_all_kinds() {
        assert_eq!(Focus::parse(Some("today")), Focus::Today);
        assert_eq!(Focus::parse(Some("big_book")), Focus::BigBook);
        assert_eq!(Focus::parse(Some("traditions")), Focus::Traditions);
        assert_eq!(Focus::parse(Some("concepts")), Focus::Concepts);
        assert_eq!(Focus::parse(Some("slogans")), Focus::Slogans);
        assert_eq!(Focus::parse(Some("grapevine")), Focus::Grapevine);
    }

    #[test]
    fn focus_admits_matches_source_name() {
        assert!(Focus::All.admits("anything"));
        assert!(Focus::Today.admits("today"));
        assert!(!Focus::Today.admits("historical"));
        assert!(Focus::Steps.admits("step_explainers"));
        assert!(Focus::Principles.admits("step_explainers"));
        assert!(!Focus::BigBook.admits("prayers"));
    }
}
