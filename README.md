# iwiywi

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](docs/LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg?logo=rust)](https://www.rust-lang.org)
[![Release](https://img.shields.io/github/v/release/universal-grindset/iwiywi?display_name=tag&sort=semver)](https://github.com/universal-grindset/iwiywi/releases)
[![Last commit](https://img.shields.io/github/last-commit/universal-grindset/iwiywi)](https://github.com/universal-grindset/iwiywi/commits/master)

It Works If You Work It — daily AA readings, prayers, Steps, and AI-powered reflections in your terminal. Designed for a dedicated monitor. Also runs as a web server for your phone.

![iwiywi demo](demo.gif)

## Install

**Homebrew**

```
brew install universal-grindset/iwiywi/iwiywi
```

**Cargo**

```
cargo install --git https://github.com/universal-grindset/iwiywi
```

## Usage

```sh
# Open the pulse — slow cycle of readings, prayers, Steps, and more
iwiywi

# Force a refresh of today's readings
iwiywi fetch

# Install the 6am launchd job (macOS)
iwiywi install

# Serve the pulse as a web page you can open from any browser
iwiywi serve --bind 0.0.0.0 --port 8080
```

## Keyboard

| Key | Action |
|---|---|
| `n` / `N` | next / previous (or next/prev fuzzy match when searching) |
| `p` | previous item |
| `r` | random item |
| `gg` / `G` | jump to first / last item |
| `/` | fuzzy search (type query, Enter to jump, Esc to cancel) |
| `1`–`9` `0` `-` `=` | focus on Step 1–12 (tap twice for AI meditation) |
| `*` | clear step focus + search matches |
| `a` | AI: why this reading matters (overlay, Esc closes) |
| `F` | showcase mode (fullscreen, no chrome) |
| `m` | settings menu (arrows cycle, Esc closes) |
| `f` | favorite / unfavorite |
| `c` / click | copy current item to clipboard |
| `e` | export all items to `~/.iwiywi/exports/` |
| `j` | journal (AI-seeded reflection prompt + `$EDITOR`) |
| `?` | help overlay |
| `q` | quit |
| scroll wheel | prev/next (or scroll AI overlay) |
| shift+click | bypass mouse capture for terminal text selection |

The status bar shows context-sensitive hints that change per mode (normal, search, menu, overlay, help).

## Access it anywhere

`iwiywi serve` boots an HTTP server that renders the same pulse in any browser — phone, laptop, iPad — with the same key bindings and a touch-friendly control bar. Self-host it on a VPS and point a subdomain at it, or run it on your laptop and reach it over Tailscale.

```
iwiywi serve --bind 0.0.0.0 --port 8080
# open http://<your-host>:8080
```

A `Dockerfile` and systemd units live under `deploy/`. Walkthroughs for Docker, systemd, Caddy/nginx TLS, and Tailscale-only deployment are in [`docs/deploy.md`](docs/deploy.md).

## Features

- Twelve daily AA readings, classified to a Step, refreshed every morning at 6am
- A pulse that cycles readings + Big Book + Steps + Principles + Traditions + Concepts + slogans + prayers + Grapevine + Bill W. AI reflection + Reddit community paraphrase
- **51 color palettes** — 19 general-purpose (default, warm, cool, mono, sunset, sage, dawn, dusk, ember, ocean, rose, forest, amber, slate, mint, lavender, copper, indigo, nord), 20 black-metal (frostbitten, corpse\_paint, crimson\_altar, pale\_rider, funeral, moonspell, ashen, pestilence, obsidian, winterfell, raven, bloodmoon, tundra, necrotic, cascadian, iron\_oath, hellfire, sepulchral, wraith, abyssal), 12 terminal classics (solarized\_dark/light, gruvbox\_dark/light, dracula, tokyo\_night, catppuccin, onedark, monokai, synthwave, ayu, kanagawa)
- **13 background patterns** — 4 animated (drift, wave, snow, rain) + 9 static (none, dots, frame, rule, grid, corners, dashes, vignette, margin). Frame/Corners/Rule color-code by source type.
- **Fuzzy `/` search** with score-ordered results and highlighted matched characters
- **Showcase mode** (`F`) — fullscreen quote-wall, no chrome
- **Ambient anchors** — moon phase + sober-day counter (top-right), weather via wttr.in (top-left)
- **Idle dim-down** — after 5 min of no input, the palette dims to 30%; any key restores
- **Time-of-day palette auto** — `IWIYWI_PALETTE=auto` drifts dawn → warm → cool → dusk → funeral overnight
- **View transitions** — 150ms dim-to-bright fade on every item swap
- **Text size** menu row — small / normal / large (controls column width + bold)
- **19 Focus modes** — 14 source-based (today, history, big\_book, prayers, steps, principles, grapevine, traditions, concepts, slogans, favorites, bill, community) + 5 content-based (short, long, surrender 1–3, action 4–9, maintenance 10–12)
- AI at startup: Bill W. daily reflection, Reddit community curation, daily-summary toast — all deferred off the critical path so the TUI appears instantly
- `NO_COLOR` support, minimum-size gate (60x15), shift+click bypass for text selection
- Async `EventStream + tokio::select!` event loop (per the ratatui-tui skill)
- Auto-fetches today's readings when you open it with no data for the day
- Adaptive light/dark detection from your terminal background

## Choices

```sh
# Pacing
export IWIYWI_PULSE_SECS=20         # default 20s; 0 = manual-only

# Color
export IWIYWI_THEME=auto            # light | dark | auto
export IWIYWI_PALETTE=default       # see palette list above; "auto" drifts by hour
export NO_COLOR=1                   # disable all color (monochrome fallback)

# Visual
export IWIYWI_PATTERN=drift         # drift wave snow rain none dots frame rule
                                    # grid corners dashes vignette margin
export IWIYWI_TEXT_SIZE=normal       # small | normal | large

# Cycling
export IWIYWI_ORDER=random          # random sequential by-step by-source

# Restrict to one kind of content
export IWIYWI_FOCUS=all             # all today history big_book prayers steps
                                    # principles grapevine traditions concepts slogans
                                    # favorites bill community
                                    # short long surrender action maintenance

# Sobriety counter (shown in moon anchor)
export IWIYWI_SOBER_SINCE=2023-01-15

# Weather (top-left anchor)
export IWIYWI_WEATHER_LOC=seattle   # default: auto-detected by IP via wttr.in
```

## What pulses

The pulse cycles through:

- **Today's readings** — the day's classified readings
- **Historical readings** — every prior `readings-*.json` saved in `~/.iwiywi/`
- **Big Book quotes** — verbatim passages from the public-domain portion (pp. 1–164)
- **The 12 Steps** — verbatim text of each Step
- **The 12 Principles** — Honesty, Hope, Faith, Courage, Integrity, Willingness, Humility, Brotherly Love, Justice, Perseverance, Spirituality, Service
- **The 12 Traditions** — verbatim long-form
- **The 12 Concepts for World Service** — verbatim long-form
- **AA prayers** — Serenity, Third Step, Seventh Step, Eleventh Step (St. Francis), Set Aside, Acceptance, the Promises
- **AA slogans** — HALT, One Day at a Time, Easy Does It, Live and Let Live, and 26 more
- **Grapevine** — daily Quote of the Day from grapevine.org, with a bundled fallback
- **Bill W. reflection (AI)** — an AI-generated meditation each day in the voice of a recovering elder. Honestly labeled. Cached per day under `~/.iwiywi/bill/`
- **Community (AI, Reddit)** — up to three insights from `/r/stopdrinking` and `/r/alcoholicsanonymous`, paraphrased by the AI gateway. No usernames. Cached per day under `~/.iwiywi/community/`

### AI actions

- **`a`** — overlay explaining *why this reading matters today* (cache-keyed per item; shows "AI unavailable" offline)
- **Step double-tap** — tap the same digit twice within 1.5s for an AI-generated meditation on that step (cached per day)
- **`j`** — journal: AI-generated reflection question seeds today's `~/.iwiywi/journal/` entry before opening `$EDITOR`
- **Daily summary toast** — at startup, the gateway returns a short "theme for today" line shown briefly in the status bar

## How it works

`iwiywi fetch` scrapes AA daily-reading sources (with a Wayback Machine fallback), asks the configured LLM to classify each to one of the twelve Steps with a short reason, and writes the result to `~/.iwiywi/readings-<date>.json`. `iwiywi install` writes a launchd plist that runs `iwiywi fetch` at 6:00 local time. `iwiywi` (no args) reads today's file (auto-fetches if missing), best-effort fetches Grapevine + Reddit + weather in parallel, and opens the pulse. Bill/Community/Summary AI calls run in the background after the TUI appears — no startup delay.

### Choosing an AI provider

iwiywi speaks the OpenAI chat-completions API. Two configurations are supported:

**Vercel AI Gateway** (default):
```toml
# ~/.iwiywi/config.toml
[ai]
model = "anthropic/claude-haiku-4-5"
gateway_url = "https://ai-gateway.vercel.sh/v1"
```
```sh
# ~/.iwiywi/.env
VERCEL_AI_GATEWAY_TOKEN=<your token>
```

**Azure OpenAI** (or AI Foundry):
```toml
[ai]
model = "gpt-4o-mini"   # = your deployment name
gateway_url = "https://<RESOURCE>.openai.azure.com/openai/deployments/<DEPLOYMENT>"
api_version = "2025-01-01-preview"
```
```sh
AZURE_OPENAI_API_KEY=<your key>
```

Setting `api_version` flips the auth header from `Authorization: Bearer` to `api-key:` and appends `?api-version=...` to the URL.

## Monitor-worthy combos

```sh
# Norwegian frost with falling snow
IWIYWI_PALETTE=frostbitten IWIYWI_PATTERN=snow iwiywi

# Deep night BM with corner brackets
IWIYWI_PALETTE=funeral IWIYWI_PATTERN=corners iwiywi

# Cascadian atmospheric with Perlin drift
IWIYWI_PALETTE=cascadian IWIYWI_PATTERN=drift iwiywi

# Auto-drifts dawn→warm→cool→dusk→funeral through the day
IWIYWI_PALETTE=auto IWIYWI_PATTERN=drift iwiywi

# Showcase mode: press F once launched for fullscreen quote-wall
IWIYWI_PALETTE=dracula IWIYWI_PATTERN=none iwiywi
```

<details>
<summary>Troubleshooting</summary>

**"No readings for today."** — Run `iwiywi fetch` once. Readings are keyed by local date. (Or just open `iwiywi` — it auto-fetches.)

**`VERCEL_AI_GATEWAY_TOKEN not set`.** — Add it to `~/.iwiywi/.env`. (Or switch to Azure — see "Choosing an AI provider" above.)

**`AZURE_OPENAI_API_KEY not set`.** — Add it to `~/.iwiywi/.env`. Required when `api_version` is set in `config.toml`.

**Colors look wrong.** — Set `IWIYWI_THEME=light` or `IWIYWI_THEME=dark` explicitly, or pick a different `IWIYWI_PALETTE`. Use `NO_COLOR=1` for monochrome.

**Terminal too small.** — iwiywi needs at least 60x15 cells. Below that it shows a resize message.

**Weather not showing.** — `wttr.in` may be blocked or slow. Override location with `IWIYWI_WEATHER_LOC=<city>`.

**launchd job didn't run.** — `launchctl list | grep iwiywi` to confirm it's loaded, and check `~/Library/Logs/iwiywi.log`.

</details>
