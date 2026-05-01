// <FILE>tui-vfx-compositor-next/src/masks/cls_diamond.rs</FILE> - <DESC>Diamond mask implementation</DESC>
// <VERS>VERSION: 1.3.0</VERS>
// <WCTX>Slice 6.6 §F.3 — migrate Mask trait to &VfxCellContext</WCTX>
// <CLOG>1.3.0: MINOR — is_visible signature updated to &VfxCellContext; local_x/local_y/width/height/t replace positional params.</CLOG>

use super::col_soft_edge::{calc_edge_width, is_visible_with_soft_edge};
use crate::traits::mask::Mask;
use tui_vfx_types::VfxCellContext;

/// Diamond-shaped expand mask from center.
///
/// Reveals content in a diamond (Manhattan-distance) pattern expanding from
/// the center outward. Optionally applies a soft edge for smooth transitions.
pub struct Diamond {
    /// Whether to apply soft edge blending
    pub soft_edge: bool,
}

impl Default for Diamond {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Diamond {
    /// Create a new Diamond mask.
    pub fn new(soft_edge: bool) -> Self {
        Self { soft_edge }
    }
}

impl Mask for Diamond {
    fn is_visible(&self, ctx: &VfxCellContext) -> bool {
        let progress = ctx.t as f32;

        let cx = ctx.width as f32 / 2.0;
        let cy = ctx.height as f32 / 2.0;
        let dx = (ctx.local_x as f32 - cx).abs();
        let dy = (ctx.local_y as f32 - cy).abs();
        let manhattan = dx + dy;
        let max_dim = ctx.width.max(ctx.height) as f32;
        let threshold = max_dim * progress;
        is_visible_with_soft_edge(
            manhattan,
            threshold,
            self.soft_edge,
            calc_edge_width(max_dim),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_at(x: u16, y: u16, w: u16, h: u16, t: f64) -> VfxCellContext {
        VfxCellContext::new(x, y, w, h, 0, 0, t)
    }

    #[test]
    fn test_diamond_progress_zero_not_visible() {
        let mask = Diamond::new(false);
        // At progress 0, threshold = 0, manhattan from center = 0
        // 0 < 0 is false
        assert!(!mask.is_visible(&ctx_at(5, 5, 10, 10, 0.0)));
    }

    #[test]
    fn test_diamond_progress_one_visible() {
        let mask = Diamond::new(false);
        assert!(mask.is_visible(&ctx_at(5, 5, 10, 10, 1.0)));
    }

    #[test]
    fn test_diamond_center_visible_early() {
        let mask = Diamond::new(false);
        // Center has manhattan distance 0, so any progress > 0 makes it visible
        assert!(mask.is_visible(&ctx_at(5, 5, 10, 10, 0.1)));
    }

    #[test]
    fn test_diamond_corner_needs_high_progress() {
        let mask = Diamond::new(false);
        // Corner (0,0) has manhattan distance 5+5=10 from center (5,5)
        // threshold = 10 * progress
        // At progress 0.9: threshold=9, dist=10, 10 < 9 is false
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.9)));
        // At progress 1.0: threshold=10, dist=10, 10 < 10 is still false (strict <)
        // Corner is exactly at boundary, so it's not visible without soft edge
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 1.0)));
        // But with soft edge (adds 1.0), it becomes visible
        let soft = Diamond::new(true);
        assert!(soft.is_visible(&ctx_at(0, 0, 10, 10, 1.0)));
    }

    #[test]
    fn test_diamond_soft_edge_extends_visibility() {
        let hard = Diamond::new(false);
        let soft = Diamond::new(true);
        // Soft edge adds 10% of max_dim = 1.0
        // A point just outside hard threshold should be inside soft
        // At progress 0.5, threshold=5, soft threshold=6
        // Point at (0,5) has manhattan=5, exactly at hard threshold
        let hard_vis = hard.is_visible(&ctx_at(0, 5, 10, 10, 0.5));
        let soft_vis = soft.is_visible(&ctx_at(0, 5, 10, 10, 0.5));
        // hard: 5 < 5 is false, soft: 5 < 6 is true
        assert!(!hard_vis);
        assert!(soft_vis);
    }
}

// <FILE>tui-vfx-compositor-next/src/masks/cls_diamond.rs</FILE> - <DESC>Diamond mask implementation</DESC>
// <VERS>END OF VERSION: 1.3.0</VERS>
