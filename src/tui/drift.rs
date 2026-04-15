//! Drift: flow-field particle animation for the `drift` pattern. Animates
//! a cloud of dot particles driven by 2D Perlin noise, each leaving a short
//! fading trail. Purely decorative — no timing, no text, no interaction.

use noise::{NoiseFn, Perlin};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
};
use std::time::Instant;

use crate::tui::palette::Palette;

const PARTICLES_MIN: usize = 40;
const PARTICLES_MAX: usize = 240;
const AREA_PER_PARTICLE: usize = 14;
const FIELD_SCALE: f64 = 0.05;
const TIME_SCALE: f64 = 0.35;
const MAX_STEP: f32 = 1.1;
// Trail glyphs — index 0 is the newest (brightest), index 3 is the oldest.
const TRAIL_CHARS: [&str; 4] = ["●", "•", "·", "⋅"];

/// Which physics drive the particle field. `Drift` is the original Perlin
/// flow-field; the others are simple directional modes (sinusoidal wave,
/// downward snow, fast vertical rain).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Drift,
    Wave,
    Snow,
    Rain,
}

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub trail: [Option<(u16, u16)>; 4],
}

pub struct DriftState {
    pub particles: Vec<Particle>,
    noise: Perlin,
    pub start: Instant,
    pub mode: Mode,
}

fn particle_count(w: u16, h: u16) -> usize {
    let area = (w as usize) * (h as usize);
    (area / AREA_PER_PARTICLE).clamp(PARTICLES_MIN, PARTICLES_MAX)
}

fn pseudo_rand(seed: u32, n: usize) -> f32 {
    let mut x = seed.wrapping_mul(2_654_435_761).wrapping_add(n as u32);
    x ^= x >> 13;
    x = x.wrapping_mul(0x5bd1e995);
    x ^= x >> 15;
    ((x >> 8) as f32) / ((1u32 << 24) as f32)
}

fn wrap(v: f32, max: f32) -> f32 {
    if max <= 0.0 { return 0.0; }
    let mut r = v % max;
    if r < 0.0 { r += max; }
    if r >= max { r -= max; }
    r
}

impl DriftState {
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn new(width: u16, height: u16, seed: u32) -> Self {
        Self::with_mode(width, height, seed, Mode::Drift)
    }

    pub fn with_mode(width: u16, height: u16, seed: u32, mode: Mode) -> Self {
        let count = particle_count(width, height);
        let particles = (0..count)
            .map(|i| Particle {
                x: pseudo_rand(seed, i * 2) * (width as f32),
                y: pseudo_rand(seed, i * 2 + 1) * (height as f32),
                trail: [None; 4],
            })
            .collect();
        DriftState {
            particles,
            noise: Perlin::new(seed),
            start: Instant::now(),
            mode,
        }
    }

    pub fn tick(&mut self, width: u16, height: u16) {
        if width == 0 || height == 0 { return; }
        let t = self.start.elapsed().as_secs_f64();
        for (idx, p) in self.particles.iter_mut().enumerate() {
            p.trail[3] = p.trail[2];
            p.trail[2] = p.trail[1];
            p.trail[1] = p.trail[0];
            p.trail[0] = Some((p.x as u16, p.y as u16));

            let (vx, vy) = match self.mode {
                Mode::Drift => {
                    let fx = p.x as f64 * FIELD_SCALE;
                    let fy = p.y as f64 * FIELD_SCALE;
                    let vx = self.noise.get([fx, fy, t * TIME_SCALE]) as f32;
                    let vy = self.noise.get([fx, fy, t * TIME_SCALE + 100.0]) as f32;
                    (vx.clamp(-MAX_STEP, MAX_STEP), vy.clamp(-MAX_STEP, MAX_STEP))
                }
                Mode::Wave => {
                    // Particles slide rightward; y oscillates with a sinusoid
                    // whose phase depends on x. Produces a rolling wave.
                    let vx: f32 = 0.6;
                    let phase = (p.x as f64 * 0.12 + t * 1.8).sin() as f32;
                    let vy = phase * 0.9;
                    (vx, vy)
                }
                Mode::Snow => {
                    // Slow downward fall + a gentle horizontal sway.
                    let sway = ((t * 0.6 + idx as f64 * 0.7).sin() * 0.25) as f32;
                    (sway, 0.45)
                }
                Mode::Rain => {
                    // Fast, essentially vertical.
                    (0.0, 1.6)
                }
            };

            p.x = wrap(p.x + vx, width as f32);
            p.y = wrap(p.y + vy, height as f32);
        }
    }

}

/// Draw the particles + trails into the buffer. Skips rendering when the
/// area is too small to make the effect meaningful. Trails fade over four
/// segments: newest two are drawn in the palette accent, oldest two in the
/// muted color, giving each particle visible momentum.
pub fn draw(buf: &mut Buffer, area: Rect, state: &DriftState, palette: &Palette) {
    if area.width < 40 || area.height < 15 { return; }
    let bright_style = Style::default().fg(palette.accent);
    let dim_style = Style::default().fg(palette.muted);
    for p in &state.particles {
        for (i, pos) in p.trail.iter().enumerate().rev() {
            if let Some((x, y)) = pos {
                if *x < area.width && *y < area.height {
                    let style = if i < 2 { bright_style } else { dim_style };
                    buf[(area.x + *x, area.y + *y)]
                        .set_symbol(TRAIL_CHARS[i])
                        .set_style(style);
                }
            }
        }
        let hx = p.x as u16;
        let hy = p.y as u16;
        if hx < area.width && hy < area.height {
            buf[(area.x + hx, area.y + hy)]
                .set_symbol("●")
                .set_style(bright_style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn particles_stay_in_bounds_after_ticks() {
        let mut s = DriftState::new(80, 24, 1);
        for _ in 0..200 { s.tick(80, 24); }
        for p in &s.particles {
            assert!(p.x >= 0.0 && p.x < 80.0);
            assert!(p.y >= 0.0 && p.y < 24.0);
        }
    }

    #[test]
    fn tick_handles_zero_dims_without_panic() {
        let mut s = DriftState::new(80, 24, 1);
        s.tick(0, 0);
        s.tick(80, 0);
        s.tick(0, 24);
    }

    #[test]
    fn pseudo_rand_stays_below_one() {
        for n in 0..10_000 {
            let r = pseudo_rand(1, n);
            assert!((0.0..1.0).contains(&r));
        }
    }
}
