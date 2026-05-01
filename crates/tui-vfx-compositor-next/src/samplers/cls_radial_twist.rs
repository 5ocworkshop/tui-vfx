// <FILE>tui-vfx-compositor-next/src/samplers/cls_radial_twist.rs</FILE> - <DESC>RadialTwist sampler for center-weighted coordinate warps</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>2026-04-26 packet — migrate sample() return to SamplerOutput so the orchestrator can thread the displacement delta into ctx.resolved_x.</WCTX>
// <CLOG>0.3.0: sample() now returns SamplerOutput; displacing branch carries full x/y deltas; out-of-bounds returns no_displacement().</CLOG>

use crate::traits::sampler::{Sampler, SamplerOutput};
use crate::types::cls_sampler_spec::RippleCenter;
use mixed_signals::math::radial_twist_warp;
use tui_vfx_types::VfxCellContext;

/// Center-weighted radial coordinate twist.
///
/// `RadialTwist` is the tui-vfx name for vortex/maelstrom-style source
/// sampling. It keeps the effect vocabulary substrate-oriented: content is
/// sampled through a radial twist field rather than through a demo-specific
/// screensaver name.
pub struct RadialTwist {
    twist: f32,
    center: RippleCenter,
    radius_floor: f32,
}

impl Default for RadialTwist {
    fn default() -> Self {
        Self::new(1.0, RippleCenter::Center, 0.1)
    }
}

impl RadialTwist {
    /// Create a new radial twist sampler.
    pub fn new(twist: f32, center: RippleCenter, radius_floor: f32) -> Self {
        Self {
            twist,
            center,
            radius_floor: radius_floor.abs().max(0.0001),
        }
    }

    fn center_in_cells(&self, width: u16, height: u16) -> (f32, f32) {
        match self.center {
            RippleCenter::Center => (width as f32 / 2.0, height as f32 / 2.0),
            RippleCenter::Point { x, y } => (x as f32, y as f32),
        }
    }
}

impl Sampler for RadialTwist {
    fn sample(&self, ctx: &VfxCellContext) -> SamplerOutput {
        let dest_x = ctx.local_x;
        let dest_y = ctx.local_y;
        let width = ctx.width;
        let height = ctx.height;
        let t = ctx.t;

        if width == 0 || height == 0 {
            return SamplerOutput::no_displacement();
        }

        let (center_x, center_y) = self.center_in_cells(width, height);
        let scale = width.max(height).max(1) as f32 / 2.0;
        let norm_x = (dest_x as f32 - center_x) / scale;
        let norm_y = (dest_y as f32 - center_y) / scale;
        let twist = self.twist * t as f32;
        let (warped_x, warped_y) =
            radial_twist_warp(norm_x, norm_y, 0.0, 0.0, twist, self.radius_floor);
        let src_x_f = center_x + warped_x * scale;
        let src_y_f = center_y + warped_y * scale;

        if src_x_f < 0.0 || src_y_f < 0.0 || src_x_f >= width as f32 || src_y_f >= height as f32 {
            SamplerOutput::no_displacement()
        } else {
            let src_x = src_x_f.round() as u16;
            let src_y = src_y_f.round() as u16;
            let delta_x = src_x as i32 - dest_x as i32;
            let delta_y = src_y as i32 - dest_y as i32;
            SamplerOutput::displaced(src_x, src_y, delta_x, delta_y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_twist_is_identity() {
        let sampler = RadialTwist::new(0.0, RippleCenter::Center, 0.1);
        assert_eq!(
            sampler
                .sample(&VfxCellContext::new(3, 4, 20, 10, 0, 0, 1.0))
                .source,
            Some((3, 4))
        );
    }

    #[test]
    fn twist_remaps_off_center_cells() {
        let sampler = RadialTwist::new(1.0, RippleCenter::Center, 0.1);
        let result = sampler.sample(&VfxCellContext::new(15, 5, 20, 10, 0, 0, 1.0));
        assert!(matches!(result.source, Some((_, y)) if y != 5));
    }

    #[test]
    fn center_cell_remains_finite() {
        let sampler = RadialTwist::new(8.0, RippleCenter::Center, 0.1);
        assert_eq!(
            sampler
                .sample(&VfxCellContext::new(10, 5, 20, 10, 0, 0, 1.0))
                .source,
            Some((10, 5))
        );
    }

    #[test]
    fn sample_emits_sampler_output_with_displacement_delta() {
        // Off-center cell with twist=1.0 at t=1.0 should produce non-zero delta
        let sampler = RadialTwist::new(1.0, RippleCenter::Center, 0.1);
        let out = sampler.sample(&VfxCellContext::new(15, 5, 20, 10, 0, 0, 1.0));
        assert!(out.source.is_some());
        let (src_x, src_y) = out.source.unwrap();
        assert_eq!(out.delta_x, src_x as i32 - 15);
        assert_eq!(out.delta_y, src_y as i32 - 5);
        // The twist should produce a non-trivial displacement
        assert!(out.delta_x != 0 || out.delta_y != 0);
    }
}

// <FILE>tui-vfx-compositor-next/src/samplers/cls_radial_twist.rs</FILE> - <DESC>RadialTwist sampler for center-weighted coordinate warps</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
