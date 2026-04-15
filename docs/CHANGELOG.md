# Changelog

All notable changes to iwiywi are documented here.

## [Unreleased]

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

## [0.4.0] — 2026-04-15
- Added: **Pulse system** — the drift screensaver now cycles a unified mix of today's readings, every saved historical reading, 15 verbatim Big Book quotes, 7 standard AA prayers, the 12 Steps, and the 12 Principles
- Added: `p` key — manual pulse on demand from any tab
- Added: `r` key — random surprise from Normal mode, mid-pulse re-roll inside Pulse mode
- Added: `Enter` on the Steps tab launches a step-focused pulse showing only items tagged with the current step
- Added: `IWIYWI_IDLE_SECS` env var (default 60, `0` disables) — controls when the screensaver activates
- Added: **Azure OpenAI / AI Foundry** support — set `api_version` in `[ai]` and supply `AZURE_OPENAI_API_KEY`; iwiywi flips the auth header and URL accordingly
- Added: AI-driven extraction infrastructure (`src/fetch/ai_extract.rs`) — sends archived HTML through the configured LLM with an extraction prompt
- Added: Wayback Machine fallback when live scraper fetches return empty
- Added: Auto-fetch when `iwiywi` is run with no readings for today
- Removed: 5 unworkable scraper sources (3 DNS-dead, 2 dynamic-JS aggregators)
- Removed: Vercel deployment for the mobile view — replaced by `gh gist`-published Markdown
- Changed: Quien-style README — collapsed the Astro Starlight docs site into a single root `README.md`
- Changed: Adaptive light/dark TUI palette (`IWIYWI_THEME=light|dark|auto`) replaces the rainbow step colors
- Internal: Apollo Rust best-practices audit (clippy clean with `-D warnings`, dead-code warnings silenced)

## [0.2.0] — 2026-04-14
- Added: Mobile QR code overlay in TUI (`/qr` command)
- Added: Manual fetch trigger with `r` key
- Improved: Vercel deployment stability
- Improved: TUI scroll performance

## [0.1.0] — 2026-04-07
- Initial release
- Fetch: Aggregate 12 AA readings, classify to Steps 1–12
- TUI: Scrollable feed with step colors and accent bars
- Schedule: Daily 6am fetch via launchd
- Mobile: Deploy to Vercel, QR code access
