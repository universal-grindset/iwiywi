//! Pattern: a static visual texture rendered once per pulse item, behind the
//! centered text. Not animated. Subtle by design.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, BorderType, Borders},
};

use crate::tui::palette::Palette;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    None,
    Dots,
    Frame,
    Rule,
    Drift,
}

impl Pattern {
    pub fn parse(raw: Option<&str>) -> Pattern {
        match raw {
            Some("none")  => Pattern::None,
            Some("dots")  => Pattern::Dots,
            Some("frame") => Pattern::Frame,
            Some("rule")  => Pattern::Rule,
            // Default when unset or unrecognized: drift (the swirly pretties).
            _ => Pattern::Drift,
        }
    }
}

pub fn from_env() -> Pattern {
    Pattern::parse(std::env::var("IWIYWI_PATTERN").ok().as_deref())
}

/// Draw the pattern into `area` using the palette's muted color.
/// `text_rect` is the rect where the centered text will land — patterns can
/// use it to position elements relative to the text.
pub fn draw(buf: &mut Buffer, area: Rect, text_rect: Rect, palette: &Palette, pattern: Pattern) {
    match pattern {
        // `Drift` is animated and needs `DriftState` that lives on `App`,
        // so it's rendered directly from `widgets::render_pulse` — not here.
        Pattern::None | Pattern::Drift => {}
        Pattern::Dots => draw_dots(buf, area, palette),
        Pattern::Frame => draw_frame(buf, text_rect, palette),
        Pattern::Rule => draw_rule(buf, text_rect, palette),
    }
}

fn draw_dots(buf: &mut Buffer, area: Rect, palette: &Palette) {
    if area.width < 4 || area.height < 4 { return; }
    let style = Style::default().fg(palette.muted);
    let coords = [
        (area.x + 1, area.y + 1),
        (area.x + area.width.saturating_sub(2), area.y + 1),
        (area.x + 1, area.y + area.height.saturating_sub(2)),
        (area.x + area.width.saturating_sub(2), area.y + area.height.saturating_sub(2)),
    ];
    for (x, y) in coords {
        buf[(x, y)].set_symbol("·").set_style(style);
    }
}

fn draw_frame(buf: &mut Buffer, text_rect: Rect, palette: &Palette) {
    if text_rect.width < 4 || text_rect.height < 3 { return; }
    let padded = Rect {
        x: text_rect.x.saturating_sub(2),
        y: text_rect.y.saturating_sub(1),
        width: text_rect.width + 4,
        height: text_rect.height + 2,
    };
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(palette.muted));
    ratatui::widgets::Widget::render(block, padded, buf);
}

fn draw_rule(buf: &mut Buffer, text_rect: Rect, palette: &Palette) {
    if text_rect.width < 4 { return; }
    let y = text_rect.y + 1; // just under the kind line
    let style = Style::default().fg(palette.muted);
    for x in text_rect.x..(text_rect.x + text_rect.width) {
        buf[(x, y)].set_symbol("─").set_style(style);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_parse_defaults_to_drift() {
        assert_eq!(Pattern::parse(None), Pattern::Drift);
        assert_eq!(Pattern::parse(Some("garbage")), Pattern::Drift);
    }

    #[test]
    fn pattern_parse_each() {
        assert_eq!(Pattern::parse(Some("none")), Pattern::None);
        assert_eq!(Pattern::parse(Some("dots")), Pattern::Dots);
        assert_eq!(Pattern::parse(Some("frame")), Pattern::Frame);
        assert_eq!(Pattern::parse(Some("rule")), Pattern::Rule);
        assert_eq!(Pattern::parse(Some("drift")), Pattern::Drift);
    }
}
