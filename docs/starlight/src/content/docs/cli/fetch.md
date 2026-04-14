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
