# Pulse-Only TUI — Design

## Context

iwiywi has spent its first month accreting features: tabs (All / Steps / Help), a `/qr` overlay with gist-published Markdown for mobile, drift-particle screensaver chrome, fade-in/out animations, a command bar. The user's lived-in feedback after using it: the screensaver / pulse mode is "the best part." Everything else is competing for attention with the one thing that actually delivers value.

This spec collapses iwiywi into a single experience: **the pulse is the app**. Centered text, slow cycle, no chrome, no animation, no overlays. Bundled AA-tradition content grows so the pulse has rich, faithful material to draw from. Every choice the user wants to make is exposed as an environment variable — "if there's anything alcoholics like, it's choices."

## Recommended approach

### Render

The TUI is a single full-screen pulse renderer. No tab bar, no footer, no command line, no overlays, no particles, no fade. Vertically and horizontally centered:

```
                          Step 3 · AA.org
                                Today

      The central fact of our lives today is the absolute
      certainty that our Creator has entered into our hearts
      and lives in a way which is indeed miraculous.
```

Three logical lines: **label** (bold accent), **kind** (italic muted), blank, **body** (body color, wrapped to ≤72 columns). Hard cut between items.

### Pacing

- Default 20 seconds per item (tripled from today's ~8s — "pulses seem too fast")
- `IWIYWI_PULSE_SECS=N` overrides
- Setting `0` disables auto-advance entirely (manual-only)

### Keyboard

| Key | Action |
|---|---|
| `n` | next item now |
| `p` | previous item |
| `r` | random item now |
| `1`–`9`, `0`, `-`, `=` | focus on Step N (`-` = 11, `=` = 12) |
| `*` or two consecutive `0`s | clear step focus |
| `q` | quit |

No `/`, no Tab, no Esc, no arrow keys.

### Choices (env vars)

| Var | Options | Default |
|---|---|---|
| `IWIYWI_PULSE_SECS` | any non-negative int (`0` = manual-only) | `20` |
| `IWIYWI_THEME` | `light` `dark` `auto` | `auto` |
| `IWIYWI_PALETTE` | `default` `warm` `cool` `mono` `sunset` `sage` `dawn` `dusk` | `default` |
| `IWIYWI_PATTERN` | `none` `dots` `frame` `rule` | `none` |
| `IWIYWI_ORDER` | `random` `sequential` `by-step` `by-source` | `random` |
| `IWIYWI_FOCUS` | `all` `today` `history` `big_book` `prayers` `steps` `principles` `grapevine` `traditions` `concepts` `slogans` | `all` |

#### Palette definitions

Each palette is a coherent 4-color set: (background hint, accent, body text, muted secondary).

- **default** — current GitHub-inspired (cyan accent, neutral body)
- **warm** — amber accent, sand body, terracotta muted
- **cool** — slate accent, steel body, mist muted
- **mono** — pure black/white/gray; accent is bold not colored
- **sunset** — orange accent, rust body, dusk-purple muted
- **sage** — sage-green accent, cream body, forest muted
- **dawn** — pale-pink accent, ivory body, dusty-rose muted
- **dusk** — indigo accent, lavender body, slate muted

All palettes have a `_light` and `_dark` variant; the IWIYWI_THEME (or auto-detect) picks which.

#### Patterns

The pattern is rendered once per item, statically (not animated). It exists to make the screen feel less empty without competing with the text.

- **none** — pure background
- **dots** — sparse dim dots in the four corners (8 dots total, palette muted color)
- **frame** — a thin rounded border around the centered text block
- **rule** — a single horizontal rule below the kind line, above the body

### Sources

Existing five (`today`, `history`, `big_book`, `prayers`, `steps_explainers`) plus four new corpora:

- **Grapevine** — daily Quote of the Day scraped from `grapevine.org/quote-of-the-day`. Bundled fallback: ~30 free-tier quotes that ship in the binary so the source has content even when the scrape fails.
- **Traditions** — the 12 Traditions verbatim as 12 PulseItems, all step=None.
- **Concepts** — the 12 Concepts for World Service verbatim as 12 PulseItems, all step=None.
- **Slogans** — ~30 standard AA slogans as one-line PulseItems each: HALT · One Day at a Time · Easy Does It · Live and Let Live · Keep It Simple · First Things First · Let Go and Let God · But for the Grace of God · Think Think Think · This Too Shall Pass · Progress Not Perfection · Acceptance is the Answer · Stick with the Winners · Bring the Body and the Mind Will Follow · Time Takes Time · Trust God Clean House Help Others · You Are Not Alone · Meeting Makers Make It · Don't Quit Before the Miracle Happens · Identify Don't Compare · Faith Without Works Is Dead · Restraint of Pen and Tongue · Suit Up and Show Up · Pass It On · Half Measures Availed Us Nothing · We Are Only as Sick as Our Secrets · Pain Is the Touchstone of Spiritual Growth · Stay in the Day · Yesterday Is History Tomorrow Is a Mystery · Listen and Learn.

### Scraper tightening (separate workstream, same PR)

The current selectors are too broad. AA Happy Hour pulls Daily Reflections + 24 Hours + As Bill Sees It + Grapevine + biographical content all into one blob. AA.org includes the trademark / copyright footer.

For each of the 6 retained scrapers (aa_org, hazeldon, happy_hour, silkworth, aa_online_meeting, aa_big_book), audit the live HTML and tighten the selector to a single `<p>` or `<div>` containing only the day's reading. Add a post-extract trim that drops common boilerplate phrases ("All rights reserved.", "registered trademarks", etc.).

### What goes away

| File / module | Why |
|---|---|
| `src/tui/qr.rs` | No more QR overlay |
| `src/tui/commands.rs` | No command bar |
| `src/fetch/markdown.rs` | Was the gist body renderer; gist is gone |
| `src/fetch/gist.rs` | Mobile view is gone |
| `Mode::QrOverlay`, `Mode::Command`, `Tab` enum | No modes / no tabs |
| `App.tab`, `App.scroll`, `App.step_filter`, `App.qr_url`, `App.idle_threshold`, `App.last_input` | No tabs, no idle (every state is "pulsing") |
| `DriftState.particles`, `DriftState.noise`, `DriftState::tick`, `DriftState::resize`, `Particle`, `pseudo_rand`, `wrap`, `TRAIL_CHARS`, `lerp_color`, `rgb`, `FADE_IN`/`LINGER`/`FADE_OUT`/`READING_CYCLE`, `reading_alpha` | Particles + fade gone |
| `qrcode` crate dep | unused |
| `noise` crate dep | unused |
| `[mobile]` config section + `gist_id` field + `qr_url()` helper + `MobileConfig` struct | gist is gone |
| `IWIYWI_IDLE_SECS` env var | no idle mode anymore — pulse is the only mode |

### What stays

- `src/pulse/` — module, mixer, today, historical, bundled (extended)
- `src/fetch/scraper.rs`, `src/fetch/classify.rs`, `src/fetch/ai_extract.rs` (ai_extract is for AI-extracting readings from Wayback HTML — orthogonal to gist publishing)
- Auto-fetch on startup if today's readings are missing
- Adaptive light/dark detection
- All scraper sources (subject to selector tightening)
- The Wayback Machine fallback in `scraper.rs`

### App shape after the refactor

```rust
pub struct App {
    pub mixer: PulseMixer,
    pub theme: Theme,
    pub palette: Palette,
    pub pattern: Pattern,
    pub pulse_secs: Option<Duration>,  // None = manual-only
    pub last_advance: Instant,
    pub focus_step: Option<u8>,        // runtime override of IWIYWI_FOCUS
}
```

The whole event loop becomes:

```
loop:
    draw(centered_pulse(mixer.current(), palette, pattern))
    poll(50ms):
        if key:
            match: n/p/r/q/digits
        else:
            if pulse_secs and last_advance.elapsed() >= pulse_secs:
                mixer.advance_per_order()
                last_advance = now
```

No mode. No tabs. No nesting.

## Critical files

| File | Change |
|---|---|
| `src/pulse/data/grapevine_fallback.json` | **New.** ~30 free-tier ToTD quotes, bundled. |
| `src/pulse/data/traditions.json` | **New.** 12 verbatim Traditions. |
| `src/pulse/data/concepts.json` | **New.** 12 verbatim Concepts. |
| `src/pulse/data/slogans.json` | **New.** ~30 slogans. |
| `src/pulse/bundled.rs` | Add `Traditions`, `Concepts`, `Slogans`, `GrapevineFallback` source loaders. |
| `src/pulse/grapevine.rs` | **New.** Live-scrape grapevine.org Quote of the Day; on failure, fall back to `GrapevineFallback`. |
| `src/pulse/mod.rs` | Add `IWIYWI_ORDER` parsing + `Order` enum + `PulseMixer::advance_per_order(&Order)`. Add `IWIYWI_FOCUS` parsing. |
| `src/tui/palette.rs` | **New.** `Palette` enum (8 variants × light/dark), `IWIYWI_PALETTE` parsing. Replaces / extends current `Theme` color tables. |
| `src/tui/pattern.rs` | **New.** `Pattern` enum + render-once-per-item draw helpers. |
| `src/tui/mod.rs` | **Major rewrite.** Strip everything except `App` (new shape) + `run` (new event loop). Drop tabs, command, QR mode, idle. New keys (n/p/r/q + step focus). |
| `src/tui/widgets.rs` | **Major rewrite.** Single function: `render_pulse(frame, item, palette, pattern)`. Drop all tab/footer/scroll/help-tab code. |
| `src/tui/drift.rs` | **Delete.** Animation chrome gone. |
| `src/tui/qr.rs` | **Delete.** |
| `src/tui/commands.rs` | **Delete.** |
| `src/tui/theme.rs` | **Delete.** Replaced by `palette.rs`. |
| `src/fetch/markdown.rs` | **Delete.** |
| `src/fetch/gist.rs` | **Delete.** |
| `src/fetch/mod.rs` | Remove markdown render + gist publish steps. |
| `src/fetch/scraper.rs` | Tighten the 6 retained selectors; add post-extract boilerplate trim. |
| `src/config.rs` | Drop `MobileConfig`, `gist_id`, `qr_url()`, `idle_secs()`. Add `palette()`, `pattern()`, `order()`, `focus()`, `pulse_secs()` parsers. |
| `Cargo.toml` | Remove `qrcode`, `noise`. |
| `README.md` | Full rewrite of Usage / Choices / Troubleshooting / What pulses. |
| `docs/CHANGELOG.md` | `[0.5.0]` entry. |

## Reuse

- `PulseItem`, `PulseKind`, `PulseSource`, `PulseMixer` — unchanged interfaces; mixer gets a `advance_per_order` variant.
- `Prayers`, `BigBookQuotes`, `StepExplainers`, `TodayReadings`, `HistoricalReadings` — unchanged.
- Existing color values from current `theme.rs` become the `default` palette.

## Verification

1. `cargo build --release` clean.
2. `cargo test` passes (existing 119 minus tests for deleted modules, plus tests for the new corpora and palette/pattern parsers — net should land ≥ 130).
3. `cargo run` → centered text, no chrome, slow cycle.
4. `IWIYWI_PALETTE=sunset cargo run` → orange/rust palette.
5. `IWIYWI_PATTERN=frame cargo run` → thin rounded border around the text.
6. `IWIYWI_FOCUS=prayers cargo run` → only the 7 prayers cycle.
7. `IWIYWI_ORDER=by-step cargo run` → walks Step 1 items, then Step 2, etc.
8. `IWIYWI_PULSE_SECS=0 cargo run` → no auto-advance; only `n`/`p`/`r` move.
9. Press `5` → pulse focuses on Step 5; press `*` → focus clears.
10. Press `q` → exits cleanly.
11. `cargo run -- fetch` → ≥6 readings, scrapers no longer dump page-wide content.
12. Resize terminal during pulse → text re-centers.

## Non-goals

- No mouse, no sound, no notifications.
- No personal-data writing (sobriety counter, gratitude list, journaling) — those land in v0.6+ with their own design.
- No Big Book full-text browser — separate v0.6 spec.
- No meeting finder.
- No copyrighted sources (Touchstones, 24 Hours a Day, Hazelden meditations).
- No restoration of QR / gist / mobile view.
- No restoration of drift particles or fade animation. The decision to remove these is intentional and final for this design.

## Open considerations (not blocking)

- `IWIYWI_FOCUS` and runtime number-key focus interact: number-key sets a session-only focus that overrides the env var until cleared. Documented in README.
- Slogans are short enough that `IWIYWI_PULSE_SECS=20` may feel slow for them. Acceptable — the pause is the point.
- `IWIYWI_PALETTE=mono` ignores `IWIYWI_THEME` since black-on-white inverts to white-on-black naturally; the palette table will encode both forms anyway.
