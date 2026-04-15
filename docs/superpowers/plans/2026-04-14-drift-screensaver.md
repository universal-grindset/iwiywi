# Drift Screensaver Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add an idle-triggered screensaver mode to the iwiywi TUI that renders flow-field particles with fading trails and cycles one reading at a time, exiting on any key.

**Architecture:** A new `src/tui/drift.rs` module owns particle state, a Perlin flow field (from the `noise` crate), and the render function. `App` gains idle tracking (`last_input`, `idle_threshold`, `drift: Option<DriftState>`) and a `Mode::Drift` variant. The event loop bumps `last_input` on every key event and checks for idle on every poll tick; when idle exceeds the threshold, the mode flips to `Drift`. Any key event in drift mode restores the previous mode with its scroll/tab intact. Terminal size below 40×15 gracefully degrades to a centered reading with no particles.

**Tech Stack:** Rust 2021, ratatui 0.28, crossterm 0.28, `noise = "0.9"` (new). All changes stay inside `src/tui/`, `src/config.rs`, and `Cargo.toml`.

**Spec:** `docs/superpowers/specs/2026-04-14-drift-screensaver-design.md`

---

## File Structure

| File | Role |
|------|------|
| `src/tui/drift.rs` | **New.** `DriftState`, `Particle`, `reading_alpha`, flow-field tick, render. |
| `src/tui/mod.rs` | **Modify.** Add `Mode::Drift`, extend `App` with idle fields, add `register_input`/`maybe_enter_drift`, wire event loop. |
| `src/tui/widgets.rs` | **Modify.** Delegate to `drift::render` when `Mode::Drift`. |
| `src/config.rs` | **Modify.** Add `idle_secs()` helper. |
| `Cargo.toml` | **Modify.** Add `noise = "0.9"`. |
| `README.md` | **Modify.** One-line note under Features + `IWIYWI_IDLE_SECS` in Theme section. |

---

### Task 1: Add `noise` dependency + `config::idle_secs` helper

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/config.rs`

- [ ] **Step 1.1: Write the failing tests**

Append to the existing `#[cfg(test)] mod tests` block at the bottom of `src/config.rs`:

```rust
    #[test]
    fn idle_secs_defaults_to_sixty_when_unset() {
        std::env::remove_var("IWIYWI_IDLE_SECS");
        assert_eq!(idle_secs(), Some(std::time::Duration::from_secs(60)));
    }

    #[test]
    fn idle_secs_returns_none_when_zero() {
        std::env::set_var("IWIYWI_IDLE_SECS", "0");
        assert_eq!(idle_secs(), None);
        std::env::remove_var("IWIYWI_IDLE_SECS");
    }

    #[test]
    fn idle_secs_parses_positive_value() {
        std::env::set_var("IWIYWI_IDLE_SECS", "15");
        assert_eq!(idle_secs(), Some(std::time::Duration::from_secs(15)));
        std::env::remove_var("IWIYWI_IDLE_SECS");
    }

    #[test]
    fn idle_secs_falls_back_to_default_on_garbage() {
        std::env::set_var("IWIYWI_IDLE_SECS", "not-a-number");
        assert_eq!(idle_secs(), Some(std::time::Duration::from_secs(60)));
        std::env::remove_var("IWIYWI_IDLE_SECS");
    }
```

- [ ] **Step 1.2: Run the tests — expect failure**

```
cargo test idle_secs 2>&1 | tail -10
```

Expected: compile error "cannot find function `idle_secs`".

- [ ] **Step 1.3: Implement `idle_secs`**

Add this function to `src/config.rs` (below `qr_url`, above `#[cfg(test)]`):

```rust
pub fn idle_secs() -> Option<std::time::Duration> {
    const DEFAULT: u64 = 60;
    let raw = std::env::var("IWIYWI_IDLE_SECS").ok();
    let secs: u64 = match raw.as_deref() {
        None => DEFAULT,
        Some(s) => s.parse().unwrap_or(DEFAULT),
    };
    if secs == 0 {
        None
    } else {
        Some(std::time::Duration::from_secs(secs))
    }
}
```

- [ ] **Step 1.4: Run the tests — expect pass**

```
cargo test idle_secs 2>&1 | tail -5
```

Expected: `test result: ok. 4 passed`.

- [ ] **Step 1.5: Add `noise` dependency**

In `Cargo.toml`, under `[dependencies]`, add:

```toml
noise = "0.9"
```

- [ ] **Step 1.6: Verify the dep compiles**

```
cargo build 2>&1 | tail -5
```

Expected: `Finished` with no errors.

- [ ] **Step 1.7: Commit**

```
git add Cargo.toml Cargo.lock src/config.rs
git commit -m "feat(config): add idle_secs helper and noise crate"
```

---

### Task 2: `drift.rs` scaffold with `reading_alpha`

**Files:**
- Create: `src/tui/drift.rs`
- Modify: `src/tui/mod.rs` (add `pub mod drift;`)

- [ ] **Step 2.1: Register the module**

In `src/tui/mod.rs`, add `pub mod drift;` next to the other `pub mod` lines at the top (below `pub mod commands;` etc., before `use crate::models::...`).

- [ ] **Step 2.2: Create `src/tui/drift.rs` with the `reading_alpha` function + tests**

```rust
use std::time::Duration;

pub const FADE_IN: Duration = Duration::from_millis(500);
pub const LINGER: Duration = Duration::from_millis(7_000);
pub const FADE_OUT: Duration = Duration::from_millis(500);
pub const READING_CYCLE: Duration =
    Duration::from_millis(500 + 7_000 + 500);

/// Compute the alpha (0.0 = invisible, 1.0 = full) for the currently-showing
/// reading given how long it has been visible.
pub fn reading_alpha(elapsed: Duration) -> f32 {
    if elapsed < FADE_IN {
        elapsed.as_secs_f32() / FADE_IN.as_secs_f32()
    } else if elapsed < FADE_IN + LINGER {
        1.0
    } else if elapsed < READING_CYCLE {
        let into_fade = elapsed - (FADE_IN + LINGER);
        1.0 - into_fade.as_secs_f32() / FADE_OUT.as_secs_f32()
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_zero_at_start() {
        assert!(reading_alpha(Duration::ZERO) < 0.01);
    }

    #[test]
    fn alpha_midway_through_fade_in_is_half() {
        let a = reading_alpha(FADE_IN / 2);
        assert!((a - 0.5).abs() < 0.05, "expected ~0.5, got {a}");
    }

    #[test]
    fn alpha_one_during_linger() {
        assert_eq!(reading_alpha(FADE_IN + LINGER / 2), 1.0);
    }

    #[test]
    fn alpha_midway_through_fade_out_is_half() {
        let a = reading_alpha(FADE_IN + LINGER + FADE_OUT / 2);
        assert!((a - 0.5).abs() < 0.05, "expected ~0.5, got {a}");
    }

    #[test]
    fn alpha_zero_after_full_cycle() {
        assert!(reading_alpha(READING_CYCLE + Duration::from_millis(1)) < 0.01);
    }
}
```

- [ ] **Step 2.3: Run the tests — expect pass**

```
cargo test drift:: 2>&1 | tail -10
```

Expected: `5 passed`.

- [ ] **Step 2.4: Commit**

```
git add src/tui/drift.rs src/tui/mod.rs
git commit -m "feat(drift): add reading_alpha phase function"
```

---

### Task 3: `Particle` + `DriftState::new` + `DriftState::tick`

**Files:**
- Modify: `src/tui/drift.rs`

- [ ] **Step 3.1: Write the failing tests**

Append to `src/tui/drift.rs` inside `#[cfg(test)] mod tests`:

```rust
    #[test]
    fn new_scales_particle_count_with_area() {
        let small = DriftState::new(40, 20, 1);
        let large = DriftState::new(160, 50, 1);
        assert!(large.particles.len() > small.particles.len());
        assert!(small.particles.len() >= 10);
        assert!(large.particles.len() <= 120);
    }

    #[test]
    fn particles_stay_in_bounds_after_many_ticks() {
        let mut s = DriftState::new(80, 24, 1);
        for _ in 0..200 {
            s.tick(80, 24, Duration::from_millis(50));
        }
        for p in &s.particles {
            assert!(p.x >= 0.0 && p.x < 80.0, "x out of bounds: {}", p.x);
            assert!(p.y >= 0.0 && p.y < 24.0, "y out of bounds: {}", p.y);
        }
    }

    #[test]
    fn trail_length_is_four_after_four_ticks() {
        let mut s = DriftState::new(80, 24, 1);
        for _ in 0..4 {
            s.tick(80, 24, Duration::from_millis(50));
        }
        for p in &s.particles {
            assert!(p.trail.iter().filter(|t| t.is_some()).count() == 4);
        }
    }
```

- [ ] **Step 3.2: Run the tests — expect compile failure**

```
cargo test drift:: 2>&1 | tail -5
```

Expected: "cannot find type `DriftState`".

- [ ] **Step 3.3: Implement `Particle` + `DriftState::new` + `tick`**

Add to `src/tui/drift.rs` (above `#[cfg(test)]`):

```rust
use noise::{NoiseFn, Perlin};
use std::time::Instant;

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub trail: [Option<(u16, u16)>; 4],
}

pub struct DriftState {
    pub particles: Vec<Particle>,
    noise: Perlin,
    pub start: Instant,
    pub reading_idx: usize,
    pub reading_phase_start: Instant,
}

const PARTICLES_MIN: usize = 20;
const PARTICLES_MAX: usize = 120;
const DIVISOR: usize = 30;       // tune: higher → fewer particles per area
const FIELD_SCALE: f64 = 0.06;   // spatial noise frequency
const TIME_SCALE: f64 = 0.25;    // temporal noise frequency
const MAX_STEP: f32 = 0.8;       // chars/tick cap

fn particle_count(w: u16, h: u16) -> usize {
    let area = (w as usize) * (h as usize);
    (area / DIVISOR).clamp(PARTICLES_MIN, PARTICLES_MAX)
}

fn pseudo_rand(seed: u32, n: usize) -> f32 {
    // Deterministic, zero-dep PRN for initial scatter.
    let mut x = seed.wrapping_mul(2_654_435_761).wrapping_add(n as u32);
    x ^= x >> 13;
    x = x.wrapping_mul(0x5bd1e995);
    x ^= x >> 15;
    (x as f32) / (u32::MAX as f32)
}

impl DriftState {
    pub fn new(width: u16, height: u16, seed: u32) -> Self {
        let count = particle_count(width, height);
        let particles = (0..count)
            .map(|i| Particle {
                x: pseudo_rand(seed, i * 2) * (width as f32),
                y: pseudo_rand(seed, i * 2 + 1) * (height as f32),
                trail: [None; 4],
            })
            .collect();
        let now = Instant::now();
        DriftState {
            particles,
            noise: Perlin::new(seed),
            start: now,
            reading_idx: 0,
            reading_phase_start: now,
        }
    }

    pub fn tick(&mut self, width: u16, height: u16, _dt: std::time::Duration) {
        if width == 0 || height == 0 {
            return;
        }
        let t = self.start.elapsed().as_secs_f64();
        for p in &mut self.particles {
            // shift trail: [0,1,2,3] ← [prev,0,1,2]
            p.trail[3] = p.trail[2];
            p.trail[2] = p.trail[1];
            p.trail[1] = p.trail[0];
            p.trail[0] = Some((p.x as u16, p.y as u16));

            let fx = p.x as f64 * FIELD_SCALE;
            let fy = p.y as f64 * FIELD_SCALE;
            let vx = self.noise.get([fx, fy, t * TIME_SCALE]) as f32;
            let vy = self.noise.get([fx, fy, t * TIME_SCALE + 100.0]) as f32;
            let vx = vx.clamp(-MAX_STEP, MAX_STEP);
            let vy = vy.clamp(-MAX_STEP, MAX_STEP);

            p.x = wrap(p.x + vx, width as f32);
            p.y = wrap(p.y + vy, height as f32);
        }
    }
}

fn wrap(v: f32, max: f32) -> f32 {
    if max <= 0.0 {
        return 0.0;
    }
    let mut r = v % max;
    if r < 0.0 {
        r += max;
    }
    r
}
```

- [ ] **Step 3.4: Run the tests — expect pass**

```
cargo test drift:: 2>&1 | tail -5
```

Expected: `8 passed`.

- [ ] **Step 3.5: Commit**

```
git add src/tui/drift.rs
git commit -m "feat(drift): flow-field particles with wrapped edges and trails"
```

---

### Task 4: `DriftState::resize`

**Files:**
- Modify: `src/tui/drift.rs`

- [ ] **Step 4.1: Write the failing test**

Append to `drift.rs` tests:

```rust
    #[test]
    fn resize_rescatters_particles_into_new_bounds() {
        let mut s = DriftState::new(120, 40, 1);
        s.resize(40, 20);
        for p in &s.particles {
            assert!(p.x >= 0.0 && p.x < 40.0);
            assert!(p.y >= 0.0 && p.y < 20.0);
        }
    }

    #[test]
    fn resize_adjusts_particle_count() {
        let mut s = DriftState::new(40, 20, 1);
        let before = s.particles.len();
        s.resize(160, 50);
        assert!(s.particles.len() > before);
    }
```

- [ ] **Step 4.2: Run — expect failure**

```
cargo test drift::tests::resize 2>&1 | tail -5
```

Expected: "no method named `resize`".

- [ ] **Step 4.3: Implement `resize`**

Add inside `impl DriftState`:

```rust
    pub fn resize(&mut self, width: u16, height: u16) {
        let want = particle_count(width, height);
        let seed = self.start.elapsed().as_nanos() as u32;
        // Rebuild deterministically — cheap and avoids stretching math.
        self.particles = (0..want)
            .map(|i| Particle {
                x: pseudo_rand(seed, i * 2) * (width as f32),
                y: pseudo_rand(seed, i * 2 + 1) * (height as f32),
                trail: [None; 4],
            })
            .collect();
    }
```

- [ ] **Step 4.4: Run — expect pass**

```
cargo test drift:: 2>&1 | tail -5
```

Expected: `10 passed`.

- [ ] **Step 4.5: Commit**

```
git add src/tui/drift.rs
git commit -m "feat(drift): rescatter particles on terminal resize"
```

---

### Task 5: `Mode::Drift` + `App` idle fields

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 5.1: Add `Mode::Drift` variant**

In `src/tui/mod.rs`, update the `Mode` enum:

```rust
#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Command(String),
    QrOverlay,
    Drift,
}
```

- [ ] **Step 5.2: Extend `App` with idle tracking**

Update the `App` struct and its `::new` method in `src/tui/mod.rs`:

```rust
pub struct App {
    pub readings: Vec<ClassifiedReading>,
    pub scroll: usize,
    pub mode: Mode,
    pub tab: Tab,
    pub step_filter: u8,
    pub qr_url: String,
    pub theme: theme::Theme,
    pub last_input: std::time::Instant,
    pub idle_threshold: Option<std::time::Duration>,
    pub drift: Option<drift::DriftState>,
}

impl App {
    pub fn new(
        readings: Vec<ClassifiedReading>,
        qr_url: String,
        theme: theme::Theme,
        idle_threshold: Option<std::time::Duration>,
    ) -> Self {
        App {
            readings,
            scroll: 0,
            mode: Mode::Normal,
            tab: Tab::All,
            step_filter: 1,
            qr_url,
            theme,
            last_input: std::time::Instant::now(),
            idle_threshold,
            drift: None,
        }
    }
    // ... (keep the other methods unchanged)
}
```

- [ ] **Step 5.3: Update every `App::new` call-site to pass the new arg**

Inside `src/tui/mod.rs` tests, update `fixture_app`:

```rust
    fn fixture_app() -> App {
        App::new(
            vec![
                ClassifiedReading {
                    step: 3,
                    reason: "test".to_string(),
                    source: "AA.org".to_string(),
                    title: "Daily".to_string(),
                    text: "Made a decision...".to_string(),
                    url: "https://aa.org".to_string(),
                },
                ClassifiedReading {
                    step: 7,
                    reason: "test".to_string(),
                    source: "Hazeldon".to_string(),
                    title: "Thought".to_string(),
                    text: "Humbly asked...".to_string(),
                    url: "https://hazeldon.org".to_string(),
                },
            ],
            "https://iwiywi.vercel.app".to_string(),
            theme::Theme::dark(),
            None,
        )
    }
```

And in `run()` (near the bottom of the same file), change the live-call:

```rust
    let mut app = App::new(
        readings,
        crate::config::qr_url(&config),
        theme::detect(),
        crate::config::idle_secs(),
    );
```

- [ ] **Step 5.4: Run — expect pass (nothing new to test yet)**

```
cargo build 2>&1 | tail -5 && cargo test 2>&1 | tail -5
```

Expected: builds clean, all existing tests still pass.

- [ ] **Step 5.5: Commit**

```
git add src/tui/mod.rs
git commit -m "feat(tui): add Mode::Drift and idle-tracking fields on App"
```

---

### Task 6: `App::register_input` + `App::maybe_enter_drift` + `App::exit_drift`

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 6.1: Write the failing tests**

Append to the `#[cfg(test)] mod tests` block in `src/tui/mod.rs`:

```rust
    #[test]
    fn register_input_bumps_last_input() {
        let mut app = fixture_app();
        let before = app.last_input;
        std::thread::sleep(std::time::Duration::from_millis(5));
        app.register_input();
        assert!(app.last_input > before);
    }

    #[test]
    fn register_input_exits_drift() {
        let mut app = fixture_app();
        app.mode = Mode::Drift;
        app.drift = Some(drift::DriftState::new(80, 24, 1));
        app.register_input();
        assert_eq!(app.mode, Mode::Normal);
        assert!(app.drift.is_none());
    }

    #[test]
    fn maybe_enter_drift_noop_when_threshold_none() {
        let mut app = fixture_app();
        app.idle_threshold = None;
        app.last_input = std::time::Instant::now() - std::time::Duration::from_secs(3600);
        app.maybe_enter_drift(80, 24);
        assert_eq!(app.mode, Mode::Normal);
    }

    #[test]
    fn maybe_enter_drift_noop_when_not_idle_long_enough() {
        let mut app = fixture_app();
        app.idle_threshold = Some(std::time::Duration::from_secs(60));
        app.maybe_enter_drift(80, 24);
        assert_eq!(app.mode, Mode::Normal);
    }

    #[test]
    fn maybe_enter_drift_activates_after_threshold() {
        let mut app = fixture_app();
        app.idle_threshold = Some(std::time::Duration::from_millis(10));
        std::thread::sleep(std::time::Duration::from_millis(20));
        app.maybe_enter_drift(80, 24);
        assert_eq!(app.mode, Mode::Drift);
        assert!(app.drift.is_some());
    }

    #[test]
    fn maybe_enter_drift_noop_when_readings_empty() {
        let mut app = fixture_app();
        app.readings.clear();
        app.idle_threshold = Some(std::time::Duration::from_millis(1));
        std::thread::sleep(std::time::Duration::from_millis(10));
        app.maybe_enter_drift(80, 24);
        assert_eq!(app.mode, Mode::Normal);
    }

    #[test]
    fn maybe_enter_drift_noop_when_already_in_command_mode() {
        let mut app = fixture_app();
        app.mode = Mode::Command(String::new());
        app.idle_threshold = Some(std::time::Duration::from_millis(1));
        std::thread::sleep(std::time::Duration::from_millis(10));
        app.maybe_enter_drift(80, 24);
        assert!(matches!(app.mode, Mode::Command(_)));
    }
```

- [ ] **Step 6.2: Run — expect failure**

```
cargo test maybe_enter_drift 2>&1 | tail -5
```

Expected: "no method named `maybe_enter_drift`".

- [ ] **Step 6.3: Implement the methods**

Add inside `impl App` in `src/tui/mod.rs`:

```rust
    pub fn register_input(&mut self) {
        self.last_input = std::time::Instant::now();
        if self.mode == Mode::Drift {
            self.mode = Mode::Normal;
            self.drift = None;
        }
    }

    pub fn maybe_enter_drift(&mut self, width: u16, height: u16) {
        let Some(threshold) = self.idle_threshold else {
            return;
        };
        if self.mode != Mode::Normal {
            return;
        }
        if self.readings.is_empty() {
            return;
        }
        if self.last_input.elapsed() < threshold {
            return;
        }
        self.drift = Some(drift::DriftState::new(width, height, 1));
        self.mode = Mode::Drift;
    }

    pub fn drift_tick(&mut self, width: u16, height: u16) {
        if self.mode != Mode::Drift {
            return;
        }
        if let Some(state) = self.drift.as_mut() {
            state.tick(width, height, std::time::Duration::from_millis(50));
            if state.reading_phase_start.elapsed() >= drift::READING_CYCLE {
                state.reading_idx = (state.reading_idx + 1) % self.readings.len();
                state.reading_phase_start = std::time::Instant::now();
            }
        }
    }
```

- [ ] **Step 6.4: Run — expect pass**

```
cargo test 2>&1 | tail -10
```

Expected: all tests pass, including the 7 new drift-related `App` tests.

- [ ] **Step 6.5: Commit**

```
git add src/tui/mod.rs
git commit -m "feat(tui): register_input/maybe_enter_drift/drift_tick on App"
```

---

### Task 7: Event-loop wiring

**Files:**
- Modify: `src/tui/mod.rs`

- [ ] **Step 7.1: Update the event loop**

In the `run()` function near the bottom of `src/tui/mod.rs`, replace the whole `loop { ... }` body with:

```rust
    loop {
        let size = terminal.size()?;
        terminal.draw(|f| widgets::render(f, &app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                app.register_input();
                match &app.mode {
                    Mode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('a') => app.set_tab(Tab::All),
                        KeyCode::Char('s') => app.set_tab(Tab::Steps),
                        KeyCode::Char('?') => app.set_tab(Tab::Help),
                        KeyCode::Tab => app.next_tab(),
                        KeyCode::BackTab => app.prev_tab(),
                        KeyCode::Left if app.tab == Tab::Steps => app.step_prev(),
                        KeyCode::Right if app.tab == Tab::Steps => app.step_next(),
                        KeyCode::Char('j') | KeyCode::Down => app.scroll_down(),
                        KeyCode::Char('k') | KeyCode::Up => app.scroll_up(),
                        KeyCode::Char('/') => app.enter_command_mode(),
                        _ => {}
                    },
                    Mode::Command(_) => match key.code {
                        KeyCode::Esc => app.dismiss(),
                        KeyCode::Enter => {
                            let cmd = if let Mode::Command(s) = &app.mode {
                                s.clone()
                            } else {
                                String::new()
                            };
                            app.dismiss();
                            match handle_command(&cmd) {
                                Action::ToggleQr => app.toggle_qr(),
                                Action::Unknown => {}
                            }
                        }
                        KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.push_command_char(c);
                        }
                        KeyCode::Backspace => app.pop_command_char(),
                        _ => {}
                    },
                    Mode::QrOverlay => {
                        app.dismiss();
                    }
                    Mode::Drift => {
                        // register_input already exited drift; nothing else to do.
                    }
                }
            }
        } else {
            app.maybe_enter_drift(size.width, size.height);
            app.drift_tick(size.width, size.height);
        }
    }
```

- [ ] **Step 7.2: Build and run the existing test suite**

```
cargo build 2>&1 | tail -5 && cargo test 2>&1 | tail -5
```

Expected: clean build, all prior tests pass. `size` needs the `ratatui::prelude::Size`-returning `terminal.size()` — already available in ratatui 0.28 as `Size { width, height }`.

- [ ] **Step 7.3: Commit**

```
git add src/tui/mod.rs
git commit -m "feat(tui): wire idle-enter and drift tick into event loop"
```

---

### Task 8: `drift::render` — particles + centered reading

**Files:**
- Modify: `src/tui/drift.rs`

- [ ] **Step 8.1: Add a color-interpolation helper with a test**

Append to `drift.rs` above `#[cfg(test)]`:

```rust
use ratatui::style::Color;

pub fn lerp_color(from: Color, to: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    let (fr, fg, fb) = rgb(from);
    let (tr, tg, tb) = rgb(to);
    Color::Rgb(
        (fr as f32 + (tr as f32 - fr as f32) * t) as u8,
        (fg as f32 + (tg as f32 - fg as f32) * t) as u8,
        (fb as f32 + (tb as f32 - fb as f32) * t) as u8,
    )
}

fn rgb(c: Color) -> (u8, u8, u8) {
    match c {
        Color::Rgb(r, g, b) => (r, g, b),
        _ => (128, 128, 128),
    }
}
```

And a test in the tests module:

```rust
    #[test]
    fn lerp_color_endpoints() {
        let a = Color::Rgb(0, 0, 0);
        let b = Color::Rgb(200, 200, 200);
        assert_eq!(lerp_color(a, b, 0.0), Color::Rgb(0, 0, 0));
        assert_eq!(lerp_color(a, b, 1.0), Color::Rgb(200, 200, 200));
    }

    #[test]
    fn lerp_color_midpoint() {
        let a = Color::Rgb(0, 0, 0);
        let b = Color::Rgb(100, 100, 100);
        let mid = lerp_color(a, b, 0.5);
        assert_eq!(mid, Color::Rgb(50, 50, 50));
    }
```

- [ ] **Step 8.2: Run — expect pass**

```
cargo test drift::tests::lerp 2>&1 | tail -5
```

Expected: 2 passed.

- [ ] **Step 8.3: Implement `render`**

Append to `drift.rs` above `#[cfg(test)]`:

```rust
use crate::models::ClassifiedReading;
use crate::tui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
    Frame,
};

const TRAIL_CHARS: [&str; 4] = ["•", "·", "⋅", "."];

pub fn render(
    frame: &mut Frame,
    state: &DriftState,
    theme: &Theme,
    reading: &ClassifiedReading,
    reading_alpha: f32,
) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    // Draw particle trails oldest → newest so the head sits on top.
    for p in &state.particles {
        for (i, pos) in p.trail.iter().enumerate().rev() {
            if let Some((x, y)) = pos {
                if *x < area.width && *y < area.height {
                    let ch = TRAIL_CHARS[i];
                    let color = lerp_color(theme.border, theme.muted, 1.0 - (i as f32 / 4.0));
                    buf[(area.x + *x, area.y + *y)]
                        .set_symbol(ch)
                        .set_style(Style::default().fg(color));
                }
            }
        }
        let hx = p.x as u16;
        let hy = p.y as u16;
        if hx < area.width && hy < area.height {
            buf[(area.x + hx, area.y + hy)]
                .set_symbol("•")
                .set_style(Style::default().fg(theme.accent));
        }
    }

    // Reading overlay
    if reading_alpha <= 0.0 {
        return;
    }
    let faded_accent = lerp_color(theme.border, theme.accent, reading_alpha);
    let faded_body = lerp_color(theme.border, theme.body, reading_alpha);
    let faded_muted = lerp_color(theme.border, theme.muted, reading_alpha);

    let header = Line::from(vec![
        Span::styled(
            format!("Step {}", reading.step),
            Style::default().fg(faded_accent).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("  ·  {}", reading.source),
            Style::default().fg(faded_muted),
        ),
    ]);
    let body = Line::from(Span::styled(
        reading.text.clone(),
        Style::default().fg(faded_body),
    ));

    let text = vec![header, Line::from(""), body];
    let width = (area.width as f32 * 0.6).min(72.0) as u16;
    let width = width.max(20);
    let est_height: u16 = 3 + (reading.text.len() as u16 / width.max(1)) + 1;
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(est_height)) / 2;
    let overlay = Rect {
        x,
        y,
        width: width.min(area.width.saturating_sub(x.saturating_sub(area.x))),
        height: est_height.min(area.height.saturating_sub(y.saturating_sub(area.y))),
    };

    Paragraph::new(text)
        .wrap(Wrap { trim: false })
        .render(overlay, buf);
}
```

- [ ] **Step 8.4: Build**

```
cargo build 2>&1 | tail -10
```

Expected: clean build.

- [ ] **Step 8.5: Commit**

```
git add src/tui/drift.rs
git commit -m "feat(drift): render particle trails and faded reading overlay"
```

---

### Task 9: Delegate in `widgets::render`

**Files:**
- Modify: `src/tui/widgets.rs`

- [ ] **Step 9.1: Update `render` to branch on `Mode::Drift`**

At the top of `widgets::render` (right after `let area = frame.area();`), add:

```rust
    if let Mode::Drift = &app.mode {
        if let Some(state) = &app.drift {
            let reading = &app.readings[state.reading_idx % app.readings.len()];
            let alpha = crate::tui::drift::reading_alpha(state.reading_phase_start.elapsed());
            crate::tui::drift::render(frame, state, &app.theme, reading, alpha);
            return;
        }
    }
```

- [ ] **Step 9.2: Build and run existing tests**

```
cargo build 2>&1 | tail -5 && cargo test 2>&1 | tail -5
```

Expected: clean build, all tests still pass.

- [ ] **Step 9.3: Manually verify**

Run:

```
IWIYWI_IDLE_SECS=5 cargo run
```

Expected: after 5 seconds of no input, particles drift across the screen and the first reading fades in centered. Press any key — returns to whatever tab you were on with scroll position preserved.

- [ ] **Step 9.4: Commit**

```
git add src/tui/widgets.rs
git commit -m "feat(tui): route Mode::Drift to drift::render"
```

---

### Task 10: Terminal-too-small fallback

**Files:**
- Modify: `src/tui/drift.rs`

- [ ] **Step 10.1: Update `render` to skip particles when small**

In `drift::render`, wrap the particle-drawing loop in a size check. Replace the `// Draw particle trails …` block with:

```rust
    if area.width >= 40 && area.height >= 15 {
        for p in &state.particles {
            for (i, pos) in p.trail.iter().enumerate().rev() {
                if let Some((x, y)) = pos {
                    if *x < area.width && *y < area.height {
                        let ch = TRAIL_CHARS[i];
                        let color = lerp_color(theme.border, theme.muted, 1.0 - (i as f32 / 4.0));
                        buf[(area.x + *x, area.y + *y)]
                            .set_symbol(ch)
                            .set_style(Style::default().fg(color));
                    }
                }
            }
            let hx = p.x as u16;
            let hy = p.y as u16;
            if hx < area.width && hy < area.height {
                buf[(area.x + hx, area.y + hy)]
                    .set_symbol("•")
                    .set_style(Style::default().fg(theme.accent));
            }
        }
    }
```

The reading-overlay block below it stays unchanged, so it always renders regardless of size.

- [ ] **Step 10.2: Manually verify**

Run in a narrow terminal (resize to ~35 cols):

```
IWIYWI_IDLE_SECS=3 cargo run
```

Expected: reading still centered, no particles, no panic.

- [ ] **Step 10.3: Commit**

```
git add src/tui/drift.rs
git commit -m "feat(drift): skip particles when terminal under 40x15"
```

---

### Task 11: README update

**Files:**
- Modify: `README.md`

- [ ] **Step 11.1: Add a Features bullet**

In `README.md`, under `## Features`, after the "Runs daily at 6am via launchd" line, add:

```
- Idle screensaver: flow-field drift animation cycles today's readings after 60s
```

- [ ] **Step 11.2: Document `IWIYWI_IDLE_SECS`**

In the `## Theme` section, below the `IWIYWI_THEME` block, add:

```
Set how long until the screensaver activates, or disable it:

```sh
export IWIYWI_IDLE_SECS=60   # default
export IWIYWI_IDLE_SECS=10   # faster idle
export IWIYWI_IDLE_SECS=0    # never activate
```
```

- [ ] **Step 11.3: Commit**

```
git add README.md
git commit -m "docs: document screensaver and IWIYWI_IDLE_SECS"
```

---

### Task 12: End-to-end verification

- [ ] **Step 12.1: Full test pass**

```
cargo test 2>&1 | tail -5
```

Expected: all tests pass (prior 67 + ~15 new).

- [ ] **Step 12.2: Release build**

```
cargo build --release 2>&1 | tail -5
```

Expected: clean.

- [ ] **Step 12.3: Manual smoke: default 60s idle**

```
cargo run
```

Expected behavior: after ~60s of no input, particles drift, Step 1 fades in, after ~8s Step 2 fades in. Press any key — returns to whichever tab you were on with scroll position preserved.

- [ ] **Step 12.4: Manual smoke: disabled**

```
IWIYWI_IDLE_SECS=0 cargo run
```

Expected: no screensaver regardless of how long you wait.

- [ ] **Step 12.5: Manual smoke: resize mid-drift**

With `IWIYWI_IDLE_SECS=3`, enter drift, then resize the terminal window smaller. Expected: particles adjust, no panic.

- [ ] **Step 12.6: Push**

```
git push
```

---

## Self-Review Notes

- **Spec coverage:** `reading_alpha` (Task 2) + `DriftState::new`/`tick` (Task 3) + `resize` (Task 4) + `Mode::Drift` + idle methods (Tasks 5–6) + event-loop wiring (Task 7) + `render` (Task 8) + widgets delegate (Task 9) + size fallback (Task 10) + README (Task 11) — every spec bullet has a corresponding task.
- **Type consistency:** `DriftState.reading_idx`, `reading_phase_start`, `particles`, `start` — same names used in Tasks 3, 6, 8, 9. `register_input`, `maybe_enter_drift`, `drift_tick` — same signatures used in Tasks 6, 7.
- **No placeholders:** every step has concrete code or an exact command. No "add error handling" stubs — the few fallbacks (empty readings, terminal too small) each have a dedicated task.
- **YAGNI check:** no mouse, no sound, no pause, no per-particle physics beyond the flow field. Matches the spec's "Out" list.
