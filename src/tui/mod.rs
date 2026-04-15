pub mod palette;
pub mod pattern;
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
    pub focus_step: Option<u8>,
    pub pulse_secs: Option<Duration>,
    pub last_advance: Instant,
    pub seed_counter: u32,
}

impl App {
    pub fn rebuild_mixer(&mut self) {
        self.mixer = PulseMixer::from_sources(&self.sources, self.focus_step, self.order);
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
        self.last_advance = Instant::now();
    }

    pub fn clear_step_focus(&mut self) {
        self.focus_step = None;
        self.rebuild_mixer();
        self.last_advance = Instant::now();
    }

    fn next_seed(&mut self) -> u32 {
        self.seed_counter = self.seed_counter.wrapping_add(1);
        self.seed_counter
    }
}

pub fn run() -> Result<()> {
    let cfg = config::load_config()?;
    let readings = read_readings()?;

    let today_basename = format!("readings-{}.json", chrono::Local::now().format("%Y-%m-%d"));
    let mut sources: Vec<Box<dyn PulseSource>> = vec![
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
        Box::new(pulse::grapevine::Grapevine::from_html(None)),
    ];

    let focus = pulse::focus_from_env();
    if focus != Focus::All {
        sources.retain(|s| focus.admits(s.name()));
    }

    let order = pulse::order_from_env();
    let palette = palette::from_env();
    let pattern = pattern::from_env();
    let pulse_secs = config::pulse_secs();
    let _ = cfg;

    let mixer = PulseMixer::from_sources(&sources, None, order);

    let mut app = App {
        mixer,
        sources,
        palette,
        pattern,
        order,
        focus_step: None,
        pulse_secs,
        last_advance: Instant::now(),
        seed_counter: 1,
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| widgets::render_pulse(f, app.mixer.current(), &app.palette, app.pattern))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press { continue; }
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('n') => app.next(),
                    KeyCode::Char('p') => app.prev(),
                    KeyCode::Char('r') => app.random(),
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
        } else if let Some(interval) = app.pulse_secs {
            if app.last_advance.elapsed() >= interval {
                app.next();
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
            focus_step: None,
            pulse_secs: Some(Duration::from_secs(20)),
            last_advance: Instant::now(),
            seed_counter: 1,
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
