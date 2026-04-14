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
