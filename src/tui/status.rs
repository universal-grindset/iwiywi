//! Status footer: a single row at the bottom of the pulse. Left-aligned
//! shows `position / total · focus`, right-aligned shows `Day N` if the
//! user has set `IWIYWI_SOBER_SINCE`. Between them: a thin progress bar
//! indicating time until the next auto-advance.

pub use ratatui::buffer::Buffer;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    Frame,
};

use crate::pulse::{Focus, PulseMixer};
use crate::tui::moon;
use crate::tui::palette::Palette;
use crate::tui::weather::WeatherSnapshot;

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
    /// Active `/`-search query. When `Some`, takes over the left slot.
    pub search_query: Option<&'a str>,
    /// Number of matches for a submitted search. Shown after the query
    /// exits search mode so the user knows how many `n`/`N` stops exist.
    pub search_match_count: Option<usize>,
    /// Context-sensitive keyboard hints. When non-empty, take over the
    /// right slot ahead of the sobriety day counter. Tui-design skill:
    /// "Help shows what's actionable right now, not everything ever."
    pub hints: &'a str,
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
    // Active search prompt takes over the left slot: `/query_`.
    if let Some(q) = status.search_query {
        return format!("/{q}_");
    }
    let pos = status.mixer.cursor() + 1;
    let total = status.mixer.len().max(1);
    let focus_chip = match (status.focus_step, status.focus) {
        (Some(n), _) => format!("Step {n}"),
        (None, Focus::All) => String::new(),
        (None, f) => format!("focus: {}", f.label()),
    };
    let search_chip = status.search_match_count
        .filter(|n| *n > 0)
        .map_or(String::new(), |n| format!(" · {n} match{}", if n == 1 { "" } else { "es" }));
    let paused = if status.paused { " · paused" } else { "" };
    if focus_chip.is_empty() {
        format!("{pos} / {total}{search_chip}{paused}")
    } else {
        format!("{pos} / {total} · {focus_chip}{search_chip}{paused}")
    }
}

fn right_text(status: &StatusLine) -> String {
    // Priority: transient toast > contextual hints > sobriety anchor.
    if let Some(msg) = status.toast { return msg.to_string(); }
    if !status.hints.is_empty() { return status.hints.to_string(); }
    status.sobriety_days.map_or(String::new(), |d| {
        if d < 0 { String::new() } else { format!("Day {d}") }
    })
}

/// Top-left ambient anchor: weather one-liner from wttr.in. Paired
/// visually with the moon/sober anchor in the top-right. Renders only
/// when wttr.in returned something and the viewport has room.
pub fn draw_weather_anchor(
    buf: &mut Buffer,
    area: Rect,
    palette: &Palette,
    weather: Option<&WeatherSnapshot>,
) {
    let Some(w) = weather else { return; };
    if area.width < 30 || area.height < 2 { return; }
    let text_w = w.text.chars().count() as u16 + 2;
    if area.width < text_w + 2 { return; }
    let rect = Rect { x: area.x + 1, y: area.y, width: text_w, height: 1 };
    Paragraph::new(Line::from(Span::styled(
        w.text.clone(),
        Style::default().fg(palette.muted).add_modifier(Modifier::ITALIC),
    )))
    .render(rect, buf);
}

/// Draw the ambient corner anchor: moon phase + optional `Day N` counter.
/// Top-right of `area`. Renders only if the viewport is wide enough and if
/// there's something to show (moon always shows when space permits; the
/// day counter shows when `sobriety_days` is `Some(positive)`).
pub fn draw_moon_anchor(
    buf: &mut Buffer,
    area: Rect,
    palette: &Palette,
    sobriety_days: Option<i64>,
) {
    if area.width < 30 || area.height < 3 { return; }
    let today = chrono::Local::now().date_naive();
    let idx = moon::phase_index(today);
    let glyph = moon::phase_glyph(idx);
    let day_str = sobriety_days
        .filter(|d| *d >= 0)
        .map_or(String::new(), |d| format!(" · Day {d}"));
    let text = format!("{glyph} {}{day_str}", moon::phase_name(idx));
    // Width must budget for the glyph (often 2 terminal cells for emoji
    // moon chars). Assume at most 2 extra cells for the glyph vs char count.
    let text_w = text.chars().count() as u16 + 2;
    if area.width < text_w + 2 { return; }
    let x = area.x + area.width.saturating_sub(text_w + 1);
    let y = area.y;
    let rect = Rect { x, y, width: text_w, height: 1 };
    Paragraph::new(Line::from(Span::styled(
        text,
        Style::default().fg(palette.muted).add_modifier(Modifier::ITALIC),
    )))
    .render(rect, buf);
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
            search_query: None, search_match_count: None, hints: "",
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
