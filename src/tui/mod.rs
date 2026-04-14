pub mod commands;
pub mod qr;
pub mod widgets;

use crate::models::ClassifiedReading;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    Command(String),
    QrOverlay,
}

pub struct App {
    pub readings: Vec<ClassifiedReading>,
    pub scroll: usize,
    pub mode: Mode,
    pub vercel_url: String,
}

impl App {
    pub fn new(readings: Vec<ClassifiedReading>, vercel_url: String) -> Self {
        App {
            readings,
            scroll: 0,
            mode: Mode::Normal,
            vercel_url,
        }
    }

    pub fn scroll_down(&mut self) {
        if self.scroll + 1 < self.readings.len() {
            self.scroll += 1;
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
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
        )
    }

    #[test]
    fn scroll_down_increments_within_bounds() {
        let mut app = fixture_app();
        assert_eq!(app.scroll, 0);
        app.scroll_down();
        assert_eq!(app.scroll, 1);
        app.scroll_down(); // at end — should not go past
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
}
