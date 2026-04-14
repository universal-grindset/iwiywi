# iwiywi Documentation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create user-focused documentation: repo-level docs (README, CONTRIBUTING, LICENSE, CHANGELOG) + Astro Starlight site at iwiywi.dev with install guides, CLI reference, how-it-works explainers, and troubleshooting.

**Architecture:** Two-tier docs. GitHub repo has minimal, discoverable docs; Astro Starlight (deployed via GH Actions to Vercel at iwiywi.dev) contains detailed guides and references. Both cross-link. Content hides internals; focuses on what the tool does and how to use it.

**Tech Stack:** Markdown, Astro Starlight (Astro framework), GitHub Actions, Vercel, YAML

---

## Task 1: Create Repo-Level Documentation (README, CONTRIBUTING, LICENSE, CHANGELOG)

**Files:**
- Create: `docs/README.md`
- Create: `docs/CONTRIBUTING.md`
- Create: `docs/LICENSE`
- Create: `docs/CHANGELOG.md`

- [ ] **Step 1: Create docs/ directory**

```bash
mkdir -p docs
```

- [ ] **Step 2: Write docs/README.md**

```markdown
# iwiywi — It Works If You Work It

Daily AA readings in your terminal.

## Install

```bash
brew install universal-grindset/iwiywi/iwiywi
```

## Quick Start

```bash
iwiywi install    # Set up launchd (runs at 6am daily)
iwiywi            # Open the TUI (reads today's readings)
```

## Features

- **12 AA daily readings** — Aggregated from trusted sources
- **Step classification** — Each reading linked to AA Steps 1–12
- **TUI** — Scrollable, colored by step, interactive
- **Mobile view** — QR code in TUI links to mobile-friendly page
- **Automatic** — Runs daily at 6am via launchd

## Commands

- `iwiywi` — Open TUI
- `iwiywi fetch` — Manually refresh readings
- `iwiywi install` — Set up daily scheduler
- `--help` — See all options

## Docs & Support

Full documentation: https://iwiywi.dev

Report bugs: [GitHub Issues](https://github.com/universal-grindset/iwiywi/issues)

## License

MIT — See LICENSE for details
```

- [ ] **Step 3: Write docs/CONTRIBUTING.md**

```markdown
# Contributing to iwiywi

We welcome bug reports and feedback.

## Reporting Bugs

Open an issue: https://github.com/universal-grindset/iwiywi/issues

Please include:
- What you were doing when it happened
- What went wrong
- Your macOS version (output of `sw_vers`)
- Output of `iwiywi --version`

## Feature Requests

Open an issue with label `enhancement`. Describe what you'd like and why.

## Code of Conduct

Be respectful. This is an AA-inspired tool — kind community only.
```

- [ ] **Step 4: Write docs/LICENSE**

```
MIT License

Copyright (c) 2026 universal-grindset

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

- [ ] **Step 5: Write docs/CHANGELOG.md**

```markdown
# Changelog

All notable changes to iwiywi are documented here.

## [Unreleased]

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
```

- [ ] **Step 6: Commit repo docs**

```bash
git add docs/README.md docs/CONTRIBUTING.md docs/LICENSE docs/CHANGELOG.md
git commit -m "docs: add repo-level documentation (README, CONTRIBUTING, LICENSE, CHANGELOG)"
```

---

## Task 2: Initialize Astro Starlight Project

**Files:**
- Create: `docs/starlight/astro.config.mjs`
- Create: `docs/starlight/package.json`
- Create: `docs/starlight/tsconfig.json`
- Create: `docs/starlight/.env.example`
- Create: `docs/starlight/.gitignore`
- Create: `docs/starlight/src/` directory structure

- [ ] **Step 1: Create docs/starlight/ directory and basic structure**

```bash
mkdir -p docs/starlight/{src/content/docs/{cli,guides,how-it-works,troubleshooting},src/assets}
```

- [ ] **Step 2: Write docs/starlight/package.json**

```json
{
  "name": "iwiywi-docs",
  "version": "1.0.0",
  "description": "iwiywi documentation site",
  "type": "module",
  "scripts": {
    "dev": "astro dev",
    "build": "astro build",
    "preview": "astro preview"
  },
  "dependencies": {
    "astro": "^4.0.0",
    "starlight": "^0.18.0"
  }
}
```

- [ ] **Step 3: Write docs/starlight/astro.config.mjs**

```javascript
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  integrations: [
    starlight({
      title: 'iwiywi',
      description: 'Daily AA readings in your terminal',
      favicon: '/favicon.ico',
      social: {
        github: 'https://github.com/universal-grindset/iwiywi',
      },
      sidebar: [
        {
          label: 'Getting Started',
          items: [
            { label: 'Installation', slug: 'getting-started' },
          ],
        },
        {
          label: 'CLI Reference',
          items: [
            { label: 'Overview', slug: 'cli/index' },
            { label: 'iwiywi (TUI)', slug: 'cli/iwiywi' },
            { label: 'fetch', slug: 'cli/fetch' },
            { label: 'install', slug: 'cli/install' },
          ],
        },
        {
          label: 'Guides',
          items: [
            { label: 'Your First Day', slug: 'guides/first-day' },
            { label: 'Mobile Access', slug: 'guides/mobile-access' },
            { label: 'Manual Updates', slug: 'guides/manual-updates' },
          ],
        },
        {
          label: 'How It Works',
          items: [
            { label: 'Overview', slug: 'how-it-works/index' },
            { label: 'Architecture', slug: 'how-it-works/architecture' },
            { label: 'Reading Sources', slug: 'how-it-works/sources' },
            { label: 'AI Classification', slug: 'how-it-works/classification' },
            { label: 'Vercel Deployment', slug: 'how-it-works/deployment' },
            { label: 'Daily Schedule', slug: 'how-it-works/schedule' },
          ],
        },
        {
          label: 'Troubleshooting',
          items: [
            { label: 'FAQ', slug: 'troubleshooting/index' },
          ],
        },
      ],
      customCss: [],
    }),
  ],
});
```

- [ ] **Step 4: Write docs/starlight/tsconfig.json**

```json
{
  "extends": "astro/tsconfigs/strict",
  "compilerOptions": {
    "jsx": "react-jsx",
    "jsxImportSource": "react"
  }
}
```

- [ ] **Step 5: Write docs/starlight/.env.example**

```
# Vercel deployment (optional — used during build)
# VERCEL_TOKEN=your_vercel_token_here
# VERCEL_PROJECT_ID=prj_xxxxx
```

- [ ] **Step 6: Write docs/starlight/.gitignore**

```
# Dependencies
node_modules/
package-lock.json
pnpm-lock.yaml

# Build output
dist/
.astro/

# Environment variables
.env
.env.local
.env.*.local

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db
```

- [ ] **Step 7: Commit Starlight project scaffolding**

```bash
git add docs/starlight/
git commit -m "docs: initialize Astro Starlight project structure"
```

---

## Task 3: Create Starlight Core Pages (Home, Getting Started)

**Files:**
- Create: `docs/starlight/src/content/docs/index.md`
- Create: `docs/starlight/src/content/docs/getting-started.md`

- [ ] **Step 1: Write docs/starlight/src/content/docs/index.md**

```markdown
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
```

- [ ] **Step 2: Write docs/starlight/src/content/docs/getting-started.md**

```markdown
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
```

- [ ] **Step 3: Commit core pages**

```bash
git add docs/starlight/src/content/docs/index.md docs/starlight/src/content/docs/getting-started.md
git commit -m "docs: add Starlight home and getting started pages"
```

---

## Task 4: Create CLI Reference Pages

**Files:**
- Create: `docs/starlight/src/content/docs/cli/index.md`
- Create: `docs/starlight/src/content/docs/cli/iwiywi.md`
- Create: `docs/starlight/src/content/docs/cli/fetch.md`
- Create: `docs/starlight/src/content/docs/cli/install.md`

- [ ] **Step 1: Write docs/starlight/src/content/docs/cli/index.md**

```markdown
---
title: CLI Commands
description: Complete reference for iwiywi commands
---

# CLI Reference

iwiywi has three main commands. All are optional subcommands to `iwiywi`:

| Command | Purpose |
|---------|---------|
| `iwiywi` | Open the TUI (default) |
| `iwiywi fetch` | Manually refresh readings |
| `iwiywi install` | Set up launchd scheduler |

## Global Flags

All commands support:

- `--help` — Show command help
- `--version` — Show iwiywi version

## Subcommands

- **[iwiywi (TUI)](iwiywi)** — Interactive terminal display
- **[fetch](fetch)** — Manual fetch and classify
- **[install](install)** — Scheduler setup

---

See each command for details, examples, and troubleshooting.
```

- [ ] **Step 2: Write docs/starlight/src/content/docs/cli/iwiywi.md**

```markdown
---
title: iwiywi (TUI)
description: Open the interactive terminal UI
---

# iwiywi — Terminal User Interface

```bash
iwiywi
```

Opens the interactive TUI showing today's readings.

## What You'll See

- **Colored feed** — Each reading has a colored accent bar matching its AA Step (1–12)
- **Step + Source** — e.g., "Step 3 · AA.org"
- **Reading text** — Full excerpt, wrappable
- **Navigation** — Scroll, search, commands at bottom

## Keybindings

| Key | Action |
|-----|--------|
| `↑ / ↓` | Scroll up/down |
| `j / k` | Vim-style scroll |
| `/` | Enter command mode |
| `r` | Manually run fetch |
| `q` | Quit |
| `Esc` | Exit command mode or QR overlay |

## Command Mode

Press `/` to enter command mode (you'll see a `>` prompt at the bottom).

### Available Commands

#### `qr`
Display QR code overlay (scan with phone to open mobile view). Press `Esc` to close.

```
/qr
(shows QR overlay)
```

#### `help`
(For future use)

Unknown commands are silently ignored.

## Examples

**Scroll to the top:**
```
Press: ↑ (or k) multiple times
```

**See QR code on your phone:**
```
Press: / → type qr → Enter
(overlay appears, scan with iPhone)
Press: Esc to close
```

**Manually refresh readings:**
```
Press: r
(fetches latest, updates display)
```

**Quit:**
```
Press: q
```

## Colors (Step Mapping)

Each step has a distinct color:

| Step | Color | Step | Color |
|------|-------|------|-------|
| 1 | Red | 7 | Light Red |
| 2 | Yellow | 8 | Light Yellow |
| 3 | Blue | 9 | Light Blue |
| 4 | Magenta | 10 | Light Magenta |
| 5 | Cyan | 11 | Light Cyan |
| 6 | Green | 12 | Light Green |

## Troubleshooting

**TUI won't open?**
- Try: `iwiywi --version` to verify install
- Check logs: `tail -f ~/.iwiywi/fetch.log`

**No readings showing?**
- Fetch hasn't run yet (first run is at 6am). Try: `iwiywi fetch`
- See [Readings not updating](../troubleshooting/index#readings-not-updating)

**QR code not working?**
- Ensure you're scanning with iPhone camera or QR app
- URL should be an iwiywi.vercel.app link

---

Next: Learn about [manual fetching](fetch).
```

- [ ] **Step 3: Write docs/starlight/src/content/docs/cli/fetch.md**

```markdown
---
title: fetch Command
description: Manually fetch and classify readings
---

# iwiywi fetch

```bash
iwiywi fetch
```

Manually fetch 12 AA readings from sources, classify each to Steps 1–12, and save to cache.

**Normally runs automatically at 6am.** Use this to refresh outside the schedule.

## When to Use

- **Test right after install** — Verify everything works
- **Manually refresh** — Get fresh readings anytime (or press `r` in TUI)
- **Troubleshoot** — See logs to diagnose issues
- **Script/cron** — Can be called from your own automation

## Usage

```bash
iwiywi fetch
```

## Output

On success:

```
Fetching 12 readings...
Classifying with AI...
Deploying to Vercel...
✓ 12 readings saved to ~/.iwiywi/readings-2026-04-14.json
✓ Mobile page deployed to https://iwiywi.vercel.app
```

Each reading is classified to a Step 1–12 with a brief reason.

## Logs

All fetch activity is logged to:

```
~/.iwiywi/fetch.log
```

View recent logs:

```bash
tail -f ~/.iwiywi/fetch.log
```

Logs include:
- Fetch start/end time
- Source fetch status (success/fail per source)
- AI classification results
- Vercel deploy status

## Troubleshooting

**Fetch hangs or times out?**
- Vercel AI Gateway may be slow. Try again in a moment.
- Check internet: `ping 8.8.8.8`

**"Failed to classify"?**
- Vercel API issue. Logs will show the error.
- Try manual fetch in a few minutes.

**"Deploy failed"?**
- Check Vercel token: `vercel env pull ~/.iwiywi/.env`
- Verify token is set: `cat ~/.iwiywi/.env`
- See [Troubleshooting](../troubleshooting/index)

---

Next: Learn about [install and scheduling](install).
```

- [ ] **Step 4: Write docs/starlight/src/content/docs/cli/install.md**

```markdown
---
title: install Command
description: Set up automatic daily fetching
---

# iwiywi install

```bash
iwiywi install
```

One-time setup command. Creates and loads a macOS launchd job so iwiywi fetches at 6am every morning.

## What It Does

1. **Creates** `~/.iwiywi/` directory (if missing)
2. **Generates** launchd plist: `~/Library/LaunchAgents/com.iwiywi.fetch.plist`
3. **Loads** the plist with `launchctl`
4. **Verifies** scheduler is active

## Output

```
✓ Created ~/.iwiywi/
✓ Installed launchd plist at ~/Library/LaunchAgents/com.iwiywi.fetch.plist
✓ Scheduler loaded and active
✓ First fetch: 2026-04-15 at 06:00 (local time)
```

## Schedule

- **Time**: 6am every day
- **Timezone**: Your system's local timezone (no UTC conversion needed)
- **Frequency**: Daily

To change the time, edit the plist manually:

```bash
plutil -p ~/Library/LaunchAgents/com.iwiywi.fetch.plist
# Look for <key>StartCalendarInterval</key>
# Edit Hour and Minute values, then reload:
launchctl unload ~/Library/LaunchAgents/com.iwiywi.fetch.plist
launchctl load ~/Library/LaunchAgents/com.iwiywi.fetch.plist
```

## Logs

Fetch logs are saved to:

```
~/.iwiywi/fetch.log
```

View recent activity:

```bash
tail -f ~/.iwiywi/fetch.log
```

## Uninstall

To remove the scheduler:

```bash
launchctl unload ~/Library/LaunchAgents/com.iwiywi.fetch.plist
rm ~/Library/LaunchAgents/com.iwiywi.fetch.plist
```

(Readings cache stays in `~/.iwiywi/` for backup.)

## Troubleshooting

**"Permission denied" during install?**
- Normal macOS permission prompt. Click "Allow."

**Scheduler isn't running at 6am?**
- Check if loaded: `launchctl list | grep iwiywi`
- Should show `com.iwiywi.fetch`
- If missing, run `iwiywi install` again

**Readings not updating?**
- See [Launchd not running](../troubleshooting/index#launchd-not-running)

---

Next: Explore [Guides](../guides/first-day) for walkthroughs.
```

- [ ] **Step 5: Commit CLI reference**

```bash
git add docs/starlight/src/content/docs/cli/
git commit -m "docs: add CLI reference (iwiywi, fetch, install commands)"
```

---

## Task 5: Create Guides (First Day, Mobile Access, Manual Updates)

**Files:**
- Create: `docs/starlight/src/content/docs/guides/first-day.md`
- Create: `docs/starlight/src/content/docs/guides/mobile-access.md`
- Create: `docs/starlight/src/content/docs/guides/manual-updates.md`

- [ ] **Step 1: Write docs/starlight/src/content/docs/guides/first-day.md**

```markdown
---
title: Your First Day
description: A walkthrough of your first iwiywi experience
---

# Your First Day with iwiywi

Let's walk through your first experience with iwiywi, from install to scanning on your phone.

## Step 1: Install (5 min)

```bash
brew install universal-grindset/iwiywi/iwiywi
```

Verify:

```bash
iwiywi --version
# iwiywi 0.1.0
```

## Step 2: Set Up Scheduler (1 min)

```bash
iwiywi install
```

You'll see:

```
✓ Created ~/.iwiywi/
✓ Installed launchd plist
✓ Scheduler loaded. First fetch: 2026-04-15 at 06:00
```

**This is a one-time setup.** From tomorrow, readings arrive automatically every morning.

## Step 3: Open the TUI (1 min)

```bash
iwiywi
```

You'll see a terminal UI with today's readings in a colored feed. Each reading:

- Has a **colored bar** (left side) matching its AA Step
- Shows **Step + Source** (e.g., "Step 3 · AA.org")
- Displays the **reading text**

Scroll with arrow keys (`↑ / ↓`) or vim keys (`j / k`).

## Step 4: See Your Phone

Press `/` to enter command mode:

```
/ qr [Enter]
```

A QR code will appear as a colored ASCII art overlay. Grab your phone and:

1. Open Camera app (or any QR scanner)
2. Point at the screen
3. Tap the notification that appears
4. Your phone opens the mobile view

The mobile page shows the same readings in a dark, scrollable format — perfect for reading on the couch.

Press `Esc` to close the QR code.

## Step 5: Explore Navigation

In the TUI, try:

- **Scroll**: Press `↑` or `↓` (or `j` / `k`)
- **Refresh**: Press `r` to fetch new readings right now
- **Quit**: Press `q`

That's it!

## Tomorrow (and Beyond)

At 6am, the scheduler automatically:

1. Fetches 12 readings from AA sources
2. Classifies each to a Step
3. Saves to your local cache
4. Deploys to Vercel (for mobile access)

When you open `iwiywi` after 6am, today's readings are waiting. Readings are fresh daily.

## Next Steps

- **[CLI Reference](../cli/index)** — All commands and options
- **[How It Works](../how-it-works/index)** — Understand the system
- **[Troubleshooting](../troubleshooting/index)** — If something goes wrong

---

Enjoy your daily readings! 📖
```

- [ ] **Step 2: Write docs/starlight/src/content/docs/guides/mobile-access.md**

```markdown
---
title: Mobile Access
description: View readings on your iPhone
---

# Mobile Access via QR Code

One of iwiywi's features is the mobile view — scan a QR code from your TUI and instantly browse readings on your phone.

## How It Works

The QR code in your TUI encodes a stable URL (hosted on Vercel). Scan it with your iPhone camera to open the mobile page. No special app needed.

## Step by Step

### In the TUI:

```bash
iwiywi
```

Press `/` to enter command mode, then type `qr`:

```
/ qr [Enter]
```

A QR code appears as a colored ASCII overlay.

### On Your iPhone:

1. **Open Camera** (native app)
2. **Point at screen** (at the QR code)
3. **Tap notification** (when iPhone detects the code)
4. **Opens mobile view** in Safari

The page is dark, scrollable, and mobile-optimized.

### In the Mobile View:

- **Scroll** to browse readings
- **Back** to return (or use Safari back)
- **Refresh** for latest (press refresh in Safari if needed)

## What's on the Mobile Page

- Same 12 readings as your TUI
- Each with Step number, source, and full text
- Dark background (easy on eyes)
- No JavaScript required (fast, lightweight)
- Updated daily at 6am (when scheduler runs)

## Sharing with Others

The QR code is personal to your iwiywi instance. Others scanning it see your Vercel URL. If you want to share readings with a friend, you can share the direct link:

```
https://iwiywi.vercel.app
```

They'll see today's readings (the URL is updated daily, not per-user).

## Troubleshooting

**QR code won't scan?**
- Make sure your TUI text is crisp (not too zoomed out)
- Try adjusting terminal font size
- Some fonts don't render well as QR codes; use a monospace font

**Mobile page is blank?**
- First fetch hasn't happened yet. Run: `iwiywi fetch`
- Check connection: Safari should load the Vercel page

**QR overlay won't open in TUI?**
- Try: Press `Esc` first, then `/` again
- See [Troubleshooting](../troubleshooting/index)

---

Next: [Manual Updates](manual-updates)
```

- [ ] **Step 3: Write docs/starlight/src/content/docs/guides/manual-updates.md**

```markdown
---
title: Manual Updates
description: Refresh readings outside the 6am schedule
---

# Manual Refresh

By default, iwiywi fetches new readings at 6am every morning. But you can refresh anytime with a single command.

## Use Cases

- **Just installed** — Test that everything works
- **Want fresh readings now** — No need to wait for 6am
- **In a meeting** — Get readings for that moment
- **Troubleshooting** — Check if the fetch process works

## How To

### Option 1: Press `r` in the TUI

Open iwiywi and press `r`:

```bash
iwiywi
# [TUI opens]
# Press: r
# Fetches, updates display automatically
```

Takes ~10 seconds. You'll see the feed refresh with new readings.

### Option 2: Command Line

```bash
iwiywi fetch
```

Manually runs the full fetch pipeline:

1. Scrapes 12 AA sources
2. Classifies each reading to a Step
3. Saves locally
4. Deploys to Vercel (mobile page)

Output:

```
Fetching 12 readings...
Classifying with AI...
Deploying to Vercel...
✓ 12 readings saved to ~/.iwiywi/readings-2026-04-14.json
✓ Mobile page deployed
```

## Logs

Every fetch is logged to:

```
~/.iwiywi/fetch.log
```

View recent activity:

```bash
tail -f ~/.iwiywi/fetch.log
```

This is useful for debugging if something seems off.

## Frequency

**Don't worry about fetching too often.** The system is designed to handle it. Refresh as much as you want.

(Behind the scenes, Vercel caches the deploy, so repeated deploys are fast.)

## Troubleshooting

**Fetch times out?**
- Vercel AI Gateway may be busy. Try again in a minute.

**"Failed to classify"?**
- Network issue or API downtime. Logs will show more detail.
- Try: `iwiywi fetch` again

**Mobile page not updating?**
- Wait 10 seconds (deploy takes a moment)
- Refresh mobile Safari

---

Next: [How It Works](../how-it-works/index) to understand the system.
```

- [ ] **Step 4: Commit guides**

```bash
git add docs/starlight/src/content/docs/guides/
git commit -m "docs: add guides (first day, mobile access, manual updates)"
```

---

## Task 6: Create How-It-Works Section (Peek Behind the Curtain)

**Files:**
- Create: `docs/starlight/src/content/docs/how-it-works/index.md`
- Create: `docs/starlight/src/content/docs/how-it-works/architecture.md`
- Create: `docs/starlight/src/content/docs/how-it-works/sources.md`
- Create: `docs/starlight/src/content/docs/how-it-works/classification.md`
- Create: `docs/starlight/src/content/docs/how-it-works/deployment.md`
- Create: `docs/starlight/src/content/docs/how-it-works/schedule.md`

- [ ] **Step 1: Write docs/starlight/src/content/docs/how-it-works/index.md**

```markdown
---
title: How It Works
description: Peek behind the curtain — understand what iwiywi does
---

# How It Works

iwiywi is a simple system with a few moving parts. This section explains what happens behind the scenes without diving into code internals.

**Sections:**

- **[Architecture](architecture)** — System overview, data flow
- **[Reading Sources](sources)** — Where the 12 readings come from
- **[AI Classification](classification)** — How readings are tagged to Steps
- **[Vercel Deployment](deployment)** — How the mobile page is published
- **[Daily Schedule](schedule)** — How launchd triggers automatic fetches

## The Big Picture (30 seconds)

Every morning at 6am:

1. **Fetch** — Scrape 12 AA sources
2. **Classify** — Tag each reading to a Step 1–12 using AI
3. **Deploy** — Push a mobile-optimized page to Vercel
4. **Save** — Store readings locally for offline access

Users browse via TUI (terminal) or mobile (QR code).

---

Start with [Architecture](architecture) to understand the flow.
```

- [ ] **Step 2: Write docs/starlight/src/content/docs/how-it-works/architecture.md**

```markdown
---
title: Architecture Overview
description: High-level system design
---

# Architecture Overview

Here's how iwiywi flows from fetch to user display.

## Daily Flow (6am)

```
6:00 AM
  ↓
Fetch Task (launchd triggers)
  ├─ Scrape 12 AA sources
  ├─ Parse HTML, extract reading text
  ├─ Store raw readings
  ↓
AI Classification (Vercel AI Gateway)
  ├─ Send each reading to Claude via API
  ├─ Classify to Step 1–12
  ├─ Receive classification + reason
  ↓
Save to Cache
  ├─ Write JSON to ~/.iwiywi/readings-YYYY-MM-DD.json
  ├─ Each reading: step, source, text, classification reason
  ↓
Generate Mobile Page
  ├─ Render HTML from cached readings
  ├─ Dark theme, responsive layout
  ↓
Deploy to Vercel
  ├─ Push HTML to Vercel
  ├─ Same URL updated daily
  ├─ QR code in TUI points to this URL
  ↓
Done (Logs saved)
```

## User Access

### Terminal (TUI)

```
User opens iwiywi
  ↓
Read ~/.iwiywi/readings-YYYY-MM-DD.json
  ↓
Display in colored feed (Step-colored accent bars)
  ↓
User scrolls, reads, presses /qr for mobile link
```

### Mobile

```
User scans QR code in TUI
  ↓
Opens Vercel URL (https://iwiywi.vercel.app)
  ↓
Browsable reading feed
  ↓
User bookmarks or shares URL
```

## Why This Design?

- **Local cache** — Readings stored offline; TUI works without internet
- **Vercel deploy** — Same stable URL daily; no versioning hassle
- **AI classification** — Consistent Step tagging via Claude
- **Launchd** — Native macOS scheduler (no extra app needed)
- **QR code** — Instant phone access from terminal

---

Next: Learn where the [12 sources](sources) come from.
```

- [ ] **Step 3: Write docs/starlight/src/content/docs/how-it-works/sources.md**

```markdown
---
title: Reading Sources
description: Where the 12 daily readings come from
---

# Reading Sources

iwiywi fetches from 12 AA sources daily. Each source is trusted, AA-approved content.

## The 12 Sources

1. **AA.org Daily Reflections** — aa.org/daily-reflections
2. **Hazelden Betty Ford** — hazeldenbettyford.org/thought-for-the-day
3. **AA Happy Hour** — aahappyhour.com/aa-daily-readings
4. **Google Search** — Top featured snippet for "aa thought for the day"
5. **Reddit** — r/alcoholicsanonymous daily thread
6. **Silkworth.net**
7. **AA Online Meeting** — aaonlinemeeting.net
8. **AA Big Book** — aabigbook.com
9. **Recovering Courage**
10. **One Day At A Time** — odat.us
11. **Joe and Charlie** — joeancharlie.com (A Program for You)
12. **AA History** — aahistory.com or equivalent

## How Fetching Works

At 6am, iwiywi:

1. **Sends HTTP requests** to each source
2. **Parses HTML** to extract the reading text (and title, URL)
3. **Stores raw readings** temporarily
4. **Skips failures silently** — If a source is down, it's logged but doesn't block the process
5. **Continues with available readings** — If 11 sources work, you get 11 readings (not a hard 12)

Each reading contains:

- **Source name** — e.g., "AA.org"
- **Title** — e.g., "Daily Reflections"
- **Text** — The actual reading (usually 100–300 words)
- **URL** — Link to the original source

## Source Selection Philosophy

These sources were chosen because:

- ✅ **AA-approved** — No outside interpretation, pure AA content
- ✅ **Stable URLs** — Don't move or disappear frequently
- ✅ **Accessible** — Can be scraped programmatically (no login walls)
- ✅ **Fresh** — Updated daily
- ✅ **Diverse** — Range of AA literature (Big Book, historical, contemporary)

## What Happens If a Source Goes Down?

iwiywi logs the failure and continues. You might get 11 readings instead of 12. This is intentional — **robustness over perfection.**

Check logs for details:

```bash
tail -f ~/.iwiywi/fetch.log
```

Look for messages like: `[WARN] AA.org: timeout (skipped)`

---

Next: Learn how readings are [classified to Steps](classification).
```

- [ ] **Step 4: Write docs/starlight/src/content/docs/how-it-works/classification.md**

```markdown
---
title: AI Classification
description: How each reading is tagged to a Step
---

# AI Classification

After fetching 12 readings, iwiywi uses AI to classify each one to its most relevant AA Step (1–12).

## The Process

1. **Send to Claude** — iwiywi sends each reading to Vercel AI Gateway (OpenAI-compatible endpoint)
2. **System prompt** — Claude receives a prompt: *"You are an AA step classifier. Which step does this reading relate to? Respond with JSON: {step: 1-12, reason: '...'}."*
3. **Classify** — Claude analyzes the reading text and returns a step number
4. **Reason** — Claude includes a one-sentence reason (e.g., "Surrender and trust in a Higher Power")
5. **Store** — Classification is saved alongside the reading

## Example

**Reading:**
> "Made a decision to turn our will and our lives over to the care of God as we understood Him."

**Classification:**
```json
{
  "step": 3,
  "reason": "Surrender and trust in a Higher Power"
}
```

In your TUI, you'd see:
```
▌ Step 3 · AA.org
  Made a decision to turn our will and our lives...
```

## Why AI?

- **Consistent tagging** — All 12 readings tagged the same way (no manual effort)
- **Fast** — Parallel processing; classifications happen simultaneously
- **Flexible** — Can handle diverse source formats and writing styles
- **Reason included** — Each classification has a brief explanation

## Model

iwiywi uses **Claude Haiku** via Vercel AI Gateway. Haiku is:

- Fast (milliseconds per classification)
- Accurate (understands AA Step semantics)
- Cost-effective

The model is configurable (in `~/.iwiywi/config.toml`), but Haiku is the default and recommended.

## Token Cost

Each reading costs ~50 tokens to classify. 12 readings ≈ 600 tokens per day. Vercel AI Gateway pricing applies (usually $0.03/1M input tokens).

**Bottom line:** Negligible cost; free tier usually covers daily usage.

## Limitations

- **Variability** — Different readings can be classified differently on different days (normal AI behavior)
- **Interpretation** — Classification is Claude's best guess; not authoritative AA doctrine
- **Edge cases** — Some readings span multiple steps; Claude picks the primary one

This is intentional. The tool is a guide, not a rule.

---

Next: Learn how the mobile page is [deployed](deployment).
```

- [ ] **Step 5: Write docs/starlight/src/content/docs/how-it-works/deployment.md**

```markdown
---
title: Vercel Deployment
description: How the mobile page is published
---

# Vercel Deployment

After classifying readings, iwiywi generates a mobile-optimized HTML page and deploys it to Vercel.

## The Process

1. **Generate HTML** — iwiywi renders the 12 readings into a static HTML file
2. **Push to Vercel** — Upload to a Vercel project
3. **Stable URL** — Vercel assigns a permanent URL (https://iwiywi.vercel.app)
4. **Daily update** — Same URL, updated with today's readings

## The Mobile Page

**Design:**
- Dark background (`#0d1117` — GitHub dark theme)
- Scrollable list of reading cards
- Each card: Step number badge, source, reading text
- Responsive (works on all screen sizes)
- No JavaScript (pure HTML + CSS; fast)

**Content:**
- Same 12 readings as your TUI
- Each with Step color, source name, and full text
- Stable across the day (updates at 6am)

**Example:**
```html
<div class="reading" data-step="3">
  <span class="step-badge">Step 3</span>
  <span class="source">AA.org</span>
  <p>Made a decision to turn our will...</p>
</div>
```

## How to Access

**From TUI:**
```bash
iwiywi
# Press: / → qr → Enter
# Scan QR code on your phone
```

**Direct URL:**
```
https://iwiywi.vercel.app
```

## Why Vercel?

- **Static hosting** — No backend needed; instant deploys
- **Global CDN** — Fast access worldwide
- **Free tier** — Usually covers personal usage
- **Stable URL** — Same URL daily; no versioning
- **Built-in auth** (optional) — Can password-protect if needed

## Deployment Details

- **Trigger** — After each successful fetch (6am)
- **Time** — ~5 seconds to deploy
- **Rollback** — Previous day's page stays live if deploy fails
- **Logs** — Visible in Vercel dashboard (optional)

## Customization

The URL is configured in `~/.iwiywi/config.toml`:

```toml
[vercel]
project_url = "https://iwiywi.vercel.app"
```

Advanced users can change the Vercel project; iwiywi respects your config.

---

Next: Learn about the [daily schedule](schedule).
```

- [ ] **Step 6: Write docs/starlight/src/content/docs/how-it-works/schedule.md**

```markdown
---
title: Daily Schedule
description: How launchd triggers automatic fetches
---

# Daily Schedule

iwiywi runs automatically at 6am every morning via macOS launchd. Here's how it works.

## What is launchd?

**launchd** is macOS's native job scheduler. It runs tasks on a schedule (like cron) without needing a separate app or background process.

iwiywi uses launchd to trigger the fetch automatically.

## Setup

When you run `iwiywi install`, it:

1. Creates a plist file: `~/Library/LaunchAgents/com.iwiywi.fetch.plist`
2. Loads it with `launchctl`
3. Schedules the job

The plist specifies:
- **Time**: 6:00 AM
- **Frequency**: Every day
- **Command**: `iwiywi fetch`
- **Timezone**: Your system's local time (automatic)

## How It Runs

At 6am:

1. **launchd wakes up** (per the schedule)
2. **Runs** `iwiywi fetch` in the background
3. **Logs output** to `~/.iwiywi/fetch.log`
4. **Cleans up** (no terminal window opens)

Users don't need to be logged in; launchd runs the job as a background task.

## Logs

All fetch output (success, errors, source failures) is logged to:

```
~/.iwiywi/fetch.log
```

View logs:

```bash
tail -f ~/.iwiywi/fetch.log
```

Example log:
```
[2026-04-14 06:00:00] Fetch started
[2026-04-14 06:00:02] Fetching 12 sources...
[2026-04-14 06:00:05] AA.org: ✓
[2026-04-14 06:00:06] Reddit: timeout (skipped)
[2026-04-14 06:00:12] Classification: 11/11 success
[2026-04-14 06:00:15] Deploy: ✓
[2026-04-14 06:00:15] Fetch complete
```

## Changing the Time

To change from 6am to, say, 7am:

```bash
# Edit the plist
plutil -p ~/Library/LaunchAgents/com.iwiywi.fetch.plist
```

Look for `<key>Hour</key><integer>6</integer>` and change `6` to `7`.

Then reload:

```bash
launchctl unload ~/Library/LaunchAgents/com.iwiywi.fetch.plist
launchctl load ~/Library/LaunchAgents/com.iwiywi.fetch.plist
```

## Uninstall

To remove the scheduler:

```bash
launchctl unload ~/Library/LaunchAgents/com.iwiywi.fetch.plist
rm ~/Library/LaunchAgents/com.iwiywi.fetch.plist
```

Readings stay in `~/.iwiywi/` for backup.

## Timezone

launchd automatically uses your Mac's local timezone. No UTC conversion needed. If your system is set to PST, the job runs at 6am PST.

---

That's the system! Questions? Check [Troubleshooting](../troubleshooting/index).
```

- [ ] **Step 7: Commit how-it-works section**

```bash
git add docs/starlight/src/content/docs/how-it-works/
git commit -m "docs: add how-it-works section (architecture, sources, classification, deployment, schedule)"
```

---

## Task 7: Create Troubleshooting Section

**Files:**
- Create: `docs/starlight/src/content/docs/troubleshooting/index.md`

- [ ] **Step 1: Write docs/starlight/src/content/docs/troubleshooting/index.md**

```markdown
---
title: Troubleshooting & FAQ
description: Common issues and solutions
---

# Troubleshooting & FAQ

## Launchd Not Running

**Problem:** Scheduler doesn't seem to be running. Readings aren't updating at 6am.

**Solution:**

Check if launchd job is loaded:

```bash
launchctl list | grep iwiywi
```

Should show: `com.iwiywi.fetch`

If not, reload:

```bash
iwiywi install
```

Check logs for errors:

```bash
tail -f ~/.iwiywi/fetch.log
```

---

## Readings Not Updating

**Problem:** It's past 6am, but no new readings in the TUI.

**Solution:**

1. Verify launchd ran:

```bash
tail -f ~/.iwiywi/fetch.log
```

Look for a recent entry (within last hour).

2. Manually trigger fetch:

```bash
iwiywi fetch
```

If this works, launchd may have permission issues. Try: `iwiywi install` again.

3. Check date:

Readings are saved as `readings-YYYY-MM-DD.json`. Open your TUI — does it say today's date?

```bash
ls ~/.iwiywi/readings-*.json
```

---

## TUI Won't Open

**Problem:** `iwiywi` command fails or terminal stays blank.

**Solution:**

1. Verify install:

```bash
which iwiywi
# Should show: /usr/local/bin/iwiywi (or similar)

iwiywi --version
# Should print version
```

2. Check for errors:

```bash
iwiywi 2>&1
```

3. Reinstall:

```bash
brew uninstall iwiywi
brew install universal-grindset/iwiywi/iwiywi
iwiywi install
```

4. Check logs:

```bash
tail -f ~/.iwiywi/fetch.log
```

---

## Fetch Hangs or Times Out

**Problem:** `iwiywi fetch` or scheduled fetch takes forever or fails.

**Solution:**

1. Check internet:

```bash
ping 8.8.8.8
```

2. Check Vercel API status:

Vercel AI Gateway may be slow. Wait a few minutes and retry:

```bash
iwiywi fetch
```

3. View logs:

```bash
tail -f ~/.iwiywi/fetch.log
```

Look for timeouts or API errors.

4. Retry:

```bash
iwiywi fetch
```

Most timeouts are transient.

---

## "Failed to Classify" Error

**Problem:** Fetch fails at classification step.

**Solution:**

1. Check logs:

```bash
tail -f ~/.iwiywi/fetch.log
```

Look for error message (e.g., "API timeout", "invalid token").

2. Verify Vercel token:

```bash
cat ~/.iwiywi/.env | grep VERCEL_AI_GATEWAY_TOKEN
```

Should be non-empty.

3. Refresh token:

```bash
vercel env pull ~/.iwiywi/.env
```

4. Retry:

```bash
iwiywi fetch
```

---

## Mobile Page Not Loading

**Problem:** QR code scans, but page won't load on phone.

**Solution:**

1. Check connection:

Ensure phone is on WiFi or cellular.

2. Try direct URL:

```
https://iwiywi.vercel.app
```

Open in Safari. Does it load?

3. Check Vercel:

Log in to Vercel dashboard. Is the deployment active?

4. Refresh mobile Safari:

Pull down to refresh. Try again in a few seconds (deploy may be in progress).

5. Check logs:

```bash
tail -f ~/.iwiywi/fetch.log
```

Look for deploy errors.

---

## QR Code Won't Scan

**Problem:** iPhone camera doesn't recognize the QR code overlay.

**Solution:**

1. Verify crisp display:

- Zoom in your terminal window (⌘+)
- Ensure terminal is in full screen
- Use a monospace font (Monaco, Menlo)

2. Adjust text size:

```bash
# In Terminal: Cmd + or Cmd -
# Make text larger so QR code is clearer
```

3. Try different scanner:

- Use native iPhone Camera app (most reliable)
- Or third-party QR app from App Store

4. Manual fallback:

```bash
# Use the direct URL instead
https://iwiywi.vercel.app
```

---

## "Permission Denied" During Install

**Problem:** `iwiywi install` prompts for password or fails.

**Solution:**

This is normal. macOS asks for permission to install launchd jobs. Click "Allow" when prompted.

If it fails:

```bash
iwiywi install
```

Try again, and grant the permission prompt.

---

## How Do I Change the Fetch Time?

**Problem:** Want readings at 7am instead of 6am.

**Solution:**

1. Unload the job:

```bash
launchctl unload ~/Library/LaunchAgents/com.iwiywi.fetch.plist
```

2. Edit the plist:

```bash
plutil -p ~/Library/LaunchAgents/com.iwiywi.fetch.plist
```

Look for `<key>Hour</key><integer>6</integer>` and change `6` to `7`.

3. Reload:

```bash
launchctl load ~/Library/LaunchAgents/com.iwiywi.fetch.plist
```

Verify:

```bash
launchctl list | grep iwiywi
```

---

## How Do I Uninstall?

**Problem:** Want to remove iwiywi completely.

**Solution:**

1. Remove scheduler:

```bash
launchctl unload ~/Library/LaunchAgents/com.iwiywi.fetch.plist
rm ~/Library/LaunchAgents/com.iwiywi.fetch.plist
```

2. Remove app:

```bash
brew uninstall iwiywi
```

3. Remove cache (optional):

```bash
rm -rf ~/.iwiywi
```

---

## Still Stuck?

- Check `/etc/iwiywi/fetch.log` for detailed error messages
- Open an issue: [GitHub Issues](https://github.com/universal-grindset/iwiywi/issues)
- Include: iwiywi version, macOS version, logs, and what you were doing

---

Happy reading! 📖
```

- [ ] **Step 2: Commit troubleshooting**

```bash
git add docs/starlight/src/content/docs/troubleshooting/
git commit -m "docs: add troubleshooting & FAQ section"
```

---

## Task 8: Create GitHub Actions Deploy Workflow

**Files:**
- Create: `.github/workflows/deploy-docs.yml`

- [ ] **Step 1: Write .github/workflows/deploy-docs.yml**

```yaml
name: Deploy Docs

on:
  push:
    branches: [main]
    paths:
      - 'docs/starlight/**'
      - '.github/workflows/deploy-docs.yml'
  workflow_dispatch:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: cd docs/starlight && npm install

      - name: Build Starlight
        run: cd docs/starlight && npm run build

      - name: Deploy to Vercel
        uses: vercel/action@v6
        with:
          vercel-token: ${{ secrets.VERCEL_TOKEN }}
          vercel-org-id: ${{ secrets.VERCEL_ORG_ID }}
          vercel-project-id: ${{ secrets.VERCEL_PROJECT_ID }}
          working-directory: ./docs/starlight/dist
```

- [ ] **Step 2: Add Vercel secrets to GitHub**

```bash
# Manual step: In GitHub repo settings → Secrets and variables → Actions
# Add three secrets:
# 1. VERCEL_TOKEN (from Vercel account settings)
# 2. VERCEL_ORG_ID (from Vercel dashboard)
# 3. VERCEL_PROJECT_ID (from Vercel project settings)
```

Create a `.github/workflows/README-deploy-docs.md` to document this:

```bash
cat > .github/workflows/README-deploy-docs.md << 'EOF'
# Deploy Docs Workflow

## Setup

Before this workflow runs, you must add Vercel secrets to GitHub:

1. Go to: GitHub repo → Settings → Secrets and variables → Actions
2. Add these secrets (get values from Vercel dashboard):
   - `VERCEL_TOKEN` — Personal access token from Vercel
   - `VERCEL_ORG_ID` — Organization ID
   - `VERCEL_PROJECT_ID` — Project ID for iwiywi.dev

## How It Works

On push to main (if docs/starlight/ changes):

1. Checkout code
2. Install Node dependencies
3. Build Starlight site
4. Deploy to Vercel

Site goes live at: https://iwiywi.dev

## Testing Locally

```bash
cd docs/starlight
npm install
npm run dev
# Open http://localhost:3000
```
EOF
```

- [ ] **Step 3: Commit workflow**

```bash
git add .github/workflows/deploy-docs.yml .github/workflows/README-deploy-docs.md
git commit -m "ci: add GitHub Actions workflow for Starlight deploy"
```

---

## Task 9: Configure Vercel Domain and Test

**Files:**
- Modify: Vercel project settings (no file changes; manual steps)

- [ ] **Step 1: Create Vercel project for iwiywi.dev**

```bash
# Manual: Create project in Vercel dashboard
# 1. Go to vercel.com/new
# 2. Create new project (name: iwiywi)
# 3. Copy Project ID and Org ID
```

- [ ] **Step 2: Add domain iwiywi.dev**

```bash
# Manual: In Vercel project settings → Domains
# 1. Add domain: iwiywi.dev
# 2. Follow instructions to point DNS
```

- [ ] **Step 3: Add GitHub secrets**

```bash
# Manual: GitHub repo settings → Secrets and variables → Actions
# Add:
# VERCEL_TOKEN (from Vercel account settings → Tokens)
# VERCEL_ORG_ID (from Vercel dashboard)
# VERCEL_PROJECT_ID (from Vercel project settings)
```

- [ ] **Step 4: Test deploy workflow**

```bash
# Push a test change to docs/starlight/ to trigger the workflow
echo "<!-- test -->" >> docs/starlight/src/content/docs/index.md
git add docs/starlight/src/content/docs/index.md
git commit -m "test: trigger deploy workflow"
git push origin main

# Check GitHub Actions tab → deploy-docs workflow
# Wait ~3 minutes for deploy to complete
# Verify: https://iwiywi.dev loads successfully
```

- [ ] **Step 5: Revert test change**

```bash
git revert HEAD --no-edit
git push origin main
```

- [ ] **Step 6: Commit Vercel setup notes**

```bash
cat > docs/starlight/.vercel-setup.md << 'EOF'
# Vercel Setup for iwiywi.dev

## Initial Setup

1. **Create Vercel project:**
   - vercel.com/new
   - Project name: iwiywi
   - Copy Project ID and Org ID

2. **Add domain:**
   - Vercel dashboard → Project settings → Domains
   - Add: iwiywi.dev
   - Point DNS per Vercel instructions

3. **GitHub secrets:**
   - Repo settings → Secrets and variables → Actions
   - Add VERCEL_TOKEN, VERCEL_ORG_ID, VERCEL_PROJECT_ID

## Deploy Process

- Push to main → GitHub Actions builds Starlight
- Vercel receives deploy from actions/vercel
- Site goes live at https://iwiywi.dev

## Manual Deploy (if needed)

```bash
cd docs/starlight
vercel --prod
```
EOF

git add docs/starlight/.vercel-setup.md
git commit -m "docs: add Vercel setup guide"
```

---

## Task 10: Cross-Link Everything and Final Review

**Files:**
- No new files; verify existing pages link properly

- [ ] **Step 1: Verify repo README links to Starlight**

Open `docs/README.md` and confirm it has:

```markdown
Full docs: https://iwiywi.dev
```

- [ ] **Step 2: Verify Starlight footer has repo links**

Check `docs/starlight/src/content/docs/index.md` has footer with:

```markdown
[Report a bug](https://github.com/universal-grindset/iwiywi/issues) · 
[Source code](https://github.com/universal-grindset/iwiywi) · 
[Contribute](https://github.com/universal-grindset/iwiywi/blob/main/CONTRIBUTING.md)
```

- [ ] **Step 3: Verify internal page links**

Spot-check a few pages:

```bash
# Getting started should link to CLI reference
grep -r "cli/index" docs/starlight/src/content/docs/getting-started.md

# CLI pages should link to guides
grep -r "guides/first-day" docs/starlight/src/content/docs/cli/
```

- [ ] **Step 4: Proofread all content**

```bash
# Read through each page quickly for typos, jargon, clarity
# Fix any issues in-place

# Check for:
# - "install" has consistent tone with other pages
# - No internal jargon leaking through
# - Examples are accurate (versions, paths, commands)
```

- [ ] **Step 5: Verify build succeeds locally**

```bash
cd docs/starlight
npm install
npm run build

# Should complete without errors
# Output: dist/ directory with built HTML
```

- [ ] **Step 6: Final commit**

```bash
git add -A
git commit -m "docs: complete documentation structure (repo docs + Starlight + deploy)"
```

---

## Done Signal

Documentation is complete when:

✅ README in repo + CONTRIBUTING + LICENSE + CHANGELOG exist and are concise
✅ Astro Starlight project scaffolded in docs/starlight/
✅ All pages created: home, getting-started, CLI (3 commands), guides (3), how-it-works (5), troubleshooting
✅ Cross-links working (repo ↔ Starlight)
✅ GitHub Actions deploy workflow in place
✅ Vercel domain iwiywi.dev configured
✅ Build succeeds locally: `npm run build`
✅ Content proofread (user-friendly, no jargon)
✅ All commits made
