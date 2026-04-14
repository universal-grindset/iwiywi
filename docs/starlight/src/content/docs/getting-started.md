---
title: Installation & Setup
description: Get iwiywi running in two steps
---

# Getting Started

## Prerequisites

- **macOS 12 or later** (Apple Silicon or Intel)
- **Homebrew** — [Install here](https://brew.sh) if you don't have it
- **Internet connection** — For fetching readings daily

## Installation

### Step 1: Install via Homebrew

```bash
brew install universal-grindset/iwiywi/iwiywi
```

Verify installation:

```bash
iwiywi --version
```

### Step 2: Run the Setup

```bash
iwiywi install
```

This does three things:
1. Creates `~/.iwiywi/` directory for readings and config
2. Generates a launchd plist for daily 6am runs
3. Loads the scheduler so fetches start tomorrow morning

You'll see:
```
✓ Created ~/.iwiywi/
✓ Installed launchd plist
✓ Scheduler loaded. First fetch: 2026-04-15 at 06:00
```

## Your First Run

Open iwiywi immediately (don't wait until 6am):

```bash
iwiywi
```

You'll see a TUI with today's readings. Each is tagged with its Step number (1–12) and source.

### Navigation

- `↑ / ↓` or `j / k` — Scroll up/down
- `/` — Enter command mode (try `/qr` + Enter for QR code)
- `r` — Manually trigger a fetch
- `q` — Quit

Readings update at 6am automatically. You can also fetch manually anytime with `iwiywi fetch`.

## What Happens Next?

- **6am tomorrow** — Scheduler runs, fetches 12 new readings, classifies them
- **Anytime** — Open `iwiywi` to browse today's readings
- **Phone access** — Use `/qr` in TUI to get QR code, scan on iPhone for mobile view

## Troubleshooting

**Nothing happens when I run `iwiywi`?**
- Check logs: `tail -f ~/.iwiywi/fetch.log`
- See [Troubleshooting](../troubleshooting/index)

**Readings not updating?**
- See [Launchd not running](../troubleshooting/index#launchd-not-running)

**Need help?**
- Browse [Guides](../guides/first-day) or [How It Works](../how-it-works/index)
- [Report an issue](https://github.com/universal-grindset/iwiywi/issues)

---

Next: Read about each [CLI command](../cli/index).
