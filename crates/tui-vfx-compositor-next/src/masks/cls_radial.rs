// <FILE>tui-vfx-compositor-next/src/masks/cls_radial.rs</FILE>
// <DESC>Radial mask revealing from configurable origin</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Slice 6.6 §F.3 — migrate Mask trait to &VfxCellContext</WCTX>
// <CLOG>1.2.0: MINOR — is_visible signature updated to &VfxCellContext; local_x/local_y/width/height/t replace positional params.</CLOG>

use crate::traits::mask::Mask;
use mixed_signals::prelude::{Signal, SignalContext, SurfaceDistanceSignal};
use serde::{Deserialize, Serialize};
use tui_vfx_types::VfxCellContext;

/// Origin point for radial reveal.
#[derive(
    Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RadialOrigin {
    /// Expand from center (default)
    #[default]
    Center,
    /// Expand from top-left corner
    TopLeft,
    /// Expand from top-right corner
    TopRight,
    /// Expand from bottom-left corner
    BottomLeft,
    /// Expand from bottom-right corner
    BottomRight,
    /// Custom origin as fraction (0.0-1.0 for x and y)
    Custom { x: f32, y: f32 },
}

impl RadialOrigin {
    /// Get the origin point as (x, y) fractions in 0.0-1.0 range.
    pub fn as_fraction(&self) -> (f32, f32) {
        match self {
            RadialOrigin::Center => (0.5, 0.5),
            RadialOrigin::TopLeft => (0.0, 0.0),
            RadialOrigin::TopRight => (1.0, 0.0),
            RadialOrigin::BottomLeft => (0.0, 1.0),
            RadialOrigin::BottomRight => (1.0, 1.0),
            RadialOrigin::Custom { x, y } => (*x, *y),
        }
    }
}

/// Radial mask that reveals in a circular pattern from a configurable origin.
///
/// The reveal expands outward from the origin point, creating a circular
/// or elliptical pattern depending on the widget's aspect ratio.
pub struct Radial {
    /// Origin point for the radial expansion
    pub origin: RadialOrigin,
    /// Whether to apply soft edge blending
    pub soft_edge: bool,
}

impl Default for Radial {
    fn default() -> Self {
        Self::new(RadialOrigin::Center, false)
    }
}

impl Radial {
    /// Create a new Radial mask.
    ///
    /// # Arguments
    /// * `origin` - The point from which the reveal expands
    /// * `soft_edge` - Whether to apply soft edge blending
    pub fn new(origin: RadialOrigin, soft_edge: bool) -> Self {
        Self { origin, soft_edge }
    }

    /// Create a radial mask expanding from center.
    #[allow(dead_code)]
    pub fn from_center() -> Self {
        Self::new(RadialOrigin::Center, false)
    }

    /// Create a radial mask expanding from a corner.
    #[allow(dead_code)]
    pub fn from_corner(origin: RadialOrigin) -> Self {
        Self::new(origin, false)
    }
}

impl Mask for Radial {
    fn is_visible(&self, ctx: &VfxCellContext) -> bool {
        let progress = ctx.t as f32;

        if progress <= 0.0 {
            return false;
        }
        if progress >= 1.0 {
            return true;
        }

        let (origin_x, origin_y) = self.origin.as_fraction();
        let signal_ctx = SignalContext::new(0, 0)
            .with_dimensions(ctx.width, ctx.height)
            .with_cell_position(ctx.local_x, ctx.local_y);
        let normalized_dist = SurfaceDistanceSignal::radius_from(origin_x, origin_y)
            .sample_with_context(0.0, &signal_ctx);

        if self.soft_edge {
            // Soft edge: use smooth transition
            let edge_width = 0.1;
            let threshold = progress;
            if normalized_dist < threshold - edge_width {
                true
            } else if normalized_dist > threshold {
                false
            } else {
                // Smooth transition
                let t = (threshold - normalized_dist) / edge_width;
                t > 0.5
            }
        } else {
            // Hard edge
            normalized_dist < progress
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::masks::cls_wipe::Wipe;
    use crate::types::cls_mask_spec::WipeDirection;

    fn ctx_at(x: u16, y: u16, w: u16, h: u16, t: f64) -> VfxCellContext {
        VfxCellContext::new(x, y, w, h, 0, 0, t)
    }

    #[test]
    fn test_radial_origin_fractions() {
        assert_eq!(RadialOrigin::Center.as_fraction(), (0.5, 0.5));
        assert_eq!(RadialOrigin::TopLeft.as_fraction(), (0.0, 0.0));
        assert_eq!(RadialOrigin::TopRight.as_fraction(), (1.0, 0.0));
        assert_eq!(RadialOrigin::BottomLeft.as_fraction(), (0.0, 1.0));
        assert_eq!(RadialOrigin::BottomRight.as_fraction(), (1.0, 1.0));
        assert_eq!(
            RadialOrigin::Custom { x: 0.25, y: 0.75 }.as_fraction(),
            (0.25, 0.75)
        );
    }

    #[test]
    fn test_center_at_zero_progress() {
        let mask = Radial::from_center();
        // At 0% progress, nothing visible
        assert!(!mask.is_visible(&ctx_at(5, 5, 10, 10, 0.0)));
    }

    #[test]
    fn test_center_at_full_progress() {
        let mask = Radial::from_center();
        // At 100% progress, everything visible
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 1.0)));
        assert!(mask.is_visible(&ctx_at(9, 9, 10, 10, 1.0)));
    }

    #[test]
    fn test_center_reveals_from_middle() {
        let mask = Radial::from_center();
        // At partial progress, center should be visible before corners
        // For a 10x10 grid, center is (5, 5)
        // At low progress, center visible but corners not
        assert!(mask.is_visible(&ctx_at(5, 5, 10, 10, 0.1)));
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.1)));
    }

    #[test]
    fn test_corner_origin() {
        let mask = Radial::from_corner(RadialOrigin::TopLeft);
        // At partial progress, top-left should be visible first
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 0.1)));
        // Far corner should not be visible at low progress
        assert!(!mask.is_visible(&ctx_at(9, 9, 10, 10, 0.1)));
    }

    #[test]
    fn radial_is_not_equivalent_to_center_out_wipe_on_square_surfaces() {
        let radial = Radial::from_center();
        let wipe = Wipe::new(WipeDirection::HorizontalCenterOut, false);

        // On a square surface at 50% progress, a radial reveal should still
        // hide top-center because it is outside the circular threshold, while
        // a center-out wipe reveals it immediately because only horizontal
        // distance matters.
        assert!(!radial.is_visible(&ctx_at(5, 0, 11, 11, 0.5)));
        assert!(wipe.is_visible(&ctx_at(5, 0, 11, 11, 0.5)));
    }
}

// <FILE>tui-vfx-compositor-next/src/masks/cls_radial.rs</FILE>
// <DESC>Radial mask revealing from configurable origin</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
