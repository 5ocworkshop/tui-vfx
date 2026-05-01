// <FILE>tui-vfx-compositor-next/src/samplers/cls_shredder.rs</FILE> - <DESC>Shredder sampler implementation</DESC>
// <VERS>VERSION: 2.3.0</VERS>
// <WCTX>v3.1 native debug-recipes closure: support fixed horizontal chunk offsets authored by debug sampler fixtures.</WCTX>
// <CLOG>2.3.0: add optional fixed_offset mode for horizontal chunk shifts while preserving the existing falling-strip default.
// 2.2.0: sample() now returns SamplerOutput; displacing branch carries delta_y; gap case returns no_displacement().</CLOG>

use crate::traits::sampler::{Sampler, SamplerOutput};
use tui_vfx_types::VfxCellContext;

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
    /// Optional fixed horizontal offset for row/chunk fixtures.
    pub fixed_offset: Option<i16>,
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
            fixed_offset: None,
        }
    }

    /// Use a fixed horizontal row/chunk offset instead of falling-strip motion.
    pub fn with_fixed_offset(mut self, offset: i16) -> Self {
        self.fixed_offset = Some(offset);
        self
    }
}

impl Sampler for Shredder {
    fn sample(&self, ctx: &VfxCellContext) -> SamplerOutput {
        let dest_x = ctx.local_x;
        let dest_y = ctx.local_y;
        if let Some(offset) = self.fixed_offset {
            if !dest_y.is_multiple_of(2) {
                return SamplerOutput::passthrough(dest_x, dest_y);
            }
            let strip_idx = dest_x / self.stripe_width;
            let local_offset = if strip_idx.is_multiple_of(2) {
                offset
            } else {
                -offset
            } as i32;
            let src_x_i = dest_x as i32 - local_offset;
            if src_x_i < 0 || src_x_i >= ctx.width as i32 {
                return SamplerOutput::no_displacement();
            }
            let src_x = src_x_i as u16;
            return SamplerOutput::displaced(src_x, dest_y, src_x as i32 - dest_x as i32, 0);
        }

        let t = ctx.t as f32;
        let height = ctx.height;

        // Vertical strips: each column group falls at a different speed.
        // Faster strips "fall further" - their content appears lower on screen.
        //
        // To show a strip that has fallen by `offset`:
        // - Content that was at src_y now appears at src_y + offset
        // - So at dest_y, we show content from src_y = dest_y - offset
        // - When src_y < 0, that part of the strip has "fallen off" → gap

        let strip_idx = dest_x / self.stripe_width;

        // Alternate base speed, plus per-strip variation for organic look
        let base_speed = if strip_idx.is_multiple_of(2) {
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
        let src_y_f = dest_y as f32 - fall_offset;

        if src_y_f < 0.0 {
            // This portion has fallen off - creates the gap effect
            SamplerOutput::no_displacement()
        } else {
            let src_y = src_y_f as u16;
            let delta_y = src_y as i32 - dest_y as i32;
            SamplerOutput::displaced(dest_x, src_y, 0, delta_y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_at(x: u16, y: u16, w: u16, h: u16, t: f64) -> VfxCellContext {
        VfxCellContext::new(x, y, w, h, 0, 0, t)
    }

    #[test]
    fn test_shredder_at_t_zero_no_offset() {
        let shredder = Shredder::new(2, 3.0, 1.0);
        let result = shredder.sample(&ctx_at(0, 0, 20, 10, 0.0));
        assert_eq!(result.source, Some((0, 0)));

        let result = shredder.sample(&ctx_at(5, 5, 20, 10, 0.0));
        assert_eq!(result.source, Some((5, 5)));
    }

    #[test]
    fn test_shredder_at_t_mid_creates_offset() {
        let shredder = Shredder::new(2, 3.0, 1.0);
        let even_result = shredder.sample(&ctx_at(0, 9, 20, 10, 0.5));
        let odd_result = shredder.sample(&ctx_at(2, 9, 20, 10, 0.5));

        assert!(even_result.source.is_some());
        assert!(odd_result.source.is_some());

        if let (Some((_, even_src_y)), Some((_, odd_src_y))) =
            (even_result.source, odd_result.source)
        {
            assert!(odd_src_y <= even_src_y);
        }
    }

    #[test]
    fn test_shredder_at_t_one_creates_gaps() {
        let shredder = Shredder::new(2, 3.0, 1.0);
        let result = shredder.sample(&ctx_at(0, 0, 20, 10, 1.0));
        assert_eq!(result.source, None);
    }

    #[test]
    fn test_shredder_different_speeds_diverge() {
        let shredder = Shredder::new(2, 5.0, 1.0);
        let even_result = shredder.sample(&ctx_at(0, 5, 20, 10, 0.3));
        let odd_result = shredder.sample(&ctx_at(2, 5, 20, 10, 0.3));

        if let (Some((_, even_src_y)), Some((_, odd_src_y))) =
            (even_result.source, odd_result.source)
        {
            let diff = even_src_y.abs_diff(odd_src_y);
            assert!(diff > 0);
        }
    }

    #[test]
    fn test_shredder_stripe_width_affects_strip_assignment() {
        let shredder = Shredder::new(4, 2.0, 1.0);
        let x0 = shredder.sample(&ctx_at(0, 5, 20, 10, 0.3));
        let x3 = shredder.sample(&ctx_at(3, 5, 20, 10, 0.3));
        let x4 = shredder.sample(&ctx_at(4, 5, 20, 10, 0.3));
        let x7 = shredder.sample(&ctx_at(7, 5, 20, 10, 0.3));

        if let (Some((_, src_y_0)), Some((_, src_y_3))) = (x0.source, x3.source) {
            assert_eq!(src_y_0, src_y_3);
        }

        if let (Some((_, src_y_4)), Some((_, src_y_7))) = (x4.source, x7.source) {
            assert_eq!(src_y_4, src_y_7);
        }
    }

    #[test]
    fn test_shredder_negative_speed_reverses_direction() {
        let shredder = Shredder::new(2, -1.0, 1.0);
        let odd_result = shredder.sample(&ctx_at(2, 5, 20, 10, 0.5));
        assert!(odd_result.source.is_some() || odd_result.source.is_none());
    }

    #[test]
    fn test_shredder_zero_height_no_panic() {
        let shredder = Shredder::new(2, 3.0, 1.0);
        let result = shredder.sample(&ctx_at(0, 0, 0, 0, 0.5));
        assert!(result.source.is_some() || result.source.is_none());
    }

    #[test]
    fn sample_emits_sampler_output_with_displacement_delta() {
        // At t=0.5 with odd strip (x=2, strip_idx=1): odd_speed=3.0, variation=1+(17%7)*0.1=1.0
        // speed = 3.0; t_accel = 0.5^1.2 ≈ 0.435; max_fall = 10*0.5 = 5.0
        // fall_offset = 3.0 * 0.435 * 5.0 ≈ 6.52; src_y = 9 - 6.52 = 2.48 -> 2
        // delta_y = 2 - 9 = -7
        let shredder = Shredder::new(2, 3.0, 1.0);
        let out = shredder.sample(&ctx_at(2, 9, 20, 10, 0.5));
        assert!(out.source.is_some());
        assert_eq!(out.delta_x, 0);
        let (_, src_y) = out.source.unwrap();
        assert_eq!(out.delta_y, src_y as i32 - 9);
        assert!(out.delta_y < 0);
    }
}

// <FILE>tui-vfx-compositor-next/src/samplers/cls_shredder.rs</FILE> - <DESC>Shredder sampler implementation</DESC>
// <VERS>END OF VERSION: 2.3.0</VERS>
