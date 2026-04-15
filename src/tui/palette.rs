//! Palette: an 8-variant color scheme system. Each palette has a light and a
//! dark form; `Mode` decides which form is used at runtime.

use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode { Light, Dark }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Default,
    Warm,
    Cool,
    Mono,
    Sunset,
    Sage,
    Dawn,
    Dusk,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code, reason = "mode/variant/bg are part of the public Palette surface; not all consumers use them yet")]
pub struct Palette {
    pub mode: Mode,
    pub variant: Variant,
    pub bg: Color,
    pub accent: Color,
    pub body: Color,
    pub muted: Color,
}

impl Variant {
    pub fn parse(raw: Option<&str>) -> Variant {
        match raw {
            Some("warm")    => Variant::Warm,
            Some("cool")    => Variant::Cool,
            Some("mono")    => Variant::Mono,
            Some("sunset")  => Variant::Sunset,
            Some("sage")    => Variant::Sage,
            Some("dawn")    => Variant::Dawn,
            Some("dusk")    => Variant::Dusk,
            _ => Variant::Default,
        }
    }
}

impl Palette {
    pub fn build(mode: Mode, variant: Variant) -> Self {
        let (bg, accent, body, muted) = match (mode, variant) {
            (Mode::Dark, Variant::Default) => (
                Color::Reset,
                Color::Rgb(0x00, 0xD7, 0xFF),
                Color::Rgb(0xC9, 0xD1, 0xD9),
                Color::Rgb(0xA8, 0xA8, 0xA8),
            ),
            (Mode::Light, Variant::Default) => (
                Color::Reset,
                Color::Rgb(0x09, 0x69, 0xDA),
                Color::Rgb(0x24, 0x29, 0x2F),
                Color::Rgb(0x57, 0x60, 0x6A),
            ),
            (Mode::Dark, Variant::Warm) => (
                Color::Reset,
                Color::Rgb(0xFF, 0xC1, 0x07),
                Color::Rgb(0xF5, 0xE6, 0xC8),
                Color::Rgb(0xC4, 0x86, 0x5C),
            ),
            (Mode::Light, Variant::Warm) => (
                Color::Reset,
                Color::Rgb(0xB8, 0x86, 0x0B),
                Color::Rgb(0x4A, 0x37, 0x28),
                Color::Rgb(0x8B, 0x57, 0x3A),
            ),
            (Mode::Dark, Variant::Cool) => (
                Color::Reset,
                Color::Rgb(0x83, 0x9A, 0xB1),
                Color::Rgb(0xC9, 0xD3, 0xDC),
                Color::Rgb(0x70, 0x80, 0x90),
            ),
            (Mode::Light, Variant::Cool) => (
                Color::Reset,
                Color::Rgb(0x35, 0x5A, 0x7E),
                Color::Rgb(0x2B, 0x35, 0x42),
                Color::Rgb(0x6B, 0x77, 0x82),
            ),
            (Mode::Dark, Variant::Mono) => (
                Color::Reset,
                Color::Rgb(0xFF, 0xFF, 0xFF),
                Color::Rgb(0xE0, 0xE0, 0xE0),
                Color::Rgb(0x80, 0x80, 0x80),
            ),
            (Mode::Light, Variant::Mono) => (
                Color::Reset,
                Color::Rgb(0x00, 0x00, 0x00),
                Color::Rgb(0x20, 0x20, 0x20),
                Color::Rgb(0x80, 0x80, 0x80),
            ),
            (Mode::Dark, Variant::Sunset) => (
                Color::Reset,
                Color::Rgb(0xFF, 0x7A, 0x29),
                Color::Rgb(0xD9, 0x66, 0x44),
                Color::Rgb(0x8B, 0x55, 0x82),
            ),
            (Mode::Light, Variant::Sunset) => (
                Color::Reset,
                Color::Rgb(0xC2, 0x4D, 0x09),
                Color::Rgb(0x6F, 0x2E, 0x16),
                Color::Rgb(0x4F, 0x29, 0x4A),
            ),
            (Mode::Dark, Variant::Sage) => (
                Color::Reset,
                Color::Rgb(0x9C, 0xC4, 0x8E),
                Color::Rgb(0xF3, 0xEE, 0xD8),
                Color::Rgb(0x4D, 0x6B, 0x47),
            ),
            (Mode::Light, Variant::Sage) => (
                Color::Reset,
                Color::Rgb(0x4D, 0x6B, 0x47),
                Color::Rgb(0x2A, 0x3C, 0x27),
                Color::Rgb(0x7B, 0x8E, 0x6F),
            ),
            (Mode::Dark, Variant::Dawn) => (
                Color::Reset,
                Color::Rgb(0xF5, 0xC2, 0xC7),
                Color::Rgb(0xFB, 0xF2, 0xE9),
                Color::Rgb(0xC4, 0x82, 0x82),
            ),
            (Mode::Light, Variant::Dawn) => (
                Color::Reset,
                Color::Rgb(0xC2, 0x66, 0x70),
                Color::Rgb(0x4A, 0x35, 0x35),
                Color::Rgb(0x86, 0x55, 0x59),
            ),
            (Mode::Dark, Variant::Dusk) => (
                Color::Reset,
                Color::Rgb(0x86, 0x80, 0xC8),
                Color::Rgb(0xCB, 0xC1, 0xE5),
                Color::Rgb(0x6E, 0x76, 0x8B),
            ),
            (Mode::Light, Variant::Dusk) => (
                Color::Reset,
                Color::Rgb(0x42, 0x3A, 0x82),
                Color::Rgb(0x29, 0x24, 0x4C),
                Color::Rgb(0x60, 0x5C, 0x7A),
            ),
        };
        Palette { mode, variant, bg, accent, body, muted }
    }
}

/// Detect light vs dark from `IWIYWI_THEME` (light|dark|auto) with COLORFGBG fallback.
pub fn detect_mode() -> Mode {
    match std::env::var("IWIYWI_THEME").ok().as_deref() {
        Some("light") => Mode::Light,
        Some("dark") => Mode::Dark,
        _ => auto_mode(),
    }
}

fn auto_mode() -> Mode {
    if let Ok(fgbg) = std::env::var("COLORFGBG") {
        if let Some(bg) = fgbg.split(';').nth(1).and_then(|s| s.parse::<u8>().ok()) {
            let is_light = matches!(bg, 7 | 9..=15);
            return if is_light { Mode::Light } else { Mode::Dark };
        }
    }
    Mode::Dark
}

pub fn from_env() -> Palette {
    let mode = detect_mode();
    let variant = Variant::parse(std::env::var("IWIYWI_PALETTE").ok().as_deref());
    Palette::build(mode, variant)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_parse_defaults() {
        assert_eq!(Variant::parse(None), Variant::Default);
        assert_eq!(Variant::parse(Some("garbage")), Variant::Default);
    }

    #[test]
    fn variant_parse_each() {
        for (s, v) in [
            ("warm", Variant::Warm), ("cool", Variant::Cool),
            ("mono", Variant::Mono), ("sunset", Variant::Sunset),
            ("sage", Variant::Sage), ("dawn", Variant::Dawn),
            ("dusk", Variant::Dusk),
        ] {
            assert_eq!(Variant::parse(Some(s)), v);
        }
    }

    #[test]
    fn palettes_differ_per_variant() {
        let dark_default = Palette::build(Mode::Dark, Variant::Default);
        let dark_warm    = Palette::build(Mode::Dark, Variant::Warm);
        assert_ne!(dark_default.accent, dark_warm.accent);
    }

    #[test]
    fn palettes_have_distinct_light_and_dark_bodies() {
        let l = Palette::build(Mode::Light, Variant::Default);
        let d = Palette::build(Mode::Dark, Variant::Default);
        assert_ne!(l.body, d.body);
    }
}
