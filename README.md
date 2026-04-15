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
# Open the pulse — slow cycle of readings, prayers, Steps, and more
iwiywi

# Force a refresh of today's readings
iwiywi fetch

# Install the 6am launchd job (macOS)
iwiywi install

# Serve the pulse as a web page you can open from any browser
iwiywi serve --bind 0.0.0.0 --port 8080
```

Keys: `n` next · `p` previous · `r` random · `q` quit · `1`–`9` `0` `-` `=` focus on Step N · `*` clear focus · **`m` open settings menu** (↑↓ pick, ←→ cycle, Esc close).

## Access it anywhere

`iwiywi serve` boots an HTTP server that renders the same pulse in any browser
— phone, laptop, iPad — with the same key bindings and a touch-friendly
control bar. Self-host it on a VPS and point a subdomain at it, or run it on
your laptop and reach it over Tailscale.

```
iwiywi serve --bind 0.0.0.0 --port 8080
# open http://<your-host>:8080
```

A `Dockerfile` and systemd units live at the repo root under `deploy/`.
Walkthroughs for Docker, systemd, Caddy/nginx TLS, and Tailscale-only
deployment are in [`docs/deploy.md`](docs/deploy.md).

## Features

- Twelve daily AA readings, classified to a Step, refreshed every morning at 6am
- A pulse that quietly cycles your readings + the public-domain Big Book + the 12 Steps + the 12 Principles + the 12 Traditions + the 12 Concepts + 30 slogans + standard AA prayers + Grapevine Quote of the Day
- Six env-var knobs for pacing, color, pattern, order, focus, and theme
- Auto-fetches today's readings when you open it with no data for the day
- Adaptive light/dark detection from your terminal background

## Choices

iwiywi pulses through AA content. Six env vars tune the experience:

```sh
# Pacing
export IWIYWI_PULSE_SECS=20      # default 20s; 0 = manual-only

# Color
export IWIYWI_THEME=auto         # light | dark | auto
export IWIYWI_PALETTE=default    # default warm cool mono sunset sage dawn dusk

# Visual
export IWIYWI_PATTERN=drift      # drift (default) none dots frame rule

# Cycling
export IWIYWI_ORDER=random       # random sequential by-step by-source

# Restrict to one kind of content
export IWIYWI_FOCUS=all          # all today history big_book prayers steps
                                 # principles grapevine traditions concepts slogans
```

## How it works

`iwiywi fetch` scrapes AA daily-reading sources (with a Wayback Machine fallback), asks the configured LLM to classify each to one of the twelve Steps with a short reason, and writes the result to `~/.iwiywi/readings-<date>.json`. `iwiywi install` writes a launchd plist that runs `iwiywi fetch` at 6:00 local time. `iwiywi` (no args) reads today's file (auto-fetches if missing), best-effort fetches the Grapevine Quote of the Day (5s timeout, falls back to a bundled corpus on failure), and opens the pulse.

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
- **Bill W. reflection (AI)** — an AI-generated meditation each day in the voice of a recovering elder. Honestly labeled; nothing is drawn from the copyrighted "As Bill Sees It" text. Cached per day under `~/.iwiywi/bill/`.
- **Community (AI, Reddit)** — up to three insights pulled from `/r/stopdrinking` and `/r/alcoholicsanonymous` and paraphrased by the gateway. No usernames. Cached per day under `~/.iwiywi/community/`. X/Twitter is intentionally not included — their API lacks a free read tier.

`IWIYWI_FOCUS` restricts the pulse to one of these kinds (adds: `bill`, `community`). Pressing a number key (`1`–`9`, `0`=10, `-`=11, `=`=12) focuses to one Step until you press `*`. Pressing the same digit again within 1.5s opens an AI meditation on that step.

### AI actions

- **`a`** — open an overlay with an AI explanation of *why this reading matters today*. Cache-keyed per item body; offline calls show `AI unavailable`. `Esc` or `a` again closes.
- **`j`** — journal key. Before launching `$EDITOR`, fetches a short AI-generated reflection question tied to the current reading and seeds today's journal file with it. Falls back to the static prompt if the gateway is unreachable.
- **Daily summary toast** — at startup, the gateway returns a ≤12-word "theme for today" line, shown briefly in the status bar on first render.

<details>
<summary>Troubleshooting</summary>

**"No readings for today."** — Run `iwiywi fetch` once. Readings are keyed by local date. (Or just open `iwiywi` — it auto-fetches.)

**`VERCEL_AI_GATEWAY_TOKEN not set`.** — Add it to `~/.iwiywi/.env`. (Or switch to Azure — see "Choosing an AI provider" above.)

**`AZURE_OPENAI_API_KEY not set`.** — Add it to `~/.iwiywi/.env`. Required when `api_version` is set in `config.toml`.

**Colors look wrong.** — Set `IWIYWI_THEME=light` or `IWIYWI_THEME=dark` explicitly, or pick a different `IWIYWI_PALETTE`.

**launchd job didn't run.** — `launchctl list | grep iwiywi` to confirm it's loaded, and check `~/Library/Logs/iwiywi.log`.

</details>
