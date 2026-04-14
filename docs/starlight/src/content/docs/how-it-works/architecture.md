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
