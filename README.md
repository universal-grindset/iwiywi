# iwiywi

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
# Open today's readings
iwiywi

# Manually refresh
iwiywi fetch

# Install the 6am launchd job (macOS)
iwiywi install
```

## Features

- Twelve daily readings aggregated from trusted AA sources
- Each reading classified to a Step (1–12)
- Scrollable TUI with tabbed filtering by step
- QR overlay (`/qr`) for mobile handoff
- Adaptive light/dark palette, auto-detected from terminal background
- Runs daily at 6am via launchd
- Idle screensaver: flow-field drift animation cycles today's readings after 60s

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

`iwiywi fetch` scrapes twelve AA daily-reading sources, asks the configured LLM (via Vercel AI Gateway) to classify each to one of the twelve Steps with a short reason, writes the result to `~/.iwiywi/readings-<date>.json`, renders a Markdown view, and publishes it to a GitHub Gist via `gh`. The gist URL is what the QR overlay encodes — scan it and GitHub renders the page on your phone. `iwiywi install` writes a launchd plist that runs `iwiywi fetch` at 6:00 local time. `iwiywi` (no args) reads today's file and renders the TUI.

Requirements:
- `VERCEL_AI_GATEWAY_TOKEN` in `~/.iwiywi/.env` (classification).
- `gh` CLI authenticated (`gh auth login`) for gist publishing.

<details>
<summary>Troubleshooting</summary>

**"No readings for today."** — Run `iwiywi fetch` once. Readings are keyed by local date.

**`VERCEL_AI_GATEWAY_TOKEN not set`.** — Add it to `~/.iwiywi/.env`.

**`gh gist create` fails.** — Run `gh auth login` and ensure the `gist` scope is granted (`gh auth refresh -s gist`).

**Colors look wrong.** — Set `IWIYWI_THEME=light` or `IWIYWI_THEME=dark` explicitly.

**launchd job didn't run.** — `launchctl list | grep iwiywi` to confirm it's loaded, and check `~/Library/Logs/iwiywi.log`.

**QR scan opens nothing.** — First run needs `gh` authenticated; the gist ID is stored in `~/.iwiywi/config.toml` after the first successful fetch.

</details>
