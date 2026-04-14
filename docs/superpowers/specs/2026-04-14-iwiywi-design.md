# iwiywi — Design Spec
*"It Works If You Work It"*
Date: 2026-04-14

---

## Overview

A Rust CLI tool with TUI that aggregates AA daily readings from 12 sources each morning, classifies each reading into its most relevant AA step (1–12) via AI, caches results locally, and deploys a mobile-optimized HTML page to Vercel. A QR code in the TUI links to that page for iPhone access anywhere.

---

## Architecture

Two subcommands, one binary:

```
iwiywi fetch     ← run by launchd cron at 6am local time
iwiywi           ← user-launched TUI (reads from cache)
iwiywi install   ← one-time setup: writes launchd plist, loads it
```

**Data flow:**
```
6am (launchd)
  → vercel env pull ~/.iwiywi/.env       # refresh AI Gateway token
  → scrape 12 AA sources (reqwest + scraper)
  → Vercel AI Gateway → classify each reading into Step 1–12
  → write ~/.iwiywi/readings-YYYY-MM-DD.json
  → render /tmp/iwiywi-dist/index.html (mobile dark scrollable feed)
  → vercel deploy --prod /tmp/iwiywi-dist/  # stable Vercel URL updated

User opens TUI
  → reads ~/.iwiywi/readings-YYYY-MM-DD.json
  → displays scrollable feed (ratatui + crossterm)
  → /qr → QR overlay (qrcode crate) pointing to Vercel URL
  → q   → quit
  → r   → manually re-run fetch
```

---

## Sources (12 total)

Three confirmed scrapers plus nine to be verified during implementation via live web search. All must be AA-approved content.

**Confirmed:**
1. aa.org/daily-reflections
2. hazeldenbettyford.org/thought-for-the-day
3. aahappyhour.com/aa-daily-readings/

**Search-based:**
4. Google: top result / featured snippet for "aa thought for the day"
5. Reddit: r/alcoholicsanonymous daily thread

**To confirm during implementation (candidates):**
6. silkworth.net
7. aaonlinemeeting.net
8. aabigbook.com
9. recoveringcourage.com
10. odat.us (One Day At A Time)
11. joeancharlie.com — A Program for You (Joe and Charlie)
12. aahistory.com or equivalent

Each scraper returns: `{ source, title, text, url }`. If a source is unreachable, it is skipped silently and logged.

---

## AI Classification

**Provider:** Vercel AI Gateway (OpenAI-compatible API)
**Model:** `anthropic/claude-haiku-4-5` (configurable)
**Auth:** `VERCEL_AI_GATEWAY_TOKEN` env var, managed via `vercel env`

Request: standard OpenAI chat completion format via `reqwest`.

System prompt:
```
You are an AA step classifier. Given a daily reading excerpt, return the
single most relevant AA step number (1–12) and a one-sentence reason.
Respond with JSON only: {"step": 3, "reason": "..."}
```

Output stored per reading:
```json
{
  "step": 3,
  "reason": "Surrender and trust in a Higher Power",
  "source": "AA.org",
  "title": "Daily Reflections",
  "text": "Made a decision to turn our will...",
  "url": "https://www.aa.org/daily-reflections"
}
```

All 12 readings classified in parallel (tokio async tasks).

---

## Step Color Palette (TUI)

Each step maps to a distinct ratatui `Color`. 12 colors cycling through:

| Steps | Color |
|-------|-------|
| 1 | Red |
| 2 | Yellow |
| 3 | Blue |
| 4 | Magenta |
| 5 | Cyan |
| 6 | Green |
| 7 | LightRed |
| 8 | LightYellow |
| 9 | LightBlue |
| 10 | LightMagenta |
| 11 | LightCyan |
| 12 | LightGreen |

---

## TUI Layout

Built with `ratatui` + `crossterm`. (You mentioned "OpenTUI" — if this refers to a specific crate, swap it in during implementation. ratatui is the de-facto standard Rust TUI and is used here as the baseline.)

**Reading card style (Option C — left accent bar):**
```
▌ Step 3 · AA.org
  "Made a decision to turn our will and our lives
   over to the care of God as we understood Him."
  ─────────────────────────────────────────────────
▌ Step 7 · Hazeldon Betty Ford
  "Humbly asked Him to remove our shortcomings."
  ─────────────────────────────────────────────────
```

The `▌` character and step number are colored per the palette above. Reading text is white/light gray.

**Keybindings:**
- `↑ / ↓` or `j / k` — scroll
- `/` — enter command mode (vim-style); type `qr` + Enter to toggle QR overlay
- `r` — re-run fetch manually
- `Esc` — dismiss command mode or QR overlay
- `q` — quit

**Command mode:** Pressing `/` opens a one-line input bar at the bottom of the TUI. Supported commands: `qr`. Unrecognized commands are ignored.

**QR overlay:** Rendered in Unicode block characters via `qrcode` crate. Points to the Vercel deployment URL stored in `~/.iwiywi/config.toml`.

---

## Mobile Page (Vercel)

Single `index.html` written to `/tmp/iwiywi-dist/` and deployed via `vercel deploy --prod /tmp/iwiywi-dist/`.

- Dark background (`#0d1117`)
- Scrollable list of reading cards matching TUI style
- Each card: step number badge, source name, reading text
- Responsive, no JS required
- URL is stable — same Vercel project URL updated daily

---

## Cloud / Vercel CLI Usage

All Vercel interaction uses the `vercel` CLI (invoked as a subprocess from Rust via `std::process::Command`):

| Operation | Command |
|-----------|---------|
| Initial project setup | `vercel projects create iwiywi` |
| Store AI token | `vercel env add VERCEL_AI_GATEWAY_TOKEN` |
| Refresh token locally | `vercel env pull ~/.iwiywi/.env` |
| Deploy daily HTML | `vercel deploy --prod ./dist/` |

---

## Scheduling (macOS launchd)

`iwiywi install` writes and loads:
`~/Library/LaunchAgents/com.iwiywi.fetch.plist`

```xml
<key>StartCalendarInterval</key>
<dict>
  <key>Hour</key><integer>6</integer>
  <key>Minute</key><integer>0</integer>
</dict>
```

Timezone: launchd uses the system local timezone automatically. No UTC conversion needed.

Stdout/stderr logged to `~/.iwiywi/fetch.log`.

---

## Local Storage

```
~/.iwiywi/
  config.toml                  # vercel_url, model, etc.
  .env                         # vercel env pull output (gitignored)
  readings-YYYY-MM-DD.json     # today's classified readings
  fetch.log                    # cron run logs
```

`config.toml` example:
```toml
[ai]
model = "anthropic/claude-haiku-4-5"
gateway_url = "https://ai-gateway.vercel.sh/v1"

[vercel]
project_url = "https://iwiywi.vercel.app"
```

---

## Rust Crates

| Crate | Purpose |
|-------|---------|
| `ratatui` | TUI framework |
| `crossterm` | Terminal backend |
| `reqwest` | HTTP (scraping + AI Gateway calls) |
| `scraper` | HTML parsing |
| `tokio` | Async runtime |
| `serde` / `serde_json` | JSON serialization |
| `qrcode` | QR code generation in terminal |
| `toml` | Config file parsing |
| `clap` | CLI argument parsing |
| `chrono` | Date handling |

---

## Distribution & Release

**Homebrew tap** (primary install method):
```
brew install universal-grindset/iwiywi/iwiywi
```
Maintained in a separate repo: `github.com/universal-grindset/homebrew-iwiywi`
Formula points to the GitHub Release tarball + SHA256.

**GitHub Releases** (binary artifacts):
- Repo: `https://github.com/universal-grindset/iwiywi`
- Tagged releases: `v0.1.0`, `v0.2.0`, etc. (SemVer)
- Pre-built binaries attached: `iwiywi-aarch64-apple-darwin.tar.gz`

**GitHub Actions release pipeline** (`.github/workflows/release.yml`):
- Triggers on `git tag v*`
- Builds `aarch64-apple-darwin` binary via `cargo build --release`
- Creates GitHub Release, uploads binary tarball
- Prints SHA256 for Homebrew formula update

**Update path for users:** `brew upgrade iwiywi`

---

## Verification

1. `cargo build` — compiles cleanly
2. `iwiywi install` — launchd plist created and loaded
3. `iwiywi fetch` — runs manually, scrapes sources, classifies, deploys
4. `~/.iwiywi/readings-YYYY-MM-DD.json` — exists with 12 entries, each has `step` 1–12
5. Vercel URL loads mobile page with today's readings
6. `iwiywi` — TUI opens, readings visible with colored accent bars
7. `/qr` in TUI — QR overlay appears, scan with iPhone opens Vercel page
