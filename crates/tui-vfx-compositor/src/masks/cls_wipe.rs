// <FILE>tui-vfx-compositor/src/masks/cls_wipe.rs</FILE>
// <DESC>Linear wipe mask with cardinal, diagonal, centre-out, edges-in, and corner-arc directions</DESC>
// <VERS>VERSION: 2.1.0</VERS>
// <WCTX>Slice 6.6 §F.3 — migrate Mask trait to &VfxCellContext</WCTX>
// <CLOG>2.1.0: MINOR — is_visible signature updated to &VfxCellContext; local_x/local_y/width/height/t replace positional params.</CLOG>

use super::col_soft_edge::calc_edge_width;
use crate::traits::mask::Mask;
use crate::types::cls_mask_spec::WipeDirection;
use tui_vfx_geometry::{wipe_progress, wipe_visible_at};
use tui_vfx_types::VfxCellContext;

/// Linear wipe mask that reveals/hides from one edge to another.
///
/// When `invert` is false (default), this is a "reveal" mask:
/// - At t=0: nothing visible
/// - At t=1: everything visible
/// - Direction specifies where content appears from
///
/// When `invert` is true, this is a "hide" mask:
/// - At t=1: everything visible (hide hasn't started)
/// - At t=0: nothing visible (hide complete)
/// - Direction specifies where content disappears to
///
/// The hide behavior is designed for exit animations where t goes 1→0.
/// Internally, hide masks use `1-t` so the wipe progresses forward
/// as the animation progresses backward.
pub struct Wipe {
    /// Direction of the wipe
    pub direction: WipeDirection,
    /// Whether to apply soft edge blending
    pub soft_edge: bool,
    /// Whether to invert mask values (true for "hide" semantics)
    pub invert: bool,
}

impl Default for Wipe {
    fn default() -> Self {
        Self::new_with_invert(WipeDirection::LeftToRight, false, false)
    }
}

impl Wipe {
    /// Create a new Wipe mask (reveal mode).
    #[allow(dead_code)]
    pub fn new(direction: WipeDirection, soft_edge: bool) -> Self {
        Self::new_with_invert(direction, soft_edge, false)
    }

    /// Create a new Wipe mask with explicit invert control.
    ///
    /// - `invert: false` → reveal mask (content appears in direction)
    /// - `invert: true` → hide mask (content disappears in direction)
    pub fn new_with_invert(direction: WipeDirection, soft_edge: bool, invert: bool) -> Self {
        Self {
            direction,
            soft_edge,
            invert,
        }
    }
}

impl Mask for Wipe {
    fn is_visible(&self, ctx: &VfxCellContext) -> bool {
        // For hide masks, use 1-t so the wipe progresses forward as animation goes backward.
        // Exit animations run t from 1→0, so:
        //   - At t=1 (exit start): effective_progress=0, wipe at start, everything visible
        //   - At t=0 (exit end): effective_progress=1, wipe complete, nothing visible
        let effective_progress = if self.invert { 1.0 - ctx.t } else { ctx.t };
        let effective_progress_f32 = effective_progress as f32;

        // Cardinal / diagonal / centre-out / edges-in: use the shared
        // (position, size) helper so the soft-edge and invert paths can
        // continue to apply edge_width directly. Corner-arc variants
        // return None and route through `wipe_visible_at` below.
        if let Some((position, size)) = wipe_progress(
            self.direction,
            ctx.local_x,
            ctx.local_y,
            ctx.width,
            ctx.height,
        ) {
            // Handle edge case where size is 0
            if size <= 0.0 {
                let visible = effective_progress_f32 > 0.0;
                return if self.invert { !visible } else { visible };
            }
            let threshold = size * effective_progress_f32;
            // Soft edge extends visibility for reveal masks (smooth leading edge).
            // For hide masks (invert=true), soft edge extension breaks the semantics
            // because it gets inverted, causing premature hiding. We disable soft edge
            // extension for hide masks - the visual smoothness comes from the fade/tint
            // effect that typically accompanies wipe transitions.
            let visible = if self.soft_edge && !self.invert {
                let edge_width = calc_edge_width(size);
                position < threshold + edge_width
            } else {
                position < threshold
            };
            return if self.invert { !visible } else { visible };
        }

        // Corner-arc variants. soft_edge is intentionally a no-op for these
        // — the radial wavefront is already smooth at typical render sizes,
        // and applying calc_edge_width to a Euclidean threshold isn't
        // semantically equivalent. Author-friendly soft-edge for corner
        // arcs can be layered on later if it's requested.
        wipe_visible_at(
            self.direction,
            ctx.local_x,
            ctx.local_y,
            ctx.width,
            ctx.height,
            effective_progress_f32,
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
    fn test_wipe_left_to_right() {
        let mask = Wipe::default();
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 0.5)));
        assert!(mask.is_visible(&ctx_at(4, 0, 10, 10, 0.5)));
        assert!(!mask.is_visible(&ctx_at(5, 0, 10, 10, 0.5)));
        assert!(!mask.is_visible(&ctx_at(9, 0, 10, 10, 0.5)));
    }

    #[test]
    fn test_wipe_right_to_left_direction() {
        let mask = Wipe::new(WipeDirection::RightToLeft, false);
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.5)));
        assert!(!mask.is_visible(&ctx_at(4, 0, 10, 10, 0.5)));
        assert!(mask.is_visible(&ctx_at(5, 0, 10, 10, 0.5)));
        assert!(mask.is_visible(&ctx_at(9, 0, 10, 10, 0.5)));
    }

    #[test]
    fn test_wipe_top_to_bottom_direction() {
        let mask = Wipe::new(WipeDirection::TopToBottom, false);
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 0.5)));
        assert!(mask.is_visible(&ctx_at(0, 4, 10, 10, 0.5)));
        assert!(!mask.is_visible(&ctx_at(0, 5, 10, 10, 0.5)));
        assert!(!mask.is_visible(&ctx_at(0, 9, 10, 10, 0.5)));
    }

    #[test]
    fn test_wipe_bottom_to_top_direction() {
        let mask = Wipe::new(WipeDirection::BottomToTop, false);
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.5)));
        assert!(!mask.is_visible(&ctx_at(0, 4, 10, 10, 0.5)));
        assert!(mask.is_visible(&ctx_at(0, 5, 10, 10, 0.5)));
        assert!(mask.is_visible(&ctx_at(0, 9, 10, 10, 0.5)));
    }

    #[test]
    fn test_wipe_top_left_to_bottom_right() {
        let mask = Wipe::new(WipeDirection::TopLeftToBottomRight, false);
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 0.5)));
        assert!(mask.is_visible(&ctx_at(4, 4, 10, 10, 0.5)));
        assert!(!mask.is_visible(&ctx_at(5, 5, 10, 10, 0.5)));
        assert!(!mask.is_visible(&ctx_at(9, 9, 10, 10, 0.5)));
    }

    #[test]
    fn test_wipe_bottom_right_to_top_left() {
        let mask = Wipe::new(WipeDirection::BottomRightToTopLeft, false);
        assert!(mask.is_visible(&ctx_at(9, 9, 10, 10, 0.5)));
        assert!(mask.is_visible(&ctx_at(5, 5, 10, 10, 0.5)));
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.5)));
    }

    #[test]
    fn test_wipe_top_right_to_bottom_left() {
        let mask = Wipe::new(WipeDirection::TopRightToBottomLeft, false);
        assert!(mask.is_visible(&ctx_at(9, 0, 10, 10, 0.5)));
        assert!(mask.is_visible(&ctx_at(5, 3, 10, 10, 0.5)));
        assert!(!mask.is_visible(&ctx_at(0, 9, 10, 10, 0.5)));
    }

    #[test]
    fn test_wipe_bottom_left_to_top_right() {
        let mask = Wipe::new(WipeDirection::BottomLeftToTopRight, false);
        assert!(mask.is_visible(&ctx_at(0, 9, 10, 10, 0.5)));
        assert!(mask.is_visible(&ctx_at(3, 5, 10, 10, 0.5)));
        assert!(!mask.is_visible(&ctx_at(9, 0, 10, 10, 0.5)));
    }

    #[test]
    fn test_diagonal_wipe_at_extremes() {
        let mask = Wipe::new(WipeDirection::TopLeftToBottomRight, false);
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.0)));
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 1.0)));
        assert!(mask.is_visible(&ctx_at(9, 9, 10, 10, 1.0)));
    }

    #[test]
    fn test_horizontal_center_out() {
        let mask = Wipe::new(WipeDirection::HorizontalCenterOut, false);
        // At progress 0, nothing visible
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.0)));
        assert!(!mask.is_visible(&ctx_at(5, 0, 10, 10, 0.0)));
        // At progress 0.5, center columns visible, edges not
        assert!(mask.is_visible(&ctx_at(4, 0, 10, 10, 0.5))); // Near center
        assert!(mask.is_visible(&ctx_at(5, 0, 10, 10, 0.5))); // Near center
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.5))); // Left edge
        assert!(!mask.is_visible(&ctx_at(9, 0, 10, 10, 0.5))); // Right edge
        // At progress 1.0, everything visible
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 1.0)));
        assert!(mask.is_visible(&ctx_at(9, 0, 10, 10, 1.0)));
    }

    #[test]
    fn test_vertical_center_out() {
        let mask = Wipe::new(WipeDirection::VerticalCenterOut, false);
        // At progress 0, nothing visible
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.0)));
        assert!(!mask.is_visible(&ctx_at(0, 5, 10, 10, 0.0)));
        // At progress 0.5, center rows visible, edges not
        assert!(mask.is_visible(&ctx_at(0, 4, 10, 10, 0.5))); // Near center
        assert!(mask.is_visible(&ctx_at(0, 5, 10, 10, 0.5))); // Near center
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.5))); // Top edge
        assert!(!mask.is_visible(&ctx_at(0, 9, 10, 10, 0.5))); // Bottom edge
        // At progress 1.0, everything visible
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 1.0)));
        assert!(mask.is_visible(&ctx_at(0, 9, 10, 10, 1.0)));
    }

    #[test]
    fn test_horizontal_edges_in() {
        let mask = Wipe::new(WipeDirection::HorizontalEdgesIn, false);
        // At progress 0, nothing visible
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.0)));
        assert!(!mask.is_visible(&ctx_at(5, 0, 10, 10, 0.0)));
        // At progress 0.5, edges visible, center not
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 0.5))); // Left edge
        assert!(mask.is_visible(&ctx_at(9, 0, 10, 10, 0.5))); // Right edge
        assert!(!mask.is_visible(&ctx_at(4, 0, 10, 10, 0.5))); // Near center
        assert!(!mask.is_visible(&ctx_at(5, 0, 10, 10, 0.5))); // Near center
        // At progress 1.0, everything visible
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 1.0)));
        assert!(mask.is_visible(&ctx_at(5, 0, 10, 10, 1.0)));
    }

    #[test]
    fn test_vertical_edges_in() {
        let mask = Wipe::new(WipeDirection::VerticalEdgesIn, false);
        // At progress 0, nothing visible
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.0)));
        assert!(!mask.is_visible(&ctx_at(0, 5, 10, 10, 0.0)));
        // At progress 0.5, edges visible, center not
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 0.5))); // Top edge
        assert!(mask.is_visible(&ctx_at(0, 9, 10, 10, 0.5))); // Bottom edge
        assert!(!mask.is_visible(&ctx_at(0, 4, 10, 10, 0.5))); // Near center
        assert!(!mask.is_visible(&ctx_at(0, 5, 10, 10, 0.5))); // Near center
        // At progress 1.0, everything visible
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 1.0)));
        assert!(mask.is_visible(&ctx_at(0, 5, 10, 10, 1.0)));
    }

    #[test]
    fn test_wipe_hide_left_to_right() {
        // Normal reveal at t=0.5: left side visible, right hidden
        let reveal = Wipe::new(WipeDirection::LeftToRight, false);
        assert!(reveal.is_visible(&ctx_at(0, 0, 10, 10, 0.5))); // Left visible
        assert!(!reveal.is_visible(&ctx_at(9, 0, 10, 10, 0.5))); // Right hidden

        // Hide at t=0.5 (mid-exit): effective_progress = 0.5, then output inverted
        // Left side: base_visible=true (position < threshold), inverted=false → HIDDEN
        // Right side: base_visible=false (position >= threshold), inverted=true → VISIBLE
        let hide = Wipe::new_with_invert(WipeDirection::LeftToRight, false, true);
        assert!(!hide.is_visible(&ctx_at(0, 0, 10, 10, 0.5))); // Left HIDDEN (wipe has passed)
        assert!(hide.is_visible(&ctx_at(9, 0, 10, 10, 0.5))); // Right VISIBLE (wipe hasn't reached)
    }

    #[test]
    fn test_wipe_hide_at_extremes() {
        // Hide mask is designed for exit animations where t goes 1→0
        let reveal = Wipe::new(WipeDirection::LeftToRight, false);
        let hide = Wipe::new_with_invert(WipeDirection::LeftToRight, false, true);

        // At t=0: reveal shows nothing
        assert!(!reveal.is_visible(&ctx_at(5, 5, 10, 10, 0.0)));
        // At t=0 (exit end): effective_progress=1, threshold=size, base_visible=true, inverted=false
        // Result: NOTHING visible (exit complete, content fully hidden)
        assert!(!hide.is_visible(&ctx_at(5, 5, 10, 10, 0.0)));

        // At t=1: reveal shows everything
        assert!(reveal.is_visible(&ctx_at(5, 5, 10, 10, 1.0)));
        // At t=1 (exit start): effective_progress=0, threshold=0, base_visible=false, inverted=true
        // Result: EVERYTHING visible (exit hasn't started)
        assert!(hide.is_visible(&ctx_at(5, 5, 10, 10, 1.0)));
    }

    #[test]
    fn test_wipe_hide_soft_edge_no_premature_hiding() {
        // Regression test: soft edge should not cause hide to "lead" the animation
        // Previously, at t=1.0 (exit start), leftmost pixels were already hidden
        // because soft edge extended the "visible" zone, which became "hidden" after invert
        let hide_soft = Wipe::new_with_invert(WipeDirection::LeftToRight, true, true);

        // At t=1.0 (exit start): EVERYTHING should be visible, including position 0
        // With the bug: position 0 would be hidden because soft edge extended base visibility
        assert!(
            hide_soft.is_visible(&ctx_at(0, 0, 100, 10, 1.0)),
            "Leftmost pixel should be visible at exit start"
        );
        assert!(
            hide_soft.is_visible(&ctx_at(99, 0, 100, 10, 1.0)),
            "Rightmost pixel should be visible at exit start"
        );

        // At t=0.0 (exit end): EVERYTHING should be hidden
        assert!(
            !hide_soft.is_visible(&ctx_at(0, 0, 100, 10, 0.0)),
            "Leftmost pixel should be hidden at exit end"
        );
        assert!(
            !hide_soft.is_visible(&ctx_at(99, 0, 100, 10, 0.0)),
            "Rightmost pixel should be hidden at exit end"
        );

        // At t=0.5 (mid-exit): approximately half should be hidden
        // With soft edge, the boundary should have a trailing visible edge
        assert!(
            !hide_soft.is_visible(&ctx_at(0, 0, 100, 10, 0.5)),
            "Leftmost pixel should be hidden at mid-exit"
        );
        assert!(
            hide_soft.is_visible(&ctx_at(99, 0, 100, 10, 0.5)),
            "Rightmost pixel should be visible at mid-exit"
        );
    }

    #[test]
    fn test_wipe_hide_diagonal() {
        let reveal = Wipe::new(WipeDirection::TopLeftToBottomRight, false);
        let hide = Wipe::new_with_invert(WipeDirection::TopLeftToBottomRight, false, true);

        // At t=0.5: reveal shows top-left, hides bottom-right
        assert!(reveal.is_visible(&ctx_at(0, 0, 10, 10, 0.5)));
        assert!(!reveal.is_visible(&ctx_at(9, 9, 10, 10, 0.5)));

        // At t=0.5: hide has hidden top-left, bottom-right still visible
        assert!(!hide.is_visible(&ctx_at(0, 0, 10, 10, 0.5))); // Top-left HIDDEN
        assert!(hide.is_visible(&ctx_at(9, 9, 10, 10, 0.5))); // Bottom-right VISIBLE
    }
}

// <FILE>tui-vfx-compositor/src/masks/cls_wipe.rs</FILE>
// <DESC>Linear wipe mask with cardinal, diagonal, centre-out, edges-in, and corner-arc directions</DESC>
// <VERS>END OF VERSION: 2.1.0</VERS>
