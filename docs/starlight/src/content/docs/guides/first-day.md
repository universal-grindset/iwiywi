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

Enjoy your daily readings!
