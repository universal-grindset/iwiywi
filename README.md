# iwiywi

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](docs/LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg?logo=rust)](https://www.rust-lang.org)
[![Release](https://img.shields.io/github/v/release/universal-grindset/iwiywi?display_name=tag&sort=semver)](https://github.com/universal-grindset/iwiywi/releases)
[![Last commit](https://img.shields.io/github/last-commit/universal-grindset/iwiywi)](https://github.com/universal-grindset/iwiywi/commits/master)

It Works If You Work It — twelve daily AA readings in your terminal, classified to the Steps.

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

```
# Open today's readings (auto-fetches first if none for today)
iwiywi

# Force a refresh
iwiywi fetch

# Install the 6am launchd job (macOS)
iwiywi install
```

Keys inside the TUI: `a` / `s` / `?` switch tabs · `Tab` cycles · `j`/`k` scroll · `←/→` change step on the Steps tab · `p` enter pulse · `r` random surprise (or re-roll inside pulse) · `Enter` on Steps tab launches step-focused pulse · `/qr` mobile QR · `q` quit.

## Features

- Twelve daily readings aggregated from trusted AA sources
- Each reading classified to a Step (1–12)
- Scrollable TUI with tabbed filtering by step
- QR overlay (`/qr`) for mobile handoff
- Adaptive light/dark palette, auto-detected from terminal background
- Runs daily at 6am via launchd
- Idle screensaver: flow-field drift animation cycles your readings after 60s
- On-demand pulse with `p` (any tab) or `r` (single random item from anywhere)
- Step-focused pulse: pick a step on the Steps tab and press Enter

## Theme

iwiywi picks light or dark colors from your terminal background. Override if detection gets it wrong:

```sh
export IWIYWI_THEME=light  # force light palette
export IWIYWI_THEME=dark   # force dark palette
export IWIYWI_THEME=auto   # auto-detect (default)
```

Set how long until the screensaver activates, or disable it:

```sh
export IWIYWI_IDLE_SECS=60   # default
export IWIYWI_IDLE_SECS=10   # faster idle
export IWIYWI_IDLE_SECS=0    # never activate
```

## How it works

`iwiywi fetch` scrapes AA daily-reading sources (with a Wayback Machine fallback), asks the configured LLM to classify each to one of the twelve Steps with a short reason, writes the result to `~/.iwiywi/readings-<date>.json`, renders a Markdown view, and publishes it to a GitHub Gist via `gh`. The gist URL is what the QR overlay encodes — scan it and GitHub renders the page on your phone. `iwiywi install` writes a launchd plist that runs `iwiywi fetch` at 6:00 local time. `iwiywi` (no args) reads today's file (auto-fetches if missing) and renders the TUI.

### Choosing an AI provider

iwiywi speaks the OpenAI chat-completions API. Two configurations are supported out of the box:

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
api_version = "2024-08-01-preview"
```
```sh
AZURE_OPENAI_API_KEY=<your key>
```

Setting `api_version` flips the auth header from `Authorization: Bearer` to `api-key:` and appends `?api-version=…` to the URL.

`gh` CLI authenticated (`gh auth login` with `gist` scope) is required for gist publishing.

## What pulses

The pulse animation cycles through a mix of:

- **Today's readings** — the day's classified readings.
- **Historical readings** — every prior `readings-*.json` saved in `~/.iwiywi/`.
- **Big Book quotes** — verbatim passages from the public-domain portion (pp. 1–164).
- **AA prayers** — Serenity, Third Step, Seventh Step, Eleventh Step (St. Francis), Set Aside, Acceptance, the Promises.
- **The 12 Steps** — verbatim text of each Step.
- **The 12 Principles** — Honesty, Hope, Faith, Courage, Integrity, Willingness, Humility, Brotherly Love, Justice, Perseverance, Spirituality, Service.

A step-focused pulse (`Enter` on the Steps tab) shows only items tagged with the current step — including the Step text and its Principle, so the mixer is never empty.

<details>
<summary>Troubleshooting</summary>

**"No readings for today."** — Run `iwiywi fetch` once. Readings are keyed by local date.

**`VERCEL_AI_GATEWAY_TOKEN not set`.** — Add it to `~/.iwiywi/.env`. (Or switch to Azure — see "Choosing an AI provider" above.)

**`AZURE_OPENAI_API_KEY not set`.** — Same — add it to `~/.iwiywi/.env`. Required when `api_version` is set in `config.toml`.

**`gh gist create` fails.** — Run `gh auth login` and ensure the `gist` scope is granted (`gh auth refresh -s gist`).

**Colors look wrong.** — Set `IWIYWI_THEME=light` or `IWIYWI_THEME=dark` explicitly.

**launchd job didn't run.** — `launchctl list | grep iwiywi` to confirm it's loaded, and check `~/Library/Logs/iwiywi.log`.

**QR scan opens nothing.** — First run needs `gh` authenticated; the gist ID is stored in `~/.iwiywi/config.toml` after the first successful fetch.

</details>
