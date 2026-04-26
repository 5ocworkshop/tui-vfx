// <FILE>tui-vfx-compositor/src/samplers/cls_radial_twist.rs</FILE> - <DESC>RadialTwist sampler for center-weighted coordinate warps</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Slice 6.6 §F.4 — migrate Sampler trait to take &VfxCellContext</WCTX>
// <CLOG>0.2.0: sample() now takes &VfxCellContext; dest_x/dest_y/width/height/t reach via ctx fields.</CLOG>

use crate::traits::sampler::Sampler;
use tui_vfx_types::VfxCellContext;
use crate::types::cls_sampler_spec::RippleCenter;
use mixed_signals::math::radial_twist_warp;

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
    fn sample(&self, ctx: &VfxCellContext) -> Option<(u16, u16)> {
        let dest_x = ctx.local_x;
        let dest_y = ctx.local_y;
        let width = ctx.width;
        let height = ctx.height;
        let t = ctx.t;

        if width == 0 || height == 0 {
            return None;
        }

        let (center_x, center_y) = self.center_in_cells(width, height);
        let scale = width.max(height).max(1) as f32 / 2.0;
        let norm_x = (dest_x as f32 - center_x) / scale;
        let norm_y = (dest_y as f32 - center_y) / scale;
        let twist = self.twist * t as f32;
        let (warped_x, warped_y) =
            radial_twist_warp(norm_x, norm_y, 0.0, 0.0, twist, self.radius_floor);
        let src_x = center_x + warped_x * scale;
        let src_y = center_y + warped_y * scale;

        if src_x < 0.0 || src_y < 0.0 || src_x >= width as f32 || src_y >= height as f32 {
            None
        } else {
            Some((src_x.round() as u16, src_y.round() as u16))
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
            sampler.sample(&VfxCellContext::new(3, 4, 20, 10, 0, 0, 1.0)),
            Some((3, 4))
        );
    }

    #[test]
    fn twist_remaps_off_center_cells() {
        let sampler = RadialTwist::new(1.0, RippleCenter::Center, 0.1);
        let result = sampler.sample(&VfxCellContext::new(15, 5, 20, 10, 0, 0, 1.0));
        assert!(matches!(result, Some((_, y)) if y != 5));
    }

    #[test]
    fn center_cell_remains_finite() {
        let sampler = RadialTwist::new(8.0, RippleCenter::Center, 0.1);
        assert_eq!(
            sampler.sample(&VfxCellContext::new(10, 5, 20, 10, 0, 0, 1.0)),
            Some((10, 5))
        );
    }
}

// <FILE>tui-vfx-compositor/src/samplers/cls_radial_twist.rs</FILE> - <DESC>RadialTwist sampler for center-weighted coordinate warps</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
