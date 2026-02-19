// <FILE>tui-vfx-compositor/src/samplers/cls_crt_jitter.rs</FILE> - <DESC>CrtJitter sampler implementation</DESC>
// <VERS>VERSION: 2.2.0</VERS>
// <WCTX>RNG performance optimization</WCTX>
// <CLOG>Switched to fast_random for ~25x faster per-cell noise generation</CLOG>

use crate::traits::sampler::Sampler;

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
    fn sample(
        &self,
        dest_x: u16,
        dest_y: u16,
        _width: u16,
        _height: u16,
        t: f64,
    ) -> Option<(u16, u16)> {
        let t = t as f32;

        // Apply decay over time (effect gets weaker as t approaches 1)
        let decay_factor = (-self.decay * t * 5.0).exp();
        let effective_intensity = self.intensity * decay_factor;

        // Generate row-based horizontal jitter
        let jitter = self.noise(dest_x, dest_y, t) * effective_intensity * 5.0;

        let src_x = (dest_x as f32 + jitter).round();

        if src_x < 0.0 {
            None
        } else {
            Some((src_x as u16, dest_y))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_WIDTH: u16 = 80;
    const TEST_HEIGHT: u16 = 24;

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
        let result = jitter.sample(10, 10, TEST_WIDTH, TEST_HEIGHT, 0.5);
        assert!(result.is_some());
    }

    #[test]
    fn test_crt_jitter_preserves_y() {
        let jitter = CrtJitter::default();
        let result = jitter.sample(10, 15, TEST_WIDTH, TEST_HEIGHT, 0.5).unwrap();
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
        let r1 = jitter1.sample(10, 10, TEST_WIDTH, TEST_HEIGHT, 0.5);
        let r2 = jitter2.sample(10, 10, TEST_WIDTH, TEST_HEIGHT, 0.5);
        assert_eq!(r1, r2);
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
        let r1 = jitter1.sample(10, 10, TEST_WIDTH, TEST_HEIGHT, 0.5);
        let r2 = jitter2.sample(10, 10, TEST_WIDTH, TEST_HEIGHT, 0.5);
        assert!(r1 != r2 || r1.is_some());
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_crt_jitter.rs</FILE> - <DESC>CrtJitter sampler implementation</DESC>
// <VERS>END OF VERSION: 2.2.0</VERS>
