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
