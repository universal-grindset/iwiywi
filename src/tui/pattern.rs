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
    // Minimal static patterns.
    Grid,
    Corners,
    Dashes,
    Vignette,
    Margin,
}

impl Pattern {
    pub const ALL: [Pattern; 13] = [
        Pattern::Drift, Pattern::Wave, Pattern::Snow, Pattern::Rain,
        Pattern::None, Pattern::Dots, Pattern::Frame, Pattern::Rule,
        Pattern::Grid, Pattern::Corners, Pattern::Dashes, Pattern::Vignette,
        Pattern::Margin,
    ];

    pub fn parse(raw: Option<&str>) -> Pattern {
        match raw {
            Some("none")     => Pattern::None,
            Some("dots")     => Pattern::Dots,
            Some("frame")    => Pattern::Frame,
            Some("rule")     => Pattern::Rule,
            Some("wave")     => Pattern::Wave,
            Some("snow")     => Pattern::Snow,
            Some("rain")     => Pattern::Rain,
            Some("grid")     => Pattern::Grid,
            Some("corners")  => Pattern::Corners,
            Some("dashes")   => Pattern::Dashes,
            Some("vignette") => Pattern::Vignette,
            Some("margin")   => Pattern::Margin,
            // Default when unset or unrecognized: drift (the swirly pretties).
            _ => Pattern::Drift,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Pattern::None     => "none",
            Pattern::Dots     => "dots",
            Pattern::Frame    => "frame",
            Pattern::Rule     => "rule",
            Pattern::Drift    => "drift",
            Pattern::Wave     => "wave",
            Pattern::Snow     => "snow",
            Pattern::Rain     => "rain",
            Pattern::Grid     => "grid",
            Pattern::Corners  => "corners",
            Pattern::Dashes   => "dashes",
            Pattern::Vignette => "vignette",
            Pattern::Margin   => "margin",
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

/// Draw the pattern into `area`. `text_rect` is the rect where the
/// centered text will land. `kind` is the current pulse item's source
/// type — Frame/Corners/Rule use its frame_tint so the border color
/// hints at the content source (Prayer=dusty blue, BigBook=warm gold,
/// Step=purple, etc.). Other patterns ignore it and use palette.muted.
pub fn draw(
    buf: &mut Buffer,
    area: Rect,
    text_rect: Rect,
    palette: &Palette,
    pattern: Pattern,
    kind: Option<crate::pulse::PulseKind>,
) {
    let tint = kind.map_or(palette.muted, |k| k.frame_tint());
    match pattern {
        // Animated patterns (drift/wave/snow/rain) draw directly from
        // `widgets::render_pulse` via the shared DriftState — not here.
        Pattern::None | Pattern::Drift | Pattern::Wave | Pattern::Snow | Pattern::Rain => {}
        Pattern::Dots     => draw_dots(buf, area, palette),
        Pattern::Frame    => draw_frame(buf, text_rect, tint),
        Pattern::Rule     => draw_rule(buf, text_rect, tint),
        Pattern::Grid     => draw_grid(buf, area, palette),
        Pattern::Corners  => draw_corners(buf, text_rect, tint),
        Pattern::Dashes   => draw_dashes(buf, area, palette),
        Pattern::Vignette => draw_vignette(buf, area, palette),
        Pattern::Margin   => draw_margin(buf, area, palette),
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

fn draw_frame(buf: &mut Buffer, text_rect: Rect, tint: ratatui::style::Color) {
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
        .border_style(Style::default().fg(tint));
    ratatui::widgets::Widget::render(block, padded, buf);
}

fn draw_rule(buf: &mut Buffer, text_rect: Rect, tint: ratatui::style::Color) {
    if text_rect.width < 4 { return; }
    let y = text_rect.y + 1; // just under the kind line
    let style = Style::default().fg(tint);
    for x in text_rect.x..(text_rect.x + text_rect.width) {
        buf[(x, y)].set_symbol("─").set_style(style);
    }
}

/// Sparse dot-grid — one `·` every 8 cols × 4 rows. Reads as very faint
/// graph paper.
fn draw_grid(buf: &mut Buffer, area: Rect, palette: &Palette) {
    if area.width < 10 || area.height < 4 { return; }
    let style = Style::default().fg(palette.muted);
    let step_x: u16 = 8;
    let step_y: u16 = 4;
    let mut y = area.y + (step_y / 2);
    while y < area.y + area.height {
        let mut x = area.x + (step_x / 2);
        while x < area.x + area.width {
            buf[(x, y)].set_symbol("·").set_style(style);
            x += step_x;
        }
        y += step_y;
    }
}

/// L-bracket markers at the four corners of the centered text rect.
/// Subtle frame cue without drawing the whole box.
fn draw_corners(buf: &mut Buffer, text_rect: Rect, tint: ratatui::style::Color) {
    if text_rect.width < 6 || text_rect.height < 4 { return; }
    // Pad out one cell so the brackets don't touch the text.
    let x0 = text_rect.x.saturating_sub(1);
    let y0 = text_rect.y.saturating_sub(1);
    let x1 = text_rect.x + text_rect.width;
    let y1 = text_rect.y + text_rect.height;
    let style = Style::default().fg(tint);
    for (x, y, s) in [
        (x0, y0, "┌"), (x1, y0, "┐"),
        (x0, y1, "└"), (x1, y1, "┘"),
    ] {
        if x < buf.area.right() && y < buf.area.bottom() {
            buf[(x, y)].set_symbol(s).set_style(style);
        }
    }
}

/// Horizontal dash line at the very top and bottom rows of `area` —
/// minimal typographic top/bottom markers.
fn draw_dashes(buf: &mut Buffer, area: Rect, palette: &Palette) {
    if area.width < 10 || area.height < 3 { return; }
    let style = Style::default().fg(palette.muted);
    let top = area.y;
    let bottom = area.y + area.height - 1;
    for x in area.x..(area.x + area.width) {
        buf[(x, top)].set_symbol("─").set_style(style);
        buf[(x, bottom)].set_symbol("─").set_style(style);
    }
}

/// Dots denser near the four corners, falling off toward the center.
/// Uses the same xorshift hash as `dots` for a consistent texture, but
/// samples more aggressively in a corner-proximity weighted way.
fn draw_vignette(buf: &mut Buffer, area: Rect, palette: &Palette) {
    if area.width < 12 || area.height < 6 { return; }
    let style = Style::default().fg(palette.muted);
    let w = u32::from(area.width);
    let h = u32::from(area.height);
    // 5-cell-radius corner clusters at all four corners.
    let r: u32 = 5;
    let corners = [
        (0u32, 0u32),
        (w.saturating_sub(1), 0),
        (0, h.saturating_sub(1)),
        (w.saturating_sub(1), h.saturating_sub(1)),
    ];
    for (cx, cy) in corners {
        for i in 0..(r * r) {
            let dx = pseudo_rand(0xC0FF_EE42, i as usize) % (r * 2);
            let dy = pseudo_rand(0xC0FF_EE42, i as usize * 2 + 1) % (r * 2);
            let x = cx.saturating_add_signed((dx as i32) - (r as i32));
            let y = cy.saturating_add_signed((dy as i32) - (r as i32));
            if x < w && y < h {
                buf[(area.x + x as u16, area.y + y as u16)]
                    .set_symbol("·")
                    .set_style(style);
            }
        }
    }
}

/// Thin vertical bars at the far-left and far-right columns of `area`,
/// broken into short runs so the effect stays quiet.
fn draw_margin(buf: &mut Buffer, area: Rect, palette: &Palette) {
    if area.width < 4 || area.height < 6 { return; }
    let style = Style::default().fg(palette.muted);
    let left = area.x;
    let right = area.x + area.width - 1;
    // Every third row — leaves two cells of breathing room between ticks.
    let mut y = area.y + 1;
    while y < area.y + area.height - 1 {
        buf[(left, y)].set_symbol("│").set_style(style);
        buf[(right, y)].set_symbol("│").set_style(style);
        y += 3;
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
    fn pattern_parse_each_round_trips() {
        for p in Pattern::ALL {
            assert_eq!(Pattern::parse(Some(p.label())), p,
                "pattern {} failed round-trip", p.label());
        }
    }

    #[test]
    fn all_patterns_covers_enum() {
        assert_eq!(Pattern::ALL.len(), 13);
    }

    #[test]
    fn animated_flag_matches_drift_modes() {
        for p in Pattern::ALL {
            let animated = p.is_animated();
            let expected = matches!(p,
                Pattern::Drift | Pattern::Wave | Pattern::Snow | Pattern::Rain);
            assert_eq!(animated, expected,
                "is_animated disagrees for {}", p.label());
        }
    }

    #[test]
    fn new_minimal_patterns_are_static() {
        // Sanity: none of the five new minimal patterns claim animation.
        for p in [Pattern::Grid, Pattern::Corners, Pattern::Dashes,
                  Pattern::Vignette, Pattern::Margin] {
            assert!(!p.is_animated(),
                "new pattern {} shouldn't be animated", p.label());
        }
    }
}
