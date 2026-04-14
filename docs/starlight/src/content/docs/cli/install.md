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
