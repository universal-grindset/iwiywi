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
    Wave,
    Snow,
    Rain,
}

impl Pattern {
    pub const ALL: [Pattern; 8] = [
        Pattern::Drift, Pattern::Wave, Pattern::Snow, Pattern::Rain,
        Pattern::None, Pattern::Dots, Pattern::Frame, Pattern::Rule,
    ];

    pub fn parse(raw: Option<&str>) -> Pattern {
        match raw {
            Some("none")  => Pattern::None,
            Some("dots")  => Pattern::Dots,
            Some("frame") => Pattern::Frame,
            Some("rule")  => Pattern::Rule,
            Some("wave")  => Pattern::Wave,
            Some("snow")  => Pattern::Snow,
            Some("rain")  => Pattern::Rain,
            // Default when unset or unrecognized: drift (the swirly pretties).
            _ => Pattern::Drift,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Pattern::None  => "none",
            Pattern::Dots  => "dots",
            Pattern::Frame => "frame",
            Pattern::Rule  => "rule",
            Pattern::Drift => "drift",
            Pattern::Wave  => "wave",
            Pattern::Snow  => "snow",
            Pattern::Rain  => "rain",
        }
    }

    /// True for patterns that need an animated DriftState.
    pub fn is_animated(&self) -> bool {
        matches!(self, Pattern::Drift | Pattern::Wave | Pattern::Snow | Pattern::Rain)
    }

    /// The drift::Mode each animated pattern uses. Non-animated patterns
    /// map to `Drift` as a harmless default (caller shouldn't use it).
    pub fn drift_mode(&self) -> crate::tui::drift::Mode {
        use crate::tui::drift::Mode;
        match self {
            Pattern::Wave => Mode::Wave,
            Pattern::Snow => Mode::Snow,
            Pattern::Rain => Mode::Rain,
            _ => Mode::Drift,
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
        // Animated patterns (drift/wave/snow/rain) draw directly from
        // `widgets::render_pulse` via the shared DriftState — not here.
        Pattern::None | Pattern::Drift | Pattern::Wave | Pattern::Snow | Pattern::Rain => {}
        Pattern::Dots => draw_dots(buf, area, palette),
        Pattern::Frame => draw_frame(buf, text_rect, palette),
        Pattern::Rule => draw_rule(buf, text_rect, palette),
    }
}

fn draw_dots(buf: &mut Buffer, area: Rect, palette: &Palette) {
    if area.width < 10 || area.height < 4 { return; }
    let style = Style::default().fg(palette.muted);
    // Deterministic scattered field — about 1 dot per 18 cells. Same
    // xorshift-mult hash as the drift particles so the density looks
    // consistent across the two patterns when users switch.
    let total = (area.width as usize) * (area.height as usize);
    let count = (total / 18).clamp(30, 500);
    for i in 0..count {
        let hx = pseudo_rand(0xD14D_B33F, i * 2);
        let hy = pseudo_rand(0xD14D_B33F, i * 2 + 1);
        let x = (hx % u32::from(area.width)) as u16;
        let y = (hy % u32::from(area.height)) as u16;
        buf[(area.x + x, area.y + y)]
            .set_symbol("·")
            .set_style(style);
    }
}

fn pseudo_rand(seed: u32, n: usize) -> u32 {
    let mut x = seed.wrapping_mul(2_654_435_761).wrapping_add(n as u32);
    x ^= x >> 13;
    x = x.wrapping_mul(0x5bd1_e995);
    x ^= x >> 15;
    x
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
