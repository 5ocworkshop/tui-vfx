// <FILE>tui-vfx-style/src/models/cls_reveal_wipe_shader.rs</FILE> - <DESC>RevealWipe shader: progressively reveals text along a chosen wipe direction</DESC>
// <VERS>VERSION: 2.1.0</VERS>
// <WCTX>v3.1 native debug-recipes closure: preserve reveal-wipe foreground color in native compositor lowering.</WCTX>
// <CLOG>2.1.0: add optional color for revealed cells while preserving the previous color-preserving default.
// 2.0.0: MAJOR — RevealDirection becomes a re-export of tui_vfx_geometry::WipeDirection (full 20-variant vocabulary including diagonal sweeps, centre-out, edges-in, and corner-out / corner-in arcs). style_at delegates to tui_vfx_geometry::wipe_visible_at, so the shader and the Wipe mask share the same per-cell visibility math. Existing recipes that say `direction: "left_to_right"` etc. continue to parse and behave identically. New directions are immediately available via the same field.
// 1.0.0: Initial implementation with 4 cardinal directions</CLOG>

use crate::{
    models::ColorConfig,
    traits::{ShaderContext, StyleShader},
};
use serde::{Deserialize, Serialize};
use tui_vfx_geometry::{WipeDirection, wipe_visible_at};
use tui_vfx_types::Style;

/// Type alias for the canonical wipe-direction vocabulary.
///
/// The shader historically defined its own four-cardinal `RevealDirection`
/// enum. As of 2.0.0 it re-uses [`tui_vfx_geometry::WipeDirection`] so the
/// shader, the `Wipe` mask in `tui-vfx-compositor`, and the V3 grouped
/// reveal family all share one direction vocabulary — including the
/// cardinal, diagonal, centre-out, edges-in, and corner-out / corner-in
/// arc variants.
///
/// Existing JSON recipes that author `"direction": "left_to_right"`
/// continue to work unchanged; the new variants (e.g.
/// `"horizontal_center_out"`, `"corner_down_top_left"`) are now
/// available at the shader layer too.
pub type RevealDirection = WipeDirection;

/// RevealWipe shader: progressively reveals text by hiding unrevealed cells.
///
/// Unlike `Highlighter` which changes colours in the revealed area,
/// `RevealWipe` hides unrevealed cells by setting their foreground to
/// match the background, making text invisible until the wipe passes.
///
/// This preserves the original styling of revealed content — perfect for
/// "draw-in" effects where text appears progressively. The full wipe
/// direction vocabulary lives in [`RevealDirection`] (= `WipeDirection`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct RevealWipeShader {
    /// Direction of the reveal animation. See [`RevealDirection`] for the
    /// full vocabulary; defaults to `LeftToRight`.
    #[serde(default)]
    pub direction: RevealDirection,
    /// Optional foreground color applied to revealed cells.
    ///
    /// When absent, revealed cells preserve their incoming foreground color.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<ColorConfig>,
}

impl Default for RevealWipeShader {
    fn default() -> Self {
        Self {
            direction: RevealDirection::LeftToRight,
            color: None,
        }
    }
}

impl StyleShader for RevealWipeShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        // Delegate to the shared wipe visibility helper. ctx.t in [0, 1]
        // is the wipe progress; for cells where the wavefront has passed,
        // we keep the base style (revealed); for unrevealed cells we set
        // fg = bg so the text is invisible against its own background.
        let progress = (ctx.t as f32).clamp(0.0, 1.0);
        if wipe_visible_at(
            self.direction,
            ctx.local_x,
            ctx.local_y,
            ctx.width,
            ctx.height,
            progress,
        ) {
            let mut style = base;
            if let Some(color) = self.color {
                style.fg = color.into();
            }
            style
        } else {
            let mut style = base;
            style.fg = base.bg;
            style
        }
    }

    fn name(&self) -> &'static str {
        "RevealWipe"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test_support::{make_ctx_at, make_style};

    fn revealed(shader: &RevealWipeShader, x: u16, y: u16, w: u16, h: u16, t: f64) -> bool {
        let ctx = make_ctx_at(x, y, w, h, t);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        // "Revealed" means the shader returned the base unchanged
        // (i.e. fg was not collapsed to bg).
        result.fg == base.fg
    }

    #[test]
    fn left_to_right_at_half_progress_reveals_left_half() {
        let shader = RevealWipeShader {
            direction: RevealDirection::LeftToRight,
            color: None,
        };
        assert!(revealed(&shader, 0, 0, 10, 1, 0.5));
        assert!(revealed(&shader, 4, 0, 10, 1, 0.5));
        assert!(!revealed(&shader, 5, 0, 10, 1, 0.5));
        assert!(!revealed(&shader, 9, 0, 10, 1, 0.5));
    }

    #[test]
    fn horizontal_center_out_at_half_progress_reveals_centre() {
        let shader = RevealWipeShader {
            direction: RevealDirection::HorizontalCenterOut,
            color: None,
        };
        assert!(revealed(&shader, 4, 0, 10, 1, 0.5));
        assert!(revealed(&shader, 5, 0, 10, 1, 0.5));
        assert!(!revealed(&shader, 0, 0, 10, 1, 0.5));
        assert!(!revealed(&shader, 9, 0, 10, 1, 0.5));
    }

    #[test]
    fn vertical_edges_in_at_half_progress_reveals_edges() {
        let shader = RevealWipeShader {
            direction: RevealDirection::VerticalEdgesIn,
            color: None,
        };
        assert!(revealed(&shader, 0, 0, 10, 10, 0.5));
        assert!(revealed(&shader, 0, 9, 10, 10, 0.5));
        assert!(!revealed(&shader, 0, 4, 10, 10, 0.5));
    }

    #[test]
    fn corner_out_from_top_left_reveals_anchor_corner_first() {
        let shader = RevealWipeShader {
            direction: RevealDirection::CornerOutFromTopLeft,
            color: None,
        };
        assert!(revealed(&shader, 0, 0, 10, 10, 0.3));
        assert!(!revealed(&shader, 9, 9, 10, 10, 0.3));
    }

    #[test]
    fn corner_in_to_bottom_right_hides_anchor_corner_first() {
        let shader = RevealWipeShader {
            direction: RevealDirection::CornerInToBottomRight,
            color: None,
        };
        // CornerIn collapses TOWARD the corner — so the anchor corner is
        // the LAST to be revealed. At progress=0.3 the BR corner is still
        // hidden but the opposite corner (TL) should be revealed.
        assert!(!revealed(&shader, 9, 9, 10, 10, 0.3));
        assert!(revealed(&shader, 0, 0, 10, 10, 0.3));
    }

    #[test]
    fn progress_zero_hides_all_cells() {
        let shader = RevealWipeShader::default();
        for y in 0..5 {
            for x in 0..5 {
                assert!(
                    !revealed(&shader, x, y, 5, 5, 0.0),
                    "({x},{y}) should be hidden at progress=0"
                );
            }
        }
    }

    #[test]
    fn progress_one_reveals_all_cells() {
        let shader = RevealWipeShader::default();
        for y in 0..5 {
            for x in 0..5 {
                assert!(
                    revealed(&shader, x, y, 5, 5, 1.0),
                    "({x},{y}) should be revealed at progress=1"
                );
            }
        }
    }

    #[test]
    fn legacy_serde_field_continues_to_parse() {
        let json = r#"{"direction":"left_to_right"}"#;
        let parsed: RevealWipeShader = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.direction, RevealDirection::LeftToRight);
    }

    #[test]
    fn new_corner_directions_parse_via_authoring_alias() {
        let json = r#"{"direction":"corner_down_top_left"}"#;
        let parsed: RevealWipeShader = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.direction, RevealDirection::CornerOutFromTopLeft);
    }
}

// <FILE>tui-vfx-style/src/models/cls_reveal_wipe_shader.rs</FILE> - <DESC>RevealWipe shader</DESC>
// <VERS>END OF VERSION: 2.1.0</VERS>
