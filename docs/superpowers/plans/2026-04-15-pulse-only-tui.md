# Pulse-Only TUI v0.5 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Strip iwiywi's TUI down to a single full-screen pulse renderer — no tabs, no command bar, no QR overlay, no drift particles, no fade. Expand the bundled corpora (Traditions, Concepts, Slogans, Grapevine fallback) and add live Grapevine scraping. Ship six new env-var knobs (`IWIYWI_PULSE_SECS`, `IWIYWI_PALETTE`, `IWIYWI_PATTERN`, `IWIYWI_ORDER`, `IWIYWI_FOCUS` plus the existing `IWIYWI_THEME`).

**Architecture:** Add new sources and config plumbing first (additive, doesn't break the build). Then strip the TUI (one mega-commit because deletions and rewrites are interlocked). Then drop the now-orphaned modules (markdown.rs, gist.rs, drift.rs, qr.rs, commands.rs, theme.rs) and crate deps. Finally tighten scrapers + docs.

**Tech Stack:** Rust 2021, ratatui 0.28, crossterm 0.28, `serde_json`, `sha2` (existing). Removes: `qrcode`, `noise`.

**Spec:** `docs/superpowers/specs/2026-04-15-pulse-only-tui-design.md`

---

## File Structure

| File | Role | Status |
|---|---|---|
| `src/pulse/data/traditions.json` | 12 Traditions verbatim | New |
| `src/pulse/data/concepts.json` | 12 Concepts (short form) verbatim | New |
| `src/pulse/data/slogans.json` | 30 standard AA slogans | New |
| `src/pulse/data/grapevine_fallback.json` | ~15 Grapevine ToTD quotes for offline fallback | New |
| `src/pulse/bundled.rs` | Add `Traditions`, `Concepts`, `Slogans`, `GrapevineFallback` | Modify |
| `src/pulse/grapevine.rs` | Live-scrape grapevine.org Quote of the Day with offline fallback | New |
| `src/pulse/mod.rs` | Add `Order` enum + `advance_per_order` + `Focus` enum + parsers | Modify |
| `src/tui/palette.rs` | `Palette` enum (8 variants × light/dark) + parser | New |
| `src/tui/pattern.rs` | `Pattern` enum (4 variants) + parser + draw helpers | New |
| `src/tui/mod.rs` | Strip to pulse-only event loop | Major rewrite |
| `src/tui/widgets.rs` | Single `render_pulse` function | Major rewrite |
| `src/tui/drift.rs` | Delete | Removed |
| `src/tui/qr.rs` | Delete | Removed |
| `src/tui/commands.rs` | Delete | Removed |
| `src/tui/theme.rs` | Delete (replaced by `palette.rs`) | Removed |
| `src/fetch/markdown.rs` | Delete | Removed |
| `src/fetch/gist.rs` | Delete | Removed |
| `src/fetch/mod.rs` | Drop markdown render + gist publish | Modify |
| `src/fetch/scraper.rs` | Add boilerplate trimmer + tighten selectors | Modify |
| `src/config.rs` | Drop `MobileConfig`, `gist_id`, `qr_url`, `idle_secs`. Add `pulse_secs`, `palette`, `pattern`, `order`, `focus` | Modify |
| `Cargo.toml` | Drop `qrcode`, `noise` | Modify |
| `README.md` | Rewrite Usage / Choices / Troubleshooting / What pulses | Modify |
| `docs/CHANGELOG.md` | `[0.5.0]` entry | Modify |

---

## Phase A — Add new content (additive, build stays green)

### Task 1: Bundled Traditions corpus

**Files:**
- Create: `src/pulse/data/traditions.json`
- Modify: `src/pulse/bundled.rs`

- [ ] **Step 1.1: Author `src/pulse/data/traditions.json`**

```json
[
  { "n": 1,  "body": "Our common welfare should come first; personal recovery depends upon A.A. unity." },
  { "n": 2,  "body": "For our group purpose there is but one ultimate authority — a loving God as He may express Himself in our group conscience. Our leaders are but trusted servants; they do not govern." },
  { "n": 3,  "body": "The only requirement for A.A. membership is a desire to stop drinking." },
  { "n": 4,  "body": "Each group should be autonomous except in matters affecting other groups or A.A. as a whole." },
  { "n": 5,  "body": "Each group has but one primary purpose — to carry its message to the alcoholic who still suffers." },
  { "n": 6,  "body": "An A.A. group ought never endorse, finance, or lend the A.A. name to any related facility or outside enterprise, lest problems of money, property, and prestige divert us from our primary purpose." },
  { "n": 7,  "body": "Every A.A. group ought to be fully self-supporting, declining outside contributions." },
  { "n": 8,  "body": "Alcoholics Anonymous should remain forever nonprofessional, but our service centers may employ special workers." },
  { "n": 9,  "body": "A.A., as such, ought never be organized; but we may create service boards or committees directly responsible to those they serve." },
  { "n": 10, "body": "Alcoholics Anonymous has no opinion on outside issues; hence the A.A. name ought never be drawn into public controversy." },
  { "n": 11, "body": "Our public relations policy is based on attraction rather than promotion; we need always maintain personal anonymity at the level of press, radio, and films." },
  { "n": 12, "body": "Anonymity is the spiritual foundation of all our Traditions, ever reminding us to place principles before personalities." }
]
```

- [ ] **Step 1.2: Append to `src/pulse/bundled.rs`**

After the existing `BigBookQuotes` block, before the `#[cfg(test)]` block, add:

```rust
const TRADITIONS_JSON: &str = include_str!("data/traditions.json");

#[derive(serde::Deserialize)]
struct TraditionEntry { n: u8, body: String }

pub struct Traditions { items: Vec<PulseItem> }

impl Traditions {
    pub fn load() -> Self {
        let entries: Vec<TraditionEntry> =
            serde_json::from_str(TRADITIONS_JSON).expect("traditions.json malformed");
        let items = entries.into_iter().map(|e| PulseItem {
            kind: PulseKind::Tradition,
            step: None,
            label: format!("Tradition {}", e.n),
            body: e.body,
        }).collect();
        Traditions { items }
    }
}

impl PulseSource for Traditions {
    fn name(&self) -> &str { "traditions" }
    fn items(&self) -> &[PulseItem] { &self.items }
}
```

- [ ] **Step 1.3: Add `Tradition` variant to `PulseKind`**

In `src/pulse/mod.rs`, extend `PulseKind`:

```rust
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
```

And extend `PulseKind::display_label`:

```rust
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
```

- [ ] **Step 1.4: Add tests in `src/pulse/bundled.rs`**

```rust
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
```

- [ ] **Step 1.5: Verify**

```
cargo test pulse:: 2>&1 | tail -5
```

Expected: 4 new tests pass; total grows by 4.

- [ ] **Step 1.6: Commit**

```
git add src/pulse/data/traditions.json src/pulse/bundled.rs src/pulse/mod.rs
git commit -m "feat(pulse): bundle 12 Traditions verbatim + 4 new PulseKind variants"
```

---

### Task 2: Bundled Concepts corpus

**Files:**
- Create: `src/pulse/data/concepts.json`
- Modify: `src/pulse/bundled.rs`

- [ ] **Step 2.1: Author `src/pulse/data/concepts.json`**

```json
[
  { "n": 1,  "body": "Final responsibility and ultimate authority for A.A. world services should always reside in the collective conscience of our whole Fellowship." },
  { "n": 2,  "body": "The General Service Conference of A.A. has become, for nearly every practical purpose, the active voice and the effective conscience of our whole Society in its world affairs." },
  { "n": 3,  "body": "To insure effective leadership, we should endow each element of A.A. — the Conference, the General Service Board and its service corporations, staffs, committees, and executives — with a traditional 'Right of Decision.'" },
  { "n": 4,  "body": "At all responsible levels, we ought to maintain a traditional 'Right of Participation,' allowing a voting representation in reasonable proportion to the responsibility that each must discharge." },
  { "n": 5,  "body": "Throughout our world service structure, a traditional 'Right of Appeal' ought to prevail, so that minority opinion will be heard and personal grievances receive careful consideration." },
  { "n": 6,  "body": "On behalf of A.A. as a whole, our General Service Conference has the principal responsibility for the maintenance of our world services, and it traditionally has the final decision respecting large matters of general policy and finance." },
  { "n": 7,  "body": "The Conference recognizes that the Charter and the Bylaws of the General Service Board are legal instruments: that the Trustees are thereby fully empowered to manage and conduct all of the world service affairs of Alcoholics Anonymous." },
  { "n": 8,  "body": "The Trustees are the principal planners and administrators of overall policy and finance. They have custodial oversight of the separately incorporated and constantly active services, exercising this through their ability to elect all the directors of these entities." },
  { "n": 9,  "body": "Good service leadership at all levels is indispensable for our future functioning and safety. Primary world service leadership, once exercised by the founders, must necessarily be assumed by the Trustees." },
  { "n": 10, "body": "Every service responsibility should be matched by an equal service authority, with the scope of such authority well defined." },
  { "n": 11, "body": "The Trustees should always have the best possible committees, corporate service directors, executives, staffs, and consultants. Composition, qualifications, induction procedures, and rights and duties will always be matters of serious concern." },
  { "n": 12, "body": "The Conference shall observe the spirit of A.A. tradition, taking care that it never becomes the seat of perilous wealth or power; that sufficient operating funds and reserve be its prudent financial principle; that it place none of its members in a position of unqualified authority over others; that it reach all important decisions by discussion, vote, and whenever possible by substantial unanimity; that its actions never be personally punitive nor an incitement to public controversy; that it never perform acts of government, and that, like the Society it serves, it will always remain democratic in thought and action." }
]
```

- [ ] **Step 2.2: Append to `src/pulse/bundled.rs`**

After `Traditions`, before the test module:

```rust
const CONCEPTS_JSON: &str = include_str!("data/concepts.json");

#[derive(serde::Deserialize)]
struct ConceptEntry { n: u8, body: String }

pub struct Concepts { items: Vec<PulseItem> }

impl Concepts {
    pub fn load() -> Self {
        let entries: Vec<ConceptEntry> =
            serde_json::from_str(CONCEPTS_JSON).expect("concepts.json malformed");
        let items = entries.into_iter().map(|e| PulseItem {
            kind: PulseKind::Concept,
            step: None,
            label: format!("Concept {} for World Service", e.n),
            body: e.body,
        }).collect();
        Concepts { items }
    }
}

impl PulseSource for Concepts {
    fn name(&self) -> &str { "concepts" }
    fn items(&self) -> &[PulseItem] { &self.items }
}
```

- [ ] **Step 2.3: Tests**

```rust
    #[test]
    fn concepts_load_yields_twelve() {
        let c = Concepts::load();
        assert_eq!(c.items().len(), 12);
        assert_eq!(c.name(), "concepts");
    }

    #[test]
    fn concepts_all_concept_kind_no_step() {
        let c = Concepts::load();
        for item in c.items() {
            assert_eq!(item.kind, PulseKind::Concept);
            assert!(item.step.is_none());
            assert!(item.label.starts_with("Concept "));
        }
    }
```

- [ ] **Step 2.4: Verify + commit**

```
cargo test pulse:: 2>&1 | tail -5
git add src/pulse/data/concepts.json src/pulse/bundled.rs
git commit -m "feat(pulse): bundle 12 Concepts for World Service"
```

---

### Task 3: Bundled Slogans corpus

**Files:**
- Create: `src/pulse/data/slogans.json`
- Modify: `src/pulse/bundled.rs`

- [ ] **Step 3.1: Author `src/pulse/data/slogans.json`**

```json
[
  "HALT — never let yourself get too Hungry, Angry, Lonely, or Tired.",
  "One Day at a Time.",
  "Easy Does It.",
  "Live and Let Live.",
  "Keep It Simple.",
  "First Things First.",
  "Let Go and Let God.",
  "But for the Grace of God.",
  "Think Think Think.",
  "This Too Shall Pass.",
  "Progress Not Perfection.",
  "Acceptance is the Answer.",
  "Stick with the Winners.",
  "Bring the Body and the Mind Will Follow.",
  "Time Takes Time.",
  "Trust God, Clean House, Help Others.",
  "You Are Not Alone.",
  "Meeting Makers Make It.",
  "Don't Quit Before the Miracle Happens.",
  "Identify, Don't Compare.",
  "Faith Without Works Is Dead.",
  "Restraint of Pen and Tongue.",
  "Suit Up and Show Up.",
  "Pass It On.",
  "Half Measures Availed Us Nothing.",
  "We Are Only as Sick as Our Secrets.",
  "Pain Is the Touchstone of Spiritual Growth.",
  "Stay in the Day.",
  "Yesterday Is History, Tomorrow Is a Mystery.",
  "Listen and Learn."
]
```

- [ ] **Step 3.2: Append to `src/pulse/bundled.rs`**

```rust
const SLOGANS_JSON: &str = include_str!("data/slogans.json");

pub struct Slogans { items: Vec<PulseItem> }

impl Slogans {
    pub fn load() -> Self {
        let entries: Vec<String> =
            serde_json::from_str(SLOGANS_JSON).expect("slogans.json malformed");
        let items = entries.into_iter().map(|s| PulseItem {
            kind: PulseKind::Slogan,
            step: None,
            label: "Slogan".to_string(),
            body: s,
        }).collect();
        Slogans { items }
    }
}

impl PulseSource for Slogans {
    fn name(&self) -> &str { "slogans" }
    fn items(&self) -> &[PulseItem] { &self.items }
}
```

- [ ] **Step 3.3: Tests**

```rust
    #[test]
    fn slogans_load_yields_thirty() {
        let s = Slogans::load();
        assert_eq!(s.items().len(), 30);
        assert_eq!(s.name(), "slogans");
    }

    #[test]
    fn slogans_all_slogan_kind() {
        let s = Slogans::load();
        for item in s.items() {
            assert_eq!(item.kind, PulseKind::Slogan);
            assert!(item.step.is_none());
            assert_eq!(item.label, "Slogan");
        }
    }

    #[test]
    fn halt_slogan_present() {
        let s = Slogans::load();
        assert!(s.items().iter().any(|i| i.body.starts_with("HALT")));
    }
```

- [ ] **Step 3.4: Verify + commit**

```
cargo test pulse:: 2>&1 | tail -5
git add src/pulse/data/slogans.json src/pulse/bundled.rs
git commit -m "feat(pulse): bundle 30 standard AA slogans"
```

---

### Task 4: Grapevine — bundled fallback + live source

**Files:**
- Create: `src/pulse/data/grapevine_fallback.json`
- Create: `src/pulse/grapevine.rs`
- Modify: `src/pulse/mod.rs` (register `pub mod grapevine;`)

- [ ] **Step 4.1: Author the fallback corpus**

Create `src/pulse/data/grapevine_fallback.json`:

```json
[
  "I came for my drinking and stayed for my thinking.",
  "Religion is for people afraid of going to hell. Spirituality is for people who have already been there.",
  "If nothing changes, nothing changes.",
  "The opposite of addiction is not sobriety. The opposite of addiction is connection.",
  "We do recover.",
  "Don't drink. Go to meetings. Read the Big Book. Help others.",
  "When all else fails, the directions are right there in the book.",
  "I'm not much, but I'm all I think about.",
  "An expectation is a premeditated resentment.",
  "Sobriety delivers everything alcohol promised.",
  "We are not human beings having a spiritual experience. We are spiritual beings having a human experience.",
  "Bad things happen, and you don't have to drink over them.",
  "The road to recovery is always under construction.",
  "Feelings aren't facts.",
  "Don't compare your insides to other people's outsides."
]
```

- [ ] **Step 4.2: Create `src/pulse/grapevine.rs`**

```rust
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
        Grapevine { items }
    }

    pub fn live_url() -> &'static str { LIVE_URL }
}

impl PulseSource for Grapevine {
    fn name(&self) -> &str { "grapevine" }
    fn items(&self) -> &[PulseItem] { &self.items }
}

fn parse_quote(html: &str) -> Option<String> {
    let doc = Html::parse_document(html);
    // Grapevine's quote-day page wraps the quote in a `<blockquote>` or in
    // a `.quote-of-the-day` block. Try both. Drop the byline.
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
```

- [ ] **Step 4.3: Register module**

In `src/pulse/mod.rs`, add `pub mod grapevine;` next to the other `pub mod` lines.

- [ ] **Step 4.4: Verify + commit**

```
cargo test pulse:: 2>&1 | tail -5
git add src/pulse/data/grapevine_fallback.json src/pulse/grapevine.rs src/pulse/mod.rs
git commit -m "feat(pulse): Grapevine source with live scrape + bundled fallback"
```

---

## Phase B — Mixer + config plumbing (additive)

### Task 5: `Order` enum + `advance_per_order` + `IWIYWI_ORDER`

**Files:**
- Modify: `src/pulse/mod.rs`

- [ ] **Step 5.1: Add `Order` enum and parser to `src/pulse/mod.rs`**

Add at the top of the file (after the `use` lines, before `PulseKind`):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Random,
    Sequential,
    ByStep,
    BySource,
}

impl Order {
    pub fn parse(raw: Option<&str>) -> Order {
        match raw {
            Some("sequential") => Order::Sequential,
            Some("by-step")    => Order::ByStep,
            Some("by-source")  => Order::BySource,
            _ => Order::Random,
        }
    }
}

pub fn order_from_env() -> Order {
    Order::parse(std::env::var("IWIYWI_ORDER").ok().as_deref())
}
```

- [ ] **Step 5.2: Add `PulseMixer::advance_per_order`**

In the `impl PulseMixer` block, add:

```rust
    /// Advance the cursor according to the given order. For `Random`, jumps
    /// to a deterministic random position from `seed`. For `Sequential`,
    /// `ByStep`, and `BySource`, walks the items in the order produced by
    /// `from_sources` (which already groups by source and tags by step).
    /// `ByStep` and `BySource` are aliases for `Sequential` at the mixer
    /// level — the grouping happens at construction time via the source
    /// order in `from_sources`.
    pub fn advance_per_order(&mut self, order: Order, seed: u32) {
        match order {
            Order::Random => self.random_jump(seed),
            Order::Sequential | Order::ByStep | Order::BySource => self.advance(),
        }
    }
```

(Note: `ByStep` and `BySource` ordering is achieved by sorting `items` at construction time. We'll add that in Step 5.3.)

- [ ] **Step 5.3: Sort items in `from_sources` by Order at construction time**

This requires `from_sources` to take the order. Update the signature:

Change `from_sources(sources: &[Box<dyn PulseSource>], filter_step: Option<u8>)` to:

```rust
    pub fn from_sources(
        sources: &[Box<dyn PulseSource>],
        filter_step: Option<u8>,
        order: Order,
    ) -> Self {
        let mut items: Vec<PulseItem> = Vec::new();
        let mut seen: std::collections::HashSet<[u8; 32]> = std::collections::HashSet::new();
        for src in sources {
            for item in src.items() {
                if let Some(want) = filter_step {
                    if item.step != Some(want) { continue; }
                }
                let mut hasher = Sha256::new();
                hasher.update(src.name().as_bytes());
                hasher.update([0u8]);
                hasher.update(item.body.as_bytes());
                let digest: [u8; 32] = hasher.finalize().into();
                if seen.insert(digest) { items.push(item.clone()); }
            }
        }
        match order {
            Order::ByStep => items.sort_by_key(|i| i.step.unwrap_or(255)),
            Order::BySource => { /* preserved by source iteration order above */ }
            Order::Random | Order::Sequential => { /* preserved as appended */ }
        }
        PulseMixer { items, cursor: 0 }
    }
```

- [ ] **Step 5.4: Tests**

Add to the `#[cfg(test)] mod tests` block:

```rust
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
    fn mixer_by_step_sorts_items_by_step_number() {
        let s: Box<dyn PulseSource> = Box::new(StubSource {
            name: "s",
            items: vec![
                item(PulseKind::TodayReading, Some(7), "g"),
                item(PulseKind::TodayReading, Some(1), "a"),
                item(PulseKind::TodayReading, Some(3), "c"),
            ],
        });
        let mixer = PulseMixer::from_sources(&[s], None, Order::ByStep);
        let bodies: Vec<&str> = mixer.all().iter().map(|i| i.body.as_str()).collect();
        assert_eq!(bodies, ["a", "c", "g"]);
    }

    #[test]
    fn advance_per_order_random_uses_random_jump() {
        let s: Box<dyn PulseSource> = Box::new(StubSource {
            name: "s",
            items: (0..50).map(|n| item(PulseKind::TodayReading, None, &n.to_string())).collect(),
        });
        let mut mixer = PulseMixer::from_sources(&[s], None, Order::Random);
        let start = mixer.cursor();
        mixer.advance_per_order(Order::Random, 0xdead_beef);
        assert_ne!(mixer.cursor(), start);
    }
```

- [ ] **Step 5.5: Build expected to fail at all `from_sources` callers**

```
cargo build 2>&1 | tail -10
```

Expected: errors at every call site of `from_sources` (today widgets, mod, and tests). Fix them by adding `Order::Random` as the third arg in each call. The TUI will be rewritten in Phase C anyway, so for now just append the third arg to keep the build green.

Sites likely needing fixes:
- `src/tui/mod.rs` — `enter_pulse` body
- `src/tui/mod.rs` — `register_input_exits_drift` test
- Anywhere else it's called

Make all of them pass `Order::Random` as a literal argument for now.

- [ ] **Step 5.6: Verify**

```
cargo build 2>&1 | tail -3
cargo test 2>&1 | tail -3
```

Expected: clean build, all tests pass.

- [ ] **Step 5.7: Commit**

```
git add src/pulse/mod.rs src/tui/mod.rs
git commit -m "feat(pulse): Order enum + advance_per_order + IWIYWI_ORDER parsing"
```

---

### Task 6: `Focus` enum + `IWIYWI_FOCUS`

**Files:**
- Modify: `src/pulse/mod.rs`

- [ ] **Step 6.1: Add `Focus` enum + parser**

Add to `src/pulse/mod.rs` near the `Order` enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
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
}

impl Focus {
    pub fn parse(raw: Option<&str>) -> Focus {
        match raw {
            Some("today")      => Focus::Today,
            Some("history")    => Focus::History,
            Some("big_book")   => Focus::BigBook,
            Some("prayers")    => Focus::Prayers,
            Some("steps")      => Focus::Steps,
            Some("principles") => Focus::Principles,
            Some("grapevine")  => Focus::Grapevine,
            Some("traditions") => Focus::Traditions,
            Some("concepts")   => Focus::Concepts,
            Some("slogans")    => Focus::Slogans,
            _ => Focus::All,
        }
    }

    /// True if the given source name passes this focus filter.
    pub fn admits(&self, source_name: &str) -> bool {
        match self {
            Focus::All        => true,
            Focus::Today      => source_name == "today",
            Focus::History    => source_name == "historical",
            Focus::BigBook    => source_name == "big_book",
            Focus::Prayers    => source_name == "prayers",
            Focus::Steps | Focus::Principles => source_name == "step_explainers",
            Focus::Grapevine  => source_name == "grapevine",
            Focus::Traditions => source_name == "traditions",
            Focus::Concepts   => source_name == "concepts",
            Focus::Slogans    => source_name == "slogans",
        }
    }
}

pub fn focus_from_env() -> Focus {
    Focus::parse(std::env::var("IWIYWI_FOCUS").ok().as_deref())
}
```

- [ ] **Step 6.2: Tests**

```rust
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
```

- [ ] **Step 6.3: Verify + commit**

```
cargo test pulse:: 2>&1 | tail -5
git add src/pulse/mod.rs
git commit -m "feat(pulse): Focus enum + IWIYWI_FOCUS parsing + admits filter"
```

---

### Task 7: `pulse_secs` config helper + `IWIYWI_PULSE_SECS`

**Files:**
- Modify: `src/config.rs`

- [ ] **Step 7.1: Add `pulse_secs` parser**

In `src/config.rs`, after the existing `idle_secs` parsers, add:

```rust
const DEFAULT_PULSE_SECS: u64 = 20;

/// Parse `IWIYWI_PULSE_SECS`. Returns `Some(Duration)` for auto-advance,
/// `None` for manual-only. Default 20s when unset; 0 → None.
pub fn parse_pulse_secs(raw: Option<&str>) -> Option<std::time::Duration> {
    let secs: u64 = match raw {
        None => DEFAULT_PULSE_SECS,
        Some(s) => s.parse().unwrap_or(DEFAULT_PULSE_SECS),
    };
    if secs == 0 {
        None
    } else {
        Some(std::time::Duration::from_secs(secs))
    }
}

pub fn pulse_secs() -> Option<std::time::Duration> {
    parse_pulse_secs(std::env::var("IWIYWI_PULSE_SECS").ok().as_deref())
}
```

- [ ] **Step 7.2: Tests**

In the test module of `src/config.rs`:

```rust
    #[test]
    fn parse_pulse_secs_defaults_to_twenty_when_none() {
        assert_eq!(parse_pulse_secs(None), Some(std::time::Duration::from_secs(20)));
    }

    #[test]
    fn parse_pulse_secs_returns_none_for_zero() {
        assert_eq!(parse_pulse_secs(Some("0")), None);
    }

    #[test]
    fn parse_pulse_secs_parses_positive_value() {
        assert_eq!(parse_pulse_secs(Some("45")), Some(std::time::Duration::from_secs(45)));
    }

    #[test]
    fn parse_pulse_secs_falls_back_on_garbage() {
        assert_eq!(parse_pulse_secs(Some("xx")), Some(std::time::Duration::from_secs(20)));
    }
```

- [ ] **Step 7.3: Verify + commit**

```
cargo test parse_pulse_secs 2>&1 | tail -5
git add src/config.rs
git commit -m "feat(config): IWIYWI_PULSE_SECS — pacing for the pulse cycle"
```

---

### Task 8: `Palette` module — 8 palettes × light/dark

**Files:**
- Create: `src/tui/palette.rs`
- Modify: `src/tui/mod.rs` (register `pub mod palette;`)

- [ ] **Step 8.1: Create `src/tui/palette.rs`**

```rust
//! Palette: an 8-variant color scheme system. Each palette has a light and a
//! dark form; `Theme` decides which form is used at runtime.

use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode { Light, Dark }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Default,
    Warm,
    Cool,
    Mono,
    Sunset,
    Sage,
    Dawn,
    Dusk,
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub mode: Mode,
    pub variant: Variant,
    pub bg: Color,
    pub accent: Color,
    pub body: Color,
    pub muted: Color,
}

impl Variant {
    pub fn parse(raw: Option<&str>) -> Variant {
        match raw {
            Some("warm")    => Variant::Warm,
            Some("cool")    => Variant::Cool,
            Some("mono")    => Variant::Mono,
            Some("sunset")  => Variant::Sunset,
            Some("sage")    => Variant::Sage,
            Some("dawn")    => Variant::Dawn,
            Some("dusk")    => Variant::Dusk,
            _ => Variant::Default,
        }
    }
}

impl Palette {
    pub fn build(mode: Mode, variant: Variant) -> Self {
        let (bg, accent, body, muted) = match (mode, variant) {
            (Mode::Dark, Variant::Default) => (
                Color::Reset,
                Color::Rgb(0x00, 0xD7, 0xFF), // cyan
                Color::Rgb(0xC9, 0xD1, 0xD9),
                Color::Rgb(0xA8, 0xA8, 0xA8),
            ),
            (Mode::Light, Variant::Default) => (
                Color::Reset,
                Color::Rgb(0x09, 0x69, 0xDA),
                Color::Rgb(0x24, 0x29, 0x2F),
                Color::Rgb(0x57, 0x60, 0x6A),
            ),
            (Mode::Dark, Variant::Warm) => (
                Color::Reset,
                Color::Rgb(0xFF, 0xC1, 0x07), // amber
                Color::Rgb(0xF5, 0xE6, 0xC8), // sand
                Color::Rgb(0xC4, 0x86, 0x5C), // terracotta
            ),
            (Mode::Light, Variant::Warm) => (
                Color::Reset,
                Color::Rgb(0xB8, 0x86, 0x0B), // dark amber
                Color::Rgb(0x4A, 0x37, 0x28),
                Color::Rgb(0x8B, 0x57, 0x3A),
            ),
            (Mode::Dark, Variant::Cool) => (
                Color::Reset,
                Color::Rgb(0x83, 0x9A, 0xB1), // slate
                Color::Rgb(0xC9, 0xD3, 0xDC), // steel
                Color::Rgb(0x70, 0x80, 0x90), // mist
            ),
            (Mode::Light, Variant::Cool) => (
                Color::Reset,
                Color::Rgb(0x35, 0x5A, 0x7E),
                Color::Rgb(0x2B, 0x35, 0x42),
                Color::Rgb(0x6B, 0x77, 0x82),
            ),
            (Mode::Dark, Variant::Mono) => (
                Color::Reset,
                Color::Rgb(0xFF, 0xFF, 0xFF),
                Color::Rgb(0xE0, 0xE0, 0xE0),
                Color::Rgb(0x80, 0x80, 0x80),
            ),
            (Mode::Light, Variant::Mono) => (
                Color::Reset,
                Color::Rgb(0x00, 0x00, 0x00),
                Color::Rgb(0x20, 0x20, 0x20),
                Color::Rgb(0x80, 0x80, 0x80),
            ),
            (Mode::Dark, Variant::Sunset) => (
                Color::Reset,
                Color::Rgb(0xFF, 0x7A, 0x29), // orange
                Color::Rgb(0xD9, 0x66, 0x44), // rust
                Color::Rgb(0x8B, 0x55, 0x82), // dusk-purple
            ),
            (Mode::Light, Variant::Sunset) => (
                Color::Reset,
                Color::Rgb(0xC2, 0x4D, 0x09),
                Color::Rgb(0x6F, 0x2E, 0x16),
                Color::Rgb(0x4F, 0x29, 0x4A),
            ),
            (Mode::Dark, Variant::Sage) => (
                Color::Reset,
                Color::Rgb(0x9C, 0xC4, 0x8E), // sage
                Color::Rgb(0xF3, 0xEE, 0xD8), // cream
                Color::Rgb(0x4D, 0x6B, 0x47), // forest
            ),
            (Mode::Light, Variant::Sage) => (
                Color::Reset,
                Color::Rgb(0x4D, 0x6B, 0x47),
                Color::Rgb(0x2A, 0x3C, 0x27),
                Color::Rgb(0x7B, 0x8E, 0x6F),
            ),
            (Mode::Dark, Variant::Dawn) => (
                Color::Reset,
                Color::Rgb(0xF5, 0xC2, 0xC7), // pale pink
                Color::Rgb(0xFB, 0xF2, 0xE9), // ivory
                Color::Rgb(0xC4, 0x82, 0x82), // dusty rose
            ),
            (Mode::Light, Variant::Dawn) => (
                Color::Reset,
                Color::Rgb(0xC2, 0x66, 0x70),
                Color::Rgb(0x4A, 0x35, 0x35),
                Color::Rgb(0x86, 0x55, 0x59),
            ),
            (Mode::Dark, Variant::Dusk) => (
                Color::Reset,
                Color::Rgb(0x86, 0x80, 0xC8), // indigo
                Color::Rgb(0xCB, 0xC1, 0xE5), // lavender
                Color::Rgb(0x6E, 0x76, 0x8B), // slate
            ),
            (Mode::Light, Variant::Dusk) => (
                Color::Reset,
                Color::Rgb(0x42, 0x3A, 0x82),
                Color::Rgb(0x29, 0x24, 0x4C),
                Color::Rgb(0x60, 0x5C, 0x7A),
            ),
        };
        Palette { mode, variant, bg, accent, body, muted }
    }
}

/// Detect light vs dark from `IWIYWI_THEME` (light|dark|auto) with COLORFGBG fallback.
pub fn detect_mode() -> Mode {
    match std::env::var("IWIYWI_THEME").ok().as_deref() {
        Some("light") => Mode::Light,
        Some("dark") => Mode::Dark,
        _ => auto_mode(),
    }
}

fn auto_mode() -> Mode {
    if let Ok(fgbg) = std::env::var("COLORFGBG") {
        if let Some(bg) = fgbg.split(';').nth(1).and_then(|s| s.parse::<u8>().ok()) {
            let is_light = matches!(bg, 7 | 9..=15);
            return if is_light { Mode::Light } else { Mode::Dark };
        }
    }
    Mode::Dark
}

pub fn from_env() -> Palette {
    let mode = detect_mode();
    let variant = Variant::parse(std::env::var("IWIYWI_PALETTE").ok().as_deref());
    Palette::build(mode, variant)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_parse_defaults() {
        assert_eq!(Variant::parse(None), Variant::Default);
        assert_eq!(Variant::parse(Some("garbage")), Variant::Default);
    }

    #[test]
    fn variant_parse_each() {
        for (s, v) in [
            ("warm", Variant::Warm), ("cool", Variant::Cool),
            ("mono", Variant::Mono), ("sunset", Variant::Sunset),
            ("sage", Variant::Sage), ("dawn", Variant::Dawn),
            ("dusk", Variant::Dusk),
        ] {
            assert_eq!(Variant::parse(Some(s)), v);
        }
    }

    #[test]
    fn palettes_differ_per_variant() {
        let dark_default = Palette::build(Mode::Dark, Variant::Default);
        let dark_warm    = Palette::build(Mode::Dark, Variant::Warm);
        assert_ne!(dark_default.accent, dark_warm.accent);
    }

    #[test]
    fn palettes_have_distinct_light_and_dark_bodies() {
        let l = Palette::build(Mode::Light, Variant::Default);
        let d = Palette::build(Mode::Dark, Variant::Default);
        assert_ne!(l.body, d.body);
    }
}
```

- [ ] **Step 8.2: Register module**

In `src/tui/mod.rs`, add `pub mod palette;` next to the other `pub mod` declarations.

- [ ] **Step 8.3: Verify + commit**

```
cargo build 2>&1 | tail -3
cargo test palette 2>&1 | tail -5
git add src/tui/palette.rs src/tui/mod.rs
git commit -m "feat(tui): 8-variant Palette module with light/dark forms"
```

---

### Task 9: `Pattern` module — 4 background variants

**Files:**
- Create: `src/tui/pattern.rs`
- Modify: `src/tui/mod.rs` (register `pub mod pattern;`)

- [ ] **Step 9.1: Create `src/tui/pattern.rs`**

```rust
//! Pattern: a static visual texture rendered once per pulse item, behind the
//! centered text. Not animated. Subtle by design.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Borders},
};

use crate::tui::palette::Palette;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    None,
    Dots,
    Frame,
    Rule,
}

impl Pattern {
    pub fn parse(raw: Option<&str>) -> Pattern {
        match raw {
            Some("dots")  => Pattern::Dots,
            Some("frame") => Pattern::Frame,
            Some("rule")  => Pattern::Rule,
            _ => Pattern::None,
        }
    }
}

pub fn from_env() -> Pattern {
    Pattern::parse(std::env::var("IWIYWI_PATTERN").ok().as_deref())
}

/// Draw the pattern into `area` using the palette's muted color.
/// `text_rect` is the rect where the centered text will land — patterns can
/// use it to position elements relative to the text.
pub fn draw(buf: &mut Buffer, area: Rect, text_rect: Rect, palette: &Palette, pattern: Pattern) {
    match pattern {
        Pattern::None => {}
        Pattern::Dots => draw_dots(buf, area, palette),
        Pattern::Frame => draw_frame(buf, text_rect, palette),
        Pattern::Rule => draw_rule(buf, text_rect, palette),
    }
}

fn draw_dots(buf: &mut Buffer, area: Rect, palette: &Palette) {
    if area.width < 4 || area.height < 4 { return; }
    let style = Style::default().fg(palette.muted);
    let coords = [
        (area.x + 1, area.y + 1),
        (area.x + area.width.saturating_sub(2), area.y + 1),
        (area.x + 1, area.y + area.height.saturating_sub(2)),
        (area.x + area.width.saturating_sub(2), area.y + area.height.saturating_sub(2)),
    ];
    for (x, y) in coords {
        buf[(x, y)].set_symbol("·").set_style(style);
    }
}

fn draw_frame(buf: &mut Buffer, text_rect: Rect, palette: &Palette) {
    if text_rect.width < 4 || text_rect.height < 3 { return; }
    let padded = Rect {
        x: text_rect.x.saturating_sub(2),
        y: text_rect.y.saturating_sub(1),
        width: text_rect.width + 4,
        height: text_rect.height + 2,
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(palette.muted));
    ratatui::widgets::Widget::render(block, padded, buf);
}

fn draw_rule(buf: &mut Buffer, text_rect: Rect, palette: &Palette) {
    if text_rect.width < 4 { return; }
    let y = text_rect.y + 1; // just under the kind line
    let style = Style::default().fg(palette.muted);
    for x in text_rect.x..(text_rect.x + text_rect.width) {
        buf[(x, y)].set_symbol("─").set_style(style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_parse_defaults_to_none() {
        assert_eq!(Pattern::parse(None), Pattern::None);
        assert_eq!(Pattern::parse(Some("garbage")), Pattern::None);
    }

    #[test]
    fn pattern_parse_each() {
        assert_eq!(Pattern::parse(Some("dots")), Pattern::Dots);
        assert_eq!(Pattern::parse(Some("frame")), Pattern::Frame);
        assert_eq!(Pattern::parse(Some("rule")), Pattern::Rule);
    }
}
```

- [ ] **Step 9.2: Register**

In `src/tui/mod.rs`, add `pub mod pattern;`.

- [ ] **Step 9.3: Verify + commit**

```
cargo build 2>&1 | tail -3
cargo test pattern 2>&1 | tail -5
git add src/tui/pattern.rs src/tui/mod.rs
git commit -m "feat(tui): Pattern module — none/dots/frame/rule background variants"
```

---

## Phase C — Strip the TUI

### Task 10: Rewrite `App`, event loop, and widgets to pulse-only

**Files:**
- Modify: `src/tui/mod.rs` (major rewrite)
- Modify: `src/tui/widgets.rs` (major rewrite)

This is the big shape change. Build will be temporarily broken because referenced modules (`drift`, `qr`, `commands`, `theme`) still exist; Task 11 deletes them.

- [ ] **Step 10.1: Replace the entire contents of `src/tui/widgets.rs` with**

```rust
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
    Frame,
};

use crate::pulse::PulseItem;
use crate::tui::palette::Palette;
use crate::tui::pattern::{self, Pattern};

pub fn render_pulse(frame: &mut Frame, item: Option<&PulseItem>, palette: &Palette, pattern: Pattern) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    let Some(item) = item else { return; };

    // Layout: max 72 cols, centered. Body wraps at width.
    let width = (area.width as f32 * 0.7).min(72.0).max(20.0) as u16;
    let body_lines_estimate = (item.body.chars().count() as u16 / width.max(1)).saturating_add(1);
    let total_height = 3 + body_lines_estimate; // label + kind + blank + body lines
    let total_height = total_height.min(area.height);

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(total_height)) / 2;
    let text_rect = Rect { x, y, width, height: total_height };

    pattern::draw(buf, area, text_rect, palette, pattern);

    let label = Line::from(Span::styled(
        item.label.clone(),
        Style::default().fg(palette.accent).add_modifier(Modifier::BOLD),
    ));
    let kind = Line::from(Span::styled(
        item.kind.display_label().to_string(),
        Style::default().fg(palette.muted).add_modifier(Modifier::ITALIC),
    ));
    let body = Line::from(Span::styled(
        item.body.clone(),
        Style::default().fg(palette.body),
    ));

    Paragraph::new(vec![label, kind, Line::from(""), body])
        .wrap(Wrap { trim: false })
        .render(text_rect, buf);
}
```

- [ ] **Step 10.2: Replace the entire contents of `src/tui/mod.rs` with**

```rust
pub mod palette;
pub mod pattern;
pub mod widgets;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::{Duration, Instant};

use crate::config;
use crate::pulse::{self, Focus, Order, PulseMixer, PulseSource};
use crate::storage::read_readings;

pub struct App {
    pub mixer: PulseMixer,
    pub sources: Vec<Box<dyn PulseSource>>,
    pub palette: palette::Palette,
    pub pattern: pattern::Pattern,
    pub order: Order,
    pub focus_step: Option<u8>,         // session-only step focus from number keys
    pub pulse_secs: Option<Duration>,   // None = manual-only
    pub last_advance: Instant,
    pub seed_counter: u32,
}

impl App {
    pub fn rebuild_mixer(&mut self) {
        self.mixer = PulseMixer::from_sources(&self.sources, self.focus_step, self.order);
    }

    pub fn next(&mut self) {
        self.mixer.advance_per_order(self.order, self.next_seed());
        self.last_advance = Instant::now();
    }

    pub fn prev(&mut self) {
        // Walk backward by len-1 advances. Cheap for small mixers.
        if self.mixer.is_empty() { return; }
        let len = self.mixer.len();
        for _ in 0..len.saturating_sub(1) { self.mixer.advance(); }
        self.last_advance = Instant::now();
    }

    pub fn random(&mut self) {
        let s = self.next_seed();
        self.mixer.random_jump(s);
        self.last_advance = Instant::now();
    }

    pub fn set_step_focus(&mut self, step: u8) {
        self.focus_step = Some(step);
        self.rebuild_mixer();
        self.last_advance = Instant::now();
    }

    pub fn clear_step_focus(&mut self) {
        self.focus_step = None;
        self.rebuild_mixer();
        self.last_advance = Instant::now();
    }

    fn next_seed(&mut self) -> u32 {
        self.seed_counter = self.seed_counter.wrapping_add(1);
        self.seed_counter
    }
}

pub fn run() -> Result<()> {
    let cfg = config::load_config()?;
    let readings = read_readings()?;

    // Build sources. `today` from readings, `historical` from disk, plus all bundled.
    let today_basename = format!("readings-{}.json", chrono::Local::now().format("%Y-%m-%d"));
    let mut sources: Vec<Box<dyn PulseSource>> = vec![
        Box::new(pulse::today::TodayReadings::from_readings(&readings)),
        Box::new(pulse::historical::HistoricalReadings::load_from(
            &config::config_dir(), &today_basename,
        )),
        Box::new(pulse::bundled::BigBookQuotes::load()),
        Box::new(pulse::bundled::Prayers::load()),
        Box::new(pulse::bundled::StepExplainers::load()),
        Box::new(pulse::bundled::Traditions::load()),
        Box::new(pulse::bundled::Concepts::load()),
        Box::new(pulse::bundled::Slogans::load()),
        Box::new(pulse::grapevine::Grapevine::from_html(None)),
    ];

    // Apply IWIYWI_FOCUS env var by trimming sources to only the matching kind.
    let focus = pulse::focus_from_env();
    if focus != Focus::All {
        sources.retain(|s| focus.admits(s.name()));
    }

    let order = pulse::order_from_env();
    let palette = palette::from_env();
    let pattern = pattern::from_env();
    let pulse_secs = config::pulse_secs();
    let _ = cfg;

    let mixer = PulseMixer::from_sources(&sources, None, order);

    let mut app = App {
        mixer,
        sources,
        palette,
        pattern,
        order,
        focus_step: None,
        pulse_secs,
        last_advance: Instant::now(),
        seed_counter: 1,
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| widgets::render_pulse(f, app.mixer.current(), &app.palette, app.pattern))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('n') => app.next(),
                    KeyCode::Char('p') => app.prev(),
                    KeyCode::Char('r') => app.random(),
                    KeyCode::Char('1') => app.set_step_focus(1),
                    KeyCode::Char('2') => app.set_step_focus(2),
                    KeyCode::Char('3') => app.set_step_focus(3),
                    KeyCode::Char('4') => app.set_step_focus(4),
                    KeyCode::Char('5') => app.set_step_focus(5),
                    KeyCode::Char('6') => app.set_step_focus(6),
                    KeyCode::Char('7') => app.set_step_focus(7),
                    KeyCode::Char('8') => app.set_step_focus(8),
                    KeyCode::Char('9') => app.set_step_focus(9),
                    KeyCode::Char('0') => app.set_step_focus(10),
                    KeyCode::Char('-') => app.set_step_focus(11),
                    KeyCode::Char('=') => app.set_step_focus(12),
                    KeyCode::Char('*') => app.clear_step_focus(),
                    _ => {}
                }
            }
        } else if let Some(interval) = app.pulse_secs {
            if app.last_advance.elapsed() >= interval {
                app.next();
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ClassifiedReading;

    fn fixture_app() -> App {
        let sources: Vec<Box<dyn PulseSource>> = vec![
            Box::new(pulse::today::TodayReadings::from_readings(&[
                ClassifiedReading {
                    step: 1, reason: "r".to_string(), source: "src".to_string(),
                    title: "t".to_string(), text: "today body".to_string(),
                    url: "http://x".to_string(),
                },
                ClassifiedReading {
                    step: 3, reason: "r".to_string(), source: "src".to_string(),
                    title: "t".to_string(), text: "another today body".to_string(),
                    url: "http://x".to_string(),
                },
            ])),
        ];
        let mixer = PulseMixer::from_sources(&sources, None, Order::Random);
        App {
            mixer,
            sources,
            palette: palette::Palette::build(palette::Mode::Dark, palette::Variant::Default),
            pattern: pattern::Pattern::None,
            order: Order::Random,
            focus_step: None,
            pulse_secs: Some(Duration::from_secs(20)),
            last_advance: Instant::now(),
            seed_counter: 1,
        }
    }

    #[test]
    fn next_advances_cursor() {
        let mut app = fixture_app();
        let before = app.mixer.cursor();
        app.next();
        assert_ne!(app.mixer.cursor(), before);
    }

    #[test]
    fn set_step_focus_rebuilds_mixer_to_only_that_step() {
        let mut app = fixture_app();
        app.set_step_focus(1);
        for i in app.mixer.all() {
            assert_eq!(i.step, Some(1));
        }
    }

    #[test]
    fn clear_step_focus_restores_full_mixer() {
        let mut app = fixture_app();
        app.set_step_focus(1);
        let focused = app.mixer.len();
        app.clear_step_focus();
        assert!(app.mixer.len() > focused);
    }

    #[test]
    fn random_changes_cursor_when_len_ge_two() {
        let mut app = fixture_app();
        let start = app.mixer.cursor();
        app.random();
        assert_ne!(app.mixer.cursor(), start);
    }
}
```

- [ ] **Step 10.3: Verify build is broken (expected — Task 11 fixes it)**

```
cargo build 2>&1 | tail -10
```

Expected: errors about missing `pub mod drift;`, `pub mod qr;`, `pub mod commands;`, `pub mod theme;` references — none of those exist anymore in our new `mod.rs`, but the FILES still exist on disk and may be referenced from other crate paths. Don't commit — go straight to Task 11.

---

### Task 11: Delete chrome modules

**Files:**
- Delete: `src/tui/drift.rs`
- Delete: `src/tui/qr.rs`
- Delete: `src/tui/commands.rs`
- Delete: `src/tui/theme.rs`

- [ ] **Step 11.1: Delete the four files**

```
rm src/tui/drift.rs src/tui/qr.rs src/tui/commands.rs src/tui/theme.rs
```

- [ ] **Step 11.2: Verify build**

```
cargo build 2>&1 | tail -10
```

Expected: clean build (the files were referenced only via `pub mod` declarations in the old `tui/mod.rs`, which we replaced in Task 10).

If errors mention `crate::tui::theme` etc. from other code paths, those need their imports updated to use `crate::tui::palette` instead. Likely sites: nothing — but check.

- [ ] **Step 11.3: Run tests**

```
cargo test 2>&1 | tail -3
```

Expected: pass.

- [ ] **Step 11.4: Commit Tasks 10 + 11 together**

```
git add -A
git commit -m "feat(tui): collapse TUI to pulse-only — strip tabs/QR/drift/command bar"
```

---

## Phase D — Strip fetch/config

### Task 12: Drop gist publishing + markdown rendering + MobileConfig + idle_secs

**Files:**
- Delete: `src/fetch/markdown.rs`
- Delete: `src/fetch/gist.rs`
- Modify: `src/fetch/mod.rs`
- Modify: `src/config.rs`

- [ ] **Step 12.1: Delete the two fetch modules**

```
rm src/fetch/markdown.rs src/fetch/gist.rs
```

- [ ] **Step 12.2: Update `src/fetch/mod.rs`**

Replace the full contents with:

```rust
pub mod ai_extract;
pub mod classify;
pub mod scraper;

use anyhow::{Context, Result};
use reqwest::Client;

use crate::config::{load_env, Config};
use crate::storage::write_readings;

pub async fn run(config: &Config) -> Result<()> {
    load_env().context("loading ~/.iwiywi/.env")?;

    println!("Scraping sources...");
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;
    let raw_readings = scraper::scrape_all(&client, config).await;
    println!("Got {} raw readings", raw_readings.len());

    if raw_readings.is_empty() {
        anyhow::bail!("no readings scraped — all sources failed");
    }

    println!("Classifying readings...");
    let classify_tasks: Vec<_> = raw_readings
        .into_iter()
        .map(|r| {
            let client = client.clone();
            let config = config.clone();
            tokio::spawn(async move { classify::classify(&client, &config, r).await })
        })
        .collect();

    let mut classified = Vec::new();
    for task in classify_tasks {
        match task.await {
            Ok(Ok(r)) => classified.push(r),
            Ok(Err(e)) => eprintln!("warn: classification failed: {e}"),
            Err(e) => eprintln!("warn: classify task panicked: {e}"),
        }
    }
    println!("Classified {} readings", classified.len());

    if classified.is_empty() {
        anyhow::bail!("all readings failed classification");
    }

    write_readings(&classified).context("writing readings to disk")?;
    println!("Saved readings to {}", crate::storage::readings_path().display());

    Ok(())
}
```

- [ ] **Step 12.3: Update `src/config.rs`**

Remove the `MobileConfig` struct, the `mobile` field from `Config`, the `qr_url()` helper, and the `idle_secs` / `parse_idle_secs` functions plus their tests.

The `Config` struct becomes:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ai: AiConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ai: AiConfig {
                model: "anthropic/claude-haiku-4-5".to_string(),
                gateway_url: "https://ai-gateway.vercel.sh/v1".to_string(),
                api_version: None,
            },
        }
    }
}
```

Delete:
- `MobileConfig` struct
- `qr_url()` function
- `DEFAULT_IDLE_SECS`, `parse_idle_secs()`, `idle_secs()` functions
- All `parse_idle_secs_*` tests

Delete every test that references `mobile.gist_id`. Update the legacy-toml test (`legacy_toml_without_mobile_section_loads`) to just verify the AI section parses.

- [ ] **Step 12.4: Build + fix call sites**

```
cargo build 2>&1 | tail -10
```

Expected: errors at any remaining `qr_url`/`idle_secs`/`mobile` callers. Fix them. After Task 10's TUI rewrite, there shouldn't be any.

- [ ] **Step 12.5: Verify**

```
cargo test 2>&1 | tail -3
```

- [ ] **Step 12.6: Commit**

```
git add -A
git commit -m "feat: drop gist/markdown/MobileConfig/idle_secs — pulse-only does not need them"
```

---

### Task 13: Drop unused crate dependencies

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 13.1: Remove `qrcode` and `noise`**

In `Cargo.toml`, delete the lines:

```
qrcode = "0.14"
noise = "0.9"
```

- [ ] **Step 13.2: Verify**

```
cargo build 2>&1 | tail -3
cargo test 2>&1 | tail -3
```

Expected: clean build, all tests pass.

- [ ] **Step 13.3: Commit**

```
git add Cargo.toml Cargo.lock
git commit -m "chore: drop qrcode and noise crate deps"
```

---

## Phase E — Scraper tightening

### Task 14: Boilerplate trimmer + per-source selector tightening

**Files:**
- Modify: `src/fetch/scraper.rs`

- [ ] **Step 14.1: Add a boilerplate trimmer**

Add to `src/fetch/scraper.rs` (before the `parse_*` functions):

```rust
/// Trim trailing/embedded boilerplate that often comes with scraped content
/// (copyright lines, "all rights reserved" notices, etc.).
fn trim_boilerplate(text: &str) -> String {
    let markers = [
        "All rights reserved",
        "registered trademarks",
        "Copyright (c)",
        "© ",
        "is the official Website",
        "“Alcoholics Anonymous” and the",
    ];
    let mut out = text.to_string();
    for m in markers {
        if let Some(idx) = out.find(m) {
            // Cut at the start of the line containing the marker.
            let line_start = out[..idx].rfind('\n').map(|p| p + 1).unwrap_or(0);
            out.truncate(line_start);
        }
    }
    out.trim().to_string()
}
```

- [ ] **Step 14.2: Use it in the parsers**

For each `parse_*` function in `scraper.rs` that builds a `RawReading`, wrap the `text` field through `trim_boilerplate`. Example for `parse_aa_org`:

Find the existing line:
```rust
        text,
        url: "https://www.aa.org/daily-reflections".to_string(),
```

Change to:
```rust
        text: trim_boilerplate(&text),
        url: "https://www.aa.org/daily-reflections".to_string(),
```

Repeat for `parse_hazeldon`, `parse_happy_hour`, `parse_silkworth`, `parse_aa_online_meeting`, `parse_aa_big_book`. Each one needs `text: trim_boilerplate(&text)`.

- [ ] **Step 14.3: Tighten the AA Happy Hour selector to one section**

The current selector `.entry-content p` grabs every paragraph on the page (including 24 Hours, As Bill Sees It, Grapevine, biographical content). Tighten to the first `<p>` of the entry only. In `parse_happy_hour`, replace:

```rust
    let text: String = document
        .select(&sel)
        .map(|e| e.text().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();
```

with:

```rust
    let text: String = document
        .select(&sel)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();
```

(Same `.next()`-only-then-trim treatment for `parse_aa_org` if it currently joins multiple `<p>`s — check the current implementation and apply the same change.)

- [ ] **Step 14.4: Tests**

```rust
    #[test]
    fn trim_boilerplate_removes_trademark_footer() {
        let raw = "The actual reading text here.\nAll rights reserved. © 2026 AA World Services.";
        let cleaned = trim_boilerplate(raw);
        assert_eq!(cleaned, "The actual reading text here.");
    }

    #[test]
    fn trim_boilerplate_handles_no_markers() {
        let raw = "Just a normal reading with no boilerplate.";
        assert_eq!(trim_boilerplate(raw), raw);
    }

    #[test]
    fn trim_boilerplate_cuts_at_official_website_line() {
        let raw = "Reading body.\nThis is the official Website of the General Service Office.";
        assert_eq!(trim_boilerplate(raw), "Reading body.");
    }
```

- [ ] **Step 14.5: Verify + commit**

```
cargo test fetch::scraper 2>&1 | tail -5
git add src/fetch/scraper.rs
git commit -m "fix(scraper): trim trademark/copyright boilerplate; tighten happy_hour to first <p>"
```

---

## Phase F — Docs

### Task 15: README rewrite

**Files:**
- Modify: `README.md`

- [ ] **Step 15.1: Replace Usage section**

Find the existing `## Usage` section. Replace it with:

```
## Usage

```
# Open the pulse — slow cycle of readings, prayers, Steps, and more
iwiywi

# Force a refresh of today's readings
iwiywi fetch

# Install the 6am launchd job (macOS)
iwiywi install
```

Keys: `n` next · `p` previous · `r` random · `q` quit · `1`–`9` `0` `-` `=` focus on Step N · `*` clear focus.
```

- [ ] **Step 15.2: Replace Theme section with Choices**

Find the `## Theme` section. Replace with:

```
## Choices

iwiywi pulses through AA content. Six env vars tune the experience:

```sh
# Pacing
export IWIYWI_PULSE_SECS=20      # default 20s; 0 = manual-only

# Color
export IWIYWI_THEME=auto         # light | dark | auto
export IWIYWI_PALETTE=default    # default warm cool mono sunset sage dawn dusk

# Visual
export IWIYWI_PATTERN=none       # none dots frame rule

# Cycling
export IWIYWI_ORDER=random       # random sequential by-step by-source

# Restrict to one kind of content
export IWIYWI_FOCUS=all          # all today history big_book prayers steps
                                 # principles grapevine traditions concepts slogans
```
```

- [ ] **Step 15.3: Replace Features bullets**

Find the `## Features` section. Replace with:

```
## Features

- Twelve daily AA readings, classified to a Step, refreshed every morning at 6am
- A pulse that quietly cycles your readings + the public-domain Big Book + the 12 Steps + the 12 Principles + the 12 Traditions + the 12 Concepts + 30 slogans + standard AA prayers + Grapevine Quote of the Day
- Six env-var knobs for pacing, color, pattern, order, focus, and theme
- Auto-fetches today's readings when you open it with no data for the day
- Adaptive light/dark detection from your terminal background
```

- [ ] **Step 15.4: Update Troubleshooting**

Replace the `<details>` block with:

```
<details>
<summary>Troubleshooting</summary>

**"No readings for today."** — Run `iwiywi fetch` once. Readings are keyed by local date. (Or just open `iwiywi` — it auto-fetches.)

**`VERCEL_AI_GATEWAY_TOKEN not set`.** — Add it to `~/.iwiywi/.env`. (Or switch to Azure — see "Choosing an AI provider" above.)

**`AZURE_OPENAI_API_KEY not set`.** — Add it to `~/.iwiywi/.env`. Required when `api_version` is set in `config.toml`.

**Colors look wrong.** — Set `IWIYWI_THEME=light` or `IWIYWI_THEME=dark` explicitly, or pick a different `IWIYWI_PALETTE`.

**launchd job didn't run.** — `launchctl list | grep iwiywi` to confirm it's loaded, and check `~/Library/Logs/iwiywi.log`.

</details>
```

- [ ] **Step 15.5: Replace "What pulses" section**

Find `## What pulses`. Replace its body to include the new sources:

```
## What pulses

The pulse cycles through:

- **Today's readings** — the day's classified readings.
- **Historical readings** — every prior `readings-*.json` saved in `~/.iwiywi/`.
- **Big Book quotes** — verbatim passages from the public-domain portion (pp. 1–164).
- **The 12 Steps** — verbatim text of each Step.
- **The 12 Principles** — Honesty, Hope, Faith, Courage, Integrity, Willingness, Humility, Brotherly Love, Justice, Perseverance, Spirituality, Service.
- **The 12 Traditions** — verbatim long-form.
- **The 12 Concepts for World Service** — verbatim long-form.
- **AA prayers** — Serenity, Third Step, Seventh Step, Eleventh Step (St. Francis), Set Aside, Acceptance, the Promises.
- **AA slogans** — HALT, One Day at a Time, Easy Does It, Live and Let Live, and 26 more.
- **Grapevine** — daily Quote of the Day from grapevine.org, with a bundled fallback.

`IWIYWI_FOCUS` restricts the pulse to one of these kinds. Pressing a number key (`1`–`9`, `0`=10, `-`=11, `=`=12) focuses to one Step until you press `*`.
```

- [ ] **Step 15.6: Commit**

```
git add README.md
git commit -m "docs: rewrite README for pulse-only TUI"
```

---

### Task 16: CHANGELOG entry

**Files:**
- Modify: `docs/CHANGELOG.md`

- [ ] **Step 16.1: Add `[0.5.0]` entry**

Insert below `## [Unreleased]`:

```
## [0.5.0] — 2026-04-15
- Changed: **TUI is now pulse-only** — no tabs, no command bar, no QR overlay, no drift particles, no fade animation. The screensaver is the app.
- Added: Six env-var knobs — `IWIYWI_PULSE_SECS`, `IWIYWI_PALETTE` (8 variants), `IWIYWI_PATTERN` (4), `IWIYWI_ORDER` (4), `IWIYWI_FOCUS` (11)
- Added: 4 new corpora — 12 Traditions, 12 Concepts for World Service, 30 standard AA slogans, 15 Grapevine fallback quotes
- Added: Live Grapevine Quote of the Day scraper (`grapevine.org`)
- Added: Boilerplate trimmer for scraped readings (drops trademark/copyright footers)
- Removed: QR overlay, gist publishing, markdown render, drift particles, fade animation, tab bar, command bar, idle screensaver mode
- Removed: `qrcode` and `noise` crate dependencies
- Removed: `[mobile]` config section + `gist_id` field
- Removed: `IWIYWI_IDLE_SECS` env var (no idle mode anymore — pulse is the only mode)
- Tightened: AA Happy Hour scraper now extracts only the first paragraph (was dumping every paragraph including unrelated sections)
```

- [ ] **Step 16.2: Commit**

```
git add docs/CHANGELOG.md
git commit -m "docs: changelog 0.5.0 — pulse-only TUI"
```

---

### Task 17: End-to-end verification + push

- [ ] **Step 17.1: Full test pass**

```
cargo test 2>&1 | tail -5
```

Expected: ≥ 130 tests pass.

- [ ] **Step 17.2: Release build**

```
cargo build --release 2>&1 | tail -3
```

Expected: clean.

- [ ] **Step 17.3: Live fetch**

```
cargo run -- fetch
```

Expected: ≥ 1 reading classified, no boilerplate noise visible in `~/.iwiywi/readings-*.json`.

- [ ] **Step 17.4: Manual UI smoke**

```
cargo run
```

Verify in order:
1. Centered text appears immediately. No tabs, no footer, no border (unless `IWIYWI_PATTERN=frame`).
2. Wait 20s → next item appears.
3. Press `n` → next item now.
4. Press `p` → previous item.
5. Press `r` → random item.
6. Press `3` → only Step 3 items now cycle.
7. Press `*` → focus clears, full mix returns.
8. Press `q` → quit cleanly.

```
IWIYWI_PALETTE=sunset cargo run         # orange/rust palette
IWIYWI_PATTERN=frame cargo run          # rounded border around text
IWIYWI_FOCUS=prayers cargo run          # only the 7 prayers cycle
IWIYWI_PULSE_SECS=0 cargo run           # no auto-advance — only n/p/r move
```

- [ ] **Step 17.5: Push**

```
git push
```

---

## Self-review

**Spec coverage:**
- 8 palettes × light/dark → Task 8 ✓
- 4 patterns → Task 9 ✓
- 4 cycle orders → Task 5 ✓
- 11 focus values → Task 6 ✓
- pulse_secs (0 = manual) → Task 7 ✓
- New corpora (Traditions/Concepts/Slogans/Grapevine + fallback) → Tasks 1–4 ✓
- Strip TUI (tabs/QR/drift/command/fade) → Tasks 10, 11 ✓
- Drop markdown/gist/MobileConfig/idle_secs → Task 12 ✓
- Drop qrcode/noise deps → Task 13 ✓
- Scraper boilerplate trim + Happy Hour tightening → Task 14 ✓
- README + CHANGELOG → Tasks 15, 16 ✓
- n/p/r/q + step focus number keys → Task 10 (event loop) ✓

**Type consistency:**
- `Order`, `Focus`, `Palette`, `Pattern`, `App`, `PulseMixer::from_sources(sources, filter_step, order)`, `PulseMixer::advance_per_order(order, seed)` — same signatures across Tasks 5, 6, 8, 9, 10.

**Placeholder scan:** none. Every code step shows the actual code; every command step shows the exact command and expected output direction.

**YAGNI check:** no dynamic palette switching, no live theme reload, no config UI, no per-source pacing, no animation easing. The `_dt` parameter on `tick` and `reading_phase_start` are gone with the chrome.
