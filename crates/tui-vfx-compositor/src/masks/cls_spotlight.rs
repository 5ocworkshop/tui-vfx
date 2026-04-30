// <FILE>tui-vfx-compositor/src/masks/cls_spotlight.rs</FILE> - <DESC>Spotlight (Iris) mask implementation</DESC>
// <VERS>VERSION: 1.4.0</VERS>
// <WCTX>Slice 6.6 §F.3 — migrate Mask trait to &VfxCellContext</WCTX>
// <CLOG>1.4.0: MINOR — is_visible signature updated to &VfxCellContext; local_x/local_y/width/height/t replace positional params.</CLOG>

use crate::traits::mask::Mask;
use crate::types::cls_mask_spec::IrisShape;
use mixed_signals::prelude::{Signal, SignalContext, SpatialCoordinateSignal};
use tui_vfx_types::VfxCellContext;

/// Spotlight/Iris mask - reveals from center outward.
pub struct Spotlight {
    /// Shape of the iris reveal
    pub shape: IrisShape,
    /// Whether to apply soft edge blending
    pub soft_edge: bool,
}

impl Default for Spotlight {
    fn default() -> Self {
        Self::new(IrisShape::Circle, false)
    }
}

impl Spotlight {
    /// Create a new Spotlight/Iris mask.
    pub fn new(shape: IrisShape, soft_edge: bool) -> Self {
        Self { shape, soft_edge }
    }

    /// Calculate distance from center based on shape
    fn distance(&self, x: u16, y: u16, w: u16, h: u16) -> f32 {
        let signal_ctx = SignalContext::new(0, 0)
            .with_dimensions(w, h)
            .with_cell_position(x, y);
        let dx = SpatialCoordinateSignal::sample_surface_centered_x()
            .sample_with_context(0.0, &signal_ctx)
            * (w as f32 / 2.0);
        let dy = SpatialCoordinateSignal::sample_surface_centered_y()
            .sample_with_context(0.0, &signal_ctx)
            * (h as f32 / 2.0);

        match self.shape {
            IrisShape::Circle => {
                let max_distance = ((w as f32 / 2.0).powi(2) + (h as f32 / 2.0).powi(2)).sqrt();
                SpatialCoordinateSignal::sample_surface_radius()
                    .sample_with_context(0.0, &signal_ctx)
                    * max_distance
            }
            IrisShape::Diamond => dx.abs() + dy.abs(),
            IrisShape::Box => dx.abs().max(dy.abs()),
        }
    }
}

impl Mask for Spotlight {
    fn is_visible(&self, ctx: &VfxCellContext) -> bool {
        let progress = ctx.t as f32;

        let dist = self.distance(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        let max_dim = ctx.width.max(ctx.height) as f32;
        let max_radius = max_dim * 0.75; // Reach corners approx
        let current_radius = max_radius * progress;

        if self.soft_edge {
            // Soft edge: gradual transition
            let edge_width = max_radius * 0.1;
            dist < current_radius + edge_width
        } else {
            dist < current_radius
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

    fn old_distance(shape: IrisShape, x: u16, y: u16, w: u16, h: u16) -> f32 {
        let cx = w as f32 / 2.0;
        let cy = h as f32 / 2.0;
        let dx = x as f32 - cx;
        let dy = y as f32 - cy;

        match shape {
            IrisShape::Circle => (dx * dx + dy * dy).sqrt(),
            IrisShape::Diamond => dx.abs() + dy.abs(),
            IrisShape::Box => dx.abs().max(dy.abs()),
        }
    }

    fn old_visible(
        shape: IrisShape,
        soft_edge: bool,
        x: u16,
        y: u16,
        w: u16,
        h: u16,
        progress: f64,
    ) -> bool {
        let progress = progress as f32;
        let dist = old_distance(shape, x, y, w, h);
        let max_dim = w.max(h) as f32;
        let max_radius = max_dim * 0.75;
        let current_radius = max_radius * progress;
        if soft_edge {
            let edge_width = max_radius * 0.1;
            dist < current_radius + edge_width
        } else {
            dist < current_radius
        }
    }

    #[test]
    fn test_spotlight_center_progress_zero_not_visible() {
        let mask = Spotlight::new(IrisShape::Circle, false);
        // Center of 10x10 is (5,5), but at progress=0 radius=0
        assert!(!mask.is_visible(&ctx_at(5, 5, 10, 10, 0.0)));
    }

    #[test]
    fn test_spotlight_center_progress_one_visible() {
        let mask = Spotlight::new(IrisShape::Circle, false);
        assert!(mask.is_visible(&ctx_at(5, 5, 10, 10, 1.0)));
    }

    #[test]
    fn test_spotlight_circle_shape() {
        let mask = Spotlight::new(IrisShape::Circle, false);
        // At progress 0.5, radius covers center region
        assert!(mask.is_visible(&ctx_at(5, 5, 10, 10, 0.5))); // Center visible
    }

    #[test]
    fn test_spotlight_diamond_shape() {
        let mask = Spotlight::new(IrisShape::Diamond, false);
        assert!(mask.is_visible(&ctx_at(5, 5, 10, 10, 0.5))); // Center visible
    }

    #[test]
    fn test_spotlight_box_shape() {
        let mask = Spotlight::new(IrisShape::Box, false);
        assert!(mask.is_visible(&ctx_at(5, 5, 10, 10, 0.5))); // Center visible
    }

    #[test]
    fn test_spotlight_soft_edge_extends_visibility() {
        let hard = Spotlight::new(IrisShape::Circle, false);
        let soft = Spotlight::new(IrisShape::Circle, true);
        // Find a position that's just outside hard edge but inside soft edge
        // At progress=0.5, hard radius = 7.5 * 0.5 = 3.75
        // soft edge width = 7.5 * 0.1 = 0.75
        // A point at distance ~4 from center should be visible with soft but not hard
        // Distance from (5,5) to (9,5) is 4
        let hard_vis = hard.is_visible(&ctx_at(9, 5, 10, 10, 0.5));
        let soft_vis = soft.is_visible(&ctx_at(9, 5, 10, 10, 0.5));
        // Soft edge should make more positions visible
        assert!(soft_vis || !hard_vis); // If hard is visible, soft must be too
    }

    #[test]
    fn spotlight_matches_pre_refactor_geometry_across_shapes() {
        let cases = [
            (IrisShape::Circle, false),
            (IrisShape::Circle, true),
            (IrisShape::Diamond, false),
            (IrisShape::Diamond, true),
            (IrisShape::Box, false),
            (IrisShape::Box, true),
        ];
        let progresses = [0.0, 0.15, 0.5, 0.85, 1.0];

        for (shape, soft_edge) in cases {
            let mask = Spotlight::new(shape, soft_edge);
            for progress in progresses {
                for y in 0..10_u16 {
                    for x in 0..10_u16 {
                        assert_eq!(
                            mask.is_visible(&ctx_at(x, y, 10, 10, progress)),
                            old_visible(shape, soft_edge, x, y, 10, 10, progress),
                            "shape={shape:?} soft_edge={soft_edge} x={x} y={y} progress={progress}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn spotlight_circle_is_not_equivalent_to_center_out_wipe_on_square_surfaces() {
        let spotlight = Spotlight::new(IrisShape::Circle, false);
        let wipe = Wipe::new(WipeDirection::HorizontalCenterOut, false);

        assert!(!spotlight.is_visible(&ctx_at(5, 0, 11, 11, 0.5)));
        assert!(wipe.is_visible(&ctx_at(5, 0, 11, 11, 0.5)));
    }

    #[test]
    fn spotlight_diamond_is_not_equivalent_to_center_out_wipe_on_square_surfaces() {
        let spotlight = Spotlight::new(IrisShape::Diamond, false);
        let wipe = Wipe::new(WipeDirection::HorizontalCenterOut, false);

        // Diamond reveal depends on both axes; a horizontal center-out wipe
        // reveals the full center column immediately.
        assert!(!spotlight.is_visible(&ctx_at(5, 0, 11, 11, 0.5)));
        assert!(wipe.is_visible(&ctx_at(5, 0, 11, 11, 0.5)));
    }
}

// <FILE>tui-vfx-compositor/src/masks/cls_spotlight.rs</FILE> - <DESC>Spotlight (Iris) mask implementation</DESC>
// <VERS>END OF VERSION: 1.4.0</VERS>
