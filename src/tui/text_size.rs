//! Text size: a reading-comfort knob that controls the body column width
//! and the emphasis weight. We can't change the terminal's font size, but
//! we *can* change how much room the body gets and whether it renders bold.

use ratatui::style::Modifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextSize {
    Small,
    Normal,
    Large,
}

impl TextSize {
    pub const ALL: [TextSize; 3] = [TextSize::Small, TextSize::Normal, TextSize::Large];

    pub fn parse(raw: Option<&str>) -> TextSize {
        match raw {
            Some("small") => TextSize::Small,
            Some("large") => TextSize::Large,
            _ => TextSize::Normal,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            TextSize::Small  => "small",
            TextSize::Normal => "normal",
            TextSize::Large  => "large",
        }
    }

    /// Width ratio applied to the viewport when computing the text column.
    pub fn width_ratio(&self) -> f32 {
        match self {
            TextSize::Small  => 0.55,
            TextSize::Normal => 0.70,
            TextSize::Large  => 0.85,
        }
    }

    /// `(min, max)` clamp for the text column width in cells.
    pub fn width_clamp(&self) -> (f32, f32) {
        match self {
            TextSize::Small  => (20.0, 52.0),
            TextSize::Normal => (20.0, 72.0),
            TextSize::Large  => (30.0, 96.0),
        }
    }

    /// Extra `Modifier` bits applied to the body text. `Large` renders
    /// bold so it reads heavier even at the same cell size.
    pub fn body_modifier(&self) -> Modifier {
        match self {
            TextSize::Small  => Modifier::empty(),
            TextSize::Normal => Modifier::empty(),
            TextSize::Large  => Modifier::BOLD,
        }
    }
}

pub fn from_env() -> TextSize {
    TextSize::parse(std::env::var("IWIYWI_TEXT_SIZE").ok().as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_defaults_to_normal() {
        assert_eq!(TextSize::parse(None), TextSize::Normal);
        assert_eq!(TextSize::parse(Some("garbage")), TextSize::Normal);
    }

    #[test]
    fn parse_round_trips_via_label() {
        for t in TextSize::ALL {
            assert_eq!(TextSize::parse(Some(t.label())), t);
        }
    }

    #[test]
    fn width_ratios_are_ordered() {
        assert!(TextSize::Small.width_ratio() < TextSize::Normal.width_ratio());
        assert!(TextSize::Normal.width_ratio() < TextSize::Large.width_ratio());
    }

    #[test]
    fn width_clamps_are_sane() {
        for t in TextSize::ALL {
            let (lo, hi) = t.width_clamp();
            assert!(lo < hi);
            assert!(lo >= 20.0);
            assert!(hi <= 100.0);
        }
    }

    #[test]
    fn only_large_is_bold() {
        assert!(TextSize::Small.body_modifier().is_empty());
        assert!(TextSize::Normal.body_modifier().is_empty());
        assert!(TextSize::Large.body_modifier().contains(Modifier::BOLD));
    }

    #[test]
    fn all_covers_enum() {
        assert_eq!(TextSize::ALL.len(), 3);
    }
}
