// <FILE>tui-vfx-geometry/src/wipe/fnc_wipe_progress.rs</FILE> - <DESC>Per-cell visibility math for every WipeDirection variant; consumed by the Wipe mask and the RevealWipe shader</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Audit recommendation 1.2 + 1.3 — share the position/size and corner-arc geometry between the mask and shader surfaces. Previously duplicated inside cls_wipe.rs and (in cardinal-only form) inside cls_reveal_wipe_shader.rs.</WCTX>
// <CLOG>1.0.0: introduce wipe_progress and wipe_visible_at. wipe_progress returns the (position, size) pair for cardinal/diagonal/center/edge variants — this preserves the legacy mask-side return shape so existing soft-edge and hide-mask logic in tui-vfx-compositor continues to compose. wipe_visible_at is the new high-level predicate that handles every variant including the new corner-arc variants (where position/size aren't a meaningful 1D pair).</CLOG>

//! # Wipe-progress geometry
//!
//! Two functions:
//!
//! - [`wipe_progress`] returns the legacy `(position, size)` pair for the
//!   single-axis variants (cardinal, diagonal, centre-out, edges-in). This
//!   matches what the `Wipe` mask uses to feed its existing soft-edge and
//!   invert-for-hide logic. For the new corner-arc variants it returns
//!   `None` because they don't fit a 1D position/size model.
//!
//! - [`wipe_visible_at`] is the high-level "is this cell visible at this
//!   progress?" predicate that handles every variant including the new
//!   corner-arc forms. Both the shader and mask should generally call
//!   this; the mask falls back to its own progress/soft-edge logic for
//!   variants where it needs the (position, size) pair.

use super::WipeDirection;

/// Return the `(position, size)` pair for a cardinal/diagonal/centre/edge
/// wipe direction — the same shape the legacy `Wipe` mask geometry has
/// always returned. Returns `None` for the corner-arc variants
/// (`CornerOutFrom*`, `CornerInTo*`) because they're radial and don't fit
/// a single-axis 1D model; callers should use [`wipe_visible_at`] for
/// those.
///
/// `position < threshold` (where `threshold = size * progress`) means the
/// cell is on the revealed side of the wavefront.
pub fn wipe_progress(
    direction: WipeDirection,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
) -> Option<(f32, f32)> {
    use WipeDirection as D;
    let max_x = w.saturating_sub(1);
    let max_y = h.saturating_sub(1);
    match direction {
        D::LeftToRight | D::FromLeft => Some((x as f32, w as f32)),
        D::RightToLeft | D::FromRight => Some((max_x.saturating_sub(x) as f32, w as f32)),
        D::TopToBottom | D::FromTop => Some((y as f32, h as f32)),
        D::BottomToTop | D::FromBottom => Some((max_y.saturating_sub(y) as f32, h as f32)),
        D::TopLeftToBottomRight => {
            let max_dist = max_x + max_y;
            Some(((x + y) as f32, (max_dist + 1) as f32))
        }
        D::BottomRightToTopLeft => {
            let max_dist = max_x + max_y;
            Some((
                (max_x.saturating_sub(x) + max_y.saturating_sub(y)) as f32,
                (max_dist + 1) as f32,
            ))
        }
        D::TopRightToBottomLeft => {
            let max_dist = max_x + max_y;
            Some((
                (max_x.saturating_sub(x) + y) as f32,
                (max_dist + 1) as f32,
            ))
        }
        D::BottomLeftToTopRight => {
            let max_dist = max_x + max_y;
            Some((
                (x + max_y.saturating_sub(y)) as f32,
                (max_dist + 1) as f32,
            ))
        }
        D::HorizontalCenterOut => {
            let center = (w as f32 - 1.0) / 2.0;
            let dist_from_center = (x as f32 - center).abs();
            let half_width = center.max(w as f32 - 1.0 - center);
            Some((dist_from_center, half_width + 1.0))
        }
        D::VerticalCenterOut => {
            let center = (h as f32 - 1.0) / 2.0;
            let dist_from_center = (y as f32 - center).abs();
            let half_height = center.max(h as f32 - 1.0 - center);
            Some((dist_from_center, half_height + 1.0))
        }
        D::HorizontalEdgesIn => {
            let dist_left = x as f32;
            let dist_right = max_x.saturating_sub(x) as f32;
            let dist_edge = dist_left.min(dist_right);
            let half_width = (w as f32 - 1.0) / 2.0;
            Some((dist_edge, half_width + 1.0))
        }
        D::VerticalEdgesIn => {
            let dist_top = y as f32;
            let dist_bottom = max_y.saturating_sub(y) as f32;
            let dist_edge = dist_top.min(dist_bottom);
            let half_height = (h as f32 - 1.0) / 2.0;
            Some((dist_edge, half_height + 1.0))
        }
        D::CornerOutFromTopLeft
        | D::CornerOutFromTopRight
        | D::CornerOutFromBottomLeft
        | D::CornerOutFromBottomRight
        | D::CornerInToTopLeft
        | D::CornerInToTopRight
        | D::CornerInToBottomLeft
        | D::CornerInToBottomRight => None,
    }
}

/// Per-cell visibility predicate for every [`WipeDirection`] variant,
/// including corner-arc forms.
///
/// `progress` is the wipe progress in `[0, 1]`. When `progress <= 0` no
/// cell is visible; when `progress >= 1` every cell is visible.
///
/// For cardinal/diagonal/centre/edge variants this is equivalent to
/// computing `wipe_progress(...)`'s `(position, size)` and checking
/// `position < size * progress` — the function dispatches internally so
/// callers don't have to special-case the corner variants.
pub fn wipe_visible_at(
    direction: WipeDirection,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    progress: f32,
) -> bool {
    if progress <= 0.0 {
        return false;
    }
    if progress >= 1.0 {
        return true;
    }
    if let Some((position, size)) = wipe_progress(direction, x, y, w, h) {
        if size <= 0.0 {
            return false;
        }
        return position < size * progress;
    }
    corner_arc_visible(direction, x, y, w, h, progress)
}

fn corner_arc_visible(
    direction: WipeDirection,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    progress: f32,
) -> bool {
    use WipeDirection as D;
    let max_x = w.saturating_sub(1) as f32;
    let max_y = h.saturating_sub(1) as f32;
    let x = x as f32;
    let y = y as f32;
    // Diagonal of the enclosing rect — the maximum Euclidean distance
    // any cell can be from any of the four corners. This is the "radius"
    // we sweep through over the course of the wipe.
    let max_radius = (max_x.powi(2) + max_y.powi(2)).sqrt().max(1.0);
    let threshold = max_radius * progress;
    let dist_from = |cx: f32, cy: f32| ((x - cx).powi(2) + (y - cy).powi(2)).sqrt();
    match direction {
        D::CornerOutFromTopLeft => dist_from(0.0, 0.0) < threshold,
        D::CornerOutFromTopRight => dist_from(max_x, 0.0) < threshold,
        D::CornerOutFromBottomLeft => dist_from(0.0, max_y) < threshold,
        D::CornerOutFromBottomRight => dist_from(max_x, max_y) < threshold,
        D::CornerInToTopLeft => dist_from(0.0, 0.0) > max_radius - threshold,
        D::CornerInToTopRight => dist_from(max_x, 0.0) > max_radius - threshold,
        D::CornerInToBottomLeft => dist_from(0.0, max_y) > max_radius - threshold,
        D::CornerInToBottomRight => dist_from(max_x, max_y) > max_radius - threshold,
        _ => unreachable!("corner_arc_visible called with non-corner direction {direction:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use WipeDirection as D;

    // ─────────────────────────────────────────────────────────────────
    // Boundary semantics: progress = 0 hides everything;
    // progress >= 1 reveals everything.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn progress_zero_hides_all_cells_for_every_direction() {
        let directions = [
            D::LeftToRight,
            D::RightToLeft,
            D::TopToBottom,
            D::BottomToTop,
            D::TopLeftToBottomRight,
            D::TopRightToBottomLeft,
            D::BottomLeftToTopRight,
            D::BottomRightToTopLeft,
            D::HorizontalCenterOut,
            D::VerticalCenterOut,
            D::HorizontalEdgesIn,
            D::VerticalEdgesIn,
            D::CornerOutFromTopLeft,
            D::CornerOutFromTopRight,
            D::CornerOutFromBottomLeft,
            D::CornerOutFromBottomRight,
            D::CornerInToTopLeft,
            D::CornerInToTopRight,
            D::CornerInToBottomLeft,
            D::CornerInToBottomRight,
        ];
        for d in directions {
            for y in 0..10_u16 {
                for x in 0..10_u16 {
                    assert!(
                        !wipe_visible_at(d, x, y, 10, 10, 0.0),
                        "{d:?} ({x},{y}) should be hidden at progress=0"
                    );
                }
            }
        }
    }

    #[test]
    fn progress_one_reveals_all_cells_for_every_direction() {
        let directions = [
            D::LeftToRight,
            D::RightToLeft,
            D::TopToBottom,
            D::BottomToTop,
            D::TopLeftToBottomRight,
            D::TopRightToBottomLeft,
            D::BottomLeftToTopRight,
            D::BottomRightToTopLeft,
            D::HorizontalCenterOut,
            D::VerticalCenterOut,
            D::HorizontalEdgesIn,
            D::VerticalEdgesIn,
            D::CornerOutFromTopLeft,
            D::CornerOutFromTopRight,
            D::CornerOutFromBottomLeft,
            D::CornerOutFromBottomRight,
            D::CornerInToTopLeft,
            D::CornerInToTopRight,
            D::CornerInToBottomLeft,
            D::CornerInToBottomRight,
        ];
        for d in directions {
            for y in 0..10_u16 {
                for x in 0..10_u16 {
                    assert!(
                        wipe_visible_at(d, x, y, 10, 10, 1.0),
                        "{d:?} ({x},{y}) should be visible at progress=1"
                    );
                }
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Cardinal: behavioural parity with the legacy `Wipe` mask.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn left_to_right_at_half_progress() {
        // 10-wide → threshold = 5; cells with x < 5 visible.
        for y in 0..10 {
            assert!(wipe_visible_at(D::LeftToRight, 0, y, 10, 10, 0.5));
            assert!(wipe_visible_at(D::LeftToRight, 4, y, 10, 10, 0.5));
            assert!(!wipe_visible_at(D::LeftToRight, 5, y, 10, 10, 0.5));
            assert!(!wipe_visible_at(D::LeftToRight, 9, y, 10, 10, 0.5));
        }
    }

    #[test]
    fn right_to_left_at_half_progress() {
        // RightToLeft: position = max_x - x, so cells with (max_x - x) < 5
        // i.e. x > 4 are visible.
        for y in 0..10 {
            assert!(!wipe_visible_at(D::RightToLeft, 0, y, 10, 10, 0.5));
            assert!(!wipe_visible_at(D::RightToLeft, 4, y, 10, 10, 0.5));
            assert!(wipe_visible_at(D::RightToLeft, 5, y, 10, 10, 0.5));
            assert!(wipe_visible_at(D::RightToLeft, 9, y, 10, 10, 0.5));
        }
    }

    #[test]
    fn top_to_bottom_at_half_progress() {
        for x in 0..10 {
            assert!(wipe_visible_at(D::TopToBottom, x, 0, 10, 10, 0.5));
            assert!(wipe_visible_at(D::TopToBottom, x, 4, 10, 10, 0.5));
            assert!(!wipe_visible_at(D::TopToBottom, x, 5, 10, 10, 0.5));
        }
    }

    #[test]
    fn from_left_alias_matches_left_to_right() {
        for y in 0..10 {
            for x in 0..10 {
                assert_eq!(
                    wipe_visible_at(D::FromLeft, x, y, 10, 10, 0.5),
                    wipe_visible_at(D::LeftToRight, x, y, 10, 10, 0.5),
                );
            }
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Diagonal: TopLeftToBottomRight reveals the TL triangle first.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn top_left_to_bottom_right_reveals_corner_first() {
        assert!(wipe_visible_at(D::TopLeftToBottomRight, 0, 0, 10, 10, 0.1));
        assert!(!wipe_visible_at(D::TopLeftToBottomRight, 9, 9, 10, 10, 0.1));
    }

    // ─────────────────────────────────────────────────────────────────
    // Centre-out: at progress=0.5, near-centre cells visible; edges hidden.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn horizontal_center_out_at_half_progress() {
        for y in 0..10 {
            assert!(wipe_visible_at(D::HorizontalCenterOut, 4, y, 10, 10, 0.5));
            assert!(wipe_visible_at(D::HorizontalCenterOut, 5, y, 10, 10, 0.5));
            assert!(!wipe_visible_at(D::HorizontalCenterOut, 0, y, 10, 10, 0.5));
            assert!(!wipe_visible_at(D::HorizontalCenterOut, 9, y, 10, 10, 0.5));
        }
    }

    #[test]
    fn vertical_edges_in_at_half_progress() {
        // EdgesIn: edges visible first, centre last.
        for x in 0..10 {
            assert!(wipe_visible_at(D::VerticalEdgesIn, x, 0, 10, 10, 0.5));
            assert!(wipe_visible_at(D::VerticalEdgesIn, x, 9, 10, 10, 0.5));
            assert!(!wipe_visible_at(D::VerticalEdgesIn, x, 4, 10, 10, 0.5));
            assert!(!wipe_visible_at(D::VerticalEdgesIn, x, 5, 10, 10, 0.5));
        }
    }

    // ─────────────────────────────────────────────────────────────────
    // Corner-out: quadrant arc from a corner. At progress=0.3, the
    // anchor corner is revealed but the opposite corner is not.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn corner_out_from_top_left_reveals_top_left_first() {
        assert!(wipe_visible_at(D::CornerOutFromTopLeft, 0, 0, 10, 10, 0.3));
        assert!(!wipe_visible_at(
            D::CornerOutFromTopLeft, 9, 9, 10, 10, 0.3
        ));
    }

    #[test]
    fn corner_out_from_bottom_right_reveals_bottom_right_first() {
        assert!(wipe_visible_at(
            D::CornerOutFromBottomRight, 9, 9, 10, 10, 0.3
        ));
        assert!(!wipe_visible_at(
            D::CornerOutFromBottomRight, 0, 0, 10, 10, 0.3
        ));
    }

    #[test]
    fn corner_arc_distinct_from_diagonal_sweep_at_mid_progress() {
        // At progress = 0.5 a diagonal sweep reveals every cell with
        // (x + y) below the diagonal threshold, including (5, 0) and
        // (0, 5). The quadrant arc reveals cells within a half-radius
        // of TL — (5, 0) and (0, 5) are EXACTLY at radius 5 of (0,0),
        // and the wipe radius at progress=0.5 is sqrt(81+81)/2 ≈ 6.36,
        // so they ARE visible. Push further out to (8, 0) which is
        // beyond the half-radius:
        let diag_visible = wipe_visible_at(D::TopLeftToBottomRight, 8, 0, 10, 10, 0.5);
        let arc_visible = wipe_visible_at(D::CornerOutFromTopLeft, 8, 0, 10, 10, 0.5);
        // Diagonal sweep reveals (8, 0) at progress=0.5 (8 < 0.5 * 18 = 9 ✓)
        // Quadrant arc does NOT reveal (8, 0) at progress=0.5 (8 > 6.36)
        assert!(diag_visible);
        assert!(!arc_visible);
    }

    #[test]
    fn corner_in_inverse_of_corner_out_for_anchor_cell() {
        // CornerInToTopLeft: the TL corner is revealed LAST, not first.
        // At progress=0.3 the TL corner cell (0,0) should still be hidden.
        assert!(!wipe_visible_at(D::CornerInToTopLeft, 0, 0, 10, 10, 0.3));
        // The opposite corner (BR) should be revealed first.
        assert!(wipe_visible_at(D::CornerInToTopLeft, 9, 9, 10, 10, 0.3));
    }

    // ─────────────────────────────────────────────────────────────────
    // wipe_progress for legacy callers: returns the (position, size)
    // pair for non-corner variants, None for corner-arc variants.
    // ─────────────────────────────────────────────────────────────────
    #[test]
    fn wipe_progress_returns_some_for_non_corner_variants() {
        for d in [
            D::LeftToRight,
            D::TopToBottom,
            D::TopLeftToBottomRight,
            D::HorizontalCenterOut,
            D::VerticalEdgesIn,
        ] {
            assert!(wipe_progress(d, 5, 5, 10, 10).is_some(), "{d:?}");
        }
    }

    #[test]
    fn wipe_progress_returns_none_for_corner_variants() {
        for d in [
            D::CornerOutFromTopLeft,
            D::CornerOutFromTopRight,
            D::CornerOutFromBottomLeft,
            D::CornerOutFromBottomRight,
            D::CornerInToTopLeft,
            D::CornerInToTopRight,
            D::CornerInToBottomLeft,
            D::CornerInToBottomRight,
        ] {
            assert!(wipe_progress(d, 5, 5, 10, 10).is_none(), "{d:?}");
        }
    }
}

// <FILE>tui-vfx-geometry/src/wipe/fnc_wipe_progress.rs</FILE> - <DESC>Wipe-progress visibility math</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
