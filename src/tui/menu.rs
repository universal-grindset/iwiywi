//! Settings menu overlay. Press `m` to toggle; arrows navigate; Esc closes.
//! Five rows: Palette, Pattern, Order, Focus, Pulse secs. Changes apply live.

use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
    Frame,
};

use crate::tui::palette::Palette;

/// Small ASCII banner rendered at the top of the settings overlay.
const BANNER: &[&str] = &[
    "╭─────────────────────╮",
    "│   i w i y w i       │",
    "│   it works if you   │",
    "│   work it · v0.6    │",
    "╰─────────────────────╯",
];

pub const ROW_COUNT: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Row {
    Palette,
    Pattern,
    TextSize,
    Order,
    Focus,
    PulseSecs,
}

impl Row {
    pub fn by_index(i: usize) -> Row {
        match i % ROW_COUNT {
            0 => Row::Palette,
            1 => Row::Pattern,
            2 => Row::TextSize,
            3 => Row::Order,
            4 => Row::Focus,
            _ => Row::PulseSecs,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Row::Palette   => "Palette",
            Row::Pattern   => "Pattern",
            Row::TextSize  => "Text size",
            Row::Order     => "Order",
            Row::Focus     => "Focus",
            Row::PulseSecs => "Pulse secs",
        }
    }
}

/// Seven slots for `Pulse secs`: 0 (manual), 5, 10, 15, 20, 30, 60.
pub const PULSE_SECS_RING: [u64; 7] = [0, 5, 10, 15, 20, 30, 60];

pub fn render(
    frame: &mut Frame,
    palette: &Palette,
    cursor: usize,
    values: [String; ROW_COUNT],
) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    let width: u16 = 40;
    let height: u16 = 16;
    if area.width < width || area.height < height { return; }
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let rect = Rect { x, y, width, height };

    // Dim-out / clear the overlay rect so the menu sits on a solid panel.
    for row_y in rect.y..rect.y + rect.height {
        for col_x in rect.x..rect.x + rect.width {
            buf[(col_x, row_y)].set_symbol(" ").set_style(Style::default());
        }
    }

    let block = Block::default()
        .title(" settings ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(palette.accent));
    let inner = block.inner(rect);
    Widget::render(block, rect, buf);

    // Banner occupies the first `BANNER.len()` rows of the inner area,
    // centered horizontally. Rendered with the accent color so it pops.
    let banner_lines: Vec<Line> = BANNER
        .iter()
        .map(|s| Line::from(Span::styled(
            (*s).to_string(),
            Style::default().fg(palette.accent),
        )))
        .collect();
    let banner_rect = Rect {
        x: inner.x,
        y: inner.y,
        width: inner.width,
        height: BANNER.len() as u16,
    };
    Paragraph::new(banner_lines)
        .alignment(Alignment::Center)
        .render(banner_rect, buf);

    // Content (settings rows + hint) starts one row below the banner.
    let content_rect = Rect {
        x: inner.x,
        y: inner.y + BANNER.len() as u16 + 1,
        width: inner.width,
        height: inner.height.saturating_sub(BANNER.len() as u16 + 1),
    };

    let mut lines: Vec<Line> = Vec::with_capacity(ROW_COUNT + 2);
    for (i, value) in values.iter().enumerate() {
        let row = Row::by_index(i);
        let active = i == cursor;
        let marker = if active { "› " } else { "  " };
        let label_style = if active {
            Style::default().fg(palette.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(palette.body)
        };
        let value_style = if active {
            Style::default().fg(palette.accent)
        } else {
            Style::default().fg(palette.muted)
        };
        lines.push(Line::from(vec![
            Span::styled(marker, label_style),
            Span::styled(format!("{:<11}", row.label()), label_style),
            Span::styled(value.clone(), value_style),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "↑↓ pick  ←→ cycle  Esc close",
        Style::default().fg(palette.muted).add_modifier(Modifier::ITALIC),
    )));

    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .render(content_rect, buf);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_by_index_cycles() {
        assert_eq!(Row::by_index(0), Row::Palette);
        assert_eq!(Row::by_index(ROW_COUNT - 1), Row::PulseSecs);
        assert_eq!(Row::by_index(ROW_COUNT), Row::Palette);
    }

    #[test]
    fn all_rows_have_labels() {
        for i in 0..ROW_COUNT {
            assert!(!Row::by_index(i).label().is_empty());
        }
    }

    #[test]
    fn pulse_secs_ring_has_zero_first() {
        assert_eq!(PULSE_SECS_RING[0], 0);
    }
}
