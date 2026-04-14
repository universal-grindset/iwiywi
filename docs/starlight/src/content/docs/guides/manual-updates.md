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
