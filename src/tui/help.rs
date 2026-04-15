//! Help overlay — press `?` to show a keybinding cheatsheet. Any key
//! dismisses. Static reference; no interaction beyond open/close.

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget, Wrap},
    Frame,
};

use crate::tui::palette::Palette;

pub fn render(frame: &mut Frame, palette: &Palette) {
    let area = frame.area();
    let buf = frame.buffer_mut();

    let width: u16 = 48;
    let height: u16 = 20;
    if area.width < width || area.height < height { return; }
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let rect = Rect { x, y, width, height };

    // Clear backing so the help panel sits on solid ground.
    for row_y in rect.y..rect.y + rect.height {
        for col_x in rect.x..rect.x + rect.width {
            buf[(col_x, row_y)].set_symbol(" ").set_style(Style::default());
        }
    }

    let block = Block::default()
        .title(" Keys ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(palette.accent));
    let inner = block.inner(rect);
    Widget::render(block, rect, buf);

    let rows: &[(&str, &str)] = &[
        ("n", "next item"),
        ("p", "previous item"),
        ("r", "random item"),
        ("space", "pause / resume"),
        ("1–9 0 - =", "focus Step 1–12 · tap twice: AI meditation"),
        ("*", "clear step focus"),
        ("a", "AI: why this matters (Esc closes)"),
        ("F", "showcase mode (fullscreen)"),
        ("m", "settings menu"),
        ("f", "favorite / unfavorite"),
        ("c / click", "copy current item"),
        ("e", "export today"),
        ("j", "journal (AI-seeded prompt + $EDITOR)"),
        ("?", "this help"),
        ("q", "quit"),
    ];
    let mut lines: Vec<Line> = Vec::with_capacity(rows.len());
    for (key, desc) in rows {
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {:<11}", key),
                Style::default().fg(palette.accent).add_modifier(Modifier::BOLD),
            ),
            Span::styled((*desc).to_string(), Style::default().fg(palette.body)),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: false })
        .render(inner, buf);
}
