# Drift Screensaver — Design

## Context

iwiywi is a Rust/ratatui TUI that renders the day's twelve AA readings. The user watched [`sandydoo/flux`](https://github.com/sandydoo/flux) — a WebGL tribute to the macOS Drift screensaver — and wants that aesthetic blended into iwiywi. The blend stays entirely inside the existing terminal app: no web surface, no new binary, no screensaver bundle. The result is a new idle mode that turns the TUI into a flow-field particle animation with one reading floating through at a time.

## The 5 W's

**What.** A new `Mode::Drift` in the TUI. When active, the body fills with 60–120 particles driven by a 2D Perlin noise flow field. Each particle leaves a 4-step fading trail (`·` → `⋅` → `.` → blank). One reading from today's twelve is rendered centered, fading in over ~0.5s, lingering ~8s, then fading out. Readings rotate in order (1 → 12 → 1). Any key press returns the user to whichever tab they were on, with scroll position preserved.

**When.** The screensaver activates automatically after `IWIYWI_IDLE_SECS` seconds of no key input while the user is in `Mode::Normal`. Default is 60. Setting it to `0` disables the screensaver entirely. Inside drift mode the animation ticks every 50ms (20fps) and the reading cycles every 8s.

**Where.** A new `src/tui/drift.rs` module owns particle state, the flow field, the reading cycler, and the render function. Integration is four touch points in existing files: `App` gains idle tracking and a `Drift` variant on `Mode`; the event loop in `tui::run` checks idle every tick and dispatches drift input differently; `widgets::render` delegates to `drift::render` when the mode is `Drift`; a `drift` feature lives under the existing theme palette (no new colors).

**Who.** Single-user feature. Runs locally in the user's terminal. No sharing surface.

**How.** At each 50ms poll tick, if `Mode::Normal` and `last_input.elapsed() >= idle_threshold`, the loop calls `app.enter_drift()`. Thereafter `terminal.draw` calls `drift::render`, and the loop calls `drift::tick(&mut state, dt)` which advances every particle by sampling `noise.get([x * k, y * k, t * k])` and wrapping at screen edges. A `KeyEvent` in `Drift` mode sets `mode = Mode::Normal` and bumps `last_input`. Every subsequent key event in any mode also bumps `last_input` so the idle timer is genuinely "time since last input."

## Components

### `src/tui/drift.rs` (new)

Self-contained: owns state, tick, render, and the flow-field math.

```rust
pub struct DriftState {
    particles: Vec<Particle>,
    noise: Perlin,              // from the `noise` crate
    start: Instant,             // t=0 for field time-axis
    reading_idx: usize,
    reading_phase_start: Instant,
}

struct Particle {
    x: f32,
    y: f32,
    trail: [Option<(u16, u16)>; 4],  // last 4 screen positions
}

impl DriftState {
    pub fn new(width: u16, height: u16, seed: u32) -> Self;
    pub fn tick(&mut self, width: u16, height: u16, dt: Duration);
    pub fn resize(&mut self, width: u16, height: u16);  // rescatter particles if the area changed
}

pub fn render(
    frame: &mut Frame,
    state: &DriftState,
    theme: &Theme,
    reading: &ClassifiedReading,
    reading_alpha: f32,     // 0.0 (invisible) → 1.0 (full)
);
```

- ~60–120 particles scaled by `(width * height) / 800`, clamped.
- Flow field: `vx = noise.get([x*0.03, y*0.03, t*0.15]); vy = noise.get([x*0.03, y*0.03, t*0.15 + 100.0])`. Magnitude clamped to ~1 char/tick.
- Trail chars by index: `['•', '·', '⋅', '.']`. Particle drawn at head with `theme.accent`; trail drawn in `theme.muted`.
- Readings rendered with the existing `kv_line`/`Paragraph` helpers in `widgets.rs` but centered and tinted by `reading_alpha`.

### `src/tui/mod.rs` (modified)

- Add `Mode::Drift` variant.
- `App` gains `last_input: Instant`, `idle_threshold: Duration`, and `drift: Option<DriftState>`.
- `App::register_input()` — called at the top of every key-event branch. Bumps `last_input`. If currently `Mode::Drift`, sets `mode = Mode::Normal` and drops `drift`.
- `App::maybe_enter_drift(size)` — called each tick; if normal-mode and idle exceeds threshold, constructs `DriftState` and switches mode.
- Idle threshold read from `IWIYWI_IDLE_SECS` at startup by `crate::config` (parsed in `App::new_with_env`).

### `src/tui/widgets.rs` (modified)

- In `render`, if `app.mode == Mode::Drift`, call `drift::render(frame, state, theme, current_reading, alpha)` instead of the body+tab-bar+footer stack.
- When exiting drift, normal rendering resumes with untouched tab/scroll.

### `Cargo.toml` (modified)

Add `noise = "0.9"` (stable, zero-copy Perlin/Simplex). Pure Rust, one transitive (num-traits already in our graph via chrono).

### `src/config.rs` (modified)

Add `idle_secs()` helper: parses `IWIYWI_IDLE_SECS`, defaults to 60, returns `Option<Duration>` (None if set to 0).

## Data flow

```
  key poll (50ms)
    │
    ├─ if event:                   ─► app.register_input()
    │     ↳ bump last_input         ↳ also exits Drift if active
    │     ↳ dispatch by mode
    │
    └─ else (no event):
          app.tick(now, size)
            │
            ├─ Mode::Normal: if idle ≥ threshold → enter Drift
            └─ Mode::Drift:  drift.tick(dt); advance reading phase
          terminal.draw(…)
```

Reading phase state machine per reading (total ~8s):

```
fade_in (500ms)  →  linger (7000ms)  →  fade_out (500ms)  →  advance reading_idx
```

`reading_alpha` is a pure function of `Instant::now() - reading_phase_start`.

## Error handling

- **Terminal too small** (`width < 40 || height < 15`): drift renders the centered reading only, no particles. The animation still "exists" (state ticks) so resizing back up is seamless.
- **No readings available**: if `app.readings.is_empty()` at idle time, `maybe_enter_drift` is a no-op. This only happens pre-fetch; the TUI's main path already short-circuits on empty readings via "No readings for today."
- **`IWIYWI_IDLE_SECS` unparseable**: fall back to 60, print a one-line warning to stderr at startup.
- **`noise` crate panics** — won't; Perlin is total. No runtime error path needed.

## Testing

Unit tests in `drift.rs`:

- `reading_alpha_is_zero_before_fade_in` / `…one_during_linger` / `…zero_after_fade_out` — pure function tests with synthetic timestamps.
- `tick_wraps_particles_at_edges` — push a particle past width; verify `x` wraps.
- `resize_rescatter_respects_new_bounds` — all particles within new rect after resize.
- `trail_length_is_four_after_four_ticks`.

Unit tests in `mod.rs`:

- `register_input_exits_drift` — set mode to Drift, call register_input, expect Normal.
- `maybe_enter_drift_respects_threshold` — idle below threshold → Normal; above → Drift.
- `maybe_enter_drift_noop_when_readings_empty`.
- `idle_secs_env_zero_disables` — config helper returns None for `"0"`.

Visual verification (manual; listed in the README troubleshooting and the PR description):

1. `cargo run` → wait 60s → screensaver activates, particles flow, Step 1 fades in.
2. Wait ~8s → Step 2 appears.
3. Press `q` or any key → returns to the All tab, scroll preserved.
4. `IWIYWI_IDLE_SECS=0 cargo run` → no screensaver regardless of idle time.
5. Resize the terminal window during drift → particles rescatter, no panic.

## Scope / YAGNI

**In:** the above. Exactly that.

**Out:**

- No mouse interaction — drift is triggered by absence of input; adding mouse events to "exit" is symmetrical but also means the screensaver never activates on an idle SSH session with stray mouse reporting. Skip.
- No sound.
- No per-particle physics beyond the flow field. No gravity, no attractors, no mouse follow.
- No user-configurable palette or particle count beyond what the theme already exposes.
- No screensaver on the gist / web surface. That's a separate project if we ever want it.
- No "pause animation while reading" mode. If you want to read, press any key — you're back in the TUI.

## Files touched

| File | Change |
|------|--------|
| `src/tui/drift.rs` | **New.** Particle state, flow field, render, tick. |
| `src/tui/mod.rs` | `Mode::Drift` variant, idle tracking on `App`, `register_input` + `maybe_enter_drift`, event loop wiring. |
| `src/tui/widgets.rs` | Delegate to `drift::render` when `Mode::Drift`. |
| `src/config.rs` | `idle_secs()` helper. |
| `Cargo.toml` | Add `noise = "0.9"`. |
| `README.md` | One-line note under Features + Theme section mentioning `IWIYWI_IDLE_SECS`. |

## Verification (end-to-end)

1. `cargo build --release` clean.
2. `cargo test` — all drift + mod tests green, prior 67 stay green.
3. Run `cargo run`, leave idle 60s, see particles + reading cycle.
4. Press a key — see the previous tab and scroll position.
5. `IWIYWI_IDLE_SECS=5 cargo run` — activates in 5s.
6. `IWIYWI_IDLE_SECS=0 cargo run` — never activates.
7. Resize terminal mid-drift — no crash.
