//! AI-response modal overlay. Centered panel with a titled border, wrapped
//! body text, and a `j/k`-scrollable viewport. Shown on top of the pulse
//! when the user triggers an AI action (`a` for explain-current, digit
//! double-tap for step meditation).
//!
//! The overlay owns no network state — the TUI spawns a background thread
//! for each call and updates this struct's `status` when the result lands.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
    Frame,
};

use crate::tui::palette::Palette;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlayStatus {
    Loading,
    Ready,
    Failed(String),
}

/// Message sent from the background AI thread back into the TUI main loop.
#[derive(Debug, Clone)]
pub enum AiOutcome {
    Text(String),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct AiOverlay {
    pub title: String,
    pub body: String,
    pub scroll: u16,
    pub status: OverlayStatus,
}

impl AiOverlay {
    pub fn loading(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: "thinking…".to_string(),
            scroll: 0,
            status: OverlayStatus::Loading,
        }
    }

    pub fn apply_outcome(&mut self, outcome: AiOutcome) {
        match outcome {
            AiOutcome::Text(text) => {
                self.body = text;
                self.scroll = 0;
                self.status = OverlayStatus::Ready;
            }
            AiOutcome::Error(msg) => {
                self.body.clone_from(&msg);
                self.scroll = 0;
                self.status = OverlayStatus::Failed(msg);
            }
        }
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }
    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    /// Rough upper bound on the scroll offset: one hard-break line per `\n`
    /// plus a word-wrap allowance. Called during render with the viewport
    /// width so the cap tracks the actual wrapped line count.
    pub fn clamp_scroll(&mut self, viewport_width: u16, viewport_height: u16) {
        let w = viewport_width.max(1) as usize;
        let mut lines: u16 = 0;
        for line in self.body.split('\n') {
            let n = line.chars().count();
            lines = lines.saturating_add(n.div_ceil(w).max(1) as u16);
        }
        let max_scroll = lines.saturating_sub(viewport_height.max(1));
        if self.scroll > max_scroll {
            self.scroll = max_scroll;
        }
    }
}

pub fn render(frame: &mut Frame, palette: &Palette, overlay: &mut AiOverlay) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    let width = (area.width as f32 * 0.7).clamp(40.0, 80.0) as u16;
    let height = (area.height as f32 * 0.7).clamp(10.0, 28.0) as u16;
    if area.width < width || area.height < height {
        return;
    }

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let rect = Rect {
        x,
        y,
        width,
        height,
    };

    clear_rect(buf, rect);

    let border_style = match overlay.status {
        OverlayStatus::Failed(_) => Style::default().fg(palette.muted),
        _ => Style::default().fg(palette.accent),
    };
    let block = Block::default()
        .title(format!(" {} ", overlay.title))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style);
    let inner = block.inner(rect);
    Widget::render(block, rect, buf);

    // Reserve the last inner row for the footer hint.
    let body_rect = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: inner.height.saturating_sub(1),
    };
    let footer_y = inner.y + inner.height.saturating_sub(1);

    overlay.clamp_scroll(body_rect.width, body_rect.height);

    let body_style = match overlay.status {
        OverlayStatus::Loading => Style::default()
            .fg(palette.muted)
            .add_modifier(Modifier::ITALIC),
        _ => Style::default().fg(palette.body),
    };
    let body = Paragraph::new(overlay.body.as_str())
        .style(body_style)
        .wrap(Wrap { trim: false })
        .scroll((overlay.scroll, 0));
    body.render(body_rect, buf);

    let hint = Line::from(Span::styled(
        "j/k scroll · Esc close",
        Style::default()
            .fg(palette.muted)
            .add_modifier(Modifier::ITALIC),
    ));
    let footer_rect = Rect {
        x: inner.x,
        y: footer_y,
        width: inner.width,
        height: 1,
    };
    Paragraph::new(hint).render(footer_rect, buf);
}

fn clear_rect(buf: &mut Buffer, rect: Rect) {
    for y in rect.y..rect.y + rect.height {
        for x in rect.x..rect.x + rect.width {
            buf[(x, y)].set_symbol(" ").set_style(Style::default());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_has_expected_initial_state() {
        let o = AiOverlay::loading("T");
        assert_eq!(o.title, "T");
        assert_eq!(o.status, OverlayStatus::Loading);
        assert_eq!(o.scroll, 0);
    }

    #[test]
    fn apply_text_outcome_transitions_to_ready() {
        let mut o = AiOverlay::loading("T");
        o.scroll = 5;
        o.apply_outcome(AiOutcome::Text("done".to_string()));
        assert_eq!(o.body, "done");
        assert_eq!(o.status, OverlayStatus::Ready);
        assert_eq!(o.scroll, 0);
    }

    #[test]
    fn apply_error_outcome_transitions_to_failed() {
        let mut o = AiOverlay::loading("T");
        o.apply_outcome(AiOutcome::Error("network".to_string()));
        assert!(matches!(o.status, OverlayStatus::Failed(_)));
        assert!(o.body.contains("network"));
    }

    #[test]
    fn scroll_saturates_at_zero() {
        let mut o = AiOverlay::loading("T");
        o.scroll_up();
        assert_eq!(o.scroll, 0);
        o.scroll_down();
        o.scroll_down();
        o.scroll_up();
        assert_eq!(o.scroll, 1);
    }

    #[test]
    fn clamp_scroll_pins_to_content_end() {
        let mut o = AiOverlay::loading("T");
        o.apply_outcome(AiOutcome::Text("a\nb\nc".to_string()));
        o.scroll = 99;
        // Viewport 40 wide, 2 tall. 3 hard-break lines, max_scroll = 3-2 = 1.
        o.clamp_scroll(40, 2);
        assert_eq!(o.scroll, 1);
    }

    #[test]
    fn clamp_scroll_allows_in_range_offsets() {
        let mut o = AiOverlay::loading("T");
        o.apply_outcome(AiOutcome::Text("a\nb\nc\nd\ne".to_string()));
        o.scroll = 2;
        o.clamp_scroll(40, 2);
        // 5 lines - 2 viewport = 3 max. scroll=2 is in-range.
        assert_eq!(o.scroll, 2);
    }

    #[test]
    fn clamp_scroll_word_wraps_long_lines() {
        let mut o = AiOverlay::loading("T");
        // 100-char single line at width 10 → 10 wrapped lines.
        o.apply_outcome(AiOutcome::Text("x".repeat(100)));
        o.scroll = 99;
        o.clamp_scroll(10, 3);
        // 10 wrapped lines - 3 viewport = 7 max.
        assert_eq!(o.scroll, 7);
    }
}
