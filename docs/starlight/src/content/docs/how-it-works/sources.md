---
title: Reading Sources
description: Where the 12 daily readings come from
---

# Reading Sources

iwiywi fetches from 12 AA sources daily. Each source is trusted, AA-approved content.

## The 12 Sources

1. **AA.org Daily Reflections** — aa.org/daily-reflections
2. **Hazelden Betty Ford** — hazeldenbettyford.org/thought-for-the-day
3. **AA Happy Hour** — aahappyhour.com/aa-daily-readings
4. **Google Search** — Top featured snippet for "aa thought for the day"
5. **Reddit** — r/alcoholicsanonymous daily thread
6. **Silkworth.net**
7. **AA Online Meeting** — aaonlinemeeting.net
8. **AA Big Book** — aabigbook.com
9. **Recovering Courage**
10. **One Day At A Time** — odat.us
11. **Joe and Charlie** — joeancharlie.com (A Program for You)
12. **AA History** — aahistory.com or equivalent

## How Fetching Works

At 6am, iwiywi:

1. **Sends HTTP requests** to each source
2. **Parses HTML** to extract the reading text (and title, URL)
3. **Stores raw readings** temporarily
4. **Skips failures silently** — If a source is down, it's logged but doesn't block the process
5. **Continues with available readings** — If 11 sources work, you get 11 readings (not a hard 12)

Each reading contains:

- **Source name** — e.g., "AA.org"
- **Title** — e.g., "Daily Reflections"
- **Text** — The actual reading (usually 100–300 words)
- **URL** — Link to the original source

## Source Selection Philosophy

These sources were chosen because:

- ✅ **AA-approved** — No outside interpretation, pure AA content
- ✅ **Stable URLs** — Don't move or disappear frequently
- ✅ **Accessible** — Can be scraped programmatically (no login walls)
- ✅ **Fresh** — Updated daily
- ✅ **Diverse** — Range of AA literature (Big Book, historical, contemporary)

## What Happens If a Source Goes Down?

iwiywi logs the failure and continues. You might get 11 readings instead of 12. This is intentional — **robustness over perfection.**

Check logs for details:

```bash
tail -f ~/.iwiywi/fetch.log
```

Look for messages like: `[WARN] AA.org: timeout (skipped)`

---

Next: Learn how readings are [classified to Steps](classification).
