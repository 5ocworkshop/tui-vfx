// <FILE>tui-vfx-geometry/src/types/cls_wipe_direction.rs</FILE> - <DESC>Canonical wipe-direction vocabulary shared by mask, shader, and grouped V3 surfaces</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Capability audit recommendation 1.2 + 1.3 — single source of truth for wipe direction so the Wipe mask and the RevealWipe shader share one enum, and so the engine can carry the corner-out / corner-in directions that authors have asked for. Replaces the previous duplicated WipeDirection (mask) and RevealDirection (shader) enums.</WCTX>
// <CLOG>1.0.0: initial canonical WipeDirection enum living in tui-vfx-geometry. 16 cardinal/diagonal/centre/edge variants ported losslessly from the previous tui-vfx-compositor::types::WipeDirection. 8 new corner-out and corner-in variants added (CornerOutFromTopLeft, CornerOutFromTopRight, CornerOutFromBottomLeft, CornerOutFromBottomRight, CornerInToTopLeft, CornerInToTopRight, CornerInToBottomLeft, CornerInToBottomRight). Serde aliases include `corner_down_*` for top-corner outward forms (visual sense: wipe progresses downward from a top corner) and `corner_up_*` for bottom-corner outward forms (wipe progresses upward from a bottom corner) so author vocabulary maps either way.</CLOG>

//! # Canonical wipe-direction vocabulary
//!
//! `WipeDirection` is the single source of truth for "which direction does a
//! wipe / reveal travel?" across the engine. Both the [`Wipe`](
//! https://docs.rs/tui-vfx-compositor) mask and the
//! [`RevealWipeShader`](https://docs.rs/tui-vfx-style) consume it via
//! [`crate::wipe_geometry::wipe_progress`].
//!
//! ## Direction families
//!
//! - **Cardinal** — `LeftToRight`, `RightToLeft`, `TopToBottom`, `BottomToTop`.
//!   The wipe sweeps along one axis of the rect.
//! - **Cardinal source aliases** — `FromLeft`, `FromRight`, `FromTop`,
//!   `FromBottom`. Read more naturally in some authoring contexts. Identical
//!   semantics to the cardinals.
//! - **Diagonal** — `TopLeftToBottomRight`, `TopRightToBottomLeft`,
//!   `BottomLeftToTopRight`, `BottomRightToTopLeft`. The wipe sweeps along
//!   a corner-to-corner Manhattan diagonal.
//! - **Centre-out / edges-in** — `HorizontalCenterOut`, `VerticalCenterOut`,
//!   `HorizontalEdgesIn`, `VerticalEdgesIn`. Curtain-style barn-door
//!   reveals.
//! - **Corner-out** *(new in 1.0.0)* — `CornerOutFromTopLeft`,
//!   `CornerOutFromTopRight`, `CornerOutFromBottomLeft`,
//!   `CornerOutFromBottomRight`. The wipe expands as a quadrant arc rooted
//!   at the named corner. Authoring aliases include `corner_down_top_left`
//!   etc. for "wipe comes down from top-corner" intent.
//! - **Corner-in** *(new in 1.0.0)* — `CornerInToTopLeft`,
//!   `CornerInToTopRight`, `CornerInToBottomLeft`, `CornerInToBottomRight`.
//!   The inverse: a quadrant arc collapses toward the named corner.
//!
//! ## Difference from corner-to-corner diagonals
//!
//! The pre-1.0 corner-anchored variants only existed as Manhattan diagonals
//! (`TopLeftToBottomRight` etc.), where every cell whose `x + y` lies
//! ≤ `progress * (max_x + max_y)` is revealed. That sweep is straight-line:
//! at `progress = 0.5` the boundary is one diagonal line.
//!
//! `CornerOutFromTopLeft` instead uses Euclidean distance from the corner —
//! at `progress = 0.5`, every cell within ~half the diagonal *radius* is
//! revealed, producing a quarter-circle wavefront. Visually this is the
//! "expanding quadrant" reveal, not the slanted diagonal sweep.
//!
//! Both are useful and intentionally distinct.

use serde::{Deserialize, Serialize};

/// Direction for a wipe-style reveal/hide animation.
///
/// See module docs for the full vocabulary and how the variants differ.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum WipeDirection {
    // ─────────────────────────────────────────────────────────────────
    // Cardinal directions
    // ─────────────────────────────────────────────────────────────────
    /// Wipe from left edge to right edge.
    ///
    /// The most common wipe direction, matching natural reading order
    /// in left-to-right languages.
    #[default]
    LeftToRight,

    /// Wipe from right edge to left edge.
    RightToLeft,

    /// Wipe from top edge to bottom edge.
    TopToBottom,

    /// Wipe from bottom edge to top edge.
    BottomToTop,

    // ─────────────────────────────────────────────────────────────────
    // Diagonal (corner-to-corner Manhattan) directions
    // ─────────────────────────────────────────────────────────────────
    /// Diagonal Manhattan-distance wipe from top-left corner to
    /// bottom-right corner. Wavefront is a slanted line.
    TopLeftToBottomRight,

    /// Diagonal Manhattan-distance wipe from top-right to bottom-left.
    TopRightToBottomLeft,

    /// Diagonal Manhattan-distance wipe from bottom-left to top-right.
    BottomLeftToTopRight,

    /// Diagonal Manhattan-distance wipe from bottom-right to top-left.
    BottomRightToTopLeft,

    // ─────────────────────────────────────────────────────────────────
    // Source-based aliases (cardinal convenience)
    // ─────────────────────────────────────────────────────────────────
    /// Alias for [`LeftToRight`](Self::LeftToRight).
    #[serde(alias = "FromLeft")]
    FromLeft,

    /// Alias for [`RightToLeft`](Self::RightToLeft).
    #[serde(alias = "FromRight")]
    FromRight,

    /// Alias for [`TopToBottom`](Self::TopToBottom).
    #[serde(alias = "FromTop")]
    FromTop,

    /// Alias for [`BottomToTop`](Self::BottomToTop).
    #[serde(alias = "FromBottom")]
    FromBottom,

    // ─────────────────────────────────────────────────────────────────
    // Centre-out / edges-in (barn-door)
    // ─────────────────────────────────────────────────────────────────
    /// Horizontal wipe from centre column outward to left and right edges.
    /// Curtains opening horizontally.
    #[serde(alias = "center_out_horizontal", alias = "barn_door_horizontal")]
    HorizontalCenterOut,

    /// Vertical wipe from centre row outward to top and bottom edges.
    /// Curtains opening vertically.
    #[serde(alias = "center_out_vertical", alias = "barn_door_vertical")]
    VerticalCenterOut,

    /// Horizontal wipe from left and right edges inward to centre column.
    /// Curtains closing horizontally.
    #[serde(alias = "edges_in_horizontal", alias = "barn_door_close_horizontal")]
    HorizontalEdgesIn,

    /// Vertical wipe from top and bottom edges inward to centre row.
    /// Curtains closing vertically.
    #[serde(alias = "edges_in_vertical", alias = "barn_door_close_vertical")]
    VerticalEdgesIn,

    // ─────────────────────────────────────────────────────────────────
    // Corner-out (Euclidean quadrant arc expanding from a corner)
    //
    // `corner_down_*` aliases read as "the reveal comes down from the
    // top-corner anchor"; `corner_up_*` aliases read as "the reveal comes
    // up from the bottom-corner anchor". Both are author-facing language;
    // the underlying math is identical to the canonical corner-out form.
    // ─────────────────────────────────────────────────────────────────
    /// Quadrant arc expanding from the top-left corner.
    ///
    /// Every cell whose Euclidean distance from `(0, 0)` is below
    /// `progress * sqrt(w² + h²)` is revealed. The wavefront is a
    /// quarter-circle radiating outward. Distinct from
    /// [`TopLeftToBottomRight`](Self::TopLeftToBottomRight), which is a
    /// straight Manhattan diagonal.
    #[serde(alias = "corner_down_top_left")]
    CornerOutFromTopLeft,

    /// Quadrant arc expanding from the top-right corner.
    #[serde(alias = "corner_down_top_right")]
    CornerOutFromTopRight,

    /// Quadrant arc expanding from the bottom-left corner.
    #[serde(alias = "corner_up_bottom_left")]
    CornerOutFromBottomLeft,

    /// Quadrant arc expanding from the bottom-right corner.
    #[serde(alias = "corner_up_bottom_right")]
    CornerOutFromBottomRight,

    // ─────────────────────────────────────────────────────────────────
    // Corner-in (Euclidean quadrant arc collapsing toward a corner)
    // ─────────────────────────────────────────────────────────────────
    /// Quadrant arc collapsing toward the top-left corner. Inverse of
    /// [`CornerOutFromBottomRight`](Self::CornerOutFromBottomRight).
    CornerInToTopLeft,

    /// Quadrant arc collapsing toward the top-right corner.
    CornerInToTopRight,

    /// Quadrant arc collapsing toward the bottom-left corner.
    CornerInToBottomLeft,

    /// Quadrant arc collapsing toward the bottom-right corner.
    CornerInToBottomRight,
}

impl WipeDirection {
    /// Returns `true` if this variant is one of the four cardinal directions
    /// or a cardinal source alias (`FromLeft` etc.). Useful for code paths
    /// that want to special-case "single-axis sweep" vs. "richer geometry".
    pub fn is_cardinal(self) -> bool {
        matches!(
            self,
            Self::LeftToRight
                | Self::RightToLeft
                | Self::TopToBottom
                | Self::BottomToTop
                | Self::FromLeft
                | Self::FromRight
                | Self::FromTop
                | Self::FromBottom
        )
    }

    /// Returns `true` for the corner-out and corner-in quadrant-arc
    /// variants added in 1.0.0. These are distinct from corner-to-corner
    /// Manhattan diagonals; see module docs.
    pub fn is_corner_arc(self) -> bool {
        matches!(
            self,
            Self::CornerOutFromTopLeft
                | Self::CornerOutFromTopRight
                | Self::CornerOutFromBottomLeft
                | Self::CornerOutFromBottomRight
                | Self::CornerInToTopLeft
                | Self::CornerInToTopRight
                | Self::CornerInToBottomLeft
                | Self::CornerInToBottomRight
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_left_to_right() {
        assert_eq!(WipeDirection::default(), WipeDirection::LeftToRight);
    }

    #[test]
    fn serde_uses_snake_case() {
        let d = WipeDirection::HorizontalCenterOut;
        let json = serde_json::to_string(&d).unwrap();
        assert_eq!(json, r#""horizontal_center_out""#);
    }

    #[test]
    fn deserializes_cardinal_aliases() {
        let parsed: WipeDirection = serde_json::from_str(r#""FromLeft""#).unwrap();
        assert_eq!(parsed, WipeDirection::FromLeft);
    }

    #[test]
    fn deserializes_barn_door_aliases() {
        let a: WipeDirection = serde_json::from_str(r#""barn_door_horizontal""#).unwrap();
        assert_eq!(a, WipeDirection::HorizontalCenterOut);
        let b: WipeDirection = serde_json::from_str(r#""center_out_vertical""#).unwrap();
        assert_eq!(b, WipeDirection::VerticalCenterOut);
        let c: WipeDirection = serde_json::from_str(r#""edges_in_horizontal""#).unwrap();
        assert_eq!(c, WipeDirection::HorizontalEdgesIn);
    }

    #[test]
    fn deserializes_corner_down_and_corner_up_aliases() {
        let tl: WipeDirection = serde_json::from_str(r#""corner_down_top_left""#).unwrap();
        assert_eq!(tl, WipeDirection::CornerOutFromTopLeft);
        let tr: WipeDirection = serde_json::from_str(r#""corner_down_top_right""#).unwrap();
        assert_eq!(tr, WipeDirection::CornerOutFromTopRight);
        let bl: WipeDirection = serde_json::from_str(r#""corner_up_bottom_left""#).unwrap();
        assert_eq!(bl, WipeDirection::CornerOutFromBottomLeft);
        let br: WipeDirection = serde_json::from_str(r#""corner_up_bottom_right""#).unwrap();
        assert_eq!(br, WipeDirection::CornerOutFromBottomRight);
    }

    #[test]
    fn corner_arc_classifier_partitions_correctly() {
        for d in [
            WipeDirection::CornerOutFromTopLeft,
            WipeDirection::CornerOutFromTopRight,
            WipeDirection::CornerOutFromBottomLeft,
            WipeDirection::CornerOutFromBottomRight,
            WipeDirection::CornerInToTopLeft,
            WipeDirection::CornerInToTopRight,
            WipeDirection::CornerInToBottomLeft,
            WipeDirection::CornerInToBottomRight,
        ] {
            assert!(d.is_corner_arc(), "{d:?} should be a corner arc");
            assert!(!d.is_cardinal(), "{d:?} should not be cardinal");
        }
    }

    #[test]
    fn cardinal_classifier_covers_all_aliases() {
        for d in [
            WipeDirection::LeftToRight,
            WipeDirection::RightToLeft,
            WipeDirection::TopToBottom,
            WipeDirection::BottomToTop,
            WipeDirection::FromLeft,
            WipeDirection::FromRight,
            WipeDirection::FromTop,
            WipeDirection::FromBottom,
        ] {
            assert!(d.is_cardinal(), "{d:?} should be cardinal");
            assert!(!d.is_corner_arc(), "{d:?} should not be corner arc");
        }
    }

    #[test]
    fn diagonal_and_center_variants_are_neither_cardinal_nor_corner_arc() {
        for d in [
            WipeDirection::TopLeftToBottomRight,
            WipeDirection::HorizontalCenterOut,
            WipeDirection::VerticalEdgesIn,
        ] {
            assert!(!d.is_cardinal());
            assert!(!d.is_corner_arc());
        }
    }
}

// <FILE>tui-vfx-geometry/src/types/cls_wipe_direction.rs</FILE> - <DESC>Canonical wipe-direction vocabulary</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
