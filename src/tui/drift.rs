use std::time::Duration;

pub const FADE_IN: Duration = Duration::from_millis(500);
pub const LINGER: Duration = Duration::from_millis(7_000);
pub const FADE_OUT: Duration = Duration::from_millis(500);
pub const READING_CYCLE: Duration =
    Duration::from_millis(500 + 7_000 + 500);

/// Compute the alpha (0.0 = invisible, 1.0 = full) for the currently-showing
/// reading given how long it has been visible.
pub fn reading_alpha(elapsed: Duration) -> f32 {
    if elapsed < FADE_IN {
        elapsed.as_secs_f32() / FADE_IN.as_secs_f32()
    } else if elapsed < FADE_IN + LINGER {
        1.0
    } else if elapsed < READING_CYCLE {
        let into_fade = elapsed - (FADE_IN + LINGER);
        1.0 - into_fade.as_secs_f32() / FADE_OUT.as_secs_f32()
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_zero_at_start() {
        assert!(reading_alpha(Duration::ZERO) < 0.01);
    }

    #[test]
    fn alpha_midway_through_fade_in_is_half() {
        let a = reading_alpha(FADE_IN / 2);
        assert!((a - 0.5).abs() < 0.05, "expected ~0.5, got {a}");
    }

    #[test]
    fn alpha_one_during_linger() {
        assert_eq!(reading_alpha(FADE_IN + LINGER / 2), 1.0);
    }

    #[test]
    fn alpha_midway_through_fade_out_is_half() {
        let a = reading_alpha(FADE_IN + LINGER + FADE_OUT / 2);
        assert!((a - 0.5).abs() < 0.05, "expected ~0.5, got {a}");
    }

    #[test]
    fn alpha_zero_after_full_cycle() {
        assert!(reading_alpha(READING_CYCLE + Duration::from_millis(1)) < 0.01);
    }
}
