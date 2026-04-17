// <FILE>tui-vfx-style/src/models/cls_highlighter_shader.rs</FILE> - <DESC>Highlighter sweep shader with direction, mode, blending, and runtime bindings</DESC>
// <VERS>VERSION: 2.0.1</VERS>
// <WCTX>feat/cursor-primitive T31: clippy clean-up passing through unrelated tui-vfx-style warnings surfaced by the workspace lint gate — TextContrast has a manual `Default` impl that clippy::derivable_impls wants replaced with `#[default]` on the Black variant. No semantic change.</WCTX>
// <CLOG>PATCH: derive Default on TextContrast with #[default] on Black; remove the manual impl Default block</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Which channel(s) the highlighter ink affects.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum HighlighterApplyTo {
    /// Paint the highlighter color as the cell background (default —
    /// matches 1.0.0 behavior).
    #[serde(alias = "bg")]
    #[default]
    Background,
    /// Blend the highlighter color into the existing foreground text, tinting
    /// the glyphs without changing the cell background.
    #[serde(alias = "fg")]
    Foreground,
    /// Paint the highlighter on the background AND tint the foreground,
    /// giving a stronger full-cell highlight.
    Both,
}

/// How to treat the foreground when `apply_to = Background`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum TextContrast {
    /// Force `fg = Color::BLACK` (default — matches 1.0.0 behavior). Keeps
    /// text readable on bright highlight colors like yellow or cyan.
    #[default]
    Black,
    /// Keep whatever foreground the cell already had. Use when the base
    /// foreground is already chosen for contrast against the highlighter
    /// color (e.g., theme-authored on_primary on primary highlight).
    Preserve,
    /// Set the foreground to an explicit color for every cell the
    /// highlighter covers.
    Explicit { color: ColorConfig },
}

/// Coverage shape of the highlighter sweep.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum HighlighterMode {
    /// Ink stays behind the sweep head — everything "behind" the leading
    /// edge is highlighted, like an actual marker drawing left-to-right.
    /// Matches 1.0.0 behavior (default).
    #[default]
    Fill,
    /// Only cells within `band_width` of the sweep head are highlighted —
    /// a moving band rather than a progressive fill. Useful for repeating
    /// "scan" effects that leave no residue.
    Band,
}

/// Direction the sweep travels across the widget.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum HighlighterDirection {
    /// Left-to-right horizontal sweep (default — matches 1.0.0 behavior).
    #[default]
    Forward,
    /// Right-to-left horizontal sweep.
    Reverse,
    /// Top-to-bottom vertical sweep.
    TopDown,
    /// Bottom-to-top vertical sweep.
    BottomUp,
    /// Two sweeps starting from the center and diverging to both edges.
    CenterOut,
    /// Two sweeps starting from both edges and converging to the center.
    EdgesIn,
}

/// Restrict highlighter coverage to specific rows.
///
/// Useful for underline-wipe effects (`LastRow`), title-bar highlights
/// (`FirstRow`), or custom banded coverage (`Range`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum HighlighterRowMask {
    /// Apply to every row (default — matches 1.0.0 behavior).
    #[default]
    AllRows,
    /// Apply only to the top row (useful for banner highlights).
    FirstRow,
    /// Apply only to the bottom row (useful for underline-wipe effects).
    LastRow,
    /// Apply only to the top and bottom rows.
    TopAndBottom,
    /// Apply to rows in `[start, end]` inclusive. Out-of-range rows are
    /// skipped silently.
    Range { start: u16, end: u16 },
}

/// A highlighter-marker sweep shader.
///
/// The 2.0.0 upgrade turns this from a minimal bg-fill with hardcoded
/// black foreground into a full-featured sweep with direction, mode
/// (progressive Fill vs moving Band), blend strength, soft trailing edge,
/// runtime bindings, and row masks. All new fields default to values that
/// reproduce the 1.0.0 behavior exactly, so existing recipes that specify
/// only `{ "color": X }` continue to render identically.
///
/// # Common patterns
///
/// **Progressive text highlight (1.0.0 behavior, unchanged):**
/// ```json
/// { "type": "highlighter", "color": { "type": "yellow" } }
/// ```
///
/// **Accent tint sweeping across text without covering the background:**
/// ```json
/// {
///   "type": "highlighter",
///   "color": { "type": "rgb", "r": 200, "g": 104, "b": 32 },
///   "apply_to": "foreground",
///   "text_contrast": { "mode": "preserve" },
///   "blend_strength": 0.4
/// }
/// ```
///
/// **Underline-wipe hover indicator:**
/// ```json
/// {
///   "type": "highlighter",
///   "color": { "type": "rgb", "r": 200, "g": 104, "b": 32 },
///   "apply_to": "foreground",
///   "text_contrast": { "mode": "preserve" },
///   "row_mask": { "mode": "last_row" },
///   "speed": 2.0
/// }
/// ```
///
/// **Scanning band that leaves no residue:**
/// ```json
/// {
///   "type": "highlighter",
///   "color": { "type": "cyan" },
///   "mode": "band",
///   "band_width": 8,
///   "soft_edge": 0.6,
///   "blend_strength": 0.7
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct HighlighterShader {
    /// The highlight ink color.
    pub color: ColorConfig,

    /// Which channel(s) the ink affects. Default: `Background` (matches 1.0.0).
    #[serde(default)]
    pub apply_to: HighlighterApplyTo,

    /// How to treat the foreground when `apply_to = Background`.
    /// Default: `Black` (matches 1.0.0 hardcoded `fg = BLACK`).
    #[serde(default)]
    pub text_contrast: TextContrast,

    /// Sweep coverage shape. Default: `Fill` (matches 1.0.0 progressive fill).
    #[serde(default)]
    pub mode: HighlighterMode,

    /// Width of the moving ink band in cells when `mode = Band`. Ignored in
    /// `Fill` mode. Default: 6.
    #[serde(default = "default_band_width")]
    pub band_width: u16,

    /// Softness of the trailing edge from 0.0 (hard cutoff) to 1.0 (fully
    /// feathered). Affects the Fill-mode sweep head and the Band-mode tail.
    /// Default: 0.0 (matches 1.0.0 hard edge).
    #[serde(default = "default_soft_edge")]
    pub soft_edge: f32,

    /// How strongly the highlighter color replaces the base, from 0.0
    /// (invisible) to 1.0 (full replacement). Default: 1.0 (matches 1.0.0
    /// full-replace behavior).
    #[serde(default = "default_blend_strength")]
    pub blend_strength: f32,

    /// Optional runtime parameter key that overrides `blend_strength` per
    /// frame. Resolved values are clamped to 0.0-1.0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blend_strength_binding: Option<String>,

    /// Sweep-rate multiplier (1.0 = default time-driven sweep). Default: 1.0.
    #[serde(default = "default_speed")]
    pub speed: f32,

    /// Optional runtime parameter key that overrides `speed` per frame.
    /// Resolved values are clamped to 0.1..=10.0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed_binding: Option<String>,

    /// Direction the sweep travels. Default: `Forward` (matches 1.0.0
    /// left-to-right).
    #[serde(default)]
    pub direction: HighlighterDirection,

    /// Optional runtime parameter key that overrides `direction` per frame.
    /// Value mapping: 0 = Forward, 1 = Reverse, 2 = TopDown, 3 = BottomUp,
    /// 4 = CenterOut, 5 = EdgesIn. Out-of-range values fall back to the
    /// static `direction`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction_binding: Option<String>,

    /// Restrict coverage to specific rows. Default: `AllRows` (matches 1.0.0).
    #[serde(default)]
    pub row_mask: HighlighterRowMask,
}

fn default_band_width() -> u16 {
    6
}

fn default_soft_edge() -> f32 {
    0.0
}

fn default_blend_strength() -> f32 {
    1.0
}

fn default_speed() -> f32 {
    1.0
}

impl Default for HighlighterShader {
    fn default() -> Self {
        Self {
            color: ColorConfig::Yellow,
            apply_to: HighlighterApplyTo::Background,
            text_contrast: TextContrast::Black,
            mode: HighlighterMode::Fill,
            band_width: default_band_width(),
            soft_edge: default_soft_edge(),
            blend_strength: default_blend_strength(),
            blend_strength_binding: None,
            speed: default_speed(),
            speed_binding: None,
            direction: HighlighterDirection::Forward,
            direction_binding: None,
            row_mask: HighlighterRowMask::AllRows,
        }
    }
}

impl HighlighterShader {
    /// Construct a highlighter with only the color set and every other
    /// field at its default. Equivalent to the 1.0.0 minimal shader shape.
    pub fn new(color: ColorConfig) -> Self {
        Self {
            color,
            ..Self::default()
        }
    }

    fn effective_speed(&self, ctx: &ShaderContext) -> f32 {
        self.speed_binding
            .as_deref()
            .and_then(|b| ctx.runtime_param_f32(b))
            .unwrap_or(self.speed)
            .clamp(0.1, 10.0)
    }

    fn effective_blend_strength(&self, ctx: &ShaderContext) -> f32 {
        self.blend_strength_binding
            .as_deref()
            .and_then(|b| ctx.runtime_param_f32(b))
            .unwrap_or(self.blend_strength)
            .clamp(0.0, 1.0)
    }

    fn effective_direction(&self, ctx: &ShaderContext) -> HighlighterDirection {
        self.direction_binding
            .as_deref()
            .and_then(|b| ctx.runtime_param_u16(b))
            .and_then(|code| match code {
                0 => Some(HighlighterDirection::Forward),
                1 => Some(HighlighterDirection::Reverse),
                2 => Some(HighlighterDirection::TopDown),
                3 => Some(HighlighterDirection::BottomUp),
                4 => Some(HighlighterDirection::CenterOut),
                5 => Some(HighlighterDirection::EdgesIn),
                _ => None,
            })
            .unwrap_or(self.direction)
    }

    /// Returns `Some(coverage)` in 0.0..=1.0 when the given cell is inside
    /// the highlighter's ink (1.0 = full ink, 0.0..1.0 = feathered trailing
    /// edge), or `None` when the cell is outside the sweep entirely.
    fn coverage_at(&self, ctx: &ShaderContext, effective_t: f32) -> Option<f32> {
        let (x, y, width, height) = (ctx.local_x, ctx.local_y, ctx.width, ctx.height);

        // Project the cell onto the sweep axis based on direction. `pos`
        // is the cell's position along the sweep (0..=axis_len) and
        // `axis_len` is the total distance the sweep head travels.
        let direction = self.effective_direction(ctx);
        let (pos, axis_len) = match direction {
            HighlighterDirection::Forward => (x as f32, width.saturating_sub(1).max(1) as f32),
            HighlighterDirection::Reverse => (
                (width.saturating_sub(1)).saturating_sub(x) as f32,
                width.saturating_sub(1).max(1) as f32,
            ),
            HighlighterDirection::TopDown => {
                (y as f32, height.saturating_sub(1).max(1) as f32)
            }
            HighlighterDirection::BottomUp => (
                (height.saturating_sub(1)).saturating_sub(y) as f32,
                height.saturating_sub(1).max(1) as f32,
            ),
            HighlighterDirection::CenterOut => {
                // Two mirrored sweeps starting at width/2 and diverging to
                // both edges. Coverage = distance from center along x.
                let center = width as f32 / 2.0;
                let dist = (x as f32 - center).abs();
                (dist, center.max(1.0))
            }
            HighlighterDirection::EdgesIn => {
                // Two mirrored sweeps converging from the edges to the
                // center. Coverage = shortest distance to nearest edge.
                let from_left = x as f32;
                let from_right = (width.saturating_sub(1)).saturating_sub(x) as f32;
                let nearest = from_left.min(from_right);
                let half_width = (width as f32 / 2.0).max(1.0);
                // invert so sweep head starts at the edges (pos=0) and
                // reaches the center (pos=half_width) at t=1.0
                (half_width - nearest.min(half_width), half_width)
            }
        };

        // Sweep head position advances with t. For Fill mode the head sits
        // at `head_pos` and every cell with pos <= head_pos is inked. For
        // Band mode only cells within `band_width` trailing the head are
        // inked.
        let head_pos = effective_t * (axis_len + self.band_width as f32);

        match self.mode {
            HighlighterMode::Fill => {
                if pos > head_pos {
                    return None;
                }
                // Within the fill region. Coverage is 1.0 except near the
                // head where soft_edge feathers it.
                if self.soft_edge > 0.0 {
                    let feather = self.soft_edge * self.band_width.max(1) as f32;
                    let dist_from_head = head_pos - pos;
                    if dist_from_head < feather {
                        Some(dist_from_head / feather)
                    } else {
                        Some(1.0)
                    }
                } else {
                    Some(1.0)
                }
            }
            HighlighterMode::Band => {
                let dist = head_pos - pos;
                if !(0.0..=self.band_width as f32).contains(&dist) {
                    return None;
                }
                // Intensity peaks at the head and fades across band_width.
                let t_in_band = dist / self.band_width.max(1) as f32;
                let intensity = 1.0 - t_in_band;
                if self.soft_edge > 0.0 {
                    // Soft-edge feather biases intensity toward the head
                    Some(intensity.powf(1.0 + self.soft_edge * 2.0))
                } else {
                    Some(intensity)
                }
            }
        }
    }

    fn row_allowed(&self, ctx: &ShaderContext) -> bool {
        match self.row_mask {
            HighlighterRowMask::AllRows => true,
            HighlighterRowMask::FirstRow => ctx.local_y == 0,
            HighlighterRowMask::LastRow => ctx.local_y + 1 >= ctx.height,
            HighlighterRowMask::TopAndBottom => {
                ctx.local_y == 0 || ctx.local_y + 1 >= ctx.height
            }
            HighlighterRowMask::Range { start, end } => {
                ctx.local_y >= start && ctx.local_y <= end
            }
        }
    }

    fn resolve_text_contrast(&self, base_fg: Color, _highlight_rgb: Color) -> Color {
        match &self.text_contrast {
            TextContrast::Black => Color::BLACK,
            TextContrast::Preserve => base_fg,
            TextContrast::Explicit { color } => (*color).into(),
        }
    }
}

impl StyleShader for HighlighterShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if !self.row_allowed(ctx) {
            return base;
        }

        let effective_speed = self.effective_speed(ctx);
        let effective_t = (ctx.t as f32 * effective_speed).clamp(0.0, 1.0);

        let Some(coverage) = self.coverage_at(ctx, effective_t) else {
            return base;
        };

        let effective_blend = self.effective_blend_strength(ctx) * coverage;
        if effective_blend <= 0.0 {
            return base;
        }

        let highlight: Color = self.color.into();
        let mut style = base;

        match self.apply_to {
            HighlighterApplyTo::Background => {
                // Blend the highlighter color into the background.
                style.bg = if effective_blend >= 1.0 {
                    highlight
                } else if base.bg != Color::TRANSPARENT {
                    blend_colors(base.bg, highlight, effective_blend, ColorSpace::Rgb)
                } else {
                    // No base background — apply at full strength scaled by
                    // the blend amount (partial transparency effect via
                    // blending against black)
                    blend_colors(Color::BLACK, highlight, effective_blend, ColorSpace::Rgb)
                };
                // Resolve the foreground per text_contrast.
                let target_fg = self.resolve_text_contrast(base.fg, highlight);
                if effective_blend >= 1.0 {
                    style.fg = target_fg;
                } else if matches!(self.text_contrast, TextContrast::Preserve) {
                    // Preserve mode: don't touch fg at all
                    style.fg = base.fg;
                } else if base.fg != Color::TRANSPARENT {
                    style.fg = blend_colors(base.fg, target_fg, effective_blend, ColorSpace::Rgb);
                } else {
                    style.fg = target_fg;
                }
            }
            HighlighterApplyTo::Foreground => {
                // Tint only the foreground. Background is untouched.
                if base.fg != Color::TRANSPARENT {
                    style.fg = blend_colors(base.fg, highlight, effective_blend, ColorSpace::Rgb);
                } else {
                    style.fg = highlight;
                }
            }
            HighlighterApplyTo::Both => {
                // Affect both channels.
                if base.bg != Color::TRANSPARENT {
                    style.bg = blend_colors(base.bg, highlight, effective_blend, ColorSpace::Rgb);
                } else {
                    style.bg = highlight;
                }
                if base.fg != Color::TRANSPARENT {
                    // fg gets a softer tint so the bg change reads clearly.
                    style.fg =
                        blend_colors(base.fg, highlight, effective_blend * 0.5, ColorSpace::Rgb);
                } else {
                    style.fg = highlight;
                }
            }
        }

        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{ShaderRuntimeParamValue, ShaderRuntimeParams};
    use std::sync::Arc;

    fn ctx_at(local_x: u16, local_y: u16, width: u16, height: u16, t: f64) -> ShaderContext {
        ShaderContext::new(local_x, local_y, width, height, 0, 0, t, None, None)
    }

    fn ctx_with_params(
        local_x: u16,
        local_y: u16,
        width: u16,
        height: u16,
        t: f64,
        params: ShaderRuntimeParams,
    ) -> ShaderContext {
        ShaderContext::new(
            local_x,
            local_y,
            width,
            height,
            0,
            0,
            t,
            None,
            Some(Arc::new(params)),
        )
    }

    // ------- Backward-compat: 1.0.0 behavior must be preserved ---------

    #[test]
    fn bare_color_recipe_still_deserializes() {
        let json = r#"{"color":{"type":"yellow"}}"#;
        let shader: HighlighterShader = serde_json::from_str(json).unwrap();
        assert_eq!(shader.color, ColorConfig::Yellow);
        assert_eq!(shader.apply_to, HighlighterApplyTo::Background);
        assert_eq!(shader.mode, HighlighterMode::Fill);
        assert_eq!(shader.direction, HighlighterDirection::Forward);
    }

    #[test]
    fn fill_mode_covers_left_half_at_t_half() {
        let shader = HighlighterShader::new(ColorConfig::Yellow);
        let base = Style::default();

        // At t=0.5 with width=10, the sweep head is at x=5ish. Cells at
        // x=0,1,2 should be highlighted; cell at x=9 should not be.
        let left = shader.style_at(&ctx_at(1, 0, 10, 3, 0.5), base);
        let right = shader.style_at(&ctx_at(9, 0, 10, 3, 0.5), base);
        assert_ne!(left, base, "left half should be highlighted");
        assert_eq!(right, base, "right half should be untouched");
    }

    #[test]
    fn default_forces_fg_to_black_like_v1() {
        let shader = HighlighterShader::new(ColorConfig::Yellow);
        let base = Style {
            fg: Color::rgb(200, 200, 200),
            ..Default::default()
        };
        let styled = shader.style_at(&ctx_at(0, 0, 10, 3, 0.5), base);
        assert_eq!(styled.fg, Color::BLACK, "default TextContrast::Black must match 1.0.0 hardcoded behavior");
    }

    // ---------------- New apply_to behaviors ----------------

    #[test]
    fn apply_to_foreground_leaves_bg_untouched() {
        let mut shader = HighlighterShader::new(ColorConfig::Red);
        shader.apply_to = HighlighterApplyTo::Foreground;
        let base = Style {
            fg: Color::rgb(200, 200, 200),
            bg: Color::rgb(50, 50, 50),
            ..Default::default()
        };
        let styled = shader.style_at(&ctx_at(0, 0, 10, 3, 0.5), base);
        assert_eq!(styled.bg, base.bg, "foreground apply_to must not touch bg");
        assert_ne!(styled.fg, base.fg, "foreground apply_to must tint fg");
    }

    #[test]
    fn text_contrast_preserve_keeps_base_fg() {
        let mut shader = HighlighterShader::new(ColorConfig::Yellow);
        shader.text_contrast = TextContrast::Preserve;
        let base = Style {
            fg: Color::rgb(200, 200, 200),
            ..Default::default()
        };
        let styled = shader.style_at(&ctx_at(0, 0, 10, 3, 1.0), base);
        assert_eq!(styled.fg, base.fg, "Preserve must keep the original foreground");
    }

    #[test]
    fn text_contrast_explicit_sets_custom_fg() {
        let mut shader = HighlighterShader::new(ColorConfig::Yellow);
        shader.text_contrast = TextContrast::Explicit {
            color: ColorConfig::Rgb { r: 255, g: 0, b: 255 },
        };
        let base = Style::default();
        let styled = shader.style_at(&ctx_at(0, 0, 10, 3, 1.0), base);
        assert_eq!(styled.fg, Color::rgb(255, 0, 255));
    }

    // ---------------- New mode / direction ----------------

    #[test]
    fn band_mode_only_highlights_near_head() {
        let mut shader = HighlighterShader::new(ColorConfig::Cyan);
        shader.mode = HighlighterMode::Band;
        shader.band_width = 3;
        let base = Style::default();

        // At t=0.5 with width=20, head is around x=11. A cell at x=11 should
        // be lit (intensity peak). A cell at x=2 (far behind) must be untouched
        // in Band mode unlike Fill mode.
        let near = shader.style_at(&ctx_at(11, 0, 20, 3, 0.5), base);
        let far = shader.style_at(&ctx_at(2, 0, 20, 3, 0.5), base);
        assert_ne!(near, base, "cell at head must be highlighted in Band mode");
        assert_eq!(far, base, "cell far behind head must not be highlighted in Band mode");
    }

    #[test]
    fn direction_reverse_sweeps_right_to_left() {
        let mut shader = HighlighterShader::new(ColorConfig::Yellow);
        shader.direction = HighlighterDirection::Reverse;
        let base = Style::default();

        // At t=0.5, the right half should be highlighted (not the left).
        let right = shader.style_at(&ctx_at(9, 0, 10, 3, 0.5), base);
        let left = shader.style_at(&ctx_at(0, 0, 10, 3, 0.5), base);
        assert_ne!(right, base, "right should be highlighted first in Reverse");
        assert_eq!(left, base, "left should not be highlighted yet in Reverse");
    }

    #[test]
    fn direction_top_down_sweeps_vertically() {
        let mut shader = HighlighterShader::new(ColorConfig::Yellow);
        shader.direction = HighlighterDirection::TopDown;
        let base = Style::default();

        // At t=0.5 with height=10, top row highlighted, bottom row not.
        let top = shader.style_at(&ctx_at(5, 0, 10, 10, 0.5), base);
        let bot = shader.style_at(&ctx_at(5, 9, 10, 10, 0.5), base);
        assert_ne!(top, base);
        assert_eq!(bot, base);
    }

    // ---------------- Row mask ----------------

    #[test]
    fn row_mask_last_row_only_touches_bottom() {
        let mut shader = HighlighterShader::new(ColorConfig::Yellow);
        shader.row_mask = HighlighterRowMask::LastRow;
        let base = Style::default();

        let top = shader.style_at(&ctx_at(1, 0, 10, 3, 1.0), base);
        let bot = shader.style_at(&ctx_at(1, 2, 10, 3, 1.0), base);
        assert_eq!(top, base, "top row must be untouched with LastRow mask");
        assert_ne!(bot, base, "bottom row must be highlighted with LastRow mask");
    }

    #[test]
    fn row_mask_range_honors_bounds() {
        let mut shader = HighlighterShader::new(ColorConfig::Yellow);
        shader.row_mask = HighlighterRowMask::Range { start: 1, end: 2 };
        let base = Style::default();

        let r0 = shader.style_at(&ctx_at(1, 0, 10, 5, 1.0), base);
        let r1 = shader.style_at(&ctx_at(1, 1, 10, 5, 1.0), base);
        let r2 = shader.style_at(&ctx_at(1, 2, 10, 5, 1.0), base);
        let r3 = shader.style_at(&ctx_at(1, 3, 10, 5, 1.0), base);
        assert_eq!(r0, base);
        assert_ne!(r1, base);
        assert_ne!(r2, base);
        assert_eq!(r3, base);
    }

    // ---------------- Runtime bindings ----------------

    #[test]
    fn speed_binding_overrides_static_speed() {
        let mut shader = HighlighterShader::new(ColorConfig::Yellow);
        shader.speed = 1.0;
        shader.speed_binding = Some("rate".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("rate", ShaderRuntimeParamValue::Float(2.0));
        let ctx = ctx_with_params(0, 0, 10, 3, 0.25, params);
        // speed=2.0 at t=0.25 is equivalent to speed=1.0 at t=0.5
        let styled = shader.style_at(&ctx, Style::default());
        assert_ne!(styled, Style::default(), "cell should be highlighted once speed doubles effective_t");
    }

    #[test]
    fn direction_binding_remaps_codes() {
        let mut shader = HighlighterShader::new(ColorConfig::Yellow);
        shader.direction = HighlighterDirection::Forward;
        shader.direction_binding = Some("dir".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("dir", ShaderRuntimeParamValue::Integer(1)); // Reverse
        let ctx = ctx_with_params(9, 0, 10, 3, 0.3, params);
        let styled = shader.style_at(&ctx, Style::default());
        assert_ne!(styled, Style::default(), "binding=1 must map to Reverse");
    }

    #[test]
    fn blend_strength_binding_clamps() {
        let mut shader = HighlighterShader::new(ColorConfig::Yellow);
        shader.blend_strength = 0.0;
        shader.blend_strength_binding = Some("bs".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("bs", ShaderRuntimeParamValue::Float(2.0)); // will clamp to 1.0
        let ctx = ctx_with_params(0, 0, 10, 3, 1.0, params);
        let styled = shader.style_at(&ctx, Style::default());
        assert_ne!(styled, Style::default(), "clamped binding should produce a highlight");
    }

    // ---------------- deny_unknown_fields still works ----------------

    #[test]
    fn unknown_field_still_rejected() {
        let json = r#"{"color":{"type":"yellow"},"nonsense":123}"#;
        let result: Result<HighlighterShader, _> = serde_json::from_str(json);
        assert!(result.is_err(), "deny_unknown_fields must still reject typos");
    }
}

// <FILE>tui-vfx-style/src/models/cls_highlighter_shader.rs</FILE> - <DESC>Highlighter sweep shader with direction, mode, blending, and runtime bindings</DESC>
// <VERS>END OF VERSION: 2.0.1</VERS>
