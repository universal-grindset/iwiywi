use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Theme {
    pub mode: Mode,
    pub accent: Color,
    pub heading: Color,
    pub ok: Color,
    pub err: Color,
    pub muted: Color,
    pub border: Color,
    pub body: Color,
}

impl Theme {
    pub fn light() -> Self {
        Theme {
            mode: Mode::Light,
            accent:  Color::Rgb(0x09, 0x69, 0xDA),
            heading: Color::Rgb(0x9A, 0x67, 0x00),
            ok:      Color::Rgb(0x1A, 0x7F, 0x37),
            err:     Color::Rgb(0xCF, 0x22, 0x2E),
            muted:   Color::Rgb(0x57, 0x60, 0x6A),
            border:  Color::Rgb(0xD0, 0xD7, 0xDE),
            body:    Color::Rgb(0x24, 0x29, 0x2F),
        }
    }

    pub fn dark() -> Self {
        Theme {
            mode: Mode::Dark,
            accent:  Color::Rgb(0x00, 0xD7, 0xFF),
            heading: Color::Rgb(0xFF, 0xD7, 0x00),
            ok:      Color::Rgb(0x00, 0xFF, 0x87),
            err:     Color::Rgb(0xFF, 0x5F, 0x87),
            muted:   Color::Rgb(0xA8, 0xA8, 0xA8),
            border:  Color::Rgb(0x3A, 0x3A, 0x3A),
            body:    Color::Rgb(0xC9, 0xD1, 0xD9),
        }
    }
}

pub fn detect() -> Theme {
    match std::env::var("IWIYWI_THEME").ok().as_deref() {
        Some("light") => Theme::light(),
        Some("dark")  => Theme::dark(),
        _ => auto(),
    }
}

fn auto() -> Theme {
    if let Ok(fgbg) = std::env::var("COLORFGBG") {
        if let Some(bg) = fgbg.split(';').nth(1).and_then(|s| s.parse::<u8>().ok()) {
            // ANSI 0-6 and 8 are dark backgrounds; 7 and 9-15 are light.
            let is_light = matches!(bg, 7 | 9..=15);
            return if is_light { Theme::light() } else { Theme::dark() };
        }
    }
    Theme::dark()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_override_light() {
        std::env::set_var("IWIYWI_THEME", "light");
        assert_eq!(detect().mode, Mode::Light);
        std::env::remove_var("IWIYWI_THEME");
    }

    #[test]
    fn env_override_dark() {
        std::env::set_var("IWIYWI_THEME", "dark");
        assert_eq!(detect().mode, Mode::Dark);
        std::env::remove_var("IWIYWI_THEME");
    }

    #[test]
    fn palettes_differ() {
        assert_ne!(Theme::light().body, Theme::dark().body);
        assert_ne!(Theme::light().border, Theme::dark().border);
    }
}
