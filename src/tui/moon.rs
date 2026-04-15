//! Moon phase for the ambient corner anchor. Computes the phase from a
//! reference new-moon date (2000-01-06 18:14 UTC) and the ~29.53-day lunar
//! synodic cycle. Returns a phase index 0..8 and a short glyph pair:
//!
//!   0 new           🌑
//!   1 waxing cresc  🌒
//!   2 first quarter 🌓
//!   3 waxing gib    🌔
//!   4 full          🌕
//!   5 waning gib    🌖
//!   6 last quarter  🌗
//!   7 waning cresc  🌘
//!
//! When a terminal can't render Unicode moon glyphs, ASCII fallbacks are
//! also exposed via `phase_ascii`.

use chrono::{Datelike, NaiveDate};

const SYNODIC_DAYS: f64 = 29.530_588_853;
/// Reference new moon: 2000-01-06.
const REF_YEAR: i32 = 2000;
const REF_MONTH: u32 = 1;
const REF_DAY: u32 = 6;

pub fn phase_index(date: NaiveDate) -> u8 {
    let reference = NaiveDate::from_ymd_opt(REF_YEAR, REF_MONTH, REF_DAY)
        .expect("reference new-moon date is valid");
    let days = (date - reference).num_days() as f64;
    let cycles = days / SYNODIC_DAYS;
    let frac = cycles - cycles.floor();
    // Partition the [0,1) cycle into 8 roughly-equal arcs (new..waning cresc).
    let idx = (frac * 8.0).floor() as i64;
    (idx.rem_euclid(8)) as u8
}

pub fn phase_glyph(idx: u8) -> &'static str {
    match idx % 8 {
        0 => "🌑",
        1 => "🌒",
        2 => "🌓",
        3 => "🌔",
        4 => "🌕",
        5 => "🌖",
        6 => "🌗",
        _ => "🌘",
    }
}

#[allow(dead_code, reason = "ASCII fallback kept for terminals that don't render Unicode moon glyphs")]
pub fn phase_ascii(idx: u8) -> &'static str {
    match idx % 8 {
        0 => "( )",
        1 => "()",
        2 => "D ",
        3 => "D)",
        4 => "O ",
        5 => "(O",
        6 => " C",
        _ => "()",
    }
}

pub fn phase_name(idx: u8) -> &'static str {
    match idx % 8 {
        0 => "new",
        1 => "waxing crescent",
        2 => "first quarter",
        3 => "waxing gibbous",
        4 => "full",
        5 => "waning gibbous",
        6 => "last quarter",
        _ => "waning crescent",
    }
}

/// For today's date with local timezone.
#[allow(dead_code, reason = "convenience accessor; status uses phase_index directly")]
pub fn today() -> u8 {
    phase_index(chrono::Local::now().date_naive())
}

/// Day-of-week + phase combo for the corner. Returns `"Fri · 🌕 full"`.
#[allow(dead_code, reason = "convenience formatter; status uses individual pieces")]
pub fn short_label(date: NaiveDate) -> String {
    let idx = phase_index(date);
    let dow = match date.weekday() {
        chrono::Weekday::Mon => "Mon", chrono::Weekday::Tue => "Tue",
        chrono::Weekday::Wed => "Wed", chrono::Weekday::Thu => "Thu",
        chrono::Weekday::Fri => "Fri", chrono::Weekday::Sat => "Sat",
        chrono::Weekday::Sun => "Sun",
    };
    format!("{dow} · {} {}", phase_glyph(idx), phase_name(idx))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_date_is_new_moon() {
        let d = NaiveDate::from_ymd_opt(2000, 1, 6).unwrap();
        assert_eq!(phase_index(d), 0);
    }

    #[test]
    fn phase_is_stable_across_days() {
        // Two consecutive days should share or differ by at most one phase.
        for d in 1..=365 {
            let a = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap() + chrono::Duration::days(d);
            let b = a + chrono::Duration::days(1);
            let da = phase_index(a);
            let db = phase_index(b);
            let diff = db.wrapping_sub(da) % 8;
            assert!(diff <= 1 || diff == 0 || diff == 7,
                "phase jumped from {da} to {db} in one day");
        }
    }

    #[test]
    fn phase_glyph_covers_all_indices() {
        for i in 0..16u8 {
            let g = phase_glyph(i);
            assert!(!g.is_empty());
        }
    }

    #[test]
    fn phase_name_never_empty() {
        for i in 0..8 {
            assert!(!phase_name(i).is_empty());
        }
    }

    #[test]
    fn full_moon_falls_at_half_cycle() {
        // ~14.77 days after the reference new moon should land in full (4).
        let d = NaiveDate::from_ymd_opt(2000, 1, 6).unwrap() + chrono::Duration::days(15);
        assert_eq!(phase_index(d), 4);
    }
}
