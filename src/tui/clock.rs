//! Giant LCD-style clock rendered in the top-left of the viewport. Uses
//! 4×5 box-drawing digits that read like a proper digital-clock display
//! so the time is legible from across a room — the "dedicated monitor"
//! aesthetic. Clears its own backdrop so drift particles / pattern dots
//! don't bleed through the transparent gaps inside each digit.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
};

use crate::tui::palette::Palette;

const DIGIT_ROWS: usize = 5;
const DIGIT_COLS: usize = 4;

/// Each digit is a 4×5 grid. Non-space cells render with the muted style;
/// spaces are left blank (still blanked, because the backdrop was cleared
/// first). Glyphs drawn: `━` horizontal bar, `┃` vertical bar.
#[rustfmt::skip]
const DIGITS: [[&str; DIGIT_ROWS]; 10] = [
    // 0
    [" ━━ ",
     "┃  ┃",
     "┃  ┃",
     "┃  ┃",
     " ━━ "],
    // 1
    ["   ┃",
     "   ┃",
     "   ┃",
     "   ┃",
     "   ┃"],
    // 2
    [" ━━ ",
     "   ┃",
     " ━━ ",
     "┃   ",
     " ━━ "],
    // 3
    [" ━━ ",
     "   ┃",
     " ━━ ",
     "   ┃",
     " ━━ "],
    // 4
    ["┃  ┃",
     "┃  ┃",
     " ━━┃",
     "   ┃",
     "   ┃"],
    // 5
    [" ━━ ",
     "┃   ",
     " ━━ ",
     "   ┃",
     " ━━ "],
    // 6
    [" ━━ ",
     "┃   ",
     " ━━ ",
     "┃  ┃",
     " ━━ "],
    // 7
    [" ━━ ",
     "   ┃",
     "   ┃",
     "   ┃",
     "   ┃"],
    // 8
    [" ━━ ",
     "┃  ┃",
     " ━━ ",
     "┃  ┃",
     " ━━ "],
    // 9
    [" ━━ ",
     "┃  ┃",
     " ━━ ",
     "   ┃",
     " ━━ "],
];

const COLON_DOT: &str = "·";

/// Render `HH:MM` at the top-left of `area`. Only shown on viewports
/// roomy enough for a dedicated-monitor setup (≥ 80×20 cells) — smaller
/// terminals would have the clock crowding the centered body.
pub fn draw(buf: &mut Buffer, area: Rect, palette: &Palette) {
    // Layout: 4 digits × 4 cols + 3 gaps + 1 colon col + 2 margin = 22 cells.
    let needed_w: u16 = 22;
    let needed_h: u16 = DIGIT_ROWS as u16 + 2;
    if area.width < 80 || area.height < 20 { return; }
    if area.width < needed_w || area.height < needed_h { return; }

    let now = chrono::Local::now();
    let hh = now.format("%H").to_string();
    let mm = now.format("%M").to_string();
    let mut digits: Vec<u8> = Vec::with_capacity(4);
    for s in [&hh, &mm] {
        for c in s.chars() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d as u8);
            }
        }
    }
    if digits.len() != 4 { return; }

    // Clear the clock's bounding box first so drift particles and pattern
    // marks don't show through the transparent gaps in the digit shapes.
    // Using Style::default() (no bg) lets the terminal's own background
    // color show — cleaner than forcing palette.bg (often Reset).
    let blank = Style::default();
    for y in area.y..(area.y + needed_h).min(buf.area.bottom()) {
        for x in area.x..(area.x + needed_w).min(buf.area.right()) {
            buf[(x, y)].set_symbol(" ").set_style(blank);
        }
    }

    let style = Style::default().fg(palette.muted);
    let start_x = area.x + 1;
    let start_y = area.y + 1;

    let mut x = start_x;
    for (i, d) in digits.iter().enumerate() {
        draw_digit(buf, x, start_y, *d, style);
        x += DIGIT_COLS as u16;
        if i == 1 {
            draw_colon(buf, x, start_y, style);
            x += 1;
        }
    }
}

fn draw_digit(buf: &mut Buffer, x: u16, y: u16, digit: u8, style: Style) {
    if digit as usize >= DIGITS.len() { return; }
    let glyph = &DIGITS[digit as usize];
    for (row, line) in glyph.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let cx = x + col as u16;
            let cy = y + row as u16;
            if cx >= buf.area.right() || cy >= buf.area.bottom() { continue; }
            if c != ' ' {
                let mut tmp = [0u8; 4];
                let s = c.encode_utf8(&mut tmp).to_string();
                buf[(cx, cy)].set_symbol(&s).set_style(style);
            }
        }
    }
}

fn draw_colon(buf: &mut Buffer, x: u16, y: u16, style: Style) {
    // Two dots at rows 1 and 3 of the 5-row digit height — classic LCD colon.
    for cy in [y + 1, y + 3] {
        if x < buf.area.right() && cy < buf.area.bottom() {
            buf[(x, cy)].set_symbol(COLON_DOT).set_style(style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_digit_has_five_rows_of_four_cols() {
        for (i, digit) in DIGITS.iter().enumerate() {
            assert_eq!(digit.len(), DIGIT_ROWS, "digit {i} row count");
            for (r, row) in digit.iter().enumerate() {
                assert_eq!(row.chars().count(), DIGIT_COLS,
                    "digit {i} row {r}: expected {DIGIT_COLS} cols, got {}",
                    row.chars().count());
            }
        }
    }

    #[test]
    fn digits_use_only_known_glyphs() {
        for digit in DIGITS.iter() {
            for row in digit.iter() {
                for c in row.chars() {
                    assert!(
                        c == ' ' || c == '━' || c == '┃',
                        "unexpected char {c:?} in digit bitmap"
                    );
                }
            }
        }
    }
}
