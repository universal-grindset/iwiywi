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
