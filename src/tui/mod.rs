pub mod drift;
pub mod menu;
pub mod palette;
pub mod pattern;
pub mod status;
pub mod widgets;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::{Duration, Instant};

use crate::config;
use crate::pulse::{self, Focus, Order, PulseMixer, PulseSource};
use crate::storage::read_readings;

pub struct App {
    pub mixer: PulseMixer,
    pub sources: Vec<Box<dyn PulseSource>>,
    pub palette: palette::Palette,
    pub pattern: pattern::Pattern,
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
    /// Which row of the settings menu is highlighted (0..menu::ROW_COUNT).
    pub menu_cursor: usize,
    /// Days since `IWIYWI_SOBER_SINCE`, computed at startup. None if unset.
    pub sobriety_days: Option<i64>,
    /// When true, auto-advance is suspended (`space` toggles).
    pub paused: bool,
}

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
                // Spin up / tear down the drift particle field to match.
                if next == pattern::Pattern::Drift && self.drift.is_none() {
                    self.drift = Some(drift::DriftState::new(size_w, size_h, self.next_seed()));
                } else if next != pattern::Pattern::Drift {
                    self.drift = None;
                }
            }
            menu::Row::Order => {
                let next = pulse::cycle(&Order::ALL, self.order, delta);
                self.order = next;
                self.rebuild_mixer();
            }
            menu::Row::Focus => {
                let next = pulse::cycle(&Focus::ALL_VARIANTS, self.focus, delta);
                self.focus = next;
                self.rebuild_source_filter();
                self.rebuild_mixer();
            }
            menu::Row::PulseSecs => {
                let current = self.pulse_secs.map_or(0u64, |d| d.as_secs());
                let next = pulse::cycle(&menu::PULSE_SECS_RING, current, delta);
                self.pulse_secs = if next == 0 { None } else { Some(Duration::from_secs(next)) };
                self.last_advance = Instant::now();
            }
        }
    }

    /// After a Focus change, rebuild the sources vec so only admitted ones
    /// feed the mixer. Called from `menu_cycle` when Focus changes.
    fn rebuild_source_filter(&mut self) {
        // The source set is fixed-at-startup; we keep a canonical copy of
        // all sources and filter a view for the mixer. For now, Focus is
        // applied during `from_sources` by matching on source.name() via
        // the admits filter in PulseMixer. We only need to rebuild when the
        // source list itself changes — but since we don't drop sources at
        // runtime in pulse-only mode, this is a no-op stub. Kept as a hook
        // so `menu_cycle` reads symmetrically for Focus.
    }

    pub fn current_menu_values(&self) -> [String; menu::ROW_COUNT] {
        [
            self.palette.variant.label().to_string(),
            self.pattern.label().to_string(),
            self.order.label().to_string(),
            self.focus.label().to_string(),
            self.pulse_secs.map_or("manual".to_string(), |d| d.as_secs().to_string()),
        ]
    }
}

pub fn run(grapevine_html: Option<String>) -> Result<()> {
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
    ];

    let focus = pulse::focus_from_env();
    let order = pulse::order_from_env();
    let palette = palette::from_env();
    let pattern = pattern::from_env();
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

    // Seed the drift particle field from the initial terminal size if the
    // chosen pattern is Drift. Other patterns leave `drift` as None.
    let initial_size = terminal.size()?;
    let drift = if pattern == pattern::Pattern::Drift {
        Some(drift::DriftState::new(
            initial_size.width,
            initial_size.height,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.subsec_nanos())
                .unwrap_or(1),
        ))
    } else {
        None
    };

    let mut app = App {
        mixer,
        sources,
        palette,
        pattern,
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
    };

    loop {
        let size = terminal.size()?;
        terminal.draw(|f| {
            widgets::render_pulse(f, app.mixer.current(), &app.palette, app.pattern, app.drift.as_ref());
            let progress = if app.paused {
                None
            } else {
                app.pulse_secs.map(|interval| {
                    (app.last_advance.elapsed().as_secs_f32() / interval.as_secs_f32()).clamp(0.0, 1.0)
                })
            };
            let status_line = status::StatusLine {
                mixer: &app.mixer,
                focus: app.focus,
                focus_step: app.focus_step,
                pulse_progress: progress,
                sobriety_days: app.sobriety_days,
                paused: app.paused,
            };
            status::render(f, &app.palette, &status_line);
            if app.menu_open {
                menu::render(f, &app.palette, app.menu_cursor, app.current_menu_values());
            }
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
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
                    KeyCode::Char('n') => app.next(),
                    KeyCode::Char('p') => app.prev(),
                    KeyCode::Char('r') => app.random(),
                    KeyCode::Char(' ') => {
                        app.paused = !app.paused;
                        if !app.paused { app.last_advance = Instant::now(); }
                    }
                    KeyCode::Char('1') => app.set_step_focus(1),
                    KeyCode::Char('2') => app.set_step_focus(2),
                    KeyCode::Char('3') => app.set_step_focus(3),
                    KeyCode::Char('4') => app.set_step_focus(4),
                    KeyCode::Char('5') => app.set_step_focus(5),
                    KeyCode::Char('6') => app.set_step_focus(6),
                    KeyCode::Char('7') => app.set_step_focus(7),
                    KeyCode::Char('8') => app.set_step_focus(8),
                    KeyCode::Char('9') => app.set_step_focus(9),
                    KeyCode::Char('0') => app.set_step_focus(10),
                    KeyCode::Char('-') => app.set_step_focus(11),
                    KeyCode::Char('=') => app.set_step_focus(12),
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
