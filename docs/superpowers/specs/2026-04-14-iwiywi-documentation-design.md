# iwiywi — Documentation Design

*"It Works If You Work It" — API & User Docs*

Date: 2026-04-14

---

## Overview

A two-tier documentation strategy: **GitHub repo** for discoverability and quick reference, **Astro Starlight site** at `iwiywi.dev` as the main knowledge hub. Documentation is user-focused and minimal — install, run, use — with a "peek behind the curtain" section explaining what the tool does and why, but not exposing internals.

---

## Audience & Scope

**Primary audience:** End users (Mac) installing and operating iwiywi daily.

**What we document:**
- ✅ Installation (Homebrew)
- ✅ CLI commands and options
- ✅ Feature walkthroughs (fetch, TUI, mobile access, launchd scheduling)
- ✅ How it works at a high level (12 sources, AI classification, Vercel deploy, QR code)
- ✅ Troubleshooting and FAQ

**What we hide:**
- ❌ Implementation internals (scraper patterns, token storage, exact AI prompts)
- ❌ Architecture deep-dives
- ❌ Self-hosting or customization guides
- ❌ Development/contribution setup details (beyond "file bugs on GitHub")

---

## Two-Tier Structure

### **Tier 1: GitHub Repo (Discoverable)**

Files in repo root, visible on GitHub homepage:

| File | Purpose | Length |
|------|---------|--------|
| **README.md** | Install + quick start + feature overview | ~80 lines |
| **CONTRIBUTING.md** | Bug reports, how to contribute | ~60 lines |
| **LICENSE** | MIT boilerplate | Standard |
| **CHANGELOG.md** | Release notes (auto-generated) | Grows over time |

**Content philosophy:** Enough info for GitHub discovery; users click through to iwiywi.dev for depth.

---

### **Tier 2: Astro Starlight (iwiywi.dev)**

Main documentation hub, deployed via GitHub Actions. Content sections:

#### **Home** (`/`)
Landing page with features, quick links, call-to-action (install via Homebrew).

#### **Getting Started** (`/getting-started/`)
- Prerequisites (macOS, Homebrew)
- Installation command
- First run: `iwiywi install` and `iwiywi`
- Verification checklist

#### **CLI Reference** (`/cli/`)
Detailed command documentation:
- `iwiywi` (TUI) — keybindings, commands, navigation
- `iwiywi fetch` — manual refresh, output, troubleshooting
- `iwiywi install` — setup, what it does, uninstall
- Flags and options per command

#### **Guides** (`/guides/`)
Task-focused walkthroughs:
- "My First Day" — wake up, check readings in TUI
- "Mobile Access" — scan QR code, view on iPhone
- "Manual Updates" — force a fetch outside 6am
- "Customizing the Schedule" — change launchd time

#### **How It Works** (`/how-it-works/`)
Peek behind the curtain — accessible but detailed:
- **Architecture Overview** — What happens at 6am (fetch, classify, deploy)
- **12 Reading Sources** — Which sources are scraped, why they were chosen
- **AI Classification** — Each reading classified to Steps 1–12, how that works
- **Vercel Deployment** — How the mobile page gets published
- **Daily Schedule** — How launchd runs iwiywi every morning

*Philosophy: Explain WHAT and WHY, not implementation HOW or internals.*

#### **Troubleshooting** (`/troubleshooting/`)
FAQ and diagnostics:
- "Readings not updating" — launchd not running? Check logs
- "TUI won't open" — common startup issues
- "Mobile page not loading" — Vercel deploy issues
- "Launchd permission errors" — fixing uninstall/reinstall
- "Readings seem wrong" — classification variability

---

## Domain & Deployment

**Domain:** `iwiywi.dev` (Vercel-hosted, $13/year)

**File structure in repo:**
```
iwiywi/
├── docs/
│   ├── README.md              # Standard docs (concise)
│   ├── CONTRIBUTING.md
│   ├── LICENSE
│   ├── CHANGELOG.md
│   └── starlight/             # Astro Starlight project
│       ├── astro.config.mjs
│       ├── package.json
│       ├── tsconfig.json
│       └── src/
│           ├── content/
│           │   └── docs/      # Markdown pages
│           └── assets/        # Images, diagrams
```

**Build & Deploy:**
- GitHub Actions workflow (`.github/workflows/deploy-docs.yml`) watches `docs/starlight/` for changes
- On push to main: builds Starlight site, deploys to Vercel at iwiywi.dev
- Repo docs stay in GitHub (no separate build needed)

---

## Cross-Linking Strategy

**GitHub README** links to iwiywi.dev:
```markdown
## Full Documentation
See [iwiywi.dev](https://iwiywi.dev) for detailed guides, troubleshooting, and how it works.
```

**Starlight footer** links back to GitHub:
```
[Report a bug](https://github.com/universal-grindset/iwiywi/issues) · 
[Source code](https://github.com/universal-grindset/iwiywi) · 
[Contribute](https://github.com/universal-grindset/iwiywi/blob/main/CONTRIBUTING.md)
```

---

## Content Guidelines

### For All Pages:
- **User-focused:** Explain features from the user's perspective ("see today's readings" not "deserialize JSON")
- **Minimal:** Only what users need; hide internals
- **Practical:** Include examples, commands, screenshots where helpful
- **Friendly:** Tone matches AA-inspired philosophy (accessible, supportive)

### For "How It Works":
- **Accessible:** Assume user knows macOS and Homebrew, not software architecture
- **High-level:** Explain data flow without code or implementation details
- **Honest:** Explain limitations (e.g., "AI classification can vary")
- **Safe:** Don't document ways to modify or break the tool

---

## Maintenance

**Repo docs** (README, CONTRIBUTING, LICENSE, CHANGELOG):
- Updated with every release
- CHANGELOG auto-generated from git tags
- Part of the main repo workflow

**Starlight docs** (iwiywi.dev):
- Updated when features change
- Lives in `docs/starlight/src/content/docs/`
- Deployed automatically on push to main

**Versioning:**
- No version-specific docs (tool is simple enough for one version)
- CHANGELOG tracks breaking changes
- Starlight always documents current version

---

## Success Criteria

Documentation is complete when:
1. ✅ README in repo includes install + quick start
2. ✅ CONTRIBUTING.md includes bug reporting guidelines
3. ✅ LICENSE file present
4. ✅ CHANGELOG.md set up (first entry for v0.1.0)
5. ✅ Astro Starlight project created at `docs/starlight/`
6. ✅ All sections (`/getting-started`, `/cli`, `/guides`, `/how-it-works`, `/troubleshooting`) have content
7. ✅ iwiywi.dev domain configured and deployed
8. ✅ Cross-links working (repo ↔ Starlight)
9. ✅ GitHub Actions deploy workflow running
10. ✅ Content proofread (no jargon, user-friendly tone)
