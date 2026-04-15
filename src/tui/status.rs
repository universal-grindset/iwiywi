//! Status footer: a single row at the bottom of the pulse. Left-aligned
//! shows `position / total · focus`, right-aligned shows `Day N` if the
//! user has set `IWIYWI_SOBER_SINCE`. Between them: a thin progress bar
//! indicating time until the next auto-advance.

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    Frame,
};

use crate::pulse::{Focus, PulseMixer};
use crate::tui::palette::Palette;

pub struct StatusLine<'a> {
    pub mixer: &'a PulseMixer,
    pub focus: Focus,
    pub focus_step: Option<u8>,
    pub pulse_progress: Option<f32>, // 0.0..=1.0, None if manual-only
    pub sobriety_days: Option<i64>,
    pub paused: bool,
    /// When `Some`, this overrides the right-hand sobriety slot with a
    /// transient message (e.g. "copied", "★ saved").
    pub toast: Option<&'a str>,
}

pub fn render(frame: &mut Frame, palette: &Palette, status: &StatusLine) {
    let area = frame.area();
    if area.height < 2 { return; }
    let buf = frame.buffer_mut();
    let y = area.y + area.height - 1;
    let row = Rect { x: area.x, y, width: area.width, height: 1 };

    // Clear the row so nothing underneath bleeds through.
    for col_x in row.x..row.x + row.width {
        buf[(col_x, row.y)].set_symbol(" ").set_style(Style::default());
    }

    let left = left_text(status);
    let right = right_text(status);

    let left_w = left.chars().count() as u16;
    let right_w = right.chars().count() as u16;

    Paragraph::new(Line::from(Span::styled(
        format!(" {left}"),
        Style::default().fg(palette.muted).add_modifier(Modifier::ITALIC),
    )))
    .render(Rect { x: row.x, y: row.y, width: left_w + 1, height: 1 }, buf);

    if !right.is_empty() {
        let right_x = row.x + row.width.saturating_sub(right_w + 1);
        Paragraph::new(Line::from(Span::styled(
            format!("{right} "),
            Style::default().fg(palette.muted).add_modifier(Modifier::ITALIC),
        )))
        .render(Rect { x: right_x, y: row.y, width: right_w + 1, height: 1 }, buf);
    }

    // Progress bar spans the middle gap. Skipped when too narrow or when
    // pulse_secs is None (manual-only mode).
    let gap_left = row.x + left_w + 2;
    let gap_right = row.x + row.width.saturating_sub(right_w + 2);
    if let Some(p) = status.pulse_progress {
        if gap_right > gap_left + 4 {
            let bar_width = gap_right - gap_left;
            let filled = (p.clamp(0.0, 1.0) * bar_width as f32) as u16;
            for i in 0..bar_width {
                let x = gap_left + i;
                let style = if i < filled {
                    Style::default().fg(palette.accent)
                } else {
                    Style::default().fg(palette.muted)
                };
                buf[(x, row.y)].set_symbol("─").set_style(style);
            }
        }
    }
}

fn left_text(status: &StatusLine) -> String {
    let pos = status.mixer.cursor() + 1;
    let total = status.mixer.len().max(1);
    let focus_chip = match (status.focus_step, status.focus) {
        (Some(n), _) => format!("Step {n}"),
        (None, Focus::All) => String::new(),
        (None, f) => format!("focus: {}", f.label()),
    };
    let paused = if status.paused { " · paused" } else { "" };
    if focus_chip.is_empty() {
        format!("{pos} / {total}{paused}")
    } else {
        format!("{pos} / {total} · {focus_chip}{paused}")
    }
}

fn right_text(status: &StatusLine) -> String {
    if let Some(msg) = status.toast { return msg.to_string(); }
    status.sobriety_days.map_or(String::new(), |d| {
        if d < 0 { String::new() } else { format!("Day {d}") }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stub_mixer() -> PulseMixer {
        PulseMixer::from_sources(&[], None, crate::pulse::Order::Random)
    }

    fn stub<'a>(mixer: &'a PulseMixer) -> StatusLine<'a> {
        StatusLine {
            mixer, focus: Focus::All, focus_step: None,
            pulse_progress: None, sobriety_days: None, paused: false, toast: None,
        }
    }

    #[test]
    fn left_text_no_focus() {
        let mixer = stub_mixer();
        assert_eq!(left_text(&stub(&mixer)), "1 / 1");
    }

    #[test]
    fn left_text_step_focus() {
        let mixer = stub_mixer();
        let mut s = stub(&mixer); s.focus_step = Some(3);
        assert!(left_text(&s).contains("Step 3"));
    }

    #[test]
    fn left_text_source_focus() {
        let mixer = stub_mixer();
        let mut s = stub(&mixer); s.focus = Focus::Prayers;
        assert!(left_text(&s).contains("focus: prayers"));
    }

    #[test]
    fn left_text_paused() {
        let mixer = stub_mixer();
        let mut s = stub(&mixer); s.paused = true;
        assert!(left_text(&s).ends_with("paused"));
    }

    #[test]
    fn right_text_sobriety_day() {
        let mixer = stub_mixer();
        let mut s = stub(&mixer); s.sobriety_days = Some(1123);
        assert_eq!(right_text(&s), "Day 1123");
    }

    #[test]
    fn right_text_empty_when_unset() {
        let mixer = stub_mixer();
        assert!(right_text(&stub(&mixer)).is_empty());
    }

    #[test]
    fn right_text_empty_when_future_date() {
        let mixer = stub_mixer();
        let mut s = stub(&mixer); s.sobriety_days = Some(-5);
        assert!(right_text(&s).is_empty());
    }

    #[test]
    fn right_text_toast_overrides_sobriety() {
        let mixer = stub_mixer();
        let mut s = stub(&mixer);
        s.sobriety_days = Some(100);
        s.toast = Some("copied");
        assert_eq!(right_text(&s), "copied");
    }
}
