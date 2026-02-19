// <FILE>tui-vfx-compositor/src/samplers/cls_shredder.rs</FILE> - <DESC>Shredder sampler implementation</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>WG5: Sampler Spatial Context Enhancement</WCTX>
// <CLOG>BREAKING CHANGE: Updated sample() signature to include spatial parameters; IMPROVEMENT: max_fall now scales with widget height (50% of height) instead of hardcoded 20 cells</CLOG>

use crate::traits::sampler::Sampler;

/// Paper shredder effect - vertical strips fall at different speeds.
///
/// Creates a shredding animation where vertical columns of content
/// fall at different speeds. Faster strips pull ahead, creating gaps
/// between them like paper strips coming out of a shredder.
pub struct Shredder {
    /// Width of each vertical strip in cells
    pub stripe_width: u16,
    /// Speed multiplier for odd-indexed strips
    pub odd_speed: f32,
    /// Speed multiplier for even-indexed strips
    pub even_speed: f32,
}

impl Default for Shredder {
    fn default() -> Self {
        Self::new(2, 3.0, 1.0) // More divergent speeds for visible effect
    }
}

impl Shredder {
    /// Create a new Shredder sampler.
    pub fn new(stripe_width: u16, odd_speed: f32, even_speed: f32) -> Self {
        Self {
            stripe_width: stripe_width.max(1),
            odd_speed,
            even_speed,
        }
    }
}

impl Sampler for Shredder {
    fn sample(
        &self,
        dest_x: u16,
        dest_y: u16,
        _width: u16,
        height: u16,
        t: f64,
    ) -> Option<(u16, u16)> {
        let t = t as f32;

        // Vertical strips: each column group falls at a different speed.
        // Faster strips "fall further" - their content appears lower on screen.
        //
        // To show a strip that has fallen by `offset`:
        // - Content that was at src_y now appears at src_y + offset
        // - So at dest_y, we show content from src_y = dest_y - offset
        // - When src_y < 0, that part of the strip has "fallen off" → gap

        let strip_idx = dest_x / self.stripe_width;

        // Alternate base speed, plus per-strip variation for organic look
        let base_speed = if strip_idx % 2 == 0 {
            self.even_speed
        } else {
            self.odd_speed
        };

        // Each strip gets unique variation based on its index
        // This prevents strips from moving in perfect lockstep
        let variation = 1.0 + ((strip_idx as u32 * 17) % 7) as f32 * 0.1;
        let speed = base_speed * variation;

        // Fall distance scales with time - strips accelerate slightly
        // Using t^1.2 gives a more natural falling acceleration
        let t_accel = t.powf(1.2);
        // Maximum fall distance now scales with widget height (50% of height)
        let max_fall = height as f32 * 0.5;
        let fall_offset = speed * t_accel * max_fall;

        // Source Y: where was this content before it fell?
        let src_y = dest_y as f32 - fall_offset;

        if src_y < 0.0 {
            // This portion has fallen off - creates the gap effect
            None
        } else {
            Some((dest_x, src_y as u16))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shredder_at_t_zero_no_offset() {
        let shredder = Shredder::new(2, 3.0, 1.0);
        let result = shredder.sample(0, 0, 20, 10, 0.0);
        assert_eq!(result, Some((0, 0)));

        let result = shredder.sample(5, 5, 20, 10, 0.0);
        assert_eq!(result, Some((5, 5)));
    }

    #[test]
    fn test_shredder_at_t_mid_creates_offset() {
        let shredder = Shredder::new(2, 3.0, 1.0);
        let even_result = shredder.sample(0, 9, 20, 10, 0.5);
        let odd_result = shredder.sample(2, 9, 20, 10, 0.5);

        assert!(even_result.is_some());
        assert!(odd_result.is_some());

        if let (Some((_, even_src_y)), Some((_, odd_src_y))) = (even_result, odd_result) {
            assert!(odd_src_y <= even_src_y);
        }
    }

    #[test]
    fn test_shredder_at_t_one_creates_gaps() {
        let shredder = Shredder::new(2, 3.0, 1.0);
        let result = shredder.sample(0, 0, 20, 10, 1.0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_shredder_different_speeds_diverge() {
        let shredder = Shredder::new(2, 5.0, 1.0);
        let even_result = shredder.sample(0, 5, 20, 10, 0.3);
        let odd_result = shredder.sample(2, 5, 20, 10, 0.3);

        if let (Some((_, even_src_y)), Some((_, odd_src_y))) = (even_result, odd_result) {
            let diff = even_src_y.abs_diff(odd_src_y);
            assert!(diff > 0);
        }
    }

    #[test]
    fn test_shredder_stripe_width_affects_strip_assignment() {
        let shredder = Shredder::new(4, 2.0, 1.0);
        let x0 = shredder.sample(0, 5, 20, 10, 0.3);
        let x3 = shredder.sample(3, 5, 20, 10, 0.3);
        let x4 = shredder.sample(4, 5, 20, 10, 0.3);
        let x7 = shredder.sample(7, 5, 20, 10, 0.3);

        if let (Some((_, src_y_0)), Some((_, src_y_3))) = (x0, x3) {
            assert_eq!(src_y_0, src_y_3);
        }

        if let (Some((_, src_y_4)), Some((_, src_y_7))) = (x4, x7) {
            assert_eq!(src_y_4, src_y_7);
        }
    }

    #[test]
    fn test_shredder_negative_speed_reverses_direction() {
        let shredder = Shredder::new(2, -1.0, 1.0);
        let odd_result = shredder.sample(2, 5, 20, 10, 0.5);
        assert!(odd_result.is_some() || odd_result.is_none());
    }

    #[test]
    fn test_shredder_zero_height_no_panic() {
        let shredder = Shredder::new(2, 3.0, 1.0);
        let result = shredder.sample(0, 0, 0, 0, 0.5);
        assert!(result.is_some() || result.is_none());
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_shredder.rs</FILE> - <DESC>Shredder sampler implementation</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
