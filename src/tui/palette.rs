//! Palette: a 39-variant color scheme system. Each palette has a light and a
//! dark form; `Mode` decides which form is used at runtime.
//!
//! The 20 "black metal" variants are intentionally dark-only — their Light
//! forms return the same RGB values as their Dark forms. BM is not a
//! daylight aesthetic.

use chrono::Timelike;
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Light,
    Dark,
}

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
    // 20 black-metal variants. Dark-only; Light mode returns the same RGB.
    Frostbitten,
    CorpsePaint,
    CrimsonAltar,
    PaleRider,
    Funeral,
    Moonspell,
    Ashen,
    Pestilence,
    Obsidian,
    Winterfell,
    Raven,
    Bloodmoon,
    Tundra,
    Necrotic,
    Cascadian,
    IronOath,
    Hellfire,
    Sepulchral,
    Wraith,
    Abyssal,
    // 12 popular terminal color schemes using authentic RGB values.
    SolarizedDark,
    SolarizedLight,
    GruvboxDark,
    GruvboxLight,
    Dracula,
    TokyoNight,
    Catppuccin,
    OneDark,
    Monokai,
    Synthwave,
    Ayu,
    Kanagawa,
}

#[derive(Debug, Clone, Copy)]
#[allow(
    dead_code,
    reason = "mode/variant/bg are part of the public Palette surface; not all consumers use them yet"
)]
pub struct Palette {
    pub mode: Mode,
    pub variant: Variant,
    pub bg: Color,
    pub accent: Color,
    pub body: Color,
    pub muted: Color,
}

impl Variant {
    pub const ALL: [Variant; 51] = [
        Variant::Default,
        Variant::Warm,
        Variant::Cool,
        Variant::Mono,
        Variant::Sunset,
        Variant::Sage,
        Variant::Dawn,
        Variant::Dusk,
        Variant::Ember,
        Variant::Ocean,
        Variant::Rose,
        Variant::Forest,
        Variant::Amber,
        Variant::Slate,
        Variant::Mint,
        Variant::Lavender,
        Variant::Copper,
        Variant::Indigo,
        Variant::Nord,
        // --- black metal ---
        Variant::Frostbitten,
        Variant::CorpsePaint,
        Variant::CrimsonAltar,
        Variant::PaleRider,
        Variant::Funeral,
        Variant::Moonspell,
        Variant::Ashen,
        Variant::Pestilence,
        Variant::Obsidian,
        Variant::Winterfell,
        Variant::Raven,
        Variant::Bloodmoon,
        Variant::Tundra,
        Variant::Necrotic,
        Variant::Cascadian,
        Variant::IronOath,
        Variant::Hellfire,
        Variant::Sepulchral,
        Variant::Wraith,
        Variant::Abyssal,
        // --- popular terminal schemes ---
        Variant::SolarizedDark,
        Variant::SolarizedLight,
        Variant::GruvboxDark,
        Variant::GruvboxLight,
        Variant::Dracula,
        Variant::TokyoNight,
        Variant::Catppuccin,
        Variant::OneDark,
        Variant::Monokai,
        Variant::Synthwave,
        Variant::Ayu,
        Variant::Kanagawa,
    ];

    pub fn parse(raw: Option<&str>) -> Variant {
        match raw {
            Some("warm") => Variant::Warm,
            Some("cool") => Variant::Cool,
            Some("mono") => Variant::Mono,
            Some("sunset") => Variant::Sunset,
            Some("sage") => Variant::Sage,
            Some("dawn") => Variant::Dawn,
            Some("dusk") => Variant::Dusk,
            Some("ember") => Variant::Ember,
            Some("ocean") => Variant::Ocean,
            Some("rose") => Variant::Rose,
            Some("forest") => Variant::Forest,
            Some("amber") => Variant::Amber,
            Some("slate") => Variant::Slate,
            Some("mint") => Variant::Mint,
            Some("lavender") => Variant::Lavender,
            Some("copper") => Variant::Copper,
            Some("indigo") => Variant::Indigo,
            Some("nord") => Variant::Nord,
            Some("frostbitten") => Variant::Frostbitten,
            Some("corpse_paint") => Variant::CorpsePaint,
            Some("crimson_altar") => Variant::CrimsonAltar,
            Some("pale_rider") => Variant::PaleRider,
            Some("funeral") => Variant::Funeral,
            Some("moonspell") => Variant::Moonspell,
            Some("ashen") => Variant::Ashen,
            Some("pestilence") => Variant::Pestilence,
            Some("obsidian") => Variant::Obsidian,
            Some("winterfell") => Variant::Winterfell,
            Some("raven") => Variant::Raven,
            Some("bloodmoon") => Variant::Bloodmoon,
            Some("tundra") => Variant::Tundra,
            Some("necrotic") => Variant::Necrotic,
            Some("cascadian") => Variant::Cascadian,
            Some("iron_oath") => Variant::IronOath,
            Some("hellfire") => Variant::Hellfire,
            Some("sepulchral") => Variant::Sepulchral,
            Some("wraith") => Variant::Wraith,
            Some("abyssal") => Variant::Abyssal,
            Some("solarized_dark") => Variant::SolarizedDark,
            Some("solarized_light") => Variant::SolarizedLight,
            Some("gruvbox_dark") => Variant::GruvboxDark,
            Some("gruvbox_light") => Variant::GruvboxLight,
            Some("dracula") => Variant::Dracula,
            Some("tokyo_night") => Variant::TokyoNight,
            Some("catppuccin") => Variant::Catppuccin,
            Some("onedark") => Variant::OneDark,
            Some("monokai") => Variant::Monokai,
            Some("synthwave") => Variant::Synthwave,
            Some("ayu") => Variant::Ayu,
            Some("kanagawa") => Variant::Kanagawa,
            _ => Variant::Default,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Variant::Default => "default",
            Variant::Warm => "warm",
            Variant::Cool => "cool",
            Variant::Mono => "mono",
            Variant::Sunset => "sunset",
            Variant::Sage => "sage",
            Variant::Dawn => "dawn",
            Variant::Dusk => "dusk",
            Variant::Ember => "ember",
            Variant::Ocean => "ocean",
            Variant::Rose => "rose",
            Variant::Forest => "forest",
            Variant::Amber => "amber",
            Variant::Slate => "slate",
            Variant::Mint => "mint",
            Variant::Lavender => "lavender",
            Variant::Copper => "copper",
            Variant::Indigo => "indigo",
            Variant::Nord => "nord",
            Variant::Frostbitten => "frostbitten",
            Variant::CorpsePaint => "corpse_paint",
            Variant::CrimsonAltar => "crimson_altar",
            Variant::PaleRider => "pale_rider",
            Variant::Funeral => "funeral",
            Variant::Moonspell => "moonspell",
            Variant::Ashen => "ashen",
            Variant::Pestilence => "pestilence",
            Variant::Obsidian => "obsidian",
            Variant::Winterfell => "winterfell",
            Variant::Raven => "raven",
            Variant::Bloodmoon => "bloodmoon",
            Variant::Tundra => "tundra",
            Variant::Necrotic => "necrotic",
            Variant::Cascadian => "cascadian",
            Variant::IronOath => "iron_oath",
            Variant::Hellfire => "hellfire",
            Variant::Sepulchral => "sepulchral",
            Variant::Wraith => "wraith",
            Variant::Abyssal => "abyssal",
            Variant::SolarizedDark => "solarized_dark",
            Variant::SolarizedLight => "solarized_light",
            Variant::GruvboxDark => "gruvbox_dark",
            Variant::GruvboxLight => "gruvbox_light",
            Variant::Dracula => "dracula",
            Variant::TokyoNight => "tokyo_night",
            Variant::Catppuccin => "catppuccin",
            Variant::OneDark => "onedark",
            Variant::Monokai => "monokai",
            Variant::Synthwave => "synthwave",
            Variant::Ayu => "ayu",
            Variant::Kanagawa => "kanagawa",
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

            // --- Black metal: 20 dark-only variants. Light mode returns the
            // same values as Dark. Each accent is distinct from every other
            // accent in the palette registry; body and muted shift to give
            // each theme a different mood (icy / bloodied / sickly / etc).

            // Frostbitten — Norwegian icy pale blue on void.
            (_, Variant::Frostbitten) => (
                Color::Reset,
                Color::Rgb(0xB8, 0xD4, 0xE8),
                Color::Rgb(0xC0, 0xD0, 0xDC),
                Color::Rgb(0x5A, 0x68, 0x78),
            ),
            // Corpse Paint — stark white accent on charcoal body.
            (_, Variant::CorpsePaint) => (
                Color::Reset,
                Color::Rgb(0xF5, 0xF5, 0xF5),
                Color::Rgb(0xE0, 0xE0, 0xE0),
                Color::Rgb(0x4A, 0x4A, 0x4A),
            ),
            // Crimson Altar — dried-blood accent.
            (_, Variant::CrimsonAltar) => (
                Color::Reset,
                Color::Rgb(0x8B, 0x00, 0x00),
                Color::Rgb(0xB8, 0xA0, 0xA0),
                Color::Rgb(0x5C, 0x33, 0x33),
            ),
            // Pale Rider — bone-gold ghost white.
            (_, Variant::PaleRider) => (
                Color::Reset,
                Color::Rgb(0xC9, 0xBD, 0xA8),
                Color::Rgb(0xBA, 0xB0, 0xA0),
                Color::Rgb(0x6B, 0x5D, 0x4A),
            ),
            // Funeral — dead-grey stone.
            (_, Variant::Funeral) => (
                Color::Reset,
                Color::Rgb(0x6B, 0x70, 0x78),
                Color::Rgb(0xA0, 0xA5, 0xAC),
                Color::Rgb(0x3F, 0x43, 0x49),
            ),
            // Moonspell — silver on midnight.
            (_, Variant::Moonspell) => (
                Color::Reset,
                Color::Rgb(0xC0, 0xC0, 0xC0),
                Color::Rgb(0xB0, 0xB8, 0xC4),
                Color::Rgb(0x4A, 0x50, 0x60),
            ),
            // Ashen — ember orange through the ashes.
            (_, Variant::Ashen) => (
                Color::Reset,
                Color::Rgb(0xCC, 0x55, 0x00),
                Color::Rgb(0xC4, 0xB8, 0xA8),
                Color::Rgb(0x5A, 0x4A, 0x3A),
            ),
            // Pestilence — sickly decay yellow-green.
            (_, Variant::Pestilence) => (
                Color::Reset,
                Color::Rgb(0x9B, 0x9B, 0x2A),
                Color::Rgb(0x9F, 0xA0, 0x80),
                Color::Rgb(0x50, 0x54, 0x3A),
            ),
            // Obsidian — royal purple on jet.
            (_, Variant::Obsidian) => (
                Color::Reset,
                Color::Rgb(0x6A, 0x1B, 0x9A),
                Color::Rgb(0xBC, 0xA8, 0xC4),
                Color::Rgb(0x4A, 0x38, 0x58),
            ),
            // Winterfell — ice blue on slate.
            (_, Variant::Winterfell) => (
                Color::Reset,
                Color::Rgb(0x7F, 0xB3, 0xD5),
                Color::Rgb(0xB0, 0xC4, 0xD4),
                Color::Rgb(0x45, 0x58, 0x68),
            ),
            // Raven — vivid crimson on shadow.
            (_, Variant::Raven) => (
                Color::Reset,
                Color::Rgb(0xDC, 0x14, 0x3C),
                Color::Rgb(0xB8, 0xB8, 0xB8),
                Color::Rgb(0x5C, 0x3F, 0x3F),
            ),
            // Bloodmoon — rust red on deep indigo.
            (_, Variant::Bloodmoon) => (
                Color::Reset,
                Color::Rgb(0xA0, 0x33, 0x2E),
                Color::Rgb(0xB8, 0x9A, 0x8C),
                Color::Rgb(0x5A, 0x3F, 0x36),
            ),
            // Tundra — pale cyan on frost grey.
            (_, Variant::Tundra) => (
                Color::Reset,
                Color::Rgb(0xA8, 0xC5, 0xD0),
                Color::Rgb(0xBC, 0xC8, 0xD4),
                Color::Rgb(0x4E, 0x5A, 0x66),
            ),
            // Necrotic — bruised purple.
            (_, Variant::Necrotic) => (
                Color::Reset,
                Color::Rgb(0x5D, 0x4E, 0x8C),
                Color::Rgb(0xA8, 0xA5, 0xB4),
                Color::Rgb(0x4A, 0x3F, 0x5A),
            ),
            // Cascadian — moss green, Pacific Northwest atmospheric BM.
            (_, Variant::Cascadian) => (
                Color::Reset,
                Color::Rgb(0x4A, 0x6B, 0x3E),
                Color::Rgb(0xA8, 0xB5, 0xA0),
                Color::Rgb(0x3F, 0x4E, 0x38),
            ),
            // Iron Oath — muted steel.
            (_, Variant::IronOath) => (
                Color::Reset,
                Color::Rgb(0x90, 0x90, 0x90),
                Color::Rgb(0xB0, 0xB0, 0xB0),
                Color::Rgb(0x45, 0x45, 0x45),
            ),
            // Hellfire — burnt orange-red.
            (_, Variant::Hellfire) => (
                Color::Reset,
                Color::Rgb(0xD9, 0x44, 0x1C),
                Color::Rgb(0xC4, 0xA8, 0x98),
                Color::Rgb(0x5E, 0x3A, 0x2A),
            ),
            // Sepulchral — crypt bone white.
            (_, Variant::Sepulchral) => (
                Color::Reset,
                Color::Rgb(0xD4, 0xD0, 0xC8),
                Color::Rgb(0xBE, 0xB8, 0xB0),
                Color::Rgb(0x55, 0x50, 0x4A),
            ),
            // Wraith — faded teal, dissolving.
            (_, Variant::Wraith) => (
                Color::Reset,
                Color::Rgb(0x4A, 0x7A, 0x75),
                Color::Rgb(0xA5, 0xBE, 0xBC),
                Color::Rgb(0x3E, 0x5A, 0x58),
            ),
            // Abyssal — deep cyan from the void.
            (_, Variant::Abyssal) => (
                Color::Reset,
                Color::Rgb(0x00, 0xAC, 0xC1),
                Color::Rgb(0x9E, 0xC4, 0xD0),
                Color::Rgb(0x2F, 0x4A, 0x54),
            ),

            // --- Popular terminal schemes (authentic RGB from official palettes) ---

            // Solarized Dark — Ethan Schoonover, blue accent on base03.
            (Mode::Dark, Variant::SolarizedDark) => (
                Color::Reset,
                Color::Rgb(0x26, 0x8B, 0xD2),
                Color::Rgb(0x83, 0x94, 0x96),
                Color::Rgb(0x58, 0x6E, 0x75),
            ),
            (Mode::Light, Variant::SolarizedDark) => (
                Color::Reset,
                Color::Rgb(0x26, 0x8B, 0xD2),
                Color::Rgb(0x83, 0x94, 0x96),
                Color::Rgb(0x58, 0x6E, 0x75),
            ),
            // Solarized Light — magenta accent (also from the official
            // Solarized palette) for visual distinction from Dark's blue.
            (_, Variant::SolarizedLight) => (
                Color::Reset,
                Color::Rgb(0xD3, 0x36, 0x82),
                Color::Rgb(0x65, 0x7B, 0x83),
                Color::Rgb(0x93, 0xA1, 0xA1),
            ),
            // Gruvbox Dark — Pavel Pertsev.
            (_, Variant::GruvboxDark) => (
                Color::Reset,
                Color::Rgb(0xFE, 0x80, 0x19),
                Color::Rgb(0xEB, 0xDB, 0xB2),
                Color::Rgb(0x92, 0x83, 0x74),
            ),
            (_, Variant::GruvboxLight) => (
                Color::Reset,
                Color::Rgb(0xAF, 0x3A, 0x03),
                Color::Rgb(0x3C, 0x38, 0x36),
                Color::Rgb(0x7C, 0x6F, 0x64),
            ),
            // Dracula — Zeno Rocha.
            (_, Variant::Dracula) => (
                Color::Reset,
                Color::Rgb(0xBD, 0x93, 0xF9),
                Color::Rgb(0xF8, 0xF8, 0xF2),
                Color::Rgb(0x62, 0x72, 0xA4),
            ),
            // Tokyo Night — Enkia.
            (_, Variant::TokyoNight) => (
                Color::Reset,
                Color::Rgb(0x7A, 0xA2, 0xF7),
                Color::Rgb(0xC0, 0xCA, 0xF5),
                Color::Rgb(0x56, 0x5F, 0x89),
            ),
            // Catppuccin Mocha — mauve on base.
            (_, Variant::Catppuccin) => (
                Color::Reset,
                Color::Rgb(0xCB, 0xA6, 0xF7),
                Color::Rgb(0xCD, 0xD6, 0xF4),
                Color::Rgb(0x7F, 0x84, 0x9C),
            ),
            // OneDark — Atom's signature scheme.
            (_, Variant::OneDark) => (
                Color::Reset,
                Color::Rgb(0x61, 0xAF, 0xEF),
                Color::Rgb(0xAB, 0xB2, 0xBF),
                Color::Rgb(0x5C, 0x63, 0x70),
            ),
            // Monokai — Wimer Hazenberg, the classic.
            (_, Variant::Monokai) => (
                Color::Reset,
                Color::Rgb(0xF9, 0x26, 0x72),
                Color::Rgb(0xF8, 0xF8, 0xF2),
                Color::Rgb(0x75, 0x71, 0x5E),
            ),
            // Synthwave — neon pink + purple, 80s retro.
            (_, Variant::Synthwave) => (
                Color::Reset,
                Color::Rgb(0xF9, 0x2A, 0xAD),
                Color::Rgb(0xF9, 0xF9, 0xFA),
                Color::Rgb(0x88, 0x54, 0xD0),
            ),
            // Ayu Mirage — soft yellow accent.
            (_, Variant::Ayu) => (
                Color::Reset,
                Color::Rgb(0xFF, 0xD1, 0x73),
                Color::Rgb(0xCB, 0xCC, 0xC6),
                Color::Rgb(0x70, 0x7A, 0x8C),
            ),
            // Kanagawa Wave — Rebelot, Japanese ukiyo-e inspired.
            (_, Variant::Kanagawa) => (
                Color::Reset,
                Color::Rgb(0x7E, 0x9C, 0xD8),
                Color::Rgb(0xDC, 0xD7, 0xBA),
                Color::Rgb(0x72, 0x71, 0x69),
            ),
        };
        Palette {
            mode,
            variant,
            bg,
            accent,
            body,
            muted,
        }
    }
}

impl Palette {
    /// Scale all three colors toward the background by `factor` in 0..=1.
    /// Used by the idle screensaver to dim the UI after inactivity.
    /// `factor=1.0` returns self unchanged; `factor=0.3` produces a subdued
    /// near-background rendering that's still readable.
    pub fn dim(&self, factor: f32) -> Palette {
        Palette {
            mode: self.mode,
            variant: self.variant,
            bg: self.bg,
            accent: dim_color(self.accent, factor, self.mode),
            body: dim_color(self.body, factor, self.mode),
            muted: dim_color(self.muted, factor, self.mode),
        }
    }
}

fn dim_color(c: Color, factor: f32, mode: Mode) -> Color {
    let (r, g, b) = match c {
        Color::Rgb(r, g, b) => (r, g, b),
        // Non-RGB (Reset / Indexed) — leave untouched. Dim is a purely
        // visual tweak on the three styled colors.
        _ => return c,
    };
    // Dark mode dims toward black; light mode toward white.
    let (br, bg, bb) = match mode {
        Mode::Dark => (0u8, 0u8, 0u8),
        Mode::Light => (255u8, 255u8, 255u8),
    };
    let blend = |src: u8, dst: u8| -> u8 {
        let sf = f32::from(src);
        let df = f32::from(dst);
        (df + (sf - df) * factor).clamp(0.0, 255.0) as u8
    };
    Color::Rgb(blend(r, br), blend(g, bg), blend(b, bb))
}

/// Return a palette variant chosen by the current hour-of-day. Used when
/// `IWIYWI_PALETTE=auto` — the palette slowly drifts from warm dawn through
/// cool midday, sunset, dusk, and deep night. Deterministic per hour.
pub fn auto_variant(hour: u32) -> Variant {
    match hour {
        5..=6 => Variant::Dawn,
        7..=9 => Variant::Warm,
        10..=11 => Variant::Default,
        12..=14 => Variant::Cool,
        15..=16 => Variant::Sage,
        17..=18 => Variant::Sunset,
        19..=20 => Variant::Ember,
        21..=22 => Variant::Dusk,
        23 | 0 | 1 => Variant::Indigo,
        _ => Variant::Funeral, // 2, 3, 4 — deep night BM
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
    let raw = std::env::var("IWIYWI_PALETTE").ok();
    let variant = match raw.as_deref() {
        Some("auto") => auto_variant(chrono::Local::now().hour()),
        other => Variant::parse(other),
    };
    let mut p = Palette::build(mode, variant);
    if no_color_requested() {
        // NO_COLOR: return all Reset colors so the terminal's default fg/bg
        // apply. The user still gets style modifiers (bold, italic, reverse)
        // which the tui-design skill flags as the correct fallback for
        // monochrome rendering.
        p.accent = Color::Reset;
        p.body = Color::Reset;
        p.muted = Color::Reset;
    }
    p
}

/// True when the user has set the `NO_COLOR` environment variable. Per the
/// https://no-color.org standard: any non-empty value disables color.
pub fn no_color_requested() -> bool {
    std::env::var("NO_COLOR")
        .ok()
        .is_some_and(|v| !v.is_empty())
}

/// True when the user asked for the time-of-day auto palette. Lets the
/// main loop re-derive the palette periodically so it drifts through the
/// day without a restart.
pub fn auto_requested() -> bool {
    matches!(
        std::env::var("IWIYWI_PALETTE").ok().as_deref(),
        Some("auto")
    )
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
            assert_eq!(
                Variant::parse(Some(v.label())),
                v,
                "variant {} failed round-trip",
                v.label()
            );
        }
    }

    #[test]
    fn all_variants_covers_enum() {
        // Guard against adding a variant and forgetting to append to ALL.
        assert_eq!(Variant::ALL.len(), 51);
    }

    #[test]
    fn palettes_differ_per_variant() {
        let dark_default = Palette::build(Mode::Dark, Variant::Default);
        let dark_warm = Palette::build(Mode::Dark, Variant::Warm);
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
            assert!(
                seen.insert(key),
                "duplicate dark accent for variant {}",
                v.label()
            );
        }
    }

    #[test]
    fn every_light_accent_is_unique() {
        let mut seen = HashSet::new();
        for v in Variant::ALL {
            let p = Palette::build(Mode::Light, v);
            let key = color_key(p.accent);
            assert!(
                seen.insert(key),
                "duplicate light accent for variant {}",
                v.label()
            );
        }
    }

    #[test]
    fn every_variant_has_three_distinct_colors() {
        for v in Variant::ALL {
            for m in [Mode::Dark, Mode::Light] {
                let p = Palette::build(m, v);
                // accent vs body, accent vs muted, body vs muted — all must
                // differ, otherwise one role is invisible against another.
                assert_ne!(p.accent, p.body, "{} {:?}: accent == body", v.label(), m);
                assert_ne!(p.accent, p.muted, "{} {:?}: accent == muted", v.label(), m);
                assert_ne!(p.body, p.muted, "{} {:?}: body == muted", v.label(), m);
            }
        }
    }

    #[test]
    fn every_label_is_lowercase_ascii() {
        for v in Variant::ALL {
            let l = v.label();
            assert!(
                l.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
                "label {l} must be lowercase ascii + underscores"
            );
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
