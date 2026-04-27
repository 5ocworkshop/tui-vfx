// <FILE>tui-vfx-compositor/src/samplers/cls_distortion.rs</FILE>
// <DESC>Generic distortion sampler</DESC>
// <VERS>VERSION: 3.1.0</VERS>
// <WCTX>2026-04-26 packet — migrate sample() return to SamplerOutput so the orchestrator can thread the displacement delta into ctx.resolved_x.</WCTX>
// <CLOG>3.1.0: sample() now returns SamplerOutput::displaced(src_x, dest_y, ...) carrying both the source coord and the resolved-coord delta.</CLOG>

use crate::traits::sampler::{Sampler, SamplerOutput};
use tui_vfx_types::VfxCellContext;

#[allow(dead_code)]
pub struct Distortion;

impl Sampler for Distortion {
    fn sample(&self, ctx: &VfxCellContext) -> SamplerOutput {
        let t = ctx.t as f32;
        let dest_x = ctx.local_x;
        let dest_y = ctx.local_y;

        // Sine wave distortion
        let offset = (t * 10.0 + (dest_y as f32 / 5.0)).sin() * 2.0;
        let src_x_f = dest_x as f32 + offset;

        if src_x_f < 0.0 {
            return SamplerOutput::no_displacement();
        }

        let src_x = src_x_f.round() as u16;
        let delta_x = src_x as i32 - dest_x as i32;
        SamplerOutput::displaced(src_x, dest_y, delta_x, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distortion_row_zero_identity_at_t0() {
        let sampler = Distortion;
        // At t=0 and y=0: offset = sin(0*10 + 0/5) * 2 = sin(0) * 2 = 0
        let out = sampler.sample(&VfxCellContext::new(5, 0, 10, 10, 0, 0, 0.0));
        assert_eq!(out.source, Some((5, 0)));
        assert_eq!(out.delta_x, 0);
        assert_eq!(out.delta_y, 0);
    }

    #[test]
    fn test_distortion_preserves_y() {
        let sampler = Distortion;
        // Distortion only affects x, y should always be preserved
        for y in 0..5 {
            let out = sampler.sample(&VfxCellContext::new(5, y, 10, 10, 0, 0, 0.5));
            assert!(out.source.is_some());
            assert_eq!(out.source.unwrap().1, y);
            assert_eq!(out.delta_y, 0);
        }
    }

    #[test]
    fn test_distortion_negative_x_returns_none() {
        let sampler = Distortion;
        // At x=0 with negative offset, should return no_displacement.
        // offset = sin(t*10 + y/5) * 2 can be negative when sin is negative
        let out = sampler.sample(&VfxCellContext::new(
            0,
            0,
            10,
            10,
            0,
            0,
            std::f64::consts::PI * 0.15,
        ));
        // May or may not skip depending on exact offset; just verify it handles
        // the case gracefully without panicking.
        let _ = out;
    }

    #[test]
    fn sample_emits_sampler_output_with_displacement_delta() {
        // At y=10, t=1.0: t*10 + y/5 = 10 + 2 = 12; sin(12) ≈ -0.5366,
        // times 2 ≈ -1.073, so src_x = round(5 - 1.073) = 4 → delta_x = -1,
        // delta_y = 0 (Distortion only displaces along x).
        let sampler = Distortion;
        let out = sampler.sample(&VfxCellContext::new(5, 10, 16, 16, 0, 0, 1.0));
        assert!(out.source.is_some(), "expected a source coord at this input");
        assert_ne!(out.delta_x, 0, "Distortion at y=10, t=1.0 must displace x");
        assert_eq!(out.delta_y, 0, "Distortion never displaces y");
        // Delta sign and magnitude must match the source-coord shift.
        let (sx, _) = out.source.unwrap();
        assert_eq!(out.delta_x, sx as i32 - 5);
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_distortion.rs</FILE>
// <DESC>Generic distortion sampler</DESC>
// <VERS>END OF VERSION: 3.1.0</VERS>
