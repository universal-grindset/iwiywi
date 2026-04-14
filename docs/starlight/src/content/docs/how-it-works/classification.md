---
title: AI Classification
description: How each reading is tagged to a Step
---

# AI Classification

After fetching 12 readings, iwiywi uses AI to classify each one to its most relevant AA Step (1–12).

## The Process

1. **Send to Claude** — iwiywi sends each reading to Vercel AI Gateway (OpenAI-compatible endpoint)
2. **System prompt** — Claude receives a prompt: *"You are an AA step classifier. Which step does this reading relate to? Respond with JSON: {step: 1-12, reason: '...'}."*
3. **Classify** — Claude analyzes the reading text and returns a step number
4. **Reason** — Claude includes a one-sentence reason (e.g., "Surrender and trust in a Higher Power")
5. **Store** — Classification is saved alongside the reading

## Example

**Reading:**
> "Made a decision to turn our will and our lives over to the care of God as we understood Him."

**Classification:**
```json
{
  "step": 3,
  "reason": "Surrender and trust in a Higher Power"
}
```

In your TUI, you'd see:
```
▌ Step 3 · AA.org
  Made a decision to turn our will and our lives...
```

## Why AI?

- **Consistent tagging** — All 12 readings tagged the same way (no manual effort)
- **Fast** — Parallel processing; classifications happen simultaneously
- **Flexible** — Can handle diverse source formats and writing styles
- **Reason included** — Each classification has a brief explanation

## Model

iwiywi uses **Claude Haiku** via Vercel AI Gateway. Haiku is:

- Fast (milliseconds per classification)
- Accurate (understands AA Step semantics)
- Cost-effective

The model is configurable (in `~/.iwiywi/config.toml`), but Haiku is the default and recommended.

## Token Cost

Each reading costs ~50 tokens to classify. 12 readings ≈ 600 tokens per day. Vercel AI Gateway pricing applies (usually $0.03/1M input tokens).

**Bottom line:** Negligible cost; free tier usually covers daily usage.

## Limitations

- **Variability** — Different readings can be classified differently on different days (normal AI behavior)
- **Interpretation** — Classification is Claude's best guess; not authoritative AA doctrine
- **Edge cases** — Some readings span multiple steps; Claude picks the primary one

This is intentional. The tool is a guide, not a rule.

---

Next: Learn how the mobile page is [deployed](deployment).
