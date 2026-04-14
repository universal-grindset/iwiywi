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
