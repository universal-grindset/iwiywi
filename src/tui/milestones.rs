//! Sobriety milestones: celebrate 1, 7, 30, 60, 90, 180, 365, and yearly
//! anniversaries with a special toast message. Checked once at startup.

const MILESTONES: &[(i64, &str)] = &[
    (1,    "Day 1. You showed up. That's everything."),
    (7,    "One week. Seven days of choosing differently."),
    (30,   "30 days. A month of surrender."),
    (60,   "60 days. The fog is lifting."),
    (90,   "90 days. You've built a foundation."),
    (180,  "Six months. Half a year of freedom."),
    (365,  "One year. A miracle happened here."),
    (730,  "Two years. Service flows through you now."),
    (1095, "Three years. You carry the message."),
    (1825, "Five years. Living proof that it works."),
    (3650, "Ten years. A life rebuilt, one day at a time."),
];

/// If today is a milestone day, return the celebration message.
/// Also fires on exact yearly anniversaries after 365 days.
pub fn check(sobriety_days: Option<i64>) -> Option<&'static str> {
    let days = sobriety_days.filter(|d| *d >= 0)?;
    // Check named milestones first.
    for (mark, msg) in MILESTONES {
        if days == *mark {
            return Some(msg);
        }
    }
    // Yearly anniversaries beyond the table.
    if days > 0 && days % 365 == 0 {
        return Some("Another year. It works if you work it.");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_one_fires() {
        assert!(check(Some(1)).is_some());
    }

    #[test]
    fn day_zero_does_not_fire() {
        assert!(check(Some(0)).is_none());
    }

    #[test]
    fn named_milestones_fire() {
        for (mark, _) in MILESTONES {
            assert!(check(Some(*mark)).is_some(), "day {mark} should fire");
        }
    }

    #[test]
    fn yearly_anniversary_beyond_table() {
        assert!(check(Some(365 * 20)).is_some());
    }

    #[test]
    fn non_milestone_day_returns_none() {
        assert!(check(Some(42)).is_none());
    }

    #[test]
    fn none_returns_none() {
        assert!(check(None).is_none());
    }

    #[test]
    fn negative_returns_none() {
        assert!(check(Some(-5)).is_none());
    }
}
