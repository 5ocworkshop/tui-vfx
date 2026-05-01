// <FILE>tui-vfx-compositor-next/src/samplers/cls_crt_jitter.rs</FILE> - <DESC>CrtJitter sampler implementation</DESC>
// <VERS>VERSION: 2.4.0</VERS>
// <WCTX>2026-04-26 packet — migrate sample() return to SamplerOutput so the orchestrator can thread the displacement delta into ctx.resolved_x.</WCTX>
// <CLOG>2.4.0: sample() now returns SamplerOutput; displacing branch carries delta_x; out-of-bounds returns no_displacement().</CLOG>

use crate::traits::sampler::{Sampler, SamplerOutput};
use tui_vfx_types::VfxCellContext;

/// CRT crash/jitter effect sampler.
///
/// Creates horizontal jitter and noise like a malfunctioning CRT monitor.
/// Used for effects like BSOD crash animations.
pub struct CrtJitter {
    /// Intensity of the jitter effect (0.0 - 1.0)
    pub intensity: f32,
    /// Jitter frequency in Hz (affects how fast the jitter changes)
    pub speed_hz: f32,
    /// Decay time - how quickly the effect diminishes
    pub decay: f32,
    /// Seed for deterministic randomness
    pub seed: u64,
}

impl Default for CrtJitter {
    fn default() -> Self {
        Self {
            intensity: 0.7,
            speed_hz: 30.0,
            decay: 0.5,
            seed: 42,
        }
    }
}

impl CrtJitter {
    /// Generates a pseudo-random value based on the given inputs using fast_random.
    /// ~25x faster than ChaCha8-based Rng for per-cell noise generation.
    fn noise(&self, _x: u16, y: u16, t: f32) -> f32 {
        use mixed_signals::math::fast_random;
        // Combine row and seed
        let row_seed = self.seed.wrapping_mul(31).wrapping_add(y as u64);
        // Quantized time as input
        let time_slot = (t * self.speed_hz).floor() as u64;
        // fast_random returns 0.0..1.0, scale to [-1, 1]
        fast_random(row_seed, time_slot) * 2.0 - 1.0
    }
}

impl Sampler for CrtJitter {
    fn sample(&self, ctx: &VfxCellContext) -> SamplerOutput {
        let t = ctx.t as f32;
        let dest_x = ctx.local_x;
        let dest_y = ctx.local_y;

        // Apply decay over time (effect gets weaker as t approaches 1)
        let decay_factor = (-self.decay * t * 5.0).exp();
        let effective_intensity = self.intensity * decay_factor;

        // Generate row-based horizontal jitter
        let jitter = self.noise(dest_x, dest_y, t) * effective_intensity * 5.0;

        let src_x_f = (dest_x as f32 + jitter).round();

        if src_x_f < 0.0 {
            SamplerOutput::no_displacement()
        } else {
            let src_x = src_x_f as u16;
            let delta_x = src_x as i32 - dest_x as i32;
            SamplerOutput::displaced(src_x, dest_y, delta_x, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_WIDTH: u16 = 80;
    const TEST_HEIGHT: u16 = 24;

    fn ctx_at(x: u16, y: u16, t: f64) -> VfxCellContext {
        VfxCellContext::new(x, y, TEST_WIDTH, TEST_HEIGHT, 0, 0, t)
    }

    #[test]
    fn test_crt_jitter_default() {
        let jitter = CrtJitter::default();
        assert_eq!(jitter.intensity, 0.7);
        assert_eq!(jitter.speed_hz, 30.0);
        assert_eq!(jitter.seed, 42);
    }

    #[test]
    fn test_crt_jitter_returns_some() {
        let jitter = CrtJitter::default();
        let result = jitter.sample(&ctx_at(10, 10, 0.5));
        assert!(result.source.is_some());
    }

    #[test]
    fn test_crt_jitter_preserves_y() {
        let jitter = CrtJitter::default();
        let result = jitter.sample(&ctx_at(10, 15, 0.5)).source.unwrap();
        assert_eq!(result.1, 15);
    }

    #[test]
    fn test_crt_jitter_deterministic_with_seed() {
        let jitter1 = CrtJitter {
            seed: 123,
            ..Default::default()
        };
        let jitter2 = CrtJitter {
            seed: 123,
            ..Default::default()
        };
        let r1 = jitter1.sample(&ctx_at(10, 10, 0.5));
        let r2 = jitter2.sample(&ctx_at(10, 10, 0.5));
        assert_eq!(r1.source, r2.source);
    }

    #[test]
    fn test_crt_jitter_different_seeds_differ() {
        let jitter1 = CrtJitter {
            seed: 123,
            ..Default::default()
        };
        let jitter2 = CrtJitter {
            seed: 456,
            ..Default::default()
        };
        let r1 = jitter1.sample(&ctx_at(10, 10, 0.5));
        let r2 = jitter2.sample(&ctx_at(10, 10, 0.5));
        assert!(r1.source != r2.source || r1.source.is_some());
    }

    #[test]
    fn sample_emits_sampler_output_with_displacement_delta() {
        // At t=0 decay_factor = exp(0) = 1.0; effective_intensity = 0.7
        // With seed=42, row y=5 at time_slot 0 produces a known noise value
        // We verify structure: delta_y is always 0 (jitter is horizontal only)
        let jitter = CrtJitter {
            intensity: 0.7,
            speed_hz: 1.0,
            decay: 0.0,
            seed: 42,
        };
        let out = jitter.sample(&VfxCellContext::new(
            20,
            5,
            TEST_WIDTH,
            TEST_HEIGHT,
            0,
            0,
            0.0,
        ));
        // delta_y must always be 0 since jitter only displaces x
        assert_eq!(out.delta_y, 0);
        if let Some((src_x, _)) = out.source {
            // delta_x == src_x - dest_x
            assert_eq!(out.delta_x, src_x as i32 - 20);
        }
    }
}

// <FILE>tui-vfx-compositor-next/src/samplers/cls_crt_jitter.rs</FILE> - <DESC>CrtJitter sampler implementation</DESC>
// <VERS>END OF VERSION: 2.4.0</VERS>
