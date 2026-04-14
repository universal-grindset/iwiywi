---
title: iwiywi Documentation
description: Daily AA readings in your terminal
---

# iwiywi — It Works If You Work It

Welcome to iwiywi documentation. A simple, daily tool for AA readings on your Mac.

## What is iwiywi?

iwiywi aggregates 12 AA daily readings, classifies each into its relevant AA Step (1–12), and displays them in your terminal. Every morning at 6am, it fetches fresh readings. Open the TUI anytime to browse, or scan the QR code to view on your phone.

## Quick Links

- **[Get Started →](getting-started)** — Install and run iwiywi
- **[CLI Reference →](cli/index)** — Commands and options
- **[Guides →](guides/first-day)** — Walkthroughs and features
- **[How It Works →](how-it-works/index)** — Peek behind the curtain
- **[Troubleshooting →](troubleshooting/index)** — Common issues and fixes

## Features at a Glance

✨ **12 Daily Readings** — Trusted AA sources, fresh every morning

🤖 **Step Classification** — AI tags each reading to AA Steps 1–12

📱 **Mobile Access** — Scan QR code from TUI, view on iPhone

⏰ **Automatic** — Runs at 6am via macOS launchd

🌈 **Colored TUI** — Step-colored feed, easy to scan

## Requirements

- macOS 12+
- Homebrew
- ~5MB disk space

## Install

```bash
brew install universal-grindset/iwiywi/iwiywi
```

Then run:

```bash
iwiywi install
```

That's it. Readings arrive daily at 6am.

## First Steps

1. Run `iwiywi install` (one-time setup)
2. Open `iwiywi` to see today's readings
3. Press `/` → type `qr` → scan with your phone
4. Press `q` to quit

See [Your First Day](guides/first-day) for a full walkthrough.

---

**Questions?** Check [Troubleshooting](troubleshooting/index) or [report a bug](https://github.com/universal-grindset/iwiywi/issues).
