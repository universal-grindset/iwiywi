use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
    Frame,
};

use crate::pulse::PulseItem;
use crate::tui::drift::{self, DriftState};
use crate::tui::palette::Palette;
use crate::tui::pattern::{self, Pattern};

pub fn render_pulse(
    frame: &mut Frame,
    item: Option<&PulseItem>,
    palette: &Palette,
    pattern: Pattern,
    drift_state: Option<&DriftState>,
) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    let Some(item) = item else { return; };

    let width = (area.width as f32 * 0.7).clamp(20.0, 72.0) as u16;
    let body_lines_estimate = (item.body.chars().count() as u16 / width.max(1)).saturating_add(1);
    let total_height = 3 + body_lines_estimate;
    let total_height = total_height.min(area.height);

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(total_height)) / 2;
    let text_rect = Rect { x, y, width, height: total_height };

    // Static patterns (none/dots/frame/rule) draw here. Drift is animated
    // and needs live state; it draws below when present.
    pattern::draw(buf, area, text_rect, palette, pattern);
    if pattern == Pattern::Drift {
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
    let body = Line::from(Span::styled(
        item.body.clone(),
        Style::default().fg(palette.body),
    ));

    Paragraph::new(vec![label, kind, Line::from(""), body])
        .wrap(Wrap { trim: false })
        .render(text_rect, buf);
}
