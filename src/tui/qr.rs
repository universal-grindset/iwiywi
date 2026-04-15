use qrcode::{render::unicode, QrCode};
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::tui::theme::Theme;

pub fn render_qr_overlay(frame: &mut Frame, theme: &Theme, url: &str, area: Rect) {
    let qr_string = generate_qr_string(url);
    let lines: Vec<Line> = qr_string.lines().map(Line::from).collect();

    let qr_width = lines.first().map(|l| l.width() as u16).unwrap_or(0) + 4;
    let qr_height = lines.len() as u16 + 4;

    let x = area.x + area.width.saturating_sub(qr_width) / 2;
    let y = area.y + area.height.saturating_sub(qr_height) / 2;
    let popup = Rect {
        x,
        y,
        width: qr_width.min(area.width),
        height: qr_height.min(area.height),
    };

    let hint = format!("  Scan → {}  ", url);
    let block = Block::default()
        .title(hint)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));

    frame.render_widget(Clear, popup);
    frame.render_widget(Paragraph::new(Text::from(lines)).block(block), popup);
}

pub fn generate_qr_string(url: &str) -> String {
    match QrCode::new(url.as_bytes()) {
        Ok(code) => code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Dark)
            .light_color(unicode::Dense1x2::Light)
            .build(),
        Err(_) => format!("QR unavailable for: {url}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_qr_string_returns_non_empty_for_valid_url() {
        let s = generate_qr_string("https://iwiywi.vercel.app");
        assert!(!s.is_empty());
        assert!(s.contains('\n'));
    }

    #[test]
    fn generate_qr_string_handles_long_url() {
        let url = "https://iwiywi.vercel.app/readings/2026-04-14?source=qr&ref=tui";
        let s = generate_qr_string(url);
        assert!(!s.is_empty());
    }

    #[test]
    fn generate_qr_string_handles_empty_string() {
        let s = generate_qr_string("");
        assert!(!s.is_empty());
    }
}
