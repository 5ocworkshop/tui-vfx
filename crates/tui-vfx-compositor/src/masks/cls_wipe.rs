// <FILE>tui-vfx-compositor/src/masks/cls_wipe.rs</FILE>
// <DESC>Linear wipe mask with cardinal, diagonal, and center-out directions</DESC>
// <VERS>VERSION: 1.9.0</VERS>
// <WCTX>Fix soft edge behavior for hide masks causing premature disappearance</WCTX>
// <CLOG>Disable soft edge extension for hide masks - inversion breaks soft edge semantics, causing wipe to lead animation by ~10%</CLOG>

use super::col_soft_edge::calc_edge_width;
use crate::traits::mask::Mask;
use crate::types::cls_mask_spec::WipeDirection;

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
    fn is_visible(&self, x: u16, y: u16, w: u16, h: u16, progress: f64) -> bool {
        // For hide masks, use 1-t so the wipe progresses forward as animation goes backward.
        // Exit animations run t from 1→0, so:
        //   - At t=1 (exit start): effective_progress=0, wipe at start, everything visible
        //   - At t=0 (exit end): effective_progress=1, wipe complete, nothing visible
        let effective_progress = if self.invert {
            1.0 - progress
        } else {
            progress
        };
        let progress = effective_progress as f32;

        // Calculate position and size based on direction
        // For cardinal directions: use single axis
        // For diagonal directions: use sum of both axes
        let (position, size) = match self.direction {
            // Cardinal: horizontal
            WipeDirection::LeftToRight | WipeDirection::FromLeft => (x as f32, w as f32),
            WipeDirection::RightToLeft | WipeDirection::FromRight => {
                ((w.saturating_sub(1).saturating_sub(x)) as f32, w as f32)
            }
            // Cardinal: vertical
            WipeDirection::TopToBottom | WipeDirection::FromTop => (y as f32, h as f32),
            WipeDirection::BottomToTop | WipeDirection::FromBottom => {
                ((h.saturating_sub(1).saturating_sub(y)) as f32, h as f32)
            }
            // Diagonal: use combined x+y distance from corner
            // Size is max_distance + 1 so that at progress=1.0 all pixels are visible
            WipeDirection::TopLeftToBottomRight => {
                let max_dist = w.saturating_sub(1) + h.saturating_sub(1);
                ((x + y) as f32, (max_dist + 1) as f32)
            }
            WipeDirection::BottomRightToTopLeft => {
                let max_x = w.saturating_sub(1);
                let max_y = h.saturating_sub(1);
                let max_dist = max_x + max_y;
                (
                    ((max_x.saturating_sub(x)) + (max_y.saturating_sub(y))) as f32,
                    (max_dist + 1) as f32,
                )
            }
            WipeDirection::TopRightToBottomLeft => {
                let max_x = w.saturating_sub(1);
                let max_dist = max_x + h.saturating_sub(1);
                (
                    ((max_x.saturating_sub(x)) + y) as f32,
                    (max_dist + 1) as f32,
                )
            }
            WipeDirection::BottomLeftToTopRight => {
                let max_y = h.saturating_sub(1);
                let max_dist = w.saturating_sub(1) + max_y;
                (
                    (x + (max_y.saturating_sub(y))) as f32,
                    (max_dist + 1) as f32,
                )
            }
            // Center-out directions: distance from center determines visibility
            WipeDirection::HorizontalCenterOut => {
                // Distance from center column
                let center = (w as f32 - 1.0) / 2.0;
                let dist_from_center = (x as f32 - center).abs();
                // Half-width is max distance from center to edge
                let half_width = center.max(w as f32 - 1.0 - center);
                (dist_from_center, half_width + 1.0)
            }
            WipeDirection::VerticalCenterOut => {
                // Distance from center row
                let center = (h as f32 - 1.0) / 2.0;
                let dist_from_center = (y as f32 - center).abs();
                // Half-height is max distance from center to edge
                let half_height = center.max(h as f32 - 1.0 - center);
                (dist_from_center, half_height + 1.0)
            }
            // Edges-in directions: inverse of center-out (curtains closing)
            WipeDirection::HorizontalEdgesIn => {
                // Distance from nearest edge (left or right)
                let dist_from_left = x as f32;
                let dist_from_right = (w.saturating_sub(1).saturating_sub(x)) as f32;
                let dist_from_edge = dist_from_left.min(dist_from_right);
                let half_width = (w as f32 - 1.0) / 2.0;
                (dist_from_edge, half_width + 1.0)
            }
            WipeDirection::VerticalEdgesIn => {
                // Distance from nearest edge (top or bottom)
                let dist_from_top = y as f32;
                let dist_from_bottom = (h.saturating_sub(1).saturating_sub(y)) as f32;
                let dist_from_edge = dist_from_top.min(dist_from_bottom);
                let half_height = (h as f32 - 1.0) / 2.0;
                (dist_from_edge, half_height + 1.0)
            }
        };

        // Handle edge case where size is 0
        if size <= 0.0 {
            let visible = progress > 0.0;
            // For hide, also invert the output so t=1 shows everything, t=0 shows nothing
            return if self.invert { !visible } else { visible };
        }

        let threshold = size * progress;

        // Soft edge extends visibility for reveal masks (smooth leading edge).
        // For hide masks (invert=true), soft edge extension breaks the semantics
        // because it gets inverted, causing premature hiding. We disable soft edge
        // extension for hide masks - the visual smoothness comes from the fade/tint
        // effect that typically accompanies wipe transitions.
        let visible = if self.soft_edge && !self.invert {
            // Reveal: add edge_width for smooth leading edge
            let edge_width = calc_edge_width(size);
            position < threshold + edge_width
        } else {
            // Hide or no soft edge: use hard threshold
            position < threshold
        };

        // For hide masks, invert the output so content disappears where the wipe passes
        // Combined with progress inversion (1-t), this gives:
        //   At t=1 (exit start): effective_progress=0, threshold=0, visible=false, inverted=true ✓
        //   At t=0 (exit end): effective_progress=1, threshold=size, visible=true, inverted=false ✓
        if self.invert { !visible } else { visible }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wipe_left_to_right() {
        let mask = Wipe::default();
        assert!(mask.is_visible(0, 0, 10, 10, 0.5));
        assert!(mask.is_visible(4, 0, 10, 10, 0.5));
        assert!(!mask.is_visible(5, 0, 10, 10, 0.5));
        assert!(!mask.is_visible(9, 0, 10, 10, 0.5));
    }

    #[test]
    fn test_wipe_right_to_left_direction() {
        let mask = Wipe::new(WipeDirection::RightToLeft, false);
        assert!(!mask.is_visible(0, 0, 10, 10, 0.5));
        assert!(!mask.is_visible(4, 0, 10, 10, 0.5));
        assert!(mask.is_visible(5, 0, 10, 10, 0.5));
        assert!(mask.is_visible(9, 0, 10, 10, 0.5));
    }

    #[test]
    fn test_wipe_top_to_bottom_direction() {
        let mask = Wipe::new(WipeDirection::TopToBottom, false);
        assert!(mask.is_visible(0, 0, 10, 10, 0.5));
        assert!(mask.is_visible(0, 4, 10, 10, 0.5));
        assert!(!mask.is_visible(0, 5, 10, 10, 0.5));
        assert!(!mask.is_visible(0, 9, 10, 10, 0.5));
    }

    #[test]
    fn test_wipe_bottom_to_top_direction() {
        let mask = Wipe::new(WipeDirection::BottomToTop, false);
        assert!(!mask.is_visible(0, 0, 10, 10, 0.5));
        assert!(!mask.is_visible(0, 4, 10, 10, 0.5));
        assert!(mask.is_visible(0, 5, 10, 10, 0.5));
        assert!(mask.is_visible(0, 9, 10, 10, 0.5));
    }

    #[test]
    fn test_wipe_top_left_to_bottom_right() {
        let mask = Wipe::new(WipeDirection::TopLeftToBottomRight, false);
        assert!(mask.is_visible(0, 0, 10, 10, 0.5));
        assert!(mask.is_visible(4, 4, 10, 10, 0.5));
        assert!(!mask.is_visible(5, 5, 10, 10, 0.5));
        assert!(!mask.is_visible(9, 9, 10, 10, 0.5));
    }

    #[test]
    fn test_wipe_bottom_right_to_top_left() {
        let mask = Wipe::new(WipeDirection::BottomRightToTopLeft, false);
        assert!(mask.is_visible(9, 9, 10, 10, 0.5));
        assert!(mask.is_visible(5, 5, 10, 10, 0.5));
        assert!(!mask.is_visible(0, 0, 10, 10, 0.5));
    }

    #[test]
    fn test_wipe_top_right_to_bottom_left() {
        let mask = Wipe::new(WipeDirection::TopRightToBottomLeft, false);
        assert!(mask.is_visible(9, 0, 10, 10, 0.5));
        assert!(mask.is_visible(5, 3, 10, 10, 0.5));
        assert!(!mask.is_visible(0, 9, 10, 10, 0.5));
    }

    #[test]
    fn test_wipe_bottom_left_to_top_right() {
        let mask = Wipe::new(WipeDirection::BottomLeftToTopRight, false);
        assert!(mask.is_visible(0, 9, 10, 10, 0.5));
        assert!(mask.is_visible(3, 5, 10, 10, 0.5));
        assert!(!mask.is_visible(9, 0, 10, 10, 0.5));
    }

    #[test]
    fn test_diagonal_wipe_at_extremes() {
        let mask = Wipe::new(WipeDirection::TopLeftToBottomRight, false);
        assert!(!mask.is_visible(0, 0, 10, 10, 0.0));
        assert!(mask.is_visible(0, 0, 10, 10, 1.0));
        assert!(mask.is_visible(9, 9, 10, 10, 1.0));
    }

    #[test]
    fn test_horizontal_center_out() {
        let mask = Wipe::new(WipeDirection::HorizontalCenterOut, false);
        // At progress 0, nothing visible
        assert!(!mask.is_visible(0, 0, 10, 10, 0.0));
        assert!(!mask.is_visible(5, 0, 10, 10, 0.0));
        // At progress 0.5, center columns visible, edges not
        assert!(mask.is_visible(4, 0, 10, 10, 0.5)); // Near center
        assert!(mask.is_visible(5, 0, 10, 10, 0.5)); // Near center
        assert!(!mask.is_visible(0, 0, 10, 10, 0.5)); // Left edge
        assert!(!mask.is_visible(9, 0, 10, 10, 0.5)); // Right edge
        // At progress 1.0, everything visible
        assert!(mask.is_visible(0, 0, 10, 10, 1.0));
        assert!(mask.is_visible(9, 0, 10, 10, 1.0));
    }

    #[test]
    fn test_vertical_center_out() {
        let mask = Wipe::new(WipeDirection::VerticalCenterOut, false);
        // At progress 0, nothing visible
        assert!(!mask.is_visible(0, 0, 10, 10, 0.0));
        assert!(!mask.is_visible(0, 5, 10, 10, 0.0));
        // At progress 0.5, center rows visible, edges not
        assert!(mask.is_visible(0, 4, 10, 10, 0.5)); // Near center
        assert!(mask.is_visible(0, 5, 10, 10, 0.5)); // Near center
        assert!(!mask.is_visible(0, 0, 10, 10, 0.5)); // Top edge
        assert!(!mask.is_visible(0, 9, 10, 10, 0.5)); // Bottom edge
        // At progress 1.0, everything visible
        assert!(mask.is_visible(0, 0, 10, 10, 1.0));
        assert!(mask.is_visible(0, 9, 10, 10, 1.0));
    }

    #[test]
    fn test_horizontal_edges_in() {
        let mask = Wipe::new(WipeDirection::HorizontalEdgesIn, false);
        // At progress 0, nothing visible
        assert!(!mask.is_visible(0, 0, 10, 10, 0.0));
        assert!(!mask.is_visible(5, 0, 10, 10, 0.0));
        // At progress 0.5, edges visible, center not
        assert!(mask.is_visible(0, 0, 10, 10, 0.5)); // Left edge
        assert!(mask.is_visible(9, 0, 10, 10, 0.5)); // Right edge
        assert!(!mask.is_visible(4, 0, 10, 10, 0.5)); // Near center
        assert!(!mask.is_visible(5, 0, 10, 10, 0.5)); // Near center
        // At progress 1.0, everything visible
        assert!(mask.is_visible(0, 0, 10, 10, 1.0));
        assert!(mask.is_visible(5, 0, 10, 10, 1.0));
    }

    #[test]
    fn test_vertical_edges_in() {
        let mask = Wipe::new(WipeDirection::VerticalEdgesIn, false);
        // At progress 0, nothing visible
        assert!(!mask.is_visible(0, 0, 10, 10, 0.0));
        assert!(!mask.is_visible(0, 5, 10, 10, 0.0));
        // At progress 0.5, edges visible, center not
        assert!(mask.is_visible(0, 0, 10, 10, 0.5)); // Top edge
        assert!(mask.is_visible(0, 9, 10, 10, 0.5)); // Bottom edge
        assert!(!mask.is_visible(0, 4, 10, 10, 0.5)); // Near center
        assert!(!mask.is_visible(0, 5, 10, 10, 0.5)); // Near center
        // At progress 1.0, everything visible
        assert!(mask.is_visible(0, 0, 10, 10, 1.0));
        assert!(mask.is_visible(0, 5, 10, 10, 1.0));
    }

    #[test]
    fn test_wipe_hide_left_to_right() {
        // Normal reveal at t=0.5: left side visible, right hidden
        let reveal = Wipe::new(WipeDirection::LeftToRight, false);
        assert!(reveal.is_visible(0, 0, 10, 10, 0.5)); // Left visible
        assert!(!reveal.is_visible(9, 0, 10, 10, 0.5)); // Right hidden

        // Hide at t=0.5 (mid-exit): effective_progress = 0.5, then output inverted
        // Left side: base_visible=true (position < threshold), inverted=false → HIDDEN
        // Right side: base_visible=false (position >= threshold), inverted=true → VISIBLE
        let hide = Wipe::new_with_invert(WipeDirection::LeftToRight, false, true);
        assert!(!hide.is_visible(0, 0, 10, 10, 0.5)); // Left HIDDEN (wipe has passed)
        assert!(hide.is_visible(9, 0, 10, 10, 0.5)); // Right VISIBLE (wipe hasn't reached)
    }

    #[test]
    fn test_wipe_hide_at_extremes() {
        // Hide mask is designed for exit animations where t goes 1→0
        let reveal = Wipe::new(WipeDirection::LeftToRight, false);
        let hide = Wipe::new_with_invert(WipeDirection::LeftToRight, false, true);

        // At t=0: reveal shows nothing
        assert!(!reveal.is_visible(5, 5, 10, 10, 0.0));
        // At t=0 (exit end): effective_progress=1, threshold=size, base_visible=true, inverted=false
        // Result: NOTHING visible (exit complete, content fully hidden)
        assert!(!hide.is_visible(5, 5, 10, 10, 0.0));

        // At t=1: reveal shows everything
        assert!(reveal.is_visible(5, 5, 10, 10, 1.0));
        // At t=1 (exit start): effective_progress=0, threshold=0, base_visible=false, inverted=true
        // Result: EVERYTHING visible (exit hasn't started)
        assert!(hide.is_visible(5, 5, 10, 10, 1.0));
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
            hide_soft.is_visible(0, 0, 100, 10, 1.0),
            "Leftmost pixel should be visible at exit start"
        );
        assert!(
            hide_soft.is_visible(99, 0, 100, 10, 1.0),
            "Rightmost pixel should be visible at exit start"
        );

        // At t=0.0 (exit end): EVERYTHING should be hidden
        assert!(
            !hide_soft.is_visible(0, 0, 100, 10, 0.0),
            "Leftmost pixel should be hidden at exit end"
        );
        assert!(
            !hide_soft.is_visible(99, 0, 100, 10, 0.0),
            "Rightmost pixel should be hidden at exit end"
        );

        // At t=0.5 (mid-exit): approximately half should be hidden
        // With soft edge, the boundary should have a trailing visible edge
        assert!(
            !hide_soft.is_visible(0, 0, 100, 10, 0.5),
            "Leftmost pixel should be hidden at mid-exit"
        );
        assert!(
            hide_soft.is_visible(99, 0, 100, 10, 0.5),
            "Rightmost pixel should be visible at mid-exit"
        );
    }

    #[test]
    fn test_wipe_hide_diagonal() {
        let reveal = Wipe::new(WipeDirection::TopLeftToBottomRight, false);
        let hide = Wipe::new_with_invert(WipeDirection::TopLeftToBottomRight, false, true);

        // At t=0.5: reveal shows top-left, hides bottom-right
        assert!(reveal.is_visible(0, 0, 10, 10, 0.5));
        assert!(!reveal.is_visible(9, 9, 10, 10, 0.5));

        // At t=0.5: hide has hidden top-left, bottom-right still visible
        assert!(!hide.is_visible(0, 0, 10, 10, 0.5)); // Top-left HIDDEN
        assert!(hide.is_visible(9, 9, 10, 10, 0.5)); // Bottom-right VISIBLE
    }
}

// <FILE>tui-vfx-compositor/src/masks/cls_wipe.rs</FILE>
// <DESC>Linear wipe mask with cardinal, diagonal, and center-out directions</DESC>
// <VERS>END OF VERSION: 1.9.0</VERS>
