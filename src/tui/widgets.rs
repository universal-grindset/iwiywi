use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
    Frame,
};

use ratatui::buffer::Buffer;
use ratatui::layout::Alignment;

use crate::pulse::PulseItem;
use crate::tui::drift::{self, DriftState};
use crate::tui::palette::Palette;
use crate::tui::pattern::{self, Pattern};
use crate::tui::text_size::TextSize;

fn render_too_small(buf: &mut Buffer, area: ratatui::layout::Rect, palette: &Palette) {
    let msg = format!(
        "terminal too small\nneed at least {MIN_WIDTH}×{MIN_HEIGHT} cells"
    );
    let y = area.y + area.height / 2;
    let h = 2u16;
    let rect = ratatui::layout::Rect {
        x: area.x, y: y.saturating_sub(1),
        width: area.width, height: h.min(area.height),
    };
    Paragraph::new(msg)
        .style(Style::default().fg(palette.muted).add_modifier(Modifier::ITALIC))
        .alignment(Alignment::Center)
        .render(rect, buf);
}

pub const MIN_WIDTH: u16 = 60;
pub const MIN_HEIGHT: u16 = 15;

pub fn render_pulse(
    frame: &mut Frame,
    item: Option<&PulseItem>,
    palette: &Palette,
    pattern: Pattern,
    drift_state: Option<&DriftState>,
    text_size: TextSize,
    showcase: bool,
) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    // Minimum-size gate. Per tui-design skill: below the usable threshold,
    // show one clear message instead of corrupt layout. The caller-level
    // overlays (status/menu/moon) also check their own minimums; this is
    // the main content's floor.
    if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
        render_too_small(buf, area, palette);
        return;
    }

    let Some(item) = item else { return; };

    // Showcase mode takes over the whole frame: wider text column, larger
    // clamp ceiling, bold body. The caller still passes a TextSize but we
    // override both ratio and clamp upper bound.
    let (clamp_lo, clamp_hi) = text_size.width_clamp();
    let (ratio, clamp_hi) = if showcase {
        (0.92, (area.width as f32 - 4.0).max(40.0))
    } else {
        (text_size.width_ratio(), clamp_hi)
    };
    let width = (area.width as f32 * ratio).clamp(clamp_lo, clamp_hi) as u16;
    let w = width.max(1) as usize;
    // Per-line ceiling-divide so we never under-count, count explicit
    // newlines as hard breaks, and add a safety margin for word-wrap slack
    // (a word that doesn't fit at the end of a line gets pushed down, which
    // pure char-count division misses).
    let mut body_lines: u16 = 0;
    for line in item.body.split('\n') {
        let n = line.chars().count();
        body_lines = body_lines.saturating_add(n.div_ceil(w).max(1) as u16);
    }
    body_lines = body_lines.saturating_add(1);
    let total_height = (3 + body_lines).min(area.height);

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(total_height)) / 2;
    let text_rect = Rect { x, y, width, height: total_height };

    // Static patterns (none/dots/frame/rule) draw here. Drift is animated
    // and needs live state; it draws below when present. Frame-family
    // patterns tint by the current item's kind for a subtle source cue.
    pattern::draw(buf, area, text_rect, palette, pattern, Some(item.kind));
    if pattern.is_animated() {
        if let Some(state) = drift_state {
            drift::draw(buf, area, state, palette);
        }
    }

    let label = Line::from(Span::styled(
        item.label.clone(),
        Style::default().fg(palette.accent).add_modifier(Modifier::BOLD),
    ));
    let kind = Line::from(Span::styled(
        item.kind.display_label().to_string(),
        Style::default().fg(palette.muted).add_modifier(Modifier::ITALIC),
    ));
    let body_modifier = if showcase {
        Modifier::BOLD
    } else {
        text_size.body_modifier()
    };
    let body = Line::from(Span::styled(
        item.body.clone(),
        Style::default().fg(palette.body).add_modifier(body_modifier),
    ));

    Paragraph::new(vec![label, kind, Line::from(""), body])
        .wrap(Wrap { trim: false })
        .render(text_rect, buf);
}
