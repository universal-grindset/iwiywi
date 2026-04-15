use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::models::ClassifiedReading;
use crate::tui::theme::Theme;
use crate::tui::{App, Mode, Tab};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // tab bar
            Constraint::Min(0),    // body
            Constraint::Length(1), // footer
        ])
        .split(area);

    render_tab_bar(frame, app, chunks[0]);
    render_body(frame, app, chunks[1]);
    render_footer(frame, app, chunks[2]);

    match &app.mode {
        Mode::Command(s) => render_command_bar(frame, &app.theme, s, area),
        Mode::QrOverlay => crate::tui::qr::render_qr_overlay(frame, &app.theme, &app.qr_url, area),
        Mode::Normal | Mode::Drift => {}
    }
}

fn render_tab_bar(frame: &mut Frame, app: &App, area: Rect) {
    let date = chrono::Local::now().format("%a %b %-d").to_string();
    let mut spans: Vec<Span> = vec![
        Span::styled(" iwiywi ", Style::default().fg(app.theme.accent).add_modifier(Modifier::BOLD)),
        Span::styled(date, Style::default().fg(app.theme.muted)),
        Span::raw("   "),
    ];
    for tab in [Tab::All, Tab::Steps, Tab::Help] {
        let active = app.tab == tab;
        let key_style = if active {
            Style::default().fg(app.theme.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.theme.muted)
        };
        let label_style = if active {
            Style::default().fg(app.theme.accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(app.theme.muted)
        };
        spans.push(Span::styled(format!("[{}]", tab.key()), key_style));
        spans.push(Span::styled(format!(" {}  ", tab.label()), label_style));
    }
    let bar = Paragraph::new(Line::from(spans));
    frame.render_widget(bar, area);
}

fn render_body(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(app.theme.border));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let padded = Rect {
        x: inner.x + 1,
        y: inner.y,
        width: inner.width.saturating_sub(2),
        height: inner.height,
    };

    match app.tab {
        Tab::All => {
            let lines = reading_lines(&app.readings, app.scroll, padded.width, &app.theme);
            let body = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false });
            frame.render_widget(body, padded);
        }
        Tab::Steps => {
            let filtered: Vec<ClassifiedReading> = app
                .readings
                .iter()
                .filter(|r| r.step == app.step_filter)
                .cloned()
                .collect();
            let header = Line::from(vec![
                Span::styled(
                    format!("Step {}", app.step_filter),
                    Style::default().fg(app.theme.heading).add_modifier(Modifier::BOLD),
                ),
                Span::styled("   ←/→ to change   ", Style::default().fg(app.theme.muted)),
                Span::styled(
                    format!("({} reading{})", filtered.len(), if filtered.len() == 1 { "" } else { "s" }),
                    Style::default().fg(app.theme.muted),
                ),
            ]);
            let mut lines = vec![header, Line::from("")];
            if filtered.is_empty() {
                lines.push(Line::from(Span::styled(
                    "No readings classified to this step today.",
                    Style::default().fg(app.theme.muted).add_modifier(Modifier::ITALIC),
                )));
            } else {
                lines.extend(reading_lines(&filtered, app.scroll, padded.width, &app.theme));
            }
            let body = Paragraph::new(Text::from(lines)).wrap(Wrap { trim: false });
            frame.render_widget(body, padded);
        }
        Tab::Help => {
            let lines = vec![
                Line::from(Span::styled(
                    "Keys",
                    Style::default().fg(app.theme.heading).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                kv_line("a  s  ?", "jump to a tab", &app.theme),
                kv_line("Tab / Shift+Tab", "cycle tabs", &app.theme),
                kv_line("← / →", "change step (on Steps tab)", &app.theme),
                kv_line("j / ↓   k / ↑", "scroll", &app.theme),
                kv_line("/qr", "open QR code for mobile", &app.theme),
                kv_line("q", "quit", &app.theme),
                Line::from(""),
                Line::from(Span::styled(
                    "Theme",
                    Style::default().fg(app.theme.heading).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                kv_line("IWIYWI_THEME", "light | dark | auto", &app.theme),
            ];
            let body = Paragraph::new(Text::from(lines));
            frame.render_widget(body, padded);
        }
    }
}

fn kv_line<'a>(key: &'a str, desc: &'a str, theme: &Theme) -> Line<'a> {
    Line::from(vec![
        Span::styled(format!("  {:<18}", key), Style::default().fg(theme.accent)),
        Span::styled(desc.to_string(), Style::default().fg(theme.body)),
    ])
}

fn reading_lines<'a>(
    readings: &'a [ClassifiedReading],
    scroll: usize,
    width: u16,
    theme: &Theme,
) -> Vec<Line<'a>> {
    let wrap_to = (width as usize).saturating_sub(2).max(20);
    let mut lines: Vec<Line> = Vec::new();
    for reading in readings.iter().skip(scroll) {
        lines.push(Line::from(vec![
            Span::styled(
                format!("Step {}", reading.step),
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  ·  {}", reading.source),
                Style::default().fg(theme.muted),
            ),
        ]));
        for wrapped in wrap_text(&reading.text, wrap_to) {
            lines.push(Line::from(Span::styled(wrapped, Style::default().fg(theme.body))));
        }
        lines.push(Line::from(""));
    }
    lines
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let keys: Vec<(&str, &str)> = match app.tab {
        Tab::All => vec![
            ("j/k", "scroll"),
            ("a/s/?", "tabs"),
            ("/qr", "qr"),
            ("q", "quit"),
        ],
        Tab::Steps => vec![
            ("←/→", "step"),
            ("j/k", "scroll"),
            ("a/?", "tabs"),
            ("q", "quit"),
        ],
        Tab::Help => vec![("a", "back"), ("q", "quit")],
    };
    let mut spans: Vec<Span> = vec![Span::raw(" ")];
    for (i, (k, d)) in keys.iter().enumerate() {
        if i > 0 {
            spans.push(Span::styled("  ·  ", Style::default().fg(app.theme.muted)));
        }
        spans.push(Span::styled((*k).to_string(), Style::default().fg(app.theme.accent)));
        spans.push(Span::raw(" "));
        spans.push(Span::styled((*d).to_string(), Style::default().fg(app.theme.muted)));
    }
    let footer = Paragraph::new(Line::from(spans));
    frame.render_widget(footer, area);
}

fn render_command_bar(frame: &mut Frame, theme: &Theme, input: &str, area: Rect) {
    let bar_area = Rect {
        x: area.x,
        y: area.y + area.height - 1,
        width: area.width,
        height: 1,
    };
    let bar = Paragraph::new(Line::from(vec![
        Span::styled(" /", Style::default().fg(theme.heading)),
        Span::styled(input.to_string(), Style::default().fg(theme.body)),
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
