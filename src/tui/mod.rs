pub mod clipboard;
pub mod clock;
pub mod drift;
pub mod export;
pub mod help;
pub mod journal;
pub mod menu;
pub mod moon;
pub mod overlay;
pub mod palette;
pub mod pattern;
pub mod status;
pub mod text_size;
pub mod widgets;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use sha2::{Digest, Sha256};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crate::config::{self, Config};
use crate::fetch::ai::{post_chat, ChatOpts};
use crate::pulse::{self, Focus, Order, PulseMixer, PulseSource};
use crate::storage::read_readings;
use crate::tui::overlay::{AiOutcome, AiOverlay};

const STEP_DOUBLE_TAP_MS: u128 = 1500;

const EXPLAIN_SYSTEM_PROMPT: &str =
    "You are an AA sponsor explaining why today's reading matters for someone in recovery. \
     Two to three plain sentences. No scripture citations, no step-enumeration, no moralizing. \
     Focus on one practical takeaway a person could carry through their day.";

const MEDITATION_SYSTEM_PROMPT: &str =
    "You write a daily meditation on applying a specific AA step. \
     About 150 words, first-person, plain language, grounded in everyday recovery — no platitudes. \
     No direct quotes from copyrighted AA texts, no enumerated lists. \
     Return only the meditation prose, no heading, no sign-off.";

const JOURNAL_SEED_SYSTEM_PROMPT: &str =
    "You craft a single reflection question for a journal entry. \
     Twenty words maximum, ending with a question mark. \
     No moralizing, no platitudes. The question should surface a concrete moment \
     from the reader's day. Return only the question, no preamble.";

fn sha_hex(input: &str) -> String {
    use std::fmt::Write;
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let digest = hasher.finalize();
    let mut out = String::with_capacity(digest.len() * 2);
    for b in digest { let _ = write!(out, "{b:02x}"); }
    out
}

fn read_cache_file(path: &Path) -> Option<String> {
    let raw = std::fs::read_to_string(path).ok()?;
    let trimmed = raw.trim();
    if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
}

fn write_cache_file(path: &Path, body: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, body)
}

pub struct App {
    pub mixer: PulseMixer,
    pub sources: Vec<Box<dyn PulseSource>>,
    pub palette: palette::Palette,
    pub pattern: pattern::Pattern,
    pub text_size: text_size::TextSize,
    pub order: Order,
    pub focus: Focus,
    pub focus_step: Option<u8>,
    pub pulse_secs: Option<Duration>,
    pub last_advance: Instant,
    pub seed_counter: u32,
    /// Live particle state for the `Drift` pattern. `None` for any other
    /// pattern choice. Ticked on each idle poll in the event loop.
    pub drift: Option<drift::DriftState>,
    /// When true, the settings menu overlays the pulse.
    pub menu_open: bool,
    /// Which row of the settings menu is highlighted (`0..menu::ROW_COUNT`).
    pub menu_cursor: usize,
    /// Days since `IWIYWI_SOBER_SINCE`, computed at startup. None if unset.
    pub sobriety_days: Option<i64>,
    /// When true, auto-advance is suspended (`space` toggles).
    pub paused: bool,
    /// When true, the help overlay is showing.
    pub help_open: bool,
    /// Favorited items persisted to `~/.iwiywi/favorites.json`.
    pub favorites: pulse::favorites::Favorites,
    /// A transient toast shown briefly after an action (copy, export,
    /// favorite toggle, daily summary). Tuple: `(text, set_at, ttl)`. Rendered
    /// in the status footer. TTL lets the daily summary linger longer than
    /// the default 1.5s action-feedback toasts.
    pub toast: Option<(String, std::time::Instant, Duration)>,
    /// Open overlay (explain-current or step meditation). None = hidden.
    pub ai_overlay: Option<AiOverlay>,
    /// Receiver for in-flight AI call. `try_recv` polled on each idle tick.
    pub ai_rx: Option<mpsc::Receiver<AiOutcome>>,
    /// `reqwest::Client` reused across AI calls. None when startup build failed
    /// (no network, broken TLS, etc.) — `a` and step meditations then show
    /// an "AI unavailable" toast instead of opening an overlay.
    pub ai_client: Option<reqwest::Client>,
    /// Gateway config (model, url, `api_version`) cloned into every AI thread.
    pub ai_config: Config,
    /// `(step, pressed_at)` — second tap within `STEP_DOUBLE_TAP_MS` triggers
    /// the AI meditation overlay instead of a second focus set.
    pub last_step_press: Option<(u8, Instant)>,
    /// Fullscreen quote-wall mode. Suppresses drift, status bar, clock,
    /// moon/sober anchor. Body fills the frame. Toggled with `F`.
    pub showcase: bool,
    /// Time of the last key press — drives the idle dim-down and also
    /// lets the time-of-day palette auto-drift respond to user activity.
    pub last_input: Instant,
    /// True when the user set `IWIYWI_PALETTE=auto`; re-derives the
    /// variant from the current hour every ~60s.
    pub palette_auto: bool,
}

const IDLE_DIM_AFTER: Duration = Duration::from_secs(300);
const IDLE_DIM_FACTOR: f32 = 0.32;

impl App {
    pub fn rebuild_mixer(&mut self) {
        self.mixer = PulseMixer::from_sources_focused(
            &self.sources,
            self.focus_step,
            self.order,
            self.focus,
        );
    }

    pub fn next(&mut self) {
        let s = self.next_seed();
        self.mixer.advance_per_order(self.order, s);
        self.last_advance = Instant::now();
    }

    pub fn prev(&mut self) {
        if self.mixer.is_empty() { return; }
        let len = self.mixer.len();
        for _ in 0..len.saturating_sub(1) { self.mixer.advance(); }
        self.last_advance = Instant::now();
    }

    pub fn random(&mut self) {
        let s = self.next_seed();
        self.mixer.random_jump(s);
        self.last_advance = Instant::now();
    }

    pub fn set_step_focus(&mut self, step: u8) {
        self.focus_step = Some(step);
        self.rebuild_mixer();
        // Jump to a random item in the filtered mixer so the user gets
        // immediate visual feedback that focus changed (not the same item).
        let s = self.next_seed();
        self.mixer.random_jump(s);
        self.last_advance = Instant::now();
    }

    pub fn clear_step_focus(&mut self) {
        self.focus_step = None;
        self.rebuild_mixer();
        let s = self.next_seed();
        self.mixer.random_jump(s);
        self.last_advance = Instant::now();
    }

    fn next_seed(&mut self) -> u32 {
        self.seed_counter = self.seed_counter.wrapping_add(1);
        self.seed_counter
    }

    pub fn toggle_favorite(&mut self) {
        let Some(item) = self.mixer.current() else { return; };
        let item = item.clone();
        let msg = if self.favorites.toggle(&item) { "★ saved" } else { "★ removed" };
        self.toast = Some((msg.to_string(), Instant::now(), Duration::from_millis(1500)));
        // The mixer's Favorites source is a separate snapshot of the file,
        // so refresh it from disk before rebuilding so Focus::Favorites sees
        // the toggle immediately.
        if let Some(last) = self.sources.last_mut() {
            *last = Box::new(pulse::favorites::Favorites::load_from(
                config::config_dir().join("favorites.json"),
            ));
        }
        self.rebuild_mixer();
    }

    pub fn copy_current(&mut self) {
        let Some(item) = self.mixer.current() else { return; };
        let text = format!("{}\n\n{}\n", item.label, item.body);
        let ok = clipboard::copy(&text);
        let msg = if ok { "copied" } else { "no clipboard available" };
        self.toast = Some((msg.to_string(), Instant::now(), Duration::from_millis(1500)));
    }

    pub fn export_current(&mut self) {
        let exports_dir = config::config_dir().join("exports");
        let msg = match export::write_current(&self.mixer, exports_dir) {
            Some(path) => format!("exported → {}", path.file_name().and_then(|n| n.to_str()).unwrap_or("file")),
            None => "export failed".to_string(),
        };
        self.toast = Some((msg, Instant::now(), Duration::from_millis(1500)));
    }

    /// Open the AI-explanation overlay for the current pulse item. Cache-hit
    /// returns synchronously in Ready state; cache-miss spawns a background
    /// thread and leaves the overlay in Loading until the main loop polls
    /// `ai_rx` and applies the outcome.
    pub fn explain_current(&mut self) {
        let Some(item) = self.mixer.current() else { return; };
        let title = format!("Why this matters — {}", item.kind.display_label());
        let cache_dir = config::config_dir().join("ai_cache").join("explain");
        let cache_key = sha_hex(&item.body);
        if let Some(cached) = read_cache_file(&cache_dir.join(format!("{cache_key}.txt"))) {
            let mut overlay = AiOverlay::loading(title);
            overlay.apply_outcome(AiOutcome::Text(cached));
            self.ai_overlay = Some(overlay);
            return;
        }
        let Some(client) = self.ai_client.clone() else {
            self.set_toast("AI unavailable", 2000);
            return;
        };
        self.ai_overlay = Some(AiOverlay::loading(title));
        let system = EXPLAIN_SYSTEM_PROMPT.to_string();
        let user = format!(
            "Reading ({}):\n\n{}",
            item.kind.display_label(),
            item.body,
        );
        let cache_path = cache_dir.join(format!("{cache_key}.txt"));
        let config = self.ai_config.clone();
        let opts = ChatOpts { max_tokens: Some(300), temperature: Some(0.4), json_mode: false };
        self.spawn_ai(client, config, system, user, opts, Some(cache_path));
    }

    /// Open a step meditation overlay for step `n`. Per-day cache key.
    pub fn meditate_step(&mut self, step: u8) {
        if !(1..=12).contains(&step) { return; }
        let today = chrono::Local::now().date_naive();
        let title = format!("Meditation on Step {step}");
        let cache_dir = config::config_dir().join("ai_cache").join("meditations");
        let cache_path = cache_dir.join(format!("step-{step}-{today}.txt"));
        if let Some(cached) = read_cache_file(&cache_path) {
            let mut overlay = AiOverlay::loading(title);
            overlay.apply_outcome(AiOutcome::Text(cached));
            self.ai_overlay = Some(overlay);
            return;
        }
        let Some(client) = self.ai_client.clone() else {
            self.set_toast("AI unavailable", 2000);
            return;
        };
        self.ai_overlay = Some(AiOverlay::loading(title));
        let system = MEDITATION_SYSTEM_PROMPT.to_string();
        let user = format!(
            "Today is {today}. Write a ~150-word meditation on applying Step {step} \
             in an ordinary day of recovery. Ground it in something practical.",
        );
        let config = self.ai_config.clone();
        let opts = ChatOpts { max_tokens: Some(400), temperature: Some(0.7), json_mode: false };
        self.spawn_ai(client, config, system, user, opts, Some(cache_path));
    }

    fn spawn_ai(
        &mut self,
        client: reqwest::Client,
        config: Config,
        system: String,
        user: String,
        opts: ChatOpts,
        cache_path: Option<PathBuf>,
    ) {
        let (tx, rx) = mpsc::channel();
        self.ai_rx = Some(rx);
        std::thread::spawn(move || {
            let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
                Ok(rt) => rt,
                Err(e) => {
                    let _ = tx.send(AiOutcome::Error(format!("runtime: {e}")));
                    return;
                }
            };
            let outcome = rt.block_on(async {
                match post_chat(&client, &config, &system, &user, opts).await {
                    Ok(text) => {
                        let trimmed = text.trim().to_string();
                        if let Some(path) = cache_path.as_ref() {
                            let _ = write_cache_file(path, &trimmed);
                        }
                        AiOutcome::Text(trimmed)
                    }
                    Err(e) => AiOutcome::Error(format!("{e}")),
                }
            });
            let _ = tx.send(outcome);
        });
    }

    /// Poll the AI receiver; if a result landed, apply it to the overlay
    /// and clear the channel. Called once per event-loop idle tick.
    pub fn poll_ai(&mut self) {
        let Some(rx) = &self.ai_rx else { return; };
        match rx.try_recv() {
            Ok(outcome) => {
                if let Some(overlay) = self.ai_overlay.as_mut() {
                    overlay.apply_outcome(outcome);
                }
                self.ai_rx = None;
            }
            Err(mpsc::TryRecvError::Empty) => {}
            Err(mpsc::TryRecvError::Disconnected) => {
                if let Some(overlay) = self.ai_overlay.as_mut() {
                    overlay.apply_outcome(AiOutcome::Error("AI thread exited unexpectedly".to_string()));
                }
                self.ai_rx = None;
            }
        }
    }

    pub fn close_overlay(&mut self) {
        self.ai_overlay = None;
        // Intentionally keep `ai_rx` alive so in-flight responses don't
        // panic on send into a dropped channel — the next poll will just
        // discard the result since `ai_overlay` is None.
    }

    fn set_toast(&mut self, msg: &str, ttl_ms: u64) {
        self.toast = Some((msg.to_string(), Instant::now(), Duration::from_millis(ttl_ms)));
    }

    /// Produce a journal seed question: cache hit → immediate; cache miss
    /// with a client → blocking gateway call up to ~6s; any miss → None
    /// so `journal::open_today` falls back to the static prompt.
    pub fn journal_seed(&self) -> Option<String> {
        let item = self.mixer.current()?;
        let today = chrono::Local::now().date_naive();
        let step = item.step.unwrap_or(0);
        let cache_dir = config::config_dir().join("ai_cache").join("journal");
        let cache_path = cache_dir.join(format!("{today}-step-{step}.txt"));
        if let Some(cached) = read_cache_file(&cache_path) {
            return Some(cached);
        }
        let client = self.ai_client.clone()?;
        let config = self.ai_config.clone();
        let system = JOURNAL_SEED_SYSTEM_PROMPT.to_string();
        let user = format!(
            "Today's reading ({}, step {step}):\n\n{}\n\nWrite one reflection question.",
            item.kind.display_label(),
            item.body,
        );
        let opts = ChatOpts { max_tokens: Some(60), temperature: Some(0.5), json_mode: false };
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().ok()?;
        let result = rt.block_on(async move {
            post_chat(&client, &config, &system, &user, opts).await
        }).ok()?;
        let seed = result.trim().to_string();
        if seed.is_empty() { return None; }
        let _ = write_cache_file(&cache_path, &seed);
        Some(seed)
    }

    /// A digit key: first press focuses on the step; second press on the
    /// same digit within `STEP_DOUBLE_TAP_MS` opens the AI meditation overlay.
    pub fn handle_step_key(&mut self, step: u8) {
        let now = Instant::now();
        let is_double = matches!(
            self.last_step_press,
            Some((s, t)) if s == step && now.duration_since(t).as_millis() < STEP_DOUBLE_TAP_MS
        );
        if is_double {
            self.meditate_step(step);
            self.last_step_press = None;
        } else {
            self.set_step_focus(step);
            self.last_step_press = Some((step, now));
        }
    }

    pub fn menu_row_prev(&mut self) {
        self.menu_cursor = (self.menu_cursor + menu::ROW_COUNT - 1) % menu::ROW_COUNT;
    }

    pub fn menu_row_next(&mut self) {
        self.menu_cursor = (self.menu_cursor + 1) % menu::ROW_COUNT;
    }

    /// Cycle the currently-highlighted setting. `delta` is +1 or -1.
    pub fn menu_cycle(&mut self, delta: i32, size_w: u16, size_h: u16) {
        match menu::Row::by_index(self.menu_cursor) {
            menu::Row::Palette => {
                let next = pulse::cycle(&palette::Variant::ALL, self.palette.variant, delta);
                self.palette = palette::Palette::build(self.palette.mode, next);
            }
            menu::Row::Pattern => {
                let next = pulse::cycle(&pattern::Pattern::ALL, self.pattern, delta);
                self.pattern = next;
                // Spin up or swap the animated particle field to match the
                // new pattern's physics; tear down for static patterns.
                if next.is_animated() {
                    let seed = self.next_seed();
                    self.drift = Some(drift::DriftState::with_mode(
                        size_w, size_h, seed, next.drift_mode(),
                    ));
                } else {
                    self.drift = None;
                }
            }
            menu::Row::TextSize => {
                let next = pulse::cycle(&text_size::TextSize::ALL, self.text_size, delta);
                self.text_size = next;
            }
            menu::Row::Order => {
                let next = pulse::cycle(&Order::ALL, self.order, delta);
                self.order = next;
                self.rebuild_mixer();
            }
            menu::Row::Focus => {
                let next = pulse::cycle(&Focus::ALL_VARIANTS, self.focus, delta);
                self.focus = next;
                self.rebuild_mixer();
                // The menu overlay covers much of the pulse, so the Focus
                // change wasn't visually obvious before. Jump to a random
                // item in the filtered pool AND flash a toast with the
                // new count so the effect is unmistakable.
                let seed = self.next_seed();
                self.mixer.random_jump(seed);
                let count = self.mixer.len();
                self.toast = Some((
                    format!("Focus: {} · {count} item{}",
                        next.label(), if count == 1 { "" } else { "s" }),
                    Instant::now(),
                    Duration::from_millis(2500),
                ));
            }
            menu::Row::PulseSecs => {
                let current = self.pulse_secs.map_or(0u64, |d| d.as_secs());
                let next = pulse::cycle(&menu::PULSE_SECS_RING, current, delta);
                self.pulse_secs = if next == 0 { None } else { Some(Duration::from_secs(next)) };
                self.last_advance = Instant::now();
            }
        }
    }

    pub fn current_menu_values(&self) -> [String; menu::ROW_COUNT] {
        [
            self.palette.variant.label().to_string(),
            self.pattern.label().to_string(),
            self.text_size.label().to_string(),
            self.order.label().to_string(),
            self.focus.label().to_string(),
            self.pulse_secs.map_or("manual".to_string(), |d| d.as_secs().to_string()),
        ]
    }
}

pub fn run(
    grapevine_html: Option<String>,
    bill: pulse::bill::BillReflection,
    community: pulse::community::CommunityPulse,
    daily_summary: Option<String>,
) -> Result<()> {
    let cfg = config::load_config()?;
    let readings = read_readings()?;

    let today_basename = format!("readings-{}.json", chrono::Local::now().format("%Y-%m-%d"));
    let sources: Vec<Box<dyn PulseSource>> = vec![
        Box::new(pulse::today::TodayReadings::from_readings(&readings)),
        Box::new(pulse::historical::HistoricalReadings::load_from(
            &config::config_dir(), &today_basename,
        )),
        Box::new(pulse::bundled::BigBookQuotes::load()),
        Box::new(pulse::bundled::Prayers::load()),
        Box::new(pulse::bundled::StepExplainers::load()),
        Box::new(pulse::bundled::Traditions::load()),
        Box::new(pulse::bundled::Concepts::load()),
        Box::new(pulse::bundled::Slogans::load()),
        Box::new(pulse::grapevine::Grapevine::from_html(grapevine_html.as_deref())),
        Box::new(bill),
        Box::new(community),
        Box::new(pulse::favorites::Favorites::load_from(
            config::config_dir().join("favorites.json"),
        )),
    ];

    let focus = pulse::focus_from_env();
    let order = pulse::order_from_env();
    let palette = palette::from_env();
    let pattern = pattern::from_env();
    let text_size = text_size::from_env();
    let pulse_secs = config::pulse_secs();
    let _ = cfg;

    let mut mixer = PulseMixer::from_sources_focused(&sources, None, order, focus);
    // Start on a random item so the first thing you see isn't always today's
    // first reading. Without this, cursor=0 ⇒ first source's first item.
    if order == Order::Random {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(1);
        mixer.random_jump(seed);
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Seed the drift particle field for any animated pattern. Non-animated
    // patterns (none/dots/frame/rule) leave `drift` as None.
    let initial_size = terminal.size()?;
    let drift = if pattern.is_animated() {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.subsec_nanos())
            .unwrap_or(1);
        Some(drift::DriftState::with_mode(
            initial_size.width, initial_size.height, seed, pattern.drift_mode(),
        ))
    } else {
        None
    };

    // Seed the toast with the AI-generated daily summary (if we got one)
    // so it's visible for ~4s on first render — long enough to read.
    let initial_toast = daily_summary.map(|s| {
        (s, Instant::now(), Duration::from_secs(4))
    });

    let mut app = App {
        mixer,
        sources,
        palette,
        pattern,
        text_size,
        order,
        focus,
        focus_step: None,
        pulse_secs,
        last_advance: Instant::now(),
        seed_counter: 1,
        drift,
        menu_open: false,
        menu_cursor: 0,
        sobriety_days: config::sobriety_days(),
        paused: false,
        help_open: false,
        favorites: pulse::favorites::Favorites::load_from(
            config::config_dir().join("favorites.json"),
        ),
        toast: initial_toast,
        ai_overlay: None,
        ai_rx: None,
        ai_client: reqwest::Client::builder()
            .timeout(Duration::from_secs(12))
            .build()
            .ok(),
        ai_config: cfg.clone(),
        last_step_press: None,
        showcase: false,
        last_input: Instant::now(),
        palette_auto: palette::auto_requested(),
    };

    loop {
        let size = terminal.size()?;
        // Expire the toast once its per-toast TTL elapses.
        if let Some((_, t, ttl)) = &app.toast {
            if t.elapsed() > *ttl {
                app.toast = None;
            }
        }
        // Compute the effective palette each frame: idle dim kicks in after
        // IDLE_DIM_AFTER with no keypress. Palette-auto re-derives the
        // variant from the current hour so the display drifts through the
        // day. Both layer over app.palette without mutating it.
        let idle = app.last_input.elapsed() > IDLE_DIM_AFTER;
        let mut eff_palette = if app.palette_auto {
            palette::Palette::build(
                app.palette.mode,
                palette::auto_variant(chrono::Timelike::hour(&chrono::Local::now())),
            )
        } else {
            app.palette
        };
        if idle { eff_palette = eff_palette.dim(IDLE_DIM_FACTOR); }

        terminal.draw(|f| {
            let eff_pattern = if app.showcase { pattern::Pattern::None } else { app.pattern };
            let eff_drift = if app.showcase { None } else { app.drift.as_ref() };
            widgets::render_pulse(
                f, app.mixer.current(), &eff_palette,
                eff_pattern, eff_drift, app.text_size, app.showcase,
            );
            // Showcase mode: suppress status line, clock, moon anchor, and
            // the menu/help overlays. Keep the AI overlay so `a` still works.
            if !app.showcase {
                let frame_area = f.area();
                {
                    let buf = f.buffer_mut();
                    clock::draw(buf, frame_area, &eff_palette);
                    status::draw_moon_anchor(
                        buf, frame_area, &eff_palette, app.sobriety_days,
                    );
                }
                let progress = if app.paused {
                    None
                } else {
                    app.pulse_secs.map(|interval| {
                        (app.last_advance.elapsed().as_secs_f32() / interval.as_secs_f32()).clamp(0.0, 1.0)
                    })
                };
                let toast = app.toast.as_ref().map(|(msg, _, _)| msg.as_str());
                let status_line = status::StatusLine {
                    mixer: &app.mixer,
                    focus: app.focus,
                    focus_step: app.focus_step,
                    pulse_progress: progress,
                    sobriety_days: app.sobriety_days,
                    paused: app.paused,
                    toast,
                };
                status::render(f, &eff_palette, &status_line);
                if app.menu_open {
                    menu::render(f, &eff_palette, app.menu_cursor, app.current_menu_values());
                }
                if app.help_open {
                    help::render(f, &eff_palette);
                }
            }
            if let Some(ov) = app.ai_overlay.as_mut() {
                overlay::render(f, &eff_palette, ov);
            }
        })?;

        // Apply any completed AI call to the open overlay.
        app.poll_ai();

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
                // Any keypress wakes the UI from idle dim-down and resets
                // the idle timer.
                app.last_input = Instant::now();
                if app.help_open {
                    app.help_open = false;
                    continue;
                }
                if app.ai_overlay.is_some() {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('a' | 'q') => {
                            app.close_overlay();
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            if let Some(ov) = app.ai_overlay.as_mut() { ov.scroll_down(); }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            if let Some(ov) = app.ai_overlay.as_mut() { ov.scroll_up(); }
                        }
                        _ => {}
                    }
                    continue;
                }
                if app.menu_open {
                    match key.code {
                        KeyCode::Char('m') | KeyCode::Esc => { app.menu_open = false; continue; }
                        KeyCode::Up    => { app.menu_row_prev(); continue; }
                        KeyCode::Down  => { app.menu_row_next(); continue; }
                        KeyCode::Left  => { app.menu_cycle(-1, size.width, size.height); continue; }
                        KeyCode::Right => { app.menu_cycle( 1, size.width, size.height); continue; }
                        // Any other key closes the menu and falls through to
                        // the normal handler below (so `q`, `n`, `r`, step
                        // focus digits all still work from within the menu).
                        _ => { app.menu_open = false; }
                    }
                }
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('m') => app.menu_open = true,
                    KeyCode::Char('?') => app.help_open = true,
                    KeyCode::Char('n') => app.next(),
                    KeyCode::Char('p') => app.prev(),
                    KeyCode::Char('r') => app.random(),
                    KeyCode::Char(' ') => {
                        app.paused = !app.paused;
                        if !app.paused { app.last_advance = Instant::now(); }
                    }
                    KeyCode::Char('a') => app.explain_current(),
                    KeyCode::Char('F') => app.showcase = !app.showcase,
                    KeyCode::Char('f') => app.toggle_favorite(),
                    KeyCode::Char('c') => app.copy_current(),
                    KeyCode::Char('e') => app.export_current(),
                    KeyCode::Char('j') => {
                        let dir = config::config_dir().join("journal");
                        let seed = app.journal_seed();
                        match journal::open_today(dir, seed) {
                            Ok(p) => app.toast = Some((
                                format!("wrote {}", p.file_name().and_then(|n| n.to_str()).unwrap_or("entry")),
                                Instant::now(),
                                Duration::from_millis(1500),
                            )),
                            Err(e) => app.toast = Some((
                                format!("journal: {e}"),
                                Instant::now(),
                                Duration::from_millis(1500),
                            )),
                        }
                        // Force a full redraw now that we're back from the editor.
                        terminal.clear()?;
                    }
                    KeyCode::Char('1') => app.handle_step_key(1),
                    KeyCode::Char('2') => app.handle_step_key(2),
                    KeyCode::Char('3') => app.handle_step_key(3),
                    KeyCode::Char('4') => app.handle_step_key(4),
                    KeyCode::Char('5') => app.handle_step_key(5),
                    KeyCode::Char('6') => app.handle_step_key(6),
                    KeyCode::Char('7') => app.handle_step_key(7),
                    KeyCode::Char('8') => app.handle_step_key(8),
                    KeyCode::Char('9') => app.handle_step_key(9),
                    KeyCode::Char('0') => app.handle_step_key(10),
                    KeyCode::Char('-') => app.handle_step_key(11),
                    KeyCode::Char('=') => app.handle_step_key(12),
                    KeyCode::Char('*') => app.clear_step_focus(),
                    _ => {}
                }
            }
        } else {
            if let Some(state) = app.drift.as_mut() {
                state.tick(size.width, size.height);
            }
            if !app.paused {
                if let Some(interval) = app.pulse_secs {
                    if app.last_advance.elapsed() >= interval {
                        app.next();
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ClassifiedReading;

    fn fixture_app() -> App {
        let sources: Vec<Box<dyn PulseSource>> = vec![
            Box::new(pulse::today::TodayReadings::from_readings(&[
                ClassifiedReading {
                    step: 1, reason: "r".to_string(), source: "src".to_string(),
                    title: "t".to_string(), text: "today body".to_string(),
                    url: "http://x".to_string(),
                },
                ClassifiedReading {
                    step: 3, reason: "r".to_string(), source: "src".to_string(),
                    title: "t".to_string(), text: "another today body".to_string(),
                    url: "http://x".to_string(),
                },
            ])),
        ];
        let mixer = PulseMixer::from_sources(&sources, None, Order::Random);
        App {
            mixer,
            sources,
            palette: palette::Palette::build(palette::Mode::Dark, palette::Variant::Default),
            pattern: pattern::Pattern::None,
            text_size: text_size::TextSize::Normal,
            order: Order::Random,
            focus: Focus::All,
            focus_step: None,
            pulse_secs: Some(Duration::from_secs(20)),
            last_advance: Instant::now(),
            seed_counter: 1,
            drift: None,
            menu_open: false,
            menu_cursor: 0,
            sobriety_days: None,
            paused: false,
            help_open: false,
            favorites: pulse::favorites::Favorites::load_from(
                std::path::PathBuf::from("/tmp/iwiywi-test-favorites.json"),
            ),
            toast: None,
            ai_overlay: None,
            ai_rx: None,
            ai_client: None,
            ai_config: Config::default(),
            last_step_press: None,
            showcase: false,
            last_input: Instant::now(),
            palette_auto: false,
        }
    }

    #[test]
    fn next_advances_cursor() {
        let mut app = fixture_app();
        let before = app.mixer.cursor();
        app.next();
        assert_ne!(app.mixer.cursor(), before);
    }

    #[test]
    fn set_step_focus_rebuilds_mixer_to_only_that_step() {
        let mut app = fixture_app();
        app.set_step_focus(1);
        for i in app.mixer.all() {
            assert_eq!(i.step, Some(1));
        }
    }

    #[test]
    fn clear_step_focus_restores_full_mixer() {
        let mut app = fixture_app();
        app.set_step_focus(1);
        let focused = app.mixer.len();
        app.clear_step_focus();
        assert!(app.mixer.len() > focused);
    }

    #[test]
    fn random_changes_cursor_when_len_ge_two() {
        let mut app = fixture_app();
        let start = app.mixer.cursor();
        app.random();
        assert_ne!(app.mixer.cursor(), start);
    }
}
