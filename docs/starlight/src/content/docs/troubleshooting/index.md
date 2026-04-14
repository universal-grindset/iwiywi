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

Happy reading!
