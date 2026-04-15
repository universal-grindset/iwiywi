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

use noise::{NoiseFn, Perlin};
use std::time::Instant;

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub trail: [Option<(u16, u16)>; 4],
}

pub struct DriftState {
    pub particles: Vec<Particle>,
    noise: Perlin,
    pub start: Instant,
    pub reading_idx: usize,
    pub reading_phase_start: Instant,
}

const PARTICLES_MIN: usize = 20;
const PARTICLES_MAX: usize = 120;
const DIVISOR: usize = 30;
const FIELD_SCALE: f64 = 0.06;
const TIME_SCALE: f64 = 0.25;
const MAX_STEP: f32 = 0.8;

fn particle_count(w: u16, h: u16) -> usize {
    let area = (w as usize) * (h as usize);
    (area / DIVISOR).clamp(PARTICLES_MIN, PARTICLES_MAX)
}

fn pseudo_rand(seed: u32, n: usize) -> f32 {
    let mut x = seed.wrapping_mul(2_654_435_761).wrapping_add(n as u32);
    x ^= x >> 13;
    x = x.wrapping_mul(0x5bd1e995);
    x ^= x >> 15;
    // Use the top 24 bits so the division is exact in f32, guaranteeing [0, 1).
    ((x >> 8) as f32) / ((1u32 << 24) as f32)
}

impl DriftState {
    pub fn new(width: u16, height: u16, seed: u32) -> Self {
        let count = particle_count(width, height);
        let particles = (0..count)
            .map(|i| Particle {
                x: pseudo_rand(seed, i * 2) * (width as f32),
                y: pseudo_rand(seed, i * 2 + 1) * (height as f32),
                trail: [None; 4],
            })
            .collect();
        let now = Instant::now();
        DriftState {
            particles,
            noise: Perlin::new(seed),
            start: now,
            reading_idx: 0,
            reading_phase_start: now,
        }
    }

    pub fn tick(&mut self, width: u16, height: u16, _dt: std::time::Duration) {
        if width == 0 || height == 0 {
            return;
        }
        let t = self.start.elapsed().as_secs_f64();
        for p in &mut self.particles {
            p.trail[3] = p.trail[2];
            p.trail[2] = p.trail[1];
            p.trail[1] = p.trail[0];
            p.trail[0] = Some((p.x as u16, p.y as u16));

            let fx = p.x as f64 * FIELD_SCALE;
            let fy = p.y as f64 * FIELD_SCALE;
            let vx = self.noise.get([fx, fy, t * TIME_SCALE]) as f32;
            let vy = self.noise.get([fx, fy, t * TIME_SCALE + 100.0]) as f32;
            let vx = vx.clamp(-MAX_STEP, MAX_STEP);
            let vy = vy.clamp(-MAX_STEP, MAX_STEP);

            p.x = wrap(p.x + vx, width as f32);
            p.y = wrap(p.y + vy, height as f32);
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        let want = particle_count(width, height);
        let seed = self.start.elapsed().as_nanos() as u32;
        self.particles = (0..want)
            .map(|i| Particle {
                x: pseudo_rand(seed, i * 2) * (width as f32),
                y: pseudo_rand(seed, i * 2 + 1) * (height as f32),
                trail: [None; 4],
            })
            .collect();
    }
}

fn wrap(v: f32, max: f32) -> f32 {
    if max <= 0.0 {
        return 0.0;
    }
    let mut r = v % max;
    if r < 0.0 {
        r += max;
    }
    if r >= max {
        r -= max;
    }
    r
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

    #[test]
    fn new_scales_particle_count_with_area() {
        let small = DriftState::new(40, 20, 1);
        let large = DriftState::new(160, 50, 1);
        assert!(large.particles.len() > small.particles.len());
        assert!(small.particles.len() >= 10);
        assert!(large.particles.len() <= 120);
    }

    #[test]
    fn particles_stay_in_bounds_after_many_ticks() {
        let mut s = DriftState::new(80, 24, 1);
        for _ in 0..200 {
            s.tick(80, 24, Duration::from_millis(50));
        }
        for p in &s.particles {
            assert!(p.x >= 0.0 && p.x < 80.0, "x out of bounds: {}", p.x);
            assert!(p.y >= 0.0 && p.y < 24.0, "y out of bounds: {}", p.y);
            for t in p.trail.iter().flatten() {
                assert!(t.0 < 80, "trail x out of bounds: {}", t.0);
                assert!(t.1 < 24, "trail y out of bounds: {}", t.1);
            }
        }
    }

    #[test]
    fn trail_length_is_four_after_four_ticks() {
        let mut s = DriftState::new(80, 24, 1);
        for _ in 0..4 {
            s.tick(80, 24, Duration::from_millis(50));
        }
        for p in &s.particles {
            assert!(p.trail.iter().filter(|t| t.is_some()).count() == 4);
        }
    }

    #[test]
    fn wrap_handles_max_rounding_edge_case() {
        // Negative near-zero values should not land exactly at max due to f32 rounding.
        assert!(wrap(-1e-10_f32, 80.0) < 80.0);
        assert!(wrap(-1e-10_f32, 80.0) >= 0.0);
        // Already-in-bounds values are unchanged.
        assert_eq!(wrap(40.0, 80.0), 40.0);
        // Zero max returns zero.
        assert_eq!(wrap(50.0, 0.0), 0.0);
    }

    #[test]
    fn pseudo_rand_stays_below_one() {
        for n in 0..10_000 {
            let r = pseudo_rand(1, n);
            assert!(r >= 0.0 && r < 1.0, "pseudo_rand out of [0,1): {r} at n={n}");
        }
    }

    #[test]
    fn resize_rescatters_particles_into_new_bounds() {
        let mut s = DriftState::new(120, 40, 1);
        s.resize(40, 20);
        for p in &s.particles {
            assert!(p.x >= 0.0 && p.x < 40.0);
            assert!(p.y >= 0.0 && p.y < 20.0);
        }
    }

    #[test]
    fn resize_adjusts_particle_count() {
        let mut s = DriftState::new(40, 20, 1);
        let before = s.particles.len();
        s.resize(160, 50);
        assert!(s.particles.len() > before);
    }
}
