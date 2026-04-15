//! Giant ASCII clock rendered into the top-left of the viewport. Uses
//! 5-row-tall block digits so the time reads from across a room — the
//! "dedicated monitor" aesthetic.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
};

use crate::tui::palette::Palette;

/// Each digit is a 5×3 grid of cells. Full cell `█`, empty cell ` `.
const DIGIT_ROWS: usize = 5;
const DIGIT_COLS: usize = 3;
const COLON_COLS: usize = 1;

const FULL: &str = "█";
const COLON_DOT: &str = "·";

#[rustfmt::skip]
const DIGITS: [[&str; DIGIT_ROWS]; 10] = [
    // 0
    ["███",
     "█ █",
     "█ █",
     "█ █",
     "███"],
    // 1
    [" █ ",
     "██ ",
     " █ ",
     " █ ",
     "███"],
    // 2
    ["███",
     "  █",
     "███",
     "█  ",
     "███"],
    // 3
    ["███",
     "  █",
     " ██",
     "  █",
     "███"],
    // 4
    ["█ █",
     "█ █",
     "███",
     "  █",
     "  █"],
    // 5
    ["███",
     "█  ",
     "███",
     "  █",
     "███"],
    // 6
    ["███",
     "█  ",
     "███",
     "█ █",
     "███"],
    // 7
    ["███",
     "  █",
     "  █",
     "  █",
     "  █"],
    // 8
    ["███",
     "█ █",
     "███",
     "█ █",
     "███"],
    // 9
    ["███",
     "█ █",
     "███",
     "  █",
     "███"],
];

/// Render "HH:MM" at the top-left of `area`. Uses a single-cell gap between
/// digits and a short colon column. Muted color — meant to be glanceable, not
/// demanding. Skips rendering if the viewport can't fit it.
pub fn draw(buf: &mut Buffer, area: Rect, palette: &Palette) {
    // 4 digits × 3 cols + 3 gaps + 1 colon col = 16 cells, plus a 1-cell
    // left margin = 17 cells wide. 5 rows tall. Skip if too cramped.
    let needed_w: u16 = 17;
    let needed_h: u16 = DIGIT_ROWS as u16 + 1;
    if area.width < needed_w * 2 || area.height < needed_h + 2 { return; }

    let now = chrono::Local::now();
    let hh = now.format("%H").to_string();
    let mm = now.format("%M").to_string();
    // Collect the 4 digit chars (no colon — we draw that ourselves).
    let mut digits: Vec<u8> = Vec::with_capacity(4);
    for s in [&hh, &mm] {
        for c in s.chars() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d as u8);
            }
        }
    }
    if digits.len() != 4 { return; }

    let style = Style::default().fg(palette.muted);

    let start_x = area.x + 1;
    let start_y = area.y + 1;

    let mut x = start_x;
    for (i, d) in digits.iter().enumerate() {
        draw_digit(buf, x, start_y, *d, style);
        x += DIGIT_COLS as u16 + 1;
        // After the 2nd digit, draw a colon.
        if i == 1 {
            draw_colon(buf, x, start_y, style);
            x += COLON_COLS as u16 + 1;
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
            if c == '█' {
                buf[(cx, cy)].set_symbol(FULL).set_style(style);
            }
        }
    }
}

fn draw_colon(buf: &mut Buffer, x: u16, y: u16, style: Style) {
    // Two dots at row 1 and row 3 of the 5-row digit height.
    let cy1 = y + 1;
    let cy2 = y + 3;
    if x < buf.area.right() {
        if cy1 < buf.area.bottom() {
            buf[(x, cy1)].set_symbol(COLON_DOT).set_style(style);
        }
        if cy2 < buf.area.bottom() {
            buf[(x, cy2)].set_symbol(COLON_DOT).set_style(style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_digit_has_five_rows_of_three_cols() {
        for (i, digit) in DIGITS.iter().enumerate() {
            assert_eq!(digit.len(), DIGIT_ROWS, "digit {i} row count");
            for (r, row) in digit.iter().enumerate() {
                assert_eq!(row.chars().count(), DIGIT_COLS,
                    "digit {i} row {r}: expected {DIGIT_COLS} cols");
            }
        }
    }

    #[test]
    fn digits_use_only_full_and_space() {
        for digit in DIGITS.iter() {
            for row in digit.iter() {
                for c in row.chars() {
                    assert!(c == '█' || c == ' ',
                        "unexpected char {c:?} in digit bitmap");
                }
            }
        }
    }
}
