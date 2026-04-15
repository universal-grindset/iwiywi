pub mod commands;
pub mod drift;
pub mod qr;
pub mod theme;
pub mod widgets;

use crate::models::ClassifiedReading;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Command(String),
    QrOverlay,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    All,
    Steps,
    Help,
}

impl Tab {
    pub fn label(&self) -> &'static str {
        match self {
            Tab::All => "All",
            Tab::Steps => "Steps",
            Tab::Help => "Help",
        }
    }

    pub fn key(&self) -> char {
        match self {
            Tab::All => 'a',
            Tab::Steps => 's',
            Tab::Help => '?',
        }
    }

    pub fn next(&self) -> Tab {
        match self {
            Tab::All => Tab::Steps,
            Tab::Steps => Tab::Help,
            Tab::Help => Tab::All,
        }
    }

    pub fn prev(&self) -> Tab {
        match self {
            Tab::All => Tab::Help,
            Tab::Steps => Tab::All,
            Tab::Help => Tab::Steps,
        }
    }
}

pub struct App {
    pub readings: Vec<ClassifiedReading>,
    pub scroll: usize,
    pub mode: Mode,
    pub tab: Tab,
    pub step_filter: u8,
    pub qr_url: String,
    pub theme: theme::Theme,
}

impl App {
    pub fn new(
        readings: Vec<ClassifiedReading>,
        qr_url: String,
        theme: theme::Theme,
    ) -> Self {
        App {
            readings,
            scroll: 0,
            mode: Mode::Normal,
            tab: Tab::All,
            step_filter: 1,
            qr_url,
            theme,
        }
    }

    fn visible_len(&self) -> usize {
        match self.tab {
            Tab::All => self.readings.len(),
            Tab::Steps => self
                .readings
                .iter()
                .filter(|r| r.step == self.step_filter)
                .count(),
            Tab::Help => 0,
        }
    }

    pub fn scroll_down(&mut self) {
        let max = self.visible_len().saturating_sub(1);
        if self.scroll < max {
            self.scroll += 1;
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn set_tab(&mut self, tab: Tab) {
        if self.tab != tab {
            self.tab = tab;
            self.scroll = 0;
        }
    }

    pub fn next_tab(&mut self) { self.set_tab(self.tab.next()); }
    pub fn prev_tab(&mut self) { self.set_tab(self.tab.prev()); }

    pub fn step_next(&mut self) {
        self.step_filter = if self.step_filter >= 12 { 1 } else { self.step_filter + 1 };
        self.scroll = 0;
    }

    pub fn step_prev(&mut self) {
        self.step_filter = if self.step_filter <= 1 { 12 } else { self.step_filter - 1 };
        self.scroll = 0;
    }

    pub fn enter_command_mode(&mut self) {
        self.mode = Mode::Command(String::new());
    }

    pub fn push_command_char(&mut self, c: char) {
        if let Mode::Command(ref mut s) = self.mode {
            s.push(c);
        }
    }

    pub fn pop_command_char(&mut self) {
        if let Mode::Command(ref mut s) = self.mode {
            s.pop();
        }
    }

    pub fn dismiss(&mut self) {
        self.mode = Mode::Normal;
    }

    pub fn toggle_qr(&mut self) {
        self.mode = match self.mode {
            Mode::QrOverlay => Mode::Normal,
            _ => Mode::QrOverlay,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        )
    }

    #[test]
    fn scroll_down_increments_within_bounds() {
        let mut app = fixture_app();
        assert_eq!(app.scroll, 0);
        app.scroll_down();
        assert_eq!(app.scroll, 1);
        app.scroll_down();
        assert_eq!(app.scroll, 1);
    }

    #[test]
    fn scroll_up_does_not_underflow() {
        let mut app = fixture_app();
        app.scroll_up();
        assert_eq!(app.scroll, 0);
    }

    #[test]
    fn enter_command_mode_sets_mode() {
        let mut app = fixture_app();
        app.enter_command_mode();
        assert!(matches!(app.mode, Mode::Command(_)));
    }

    #[test]
    fn push_and_pop_command_chars() {
        let mut app = fixture_app();
        app.enter_command_mode();
        app.push_command_char('q');
        app.push_command_char('r');
        assert!(matches!(&app.mode, Mode::Command(s) if s == "qr"));
        app.pop_command_char();
        assert!(matches!(&app.mode, Mode::Command(s) if s == "q"));
    }

    #[test]
    fn dismiss_returns_to_normal() {
        let mut app = fixture_app();
        app.mode = Mode::QrOverlay;
        app.dismiss();
        assert_eq!(app.mode, Mode::Normal);
    }

    #[test]
    fn toggle_qr_from_normal_sets_overlay() {
        let mut app = fixture_app();
        app.toggle_qr();
        assert_eq!(app.mode, Mode::QrOverlay);
    }

    #[test]
    fn toggle_qr_from_overlay_returns_normal() {
        let mut app = fixture_app();
        app.mode = Mode::QrOverlay;
        app.toggle_qr();
        assert_eq!(app.mode, Mode::Normal);
    }

    #[test]
    fn next_tab_cycles_through_all_three() {
        let mut app = fixture_app();
        assert_eq!(app.tab, Tab::All);
        app.next_tab(); assert_eq!(app.tab, Tab::Steps);
        app.next_tab(); assert_eq!(app.tab, Tab::Help);
        app.next_tab(); assert_eq!(app.tab, Tab::All);
    }

    #[test]
    fn set_tab_resets_scroll() {
        let mut app = fixture_app();
        app.scroll = 1;
        app.set_tab(Tab::Steps);
        assert_eq!(app.scroll, 0);
    }

    #[test]
    fn step_next_wraps_at_twelve() {
        let mut app = fixture_app();
        app.step_filter = 12;
        app.step_next();
        assert_eq!(app.step_filter, 1);
    }

    #[test]
    fn step_prev_wraps_at_one() {
        let mut app = fixture_app();
        app.step_filter = 1;
        app.step_prev();
        assert_eq!(app.step_filter, 12);
    }
}

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use crate::config::load_config;
use crate::storage::read_readings;
use crate::tui::commands::{handle_command, Action};

pub fn run() -> Result<()> {
    let config = load_config()?;
    let readings = read_readings()?;

    if readings.is_empty() {
        println!("No readings for today. Run `iwiywi fetch` first.");
        return Ok(());
    }

    let mut app = App::new(readings, crate::config::qr_url(&config), theme::detect());

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| widgets::render(f, &app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
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
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
