//! Palette: a 19-variant color scheme system. Each palette has a light and a
//! dark form; `Mode` decides which form is used at runtime.

use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode { Light, Dark }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    // Original 8.
    Default,
    Warm,
    Cool,
    Mono,
    Sunset,
    Sage,
    Dawn,
    Dusk,
    // New 11.
    Ember,
    Ocean,
    Rose,
    Forest,
    Amber,
    Slate,
    Mint,
    Lavender,
    Copper,
    Indigo,
    Nord,
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
    pub const ALL: [Variant; 19] = [
        Variant::Default, Variant::Warm, Variant::Cool, Variant::Mono,
        Variant::Sunset, Variant::Sage, Variant::Dawn, Variant::Dusk,
        Variant::Ember, Variant::Ocean, Variant::Rose, Variant::Forest,
        Variant::Amber, Variant::Slate, Variant::Mint, Variant::Lavender,
        Variant::Copper, Variant::Indigo, Variant::Nord,
    ];

    pub fn parse(raw: Option<&str>) -> Variant {
        match raw {
            Some("warm")     => Variant::Warm,
            Some("cool")     => Variant::Cool,
            Some("mono")     => Variant::Mono,
            Some("sunset")   => Variant::Sunset,
            Some("sage")     => Variant::Sage,
            Some("dawn")     => Variant::Dawn,
            Some("dusk")     => Variant::Dusk,
            Some("ember")    => Variant::Ember,
            Some("ocean")    => Variant::Ocean,
            Some("rose")     => Variant::Rose,
            Some("forest")   => Variant::Forest,
            Some("amber")    => Variant::Amber,
            Some("slate")    => Variant::Slate,
            Some("mint")     => Variant::Mint,
            Some("lavender") => Variant::Lavender,
            Some("copper")   => Variant::Copper,
            Some("indigo")   => Variant::Indigo,
            Some("nord")     => Variant::Nord,
            _ => Variant::Default,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Variant::Default  => "default",
            Variant::Warm     => "warm",
            Variant::Cool     => "cool",
            Variant::Mono     => "mono",
            Variant::Sunset   => "sunset",
            Variant::Sage     => "sage",
            Variant::Dawn     => "dawn",
            Variant::Dusk     => "dusk",
            Variant::Ember    => "ember",
            Variant::Ocean    => "ocean",
            Variant::Rose     => "rose",
            Variant::Forest   => "forest",
            Variant::Amber    => "amber",
            Variant::Slate    => "slate",
            Variant::Mint     => "mint",
            Variant::Lavender => "lavender",
            Variant::Copper   => "copper",
            Variant::Indigo   => "indigo",
            Variant::Nord     => "nord",
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

            // --- New variants below ---

            // Ember: hot coal with radiating heat.
            (Mode::Dark, Variant::Ember) => (
                Color::Reset,
                Color::Rgb(0xFF, 0x45, 0x00),
                Color::Rgb(0xE8, 0xD0, 0xC0),
                Color::Rgb(0x8B, 0x3A, 0x1F),
            ),
            (Mode::Light, Variant::Ember) => (
                Color::Reset,
                Color::Rgb(0xC2, 0x36, 0x16),
                Color::Rgb(0x3D, 0x28, 0x17),
                Color::Rgb(0x8B, 0x45, 0x13),
            ),

            // Ocean: deep water, sea foam.
            (Mode::Dark, Variant::Ocean) => (
                Color::Reset,
                Color::Rgb(0x0D, 0xB3, 0x9E),
                Color::Rgb(0xB8, 0xE8, 0xE0),
                Color::Rgb(0x4A, 0x7C, 0x74),
            ),
            (Mode::Light, Variant::Ocean) => (
                Color::Reset,
                Color::Rgb(0x00, 0x60, 0x64),
                Color::Rgb(0x1B, 0x3A, 0x3F),
                Color::Rgb(0x5A, 0x8A, 0x8F),
            ),

            // Rose: vivid magenta accent, blush body.
            (Mode::Dark, Variant::Rose) => (
                Color::Reset,
                Color::Rgb(0xE9, 0x1E, 0x63),
                Color::Rgb(0xFF, 0xD1, 0xDC),
                Color::Rgb(0x8B, 0x2F, 0x5C),
            ),
            (Mode::Light, Variant::Rose) => (
                Color::Reset,
                Color::Rgb(0xAD, 0x14, 0x57),
                Color::Rgb(0x3E, 0x1A, 0x2A),
                Color::Rgb(0x9C, 0x43, 0x70),
            ),

            // Forest: deep pine, soft mint body.
            (Mode::Dark, Variant::Forest) => (
                Color::Reset,
                Color::Rgb(0x22, 0x8B, 0x22),
                Color::Rgb(0xD4, 0xE8, 0xC8),
                Color::Rgb(0x55, 0x6B, 0x2F),
            ),
            (Mode::Light, Variant::Forest) => (
                Color::Reset,
                Color::Rgb(0x0F, 0x4F, 0x0F),
                Color::Rgb(0x1F, 0x2E, 0x1F),
                Color::Rgb(0x4F, 0x70, 0x4F),
            ),

            // Amber: rich amber against warm cream.
            (Mode::Dark, Variant::Amber) => (
                Color::Reset,
                Color::Rgb(0xFF, 0xB0, 0x00),
                Color::Rgb(0xF5, 0xE6, 0xB8),
                Color::Rgb(0xB8, 0x86, 0x0B),
            ),
            (Mode::Light, Variant::Amber) => (
                Color::Reset,
                Color::Rgb(0x8B, 0x65, 0x08),
                Color::Rgb(0x3D, 0x2F, 0x10),
                Color::Rgb(0x7A, 0x6A, 0x1F),
            ),

            // Slate: steel blue on cool gray.
            (Mode::Dark, Variant::Slate) => (
                Color::Reset,
                Color::Rgb(0x46, 0x82, 0xB4),
                Color::Rgb(0xC0, 0xD0, 0xE0),
                Color::Rgb(0x5F, 0x7C, 0x8A),
            ),
            (Mode::Light, Variant::Slate) => (
                Color::Reset,
                Color::Rgb(0x2C, 0x4A, 0x6B),
                Color::Rgb(0x1F, 0x2A, 0x38),
                Color::Rgb(0x5A, 0x6B, 0x7A),
            ),

            // Mint: bright spring mint with pale green body.
            (Mode::Dark, Variant::Mint) => (
                Color::Reset,
                Color::Rgb(0x00, 0xFA, 0x9A),
                Color::Rgb(0xC8, 0xF0, 0xD4),
                Color::Rgb(0x3C, 0xB3, 0x71),
            ),
            (Mode::Light, Variant::Mint) => (
                Color::Reset,
                Color::Rgb(0x00, 0x69, 0x5C),
                Color::Rgb(0x1B, 0x3A, 0x2F),
                Color::Rgb(0x5D, 0x8B, 0x7A),
            ),

            // Lavender: soft purple on pale lilac.
            (Mode::Dark, Variant::Lavender) => (
                Color::Reset,
                Color::Rgb(0xC5, 0xA8, 0xFF),
                Color::Rgb(0xE6, 0xD8, 0xF5),
                Color::Rgb(0x8A, 0x6E, 0xBF),
            ),
            (Mode::Light, Variant::Lavender) => (
                Color::Reset,
                Color::Rgb(0x6A, 0x3A, 0xB8),
                Color::Rgb(0x2D, 0x1F, 0x4A),
                Color::Rgb(0x7A, 0x5D, 0x9A),
            ),

            // Copper: burnished copper on warm cream.
            (Mode::Dark, Variant::Copper) => (
                Color::Reset,
                Color::Rgb(0xD9, 0x77, 0x06),
                Color::Rgb(0xF5, 0xD0, 0xA0),
                Color::Rgb(0x8B, 0x45, 0x13),
            ),
            (Mode::Light, Variant::Copper) => (
                Color::Reset,
                Color::Rgb(0xA0, 0x52, 0x2D),
                Color::Rgb(0x3D, 0x1F, 0x10),
                Color::Rgb(0x8B, 0x5A, 0x3C),
            ),

            // Indigo: electric indigo on periwinkle.
            (Mode::Dark, Variant::Indigo) => (
                Color::Reset,
                Color::Rgb(0x7C, 0x5C, 0xFF),
                Color::Rgb(0xC8, 0xBF, 0xE8),
                Color::Rgb(0x4B, 0x3F, 0x7A),
            ),
            (Mode::Light, Variant::Indigo) => (
                Color::Reset,
                Color::Rgb(0x37, 0x30, 0xA3),
                Color::Rgb(0x1E, 0x1B, 0x4B),
                Color::Rgb(0x63, 0x66, 0xF1),
            ),

            // Nord: classic Arctic palette — frost blue on snow.
            (Mode::Dark, Variant::Nord) => (
                Color::Reset,
                Color::Rgb(0x88, 0xC0, 0xD0),
                Color::Rgb(0xEC, 0xEF, 0xF4),
                Color::Rgb(0x5E, 0x81, 0xAC),
            ),
            (Mode::Light, Variant::Nord) => (
                Color::Reset,
                Color::Rgb(0x5E, 0x81, 0xAC),
                Color::Rgb(0x2E, 0x34, 0x40),
                Color::Rgb(0x81, 0xA1, 0xC1),
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
    use std::collections::HashSet;

    #[test]
    fn variant_parse_defaults() {
        assert_eq!(Variant::parse(None), Variant::Default);
        assert_eq!(Variant::parse(Some("garbage")), Variant::Default);
    }

    #[test]
    fn variant_parse_each() {
        for v in Variant::ALL {
            // Every variant's label must round-trip through parse.
            assert_eq!(Variant::parse(Some(v.label())), v,
                "variant {} failed round-trip", v.label());
        }
    }

    #[test]
    fn all_variants_covers_enum() {
        // Guard against adding a variant and forgetting to append to ALL.
        assert_eq!(Variant::ALL.len(), 19);
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

    #[test]
    fn every_dark_accent_is_unique() {
        let mut seen = HashSet::new();
        for v in Variant::ALL {
            let p = Palette::build(Mode::Dark, v);
            let key = color_key(p.accent);
            assert!(seen.insert(key),
                "duplicate dark accent for variant {}", v.label());
        }
    }

    #[test]
    fn every_light_accent_is_unique() {
        let mut seen = HashSet::new();
        for v in Variant::ALL {
            let p = Palette::build(Mode::Light, v);
            let key = color_key(p.accent);
            assert!(seen.insert(key),
                "duplicate light accent for variant {}", v.label());
        }
    }

    #[test]
    fn every_variant_has_three_distinct_colors() {
        for v in Variant::ALL {
            for m in [Mode::Dark, Mode::Light] {
                let p = Palette::build(m, v);
                // accent vs body, accent vs muted, body vs muted — all must
                // differ, otherwise one role is invisible against another.
                assert_ne!(p.accent, p.body,
                    "{} {:?}: accent == body", v.label(), m);
                assert_ne!(p.accent, p.muted,
                    "{} {:?}: accent == muted", v.label(), m);
                assert_ne!(p.body, p.muted,
                    "{} {:?}: body == muted", v.label(), m);
            }
        }
    }

    #[test]
    fn every_label_is_lowercase_ascii() {
        for v in Variant::ALL {
            let l = v.label();
            assert!(l.chars().all(|c| c.is_ascii_lowercase()),
                "label {l} contains non-lowercase-ascii");
            assert!(!l.is_empty());
        }
    }

    fn color_key(c: Color) -> (u8, u8, u8) {
        match c {
            Color::Rgb(r, g, b) => (r, g, b),
            _ => (0, 0, 0),
        }
    }
}
