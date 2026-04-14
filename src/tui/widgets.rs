use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::tui::{App, Mode};

pub fn step_color(step: u8) -> Color {
    match step {
        1  => Color::Red,
        2  => Color::Yellow,
        3  => Color::Blue,
        4  => Color::Magenta,
        5  => Color::Cyan,
        6  => Color::Green,
        7  => Color::LightRed,
        8  => Color::LightYellow,
        9  => Color::LightBlue,
        10 => Color::LightMagenta,
        11 => Color::LightCyan,
        12 => Color::LightGreen,
        _  => Color::White,
    }
}

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Always render the feed underneath
    render_feed(frame, app, area);

    match &app.mode {
        Mode::Command(s) => render_command_bar(frame, s, area),
        Mode::QrOverlay  => {}, // rendered in Task 4
        Mode::Normal     => {}
    }
}

fn render_feed(frame: &mut Frame, app: &App, area: Rect) {
    // Split: main content + status bar at bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let visible: Vec<_> = app
        .readings
        .iter()
        .skip(app.scroll)
        .collect();

    let mut lines: Vec<Line> = vec![];

    // Header
    let date = chrono::Local::now().format("%a %b %-d %Y").to_string();
    lines.push(Line::from(vec![
        Span::styled("iwiywi  ", Style::default().fg(Color::DarkGray)),
        Span::styled(date, Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(""));

    for reading in &visible {
        let color = step_color(reading.step);
        // Accent bar + header
        lines.push(Line::from(vec![
            Span::styled("▌ ", Style::default().fg(color)),
            Span::styled(
                format!("Step {} · {}", reading.step, reading.source),
                Style::default().fg(color),
            ),
        ]));
        // Wrap text manually at ~60 chars
        for line in wrap_text(&reading.text, 60) {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(line, Style::default().fg(Color::Rgb(201, 209, 217))),
            ]));
        }
        // Divider
        lines.push(Line::from(Span::styled(
            "  ─────────────────────────────────────────────────",
            Style::default().fg(Color::Rgb(33, 38, 45)),
        )));
        lines.push(Line::from(""));
    }

    let paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default());
    frame.render_widget(paragraph, chunks[0]);

    // Status bar
    let status = Paragraph::new(Line::from(vec![
        Span::styled(" ↑↓ scroll  ", Style::default().fg(Color::DarkGray)),
        Span::styled("/qr", Style::default().fg(Color::Blue)),
        Span::styled(" QR code  ", Style::default().fg(Color::DarkGray)),
        Span::styled("q", Style::default().fg(Color::Blue)),
        Span::styled(" quit", Style::default().fg(Color::DarkGray)),
    ]));
    frame.render_widget(status, chunks[1]);
}

fn render_command_bar(frame: &mut Frame, input: &str, area: Rect) {
    let bar_area = Rect {
        x: area.x,
        y: area.y + area.height - 1,
        width: area.width,
        height: 1,
    };
    let bar = Paragraph::new(Line::from(vec![
        Span::styled("/", Style::default().fg(Color::Yellow)),
        Span::styled(input, Style::default().fg(Color::White)),
    ]));
    frame.render_widget(Clear, bar_area);
    frame.render_widget(bar, bar_area);
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = vec![];
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current = String::new();
    for word in words {
        if current.len() + word.len() + 1 > width && !current.is_empty() {
            lines.push(current.clone());
            current = word.to_string();
        } else {
            if !current.is_empty() { current.push(' '); }
            current.push_str(word);
        }
    }
    if !current.is_empty() { lines.push(current); }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_color_covers_all_steps() {
        for step in 1u8..=12 {
            let c = step_color(step);
            assert_ne!(c, Color::White, "step {step} should have a distinct color");
        }
    }

    #[test]
    fn step_color_out_of_range_returns_white() {
        assert_eq!(step_color(0), Color::White);
        assert_eq!(step_color(13), Color::White);
    }

    #[test]
    fn wrap_text_splits_at_width() {
        let text = "one two three four five six seven eight nine ten eleven twelve";
        let lines = wrap_text(text, 20);
        for line in &lines {
            assert!(line.len() <= 20, "line too long: {line:?}");
        }
        assert!(lines.len() > 1);
    }

    #[test]
    fn wrap_text_preserves_all_words() {
        let text = "hello world foo bar";
        let lines = wrap_text(text, 20);
        let rejoined = lines.join(" ");
        assert_eq!(rejoined, text);
    }
}
