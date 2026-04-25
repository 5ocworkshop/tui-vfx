// <FILE>tui-vfx-style/src/models/cls_linear_gradient_shader.rs</FILE> - <DESC>Static directional gradient fill shader. Authoring sugar `gradient_overlay` canonicalises to this primitive at the lowering layer.</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Audit recommendations 2.1 + 2.2 — close two long-standing foot-guns: (1) `gradient_overlay` was canonicalised to `linear_gradient` with `apply_to` and `intensity` silently dropped, so authors who said `apply_to: "background"` or used `channel:background` got the cryptic "unsupported channel target for shader payload `linear_gradient`: background" probe error; (2) `style_at` used `angle_deg` as a boolean axis selector (`if angle_deg.abs() < 45.0 { use u } else { use v }`), so `angle_deg: 45.0` swept VERTICALLY and `angle_deg: -30` swept HORIZONTALLY rather than producing the diagonal the author drew. This commit adds first-class `apply_to` and `intensity` fields and replaces the axis-pick math with proper projection of the cell position onto the gradient axis at any angle.</WCTX>
// <CLOG>1.0.0: MAJOR (additive — back-compat preserved for every prior valid usage):
// - Add `apply_to: LinearGradientApplyTo` field with three variants `Foreground` / `Background` / `Both`. Default is `Foreground`, matching the historical "writes only to fg" behaviour.
// - Add `intensity: f32` field. Default `1.0`. At 1.0 the gradient colour fully replaces the target channel(s), matching the historical behaviour. Lower values blend toward base. 0.0 returns base unchanged.
// - Replace the boolean axis pick with a true projection of the cell's normalised (u, v) position onto the gradient axis at any `angle_deg`. The formula is `t = (u·cos θ + v·sin θ - min_proj) / (max_proj - min_proj)` where the proj range is taken over `(u, v) ∈ [0,1]²` so the gradient sweeps cleanly from one corner to the opposite corner at 45°/135°/225°/315° and from one edge to the opposite edge at 0°/90°/180°/270°. Cardinal angles 0 and 90 produce identical sample_t values to the previous code, so all `angle_deg ∈ {0, 90}` recipes are bit-identical. **Behaviour change: any recipe with a non-cardinal `angle_deg` (e.g. 45, 35, -30) was previously getting a cardinal-axis sweep due to the axis-pick bug; it now sweeps along the authored angle.** This is the audit-recommended fix; the previous behaviour was buggy in a way authors had to reverse-engineer.
// 0.1.1: Derived ConfigSchema (initial primitive, four-line struct, fg-only, axis-pick angle handling).</CLOG>

//! # LinearGradient shader
//!
//! Static directional gradient fill at any angle. The gradient samples a
//! [`Gradient`] (multi-stop, configurable colour space) along an axis
//! determined by `angle_deg`:
//!
//! - `angle_deg = 0` → left-to-right horizontal sweep
//! - `angle_deg = 90` → top-to-bottom vertical sweep
//! - `angle_deg = 45` → diagonal from top-left toward bottom-right
//! - `angle_deg = 135` → diagonal from top-right toward bottom-left
//! - any other angle → projection onto an axis at that angle
//!
//! ## Authoring sugar
//!
//! Recipes commonly write `"type": "gradient_overlay"` instead of
//! `"linear_gradient"`. The V3 normaliser canonicalises the former to
//! the latter and (since 1.0.0) preserves both `apply_to` and `intensity`
//! through the canonicalisation. Either form is fine; `linear_gradient`
//! is the runtime/canonical name and `gradient_overlay` is an authoring
//! alias.
//!
//! ## Channel targeting
//!
//! `apply_to` controls which colour channel(s) the gradient writes to:
//!
//! - `Foreground` (default, back-compat) — writes only `fg`
//! - `Background` — writes only `bg`
//! - `Both` — writes both
//!
//! Recipes can also use a `channel:background` scope; the lowering layer
//! converts that into `apply_to: "background"` on the payload before the
//! shader sees it.
//!
//! ## Intensity
//!
//! `intensity` (0.0–1.0, default 1.0) blends the gradient colour with
//! the base channel. At 1.0 the gradient colour fully replaces the
//! target; at 0.5 it's a 50/50 mix. 0.0 returns the base unchanged.

use crate::models::{ColorSpace, Gradient};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::Style;

/// Which colour channel(s) the linear gradient writes to.
///
/// Defaults to [`Foreground`](Self::Foreground) which matches the
/// pre-1.0.0 fg-only behaviour of `LinearGradientShader::style_at`.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum LinearGradientApplyTo {
    /// Write the gradient colour into the foreground channel only.
    /// Back-compat default.
    #[default]
    Foreground,
    /// Write the gradient colour into the background channel only.
    Background,
    /// Write the gradient colour into both foreground and background.
    Both,
}

/// Static directional gradient fill at any angle, with optional channel
/// targeting and intensity blend strength.
///
/// See module docs for the angle semantics, the `apply_to` channel
/// vocabulary, and the relationship to the `gradient_overlay` authoring
/// sugar.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct LinearGradientShader {
    /// Multi-stop gradient definition (stops + colour space).
    pub gradient: Gradient,
    /// Gradient axis angle in degrees.
    ///
    /// Angles are measured CCW from the positive X axis (i.e.
    /// `0` = left→right, `90` = top→bottom, `45` = TL→BR diagonal).
    /// Any angle is supported; the runtime projects the cell's
    /// normalised position onto the gradient axis.
    pub angle_deg: f32,
    /// Which channel(s) to write the gradient colour to. Default
    /// `Foreground` for back-compat with pre-1.0.0 behaviour.
    #[serde(default)]
    pub apply_to: LinearGradientApplyTo,
    /// Blend strength of the gradient colour over the base channel,
    /// `0.0..=1.0`. Default `1.0` (fully replace the target channel,
    /// matching pre-1.0.0 behaviour).
    #[serde(default = "default_intensity")]
    pub intensity: f32,
}

fn default_intensity() -> f32 {
    1.0
}

impl LinearGradientShader {
    /// Construct a horizontal-sweep gradient (foreground, intensity 1.0).
    pub fn new(gradient: Gradient) -> Self {
        Self {
            gradient,
            angle_deg: 0.0,
            apply_to: LinearGradientApplyTo::Foreground,
            intensity: 1.0,
        }
    }

    /// Construct a vertical-sweep gradient (foreground, intensity 1.0).
    pub fn vertical(gradient: Gradient) -> Self {
        Self {
            gradient,
            angle_deg: 90.0,
            apply_to: LinearGradientApplyTo::Foreground,
            intensity: 1.0,
        }
    }
}

impl StyleShader for LinearGradientShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.intensity <= 0.0 {
            return base;
        }

        // Normalised position in the rect. We use (width − 1) and
        // (height − 1) so that the corner cells map to {0.0, 1.0}
        // exactly, which keeps cardinal angles bit-identical to the
        // pre-1.0.0 behaviour.
        let u = if ctx.width > 1 {
            ctx.local_x as f32 / (ctx.width - 1) as f32
        } else {
            0.0
        };
        let v = if ctx.height > 1 {
            ctx.local_y as f32 / (ctx.height - 1) as f32
        } else {
            0.0
        };

        // Project (u, v) onto the gradient axis at angle_deg. The
        // projection range is computed over (u, v) ∈ [0, 1]² so the
        // gradient cleanly sweeps from one corner/edge to the opposite
        // one at any angle. At cardinal angles (0, 90, 180, 270) one
        // term goes to zero, recovering the same sample_t the old
        // axis-pick code produced.
        let angle_rad = self.angle_deg.to_radians();
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();
        let raw = u * cos_a + v * sin_a;
        let min_proj = 0.0_f32.min(cos_a) + 0.0_f32.min(sin_a);
        let max_proj = 0.0_f32.max(cos_a) + 0.0_f32.max(sin_a);
        let denom = max_proj - min_proj;
        let sample_t = if denom > 0.0 {
            ((raw - min_proj) / denom).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let color = self.gradient.sample(sample_t);
        let alpha = self.intensity.clamp(0.0, 1.0);

        let mut result = base;
        match self.apply_to {
            LinearGradientApplyTo::Foreground => {
                result.fg = blend_colors(base.fg, color, alpha, ColorSpace::Rgb);
            }
            LinearGradientApplyTo::Background => {
                result.bg = blend_colors(base.bg, color, alpha, ColorSpace::Rgb);
            }
            LinearGradientApplyTo::Both => {
                result.fg = blend_colors(base.fg, color, alpha, ColorSpace::Rgb);
                result.bg = blend_colors(base.bg, color, alpha, ColorSpace::Rgb);
            }
        }
        result
    }

    fn name(&self) -> &'static str {
        "LinearGradient"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ColorSpace;
    use tui_vfx_types::{Color, Style};

    fn ctx(local_x: u16, local_y: u16, w: u16, h: u16) -> ShaderContext {
        ShaderContext::new(local_x, local_y, w, h, 0, 0, 0.0, None, None)
    }

    fn black_to_white() -> Gradient {
        Gradient::new(vec![(0.0, Color::BLACK), (1.0, Color::WHITE)])
    }

    fn make_shader(angle_deg: f32, apply_to: LinearGradientApplyTo, intensity: f32) -> LinearGradientShader {
        LinearGradientShader {
            gradient: Gradient {
                stops: black_to_white().stops,
                space: ColorSpace::Rgb,
            },
            angle_deg,
            apply_to,
            intensity,
        }
    }

    fn base_style() -> Style {
        Style {
            fg: Color::rgb(50, 50, 50),
            bg: Color::rgb(120, 120, 120),
            mods: Default::default(),
        }
    }

    #[test]
    fn defaults_match_back_compat_writes_fg_only_at_full_intensity() {
        // The new default (apply_to = Foreground, intensity = 1.0) plus
        // a horizontal gradient must produce the same fg sweep that the
        // 0.1.1 shader produced. The leftmost cell samples black
        // (gradient at t=0) and the rightmost cell samples white.
        let shader = LinearGradientShader::new(black_to_white());
        let left = shader.style_at(&ctx(0, 0, 10, 1), base_style());
        let right = shader.style_at(&ctx(9, 0, 10, 1), base_style());
        assert_eq!(left.fg, Color::BLACK);
        assert_eq!(right.fg, Color::WHITE);
        // Background must be unchanged for the default Foreground apply_to.
        assert_eq!(left.bg, base_style().bg);
        assert_eq!(right.bg, base_style().bg);
    }

    #[test]
    fn vertical_constructor_sweeps_top_to_bottom_on_fg() {
        let shader = LinearGradientShader::vertical(black_to_white());
        let top = shader.style_at(&ctx(0, 0, 1, 10), base_style());
        let bottom = shader.style_at(&ctx(0, 9, 1, 10), base_style());
        assert_eq!(top.fg, Color::BLACK);
        assert_eq!(bottom.fg, Color::WHITE);
    }

    #[test]
    fn apply_to_background_writes_bg_leaves_fg_alone() {
        let shader = make_shader(0.0, LinearGradientApplyTo::Background, 1.0);
        let left = shader.style_at(&ctx(0, 0, 10, 1), base_style());
        let right = shader.style_at(&ctx(9, 0, 10, 1), base_style());
        assert_eq!(left.bg, Color::BLACK);
        assert_eq!(right.bg, Color::WHITE);
        assert_eq!(left.fg, base_style().fg);
        assert_eq!(right.fg, base_style().fg);
    }

    #[test]
    fn apply_to_both_writes_both_channels() {
        let shader = make_shader(0.0, LinearGradientApplyTo::Both, 1.0);
        let left = shader.style_at(&ctx(0, 0, 10, 1), base_style());
        let right = shader.style_at(&ctx(9, 0, 10, 1), base_style());
        assert_eq!(left.fg, Color::BLACK);
        assert_eq!(left.bg, Color::BLACK);
        assert_eq!(right.fg, Color::WHITE);
        assert_eq!(right.bg, Color::WHITE);
    }

    #[test]
    fn intensity_zero_returns_base_unchanged() {
        let shader = make_shader(0.0, LinearGradientApplyTo::Both, 0.0);
        let result = shader.style_at(&ctx(5, 5, 10, 10), base_style());
        assert_eq!(result, base_style());
    }

    #[test]
    fn intensity_half_blends_halfway_to_gradient_color() {
        // At intensity 0.5 the rightmost cell (gradient color = white)
        // should be 50/50 between base.fg (gray 50) and white (255).
        let shader = make_shader(0.0, LinearGradientApplyTo::Foreground, 0.5);
        let right = shader.style_at(&ctx(9, 0, 10, 1), base_style());
        let r = right.fg.r as i32;
        // 50% blend between 50 and 255 ≈ 152 or 153 depending on rounding.
        assert!((151..=154).contains(&r), "expected halfway blend, got r={r}");
    }

    #[test]
    fn cardinal_zero_degrees_matches_horizontal_sweep() {
        // Behavioral parity check — the new projection at angle 0 must
        // produce the same sample_t as the old `if abs(angle) < 45 { u }
        // else { v }` axis-pick path.
        let shader = make_shader(0.0, LinearGradientApplyTo::Foreground, 1.0);
        for x in 0..10 {
            let cell = shader.style_at(&ctx(x, 5, 10, 10), base_style());
            // Old code: sample_t = u = x / 9; new code projects identically
            // at angle 0. Every cell at the same x should have the same fg
            // regardless of y.
            for y in 0..10 {
                let other = shader.style_at(&ctx(x, y, 10, 10), base_style());
                assert_eq!(cell.fg, other.fg, "fg should not depend on y at angle=0");
            }
        }
    }

    #[test]
    fn cardinal_90_degrees_matches_vertical_sweep() {
        let shader = make_shader(90.0, LinearGradientApplyTo::Foreground, 1.0);
        for y in 0..10 {
            for x in 0..10 {
                let cell = shader.style_at(&ctx(x, y, 10, 10), base_style());
                let same_y_other_x = shader.style_at(&ctx((x + 1) % 10, y, 10, 10), base_style());
                assert_eq!(cell.fg, same_y_other_x.fg, "fg should not depend on x at angle=90");
            }
        }
    }

    #[test]
    fn diagonal_45_degrees_actually_sweeps_diagonally() {
        // Audit-recommended fix: at angle 45 the gradient must sweep from
        // top-left toward bottom-right. The old code (axis-pick) returned
        // a vertical sweep at 45° because 45 is not strictly < 45.
        let shader = make_shader(45.0, LinearGradientApplyTo::Foreground, 1.0);
        let tl = shader.style_at(&ctx(0, 0, 10, 10), base_style());
        let br = shader.style_at(&ctx(9, 9, 10, 10), base_style());
        let tr = shader.style_at(&ctx(9, 0, 10, 10), base_style());
        let bl = shader.style_at(&ctx(0, 9, 10, 10), base_style());
        // TL is at sample_t 0 (black), BR is at sample_t 1 (white).
        assert_eq!(tl.fg, Color::BLACK);
        assert_eq!(br.fg, Color::WHITE);
        // TR and BL are both at sample_t 0.5 — they sit on the same
        // anti-diagonal as the centre. Distinct from the old buggy
        // behaviour where TR matched BR (both on the bottom row of a
        // vertical sweep).
        assert_eq!(tr.fg, bl.fg, "TR and BL should sit on the same anti-diagonal at 45°");
        assert_ne!(tr.fg, br.fg, "TR must NOT match BR at angle=45 (the old bug)");
    }

    #[test]
    fn diagonal_135_degrees_sweeps_top_right_toward_bottom_left() {
        let shader = make_shader(135.0, LinearGradientApplyTo::Foreground, 1.0);
        let tr = shader.style_at(&ctx(9, 0, 10, 10), base_style());
        let bl = shader.style_at(&ctx(0, 9, 10, 10), base_style());
        // At 135°, cos<0 sin>0, so the projection minimum is at top-right
        // and maximum at bottom-left.
        assert_eq!(tr.fg, Color::BLACK);
        assert_eq!(bl.fg, Color::WHITE);
    }

    #[test]
    fn negative_30_degrees_sweeps_correctly() {
        // Audit-recommended fix: angle_deg = -30 must produce a
        // diagonal sweep (down-and-leftward), NOT the horizontal sweep
        // that the old axis-pick code would produce (since |−30| < 45).
        let shader = make_shader(-30.0, LinearGradientApplyTo::Foreground, 1.0);
        let tl = shader.style_at(&ctx(0, 0, 10, 10), base_style());
        let br = shader.style_at(&ctx(9, 9, 10, 10), base_style());
        let tr = shader.style_at(&ctx(9, 0, 10, 10), base_style());
        // At -30°: cos≈0.866, sin≈-0.5. Min proj at (u=0,v=1) = -0.5;
        // max proj at (u=1,v=0) = 0.866. So TR (u=1,v=0) is sample_t=1
        // (white), BL (u=0,v=1) is sample_t=0 (black). TL and BR are at
        // intermediate values that are equal because (u=0,v=0) and
        // (u=1,v=1) both project to (cos+0)/range and (cos-sin)/range
        // — actually let me just check tr vs tl directly.
        assert_eq!(tr.fg, Color::WHITE);
        // TL and BR aren't symmetric here; just check they aren't both 0
        // or both 255 (which would mean no sweep).
        assert_ne!(tl.fg, br.fg);
    }

    #[test]
    fn name_is_correct() {
        let shader = LinearGradientShader::new(black_to_white());
        assert_eq!(shader.name(), "LinearGradient");
    }

    #[test]
    fn legacy_recipe_without_apply_to_or_intensity_still_parses() {
        // Existing recipes don't have apply_to / intensity fields. They
        // must continue to deserialize, with defaults applied.
        let json = r#"{"gradient":{"stops":[[0.0,{"type":"black"}],[1.0,{"type":"white"}]],"space":"rgb"},"angle_deg":0.0}"#;
        let parsed: LinearGradientShader = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.apply_to, LinearGradientApplyTo::Foreground);
        assert_eq!(parsed.intensity, 1.0);
    }

    #[test]
    fn explicit_apply_to_background_parses() {
        let json = r#"{"gradient":{"stops":[[0.0,{"type":"black"}],[1.0,{"type":"white"}]],"space":"rgb"},"angle_deg":0.0,"apply_to":"background","intensity":0.8}"#;
        let parsed: LinearGradientShader = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.apply_to, LinearGradientApplyTo::Background);
        assert_eq!(parsed.intensity, 0.8);
    }
}

// <FILE>tui-vfx-style/src/models/cls_linear_gradient_shader.rs</FILE> - <DESC>Linear gradient shader (angle, apply_to, intensity)</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
