---
title: iwiywi (TUI)
description: Open the interactive terminal UI
---

# iwiywi — Terminal User Interface

```bash
iwiywi
```

Opens the interactive TUI showing today's readings.

## What You'll See

- **Colored feed** — Each reading has a colored accent bar matching its AA Step (1–12)
- **Step + Source** — e.g., "Step 3 · AA.org"
- **Reading text** — Full excerpt, wrappable
- **Navigation** — Scroll, search, commands at bottom

## Keybindings

| Key | Action |
|-----|--------|
| `↑ / ↓` | Scroll up/down |
| `j / k` | Vim-style scroll |
| `/` | Enter command mode |
| `r` | Manually run fetch |
| `q` | Quit |
| `Esc` | Exit command mode or QR overlay |

## Command Mode

Press `/` to enter command mode (you'll see a `>` prompt at the bottom).

### Available Commands

#### `qr`
Display QR code overlay (scan with phone to open mobile view). Press `Esc` to close.

```
/qr
(shows QR overlay)
```

#### `help`
(For future use)

Unknown commands are silently ignored.

## Examples

**Scroll to the top:**
```
Press: ↑ (or k) multiple times
```

**See QR code on your phone:**
```
Press: / → type qr → Enter
(overlay appears, scan with iPhone)
Press: Esc to close
```

**Manually refresh readings:**
```
Press: r
(fetches latest, updates display automatically)
```

**Quit:**
```
Press: q
```

## Colors (Step Mapping)

Each step has a distinct color:

| Step | Color | Step | Color |
|------|-------|------|-------|
| 1 | Red | 7 | Light Red |
| 2 | Yellow | 8 | Light Yellow |
| 3 | Blue | 9 | Light Blue |
| 4 | Magenta | 10 | Light Magenta |
| 5 | Cyan | 11 | Light Cyan |
| 6 | Green | 12 | Light Green |

## Troubleshooting

**TUI won't open?**
- Try: `iwiywi --version` to verify install
- Check logs: `tail -f ~/.iwiywi/fetch.log`

**No readings showing?**
- Fetch hasn't run yet (first run is at 6am). Try: `iwiywi fetch`
- See [Readings not updating](../troubleshooting/index#readings-not-updating)

**QR code not working?**
- Ensure you're scanning with iPhone camera or QR app
- URL should be an iwiywi.vercel.app link

---

Next: Learn about [manual fetching](fetch).
