// <FILE>tui-vfx-style/src/models/cls_glisten_band_shader.rs</FILE> - <DESC>GlistenBand shader implementation</DESC>
// <VERS>VERSION: 2.2.0</VERS>
// <WCTX>Phase 0 P0.A followup — reconnect the vestigial speed field so it modulates the t-driven sweep rate, then add speed_binding for per-frame runtime overrides</WCTX>
// <CLOG>Reconnect speed as a scalar multiplier on the t-based sweep (scaled_t = t * effective_speed) and add speed_binding: Option<String> with per-frame f32 resolution in style_at, clamped to 0.1..=10.0 to prevent degenerate sweep rates. Recipes that previously set speed != 1.0 (skeleton_shimmer 0.7, update_shimmer_ribbon 0.8, glisten_band_directions 1.5, glisten_white_band_directions 3.0, shader_glisten_band* 1.5) will now render at the rates their authors originally intended</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Direction for the glisten band sweep.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum GlistenDirection {
    /// Sweep from start to end (default)
    #[default]
    Forward,
    /// Sweep from end to start
    Reverse,
    /// Oscillate back and forth
    PingPong,
}

/// Which color channel(s) to apply the glisten effect to.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum GlistenApplyTo {
    /// Apply to foreground only (default)
    #[serde(alias = "fg")]
    #[default]
    Foreground,
    /// Apply to background only
    #[serde(alias = "bg")]
    Background,
    /// Apply to both foreground and background
    Both,
}

/// A moving gradient band that sweeps across the widget at an angle.
///
/// Creates a "shimmer" or "glisten" effect like light reflecting off a surface.
/// Used for premium/gold effects like `golden_marquee`.
///
/// The band moves from one side to the other based on `t` and `speed`,
/// with colors transitioning from `tail` -> `head` -> `tail` within the band.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct GlistenBandShader {
    /// Speed multiplier for band movement (`1.0` = normal).
    ///
    /// Reconnected in 2.2.0 as a scalar multiplier on the `ctx.t`-driven
    /// sweep rate: `effective_t = (t * speed).fract()` for Forward, and
    /// similarly for Reverse / PingPong. `speed == 2.0` sweeps twice per
    /// original time unit; `speed == 0.5` sweeps half as often;
    /// `speed == 1.0` is the historical default. Resolved values are
    /// clamped to `0.1..=10.0` to prevent degenerate sweep rates.
    #[serde(default = "default_speed")]
    pub speed: f32,
    /// Optional runtime parameter key that overrides `speed` per frame.
    /// Resolved values are clamped to `0.1..=10.0`. Missing or
    /// unresolvable bindings fall back to the static `speed`.
    ///
    /// Used together with `direction_binding` / `blend_strength_binding`
    /// so apps can drive sweep rate, direction, and intensity from the
    /// same runtime params map (e.g. accelerate the sweep on hover,
    /// slow it during dwell).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed_binding: Option<String>,
    /// Width of the glisten band in cells
    #[serde(default = "default_band_width")]
    pub band_width: u16,
    /// Angle of the band in degrees (0 = horizontal, 90 = vertical)
    #[serde(default = "default_angle")]
    pub angle_deg: f32,
    /// Color at the leading edge of the band (brightest point)
    pub head: ColorConfig,
    /// Color at the trailing edge of the band (fades into base)
    pub tail: ColorConfig,
    /// Direction of the sweep
    #[serde(default)]
    pub direction: GlistenDirection,
    /// Optional runtime parameter key that overrides `direction` per frame.
    /// The resolved u16 maps to a `GlistenDirection` variant: 0 = Forward,
    /// 1 = Reverse, 2 = PingPong. Out-of-range values, missing bindings,
    /// and float-typed values fall back to the static `direction`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction_binding: Option<String>,
    /// Number of times to repeat (0 = continuous loop)
    #[serde(default)]
    pub repeat_count: u8,
    /// Which channel(s) to apply the effect to
    #[serde(default)]
    pub apply_to: GlistenApplyTo,
    /// How strongly the glisten blends with the base (0.0-1.0, default 0.7)
    #[serde(default = "default_blend_strength")]
    pub blend_strength: f32,
    /// Optional runtime parameter key that overrides `blend_strength` per
    /// frame. Resolved values are clamped to 0.0-1.0. Missing or
    /// unresolvable bindings fall back to the static `blend_strength`.
    ///
    /// Used together with `direction_binding` so apps can smoothly fade the
    /// glisten on/off via blend strength and flip its direction (e.g. on
    /// hover enter vs leave) from the same runtime params map.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blend_strength_binding: Option<String>,
}

fn default_speed() -> f32 {
    1.0
}
fn default_band_width() -> u16 {
    6
}
fn default_angle() -> f32 {
    25.0
}
fn default_blend_strength() -> f32 {
    0.7
}

impl Default for GlistenBandShader {
    fn default() -> Self {
        Self {
            speed: 1.0,
            speed_binding: None,
            band_width: 6,
            angle_deg: 25.0,
            head: ColorConfig::LightYellow,
            tail: ColorConfig::Rgb {
                r: 200,
                g: 160,
                b: 0,
            },
            direction: GlistenDirection::Forward,
            direction_binding: None,
            repeat_count: 0,
            apply_to: GlistenApplyTo::Foreground,
            blend_strength: 0.7,
            blend_strength_binding: None,
        }
    }
}

impl GlistenBandShader {
    /// Resolve the effective direction for the current frame, honoring
    /// `direction_binding` when a u16 runtime parameter is available.
    /// Value mapping: 0 = Forward, 1 = Reverse, 2 = PingPong. Out-of-range
    /// values, missing bindings, and type mismatches fall back to the
    /// static `direction`.
    fn effective_direction(&self, ctx: &ShaderContext) -> GlistenDirection {
        self.direction_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_u16(binding))
            .and_then(|code| match code {
                0 => Some(GlistenDirection::Forward),
                1 => Some(GlistenDirection::Reverse),
                2 => Some(GlistenDirection::PingPong),
                _ => None,
            })
            .unwrap_or(self.direction)
    }

    /// Resolve the effective blend strength for the current frame, clamped
    /// to the legal 0.0-1.0 blend range.
    fn effective_blend_strength(&self, ctx: &ShaderContext) -> f32 {
        self.blend_strength_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_f32(binding))
            .unwrap_or(self.blend_strength)
            .clamp(0.0, 1.0)
    }

    /// Resolve the effective sweep-rate multiplier for the current frame,
    /// honoring `speed_binding` when an f32 runtime parameter is available
    /// and clamping the result to the legal 0.1..=10.0 range to prevent
    /// degenerate sweep behavior (static bands or flicker).
    fn effective_speed(&self, ctx: &ShaderContext) -> f32 {
        self.speed_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_f32(binding))
            .unwrap_or(self.speed)
            .clamp(0.1, 10.0)
    }
}

impl StyleShader for GlistenBandShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let t = ctx.t as f32;
        let (x, y, width, height) = (ctx.local_x, ctx.local_y, ctx.width, ctx.height);

        // Resolve runtime-parameter bindings for this frame. direction is
        // used in the effective_t match below; blend_strength replaces
        // self.blend_strength where the glisten color is mixed into base;
        // speed multiplies the t-driven sweep rate (reconnected in 2.2.0).
        let effective_direction = self.effective_direction(ctx);
        let effective_blend_strength = self.effective_blend_strength(ctx);
        let effective_speed = self.effective_speed(ctx);

        // Convert angle to radians
        let angle_rad = self.angle_deg.to_radians();
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();

        // Project the cell position onto the band's normal vector
        // This gives us the "distance" along the direction the band moves
        let proj = (x as f32) * cos_a + (y as f32) * sin_a;

        // Calculate the maximum projection to normalize
        let max_proj = (width as f32) * cos_a.abs() + (height as f32) * sin_a.abs();

        // Calculate effective t based on direction and repeat_count.
        // scaled_t multiplies the raw t by the resolved speed so the sweep
        // rate tracks `speed` (reconnected in 2.2.0 — prior to that release
        // this line was `let scaled_t = t;` and the speed field was
        // vestigial).
        let effective_t = {
            let scaled_t = t * effective_speed;
            let cycle_t = if self.repeat_count > 0 {
                // Clamp to repeat_count cycles
                scaled_t.min(self.repeat_count as f32)
            } else {
                scaled_t
            };

            match effective_direction {
                GlistenDirection::Forward => cycle_t.fract(),
                GlistenDirection::Reverse => 1.0 - cycle_t.fract(),
                GlistenDirection::PingPong => {
                    // Oscillate: 0->1->0->1...
                    let phase = cycle_t.fract() * 2.0;
                    if phase <= 1.0 { phase } else { 2.0 - phase }
                }
            }
        };

        // Band position moves based on effective_t
        // The band sweeps across the entire range plus its width
        let sweep_range = max_proj + self.band_width as f32;
        let band_center = effective_t * sweep_range - (self.band_width as f32 / 2.0);

        // Calculate distance from the band center
        let dist_from_center = (proj - band_center).abs();
        let half_band = self.band_width as f32 / 2.0;

        // If outside the band, return base style
        if dist_from_center > half_band {
            return base;
        }

        // Calculate intensity within the band (1.0 at center, 0.0 at edges)
        let intensity = 1.0 - (dist_from_center / half_band);

        // Blend from tail -> head based on intensity
        let head_color: Color = self.head.into();
        let tail_color: Color = self.tail.into();

        // Extract RGB from colors (tui_vfx_types::Color is always RGB with alpha)
        let (hr, hg, hb) = (head_color.r, head_color.g, head_color.b);
        let (tr, tg, tb) = (tail_color.r, tail_color.g, tail_color.b);

        // Blend tail -> head based on intensity
        let r = ((tr as f32) + ((hr as f32 - tr as f32) * intensity)) as u8;
        let g = ((tg as f32) + ((hg as f32 - tg as f32) * intensity)) as u8;
        let b = ((tb as f32) + ((hb as f32 - tb as f32) * intensity)) as u8;

        let glisten_color = Color::rgb(r, g, b);
        let blend_amount = intensity * effective_blend_strength;

        // Apply the glisten color based on apply_to setting
        let mut style = base;
        match self.apply_to {
            GlistenApplyTo::Foreground => {
                if base.fg != Color::TRANSPARENT {
                    style.fg = blend_colors(base.fg, glisten_color, blend_amount, ColorSpace::Rgb);
                } else {
                    style.fg = glisten_color;
                }
            }
            GlistenApplyTo::Background => {
                if base.bg != Color::TRANSPARENT {
                    style.bg = blend_colors(base.bg, glisten_color, blend_amount, ColorSpace::Rgb);
                } else {
                    style.bg = glisten_color;
                }
            }
            GlistenApplyTo::Both => {
                if base.fg != Color::TRANSPARENT {
                    style.fg = blend_colors(base.fg, glisten_color, blend_amount, ColorSpace::Rgb);
                } else {
                    style.fg = glisten_color;
                }
                if base.bg != Color::TRANSPARENT {
                    style.bg =
                        blend_colors(base.bg, glisten_color, blend_amount * 0.5, ColorSpace::Rgb);
                }
            }
        }

        style
    }
}

#[cfg(test)]
mod binding_tests {
    use super::*;
    use crate::traits::{ShaderRuntimeParamValue, ShaderRuntimeParams};
    use std::sync::Arc;

    fn ctx_with_params(params: ShaderRuntimeParams) -> ShaderContext {
        ShaderContext::new(0, 0, 10, 5, 0, 0, 0.0, None, Some(Arc::new(params)))
    }

    fn shader() -> GlistenBandShader {
        GlistenBandShader::default()
    }

    #[test]
    fn effective_direction_resolves_code_zero_as_forward() {
        let mut s = shader();
        s.direction = GlistenDirection::PingPong;
        s.direction_binding = Some("dir".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("dir", ShaderRuntimeParamValue::Integer(0));
        let ctx = ctx_with_params(params);
        assert_eq!(s.effective_direction(&ctx), GlistenDirection::Forward);
    }

    #[test]
    fn effective_direction_resolves_code_one_as_reverse() {
        let mut s = shader();
        s.direction_binding = Some("dir".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("dir", ShaderRuntimeParamValue::Integer(1));
        let ctx = ctx_with_params(params);
        assert_eq!(s.effective_direction(&ctx), GlistenDirection::Reverse);
    }

    #[test]
    fn effective_direction_resolves_code_two_as_ping_pong() {
        let mut s = shader();
        s.direction_binding = Some("dir".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("dir", ShaderRuntimeParamValue::Integer(2));
        let ctx = ctx_with_params(params);
        assert_eq!(s.effective_direction(&ctx), GlistenDirection::PingPong);
    }

    #[test]
    fn effective_direction_out_of_range_falls_back_to_static() {
        let mut s = shader();
        s.direction = GlistenDirection::Reverse;
        s.direction_binding = Some("dir".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("dir", ShaderRuntimeParamValue::Integer(99));
        let ctx = ctx_with_params(params);
        assert_eq!(s.effective_direction(&ctx), GlistenDirection::Reverse);
    }

    #[test]
    fn effective_direction_missing_param_falls_back_to_static() {
        let mut s = shader();
        s.direction = GlistenDirection::PingPong;
        s.direction_binding = Some("missing".to_string());
        let ctx = ctx_with_params(ShaderRuntimeParams::new());
        assert_eq!(s.effective_direction(&ctx), GlistenDirection::PingPong);
    }

    #[test]
    fn effective_blend_strength_resolves_and_clamps() {
        let mut s = shader();
        s.blend_strength = 0.2;
        s.blend_strength_binding = Some("bs".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("bs", ShaderRuntimeParamValue::Float(0.9));
        let ctx = ctx_with_params(params);
        assert!((s.effective_blend_strength(&ctx) - 0.9).abs() < 1e-6);
    }

    #[test]
    fn effective_blend_strength_clamps_above_one() {
        let mut s = shader();
        s.blend_strength_binding = Some("bs".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("bs", ShaderRuntimeParamValue::Float(1.7));
        let ctx = ctx_with_params(params);
        assert_eq!(s.effective_blend_strength(&ctx), 1.0);
    }

    #[test]
    fn effective_blend_strength_missing_binding_falls_back_and_clamps_static() {
        let mut s = shader();
        s.blend_strength = 2.5; // out-of-range static
        s.blend_strength_binding = Some("missing".to_string());
        let ctx = ctx_with_params(ShaderRuntimeParams::new());
        assert_eq!(s.effective_blend_strength(&ctx), 1.0);
    }

    #[test]
    fn effective_speed_defaults_to_one_without_binding() {
        let s = shader();
        let ctx = ctx_with_params(ShaderRuntimeParams::new());
        assert!((s.effective_speed(&ctx) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn effective_speed_returns_static_field_when_no_binding() {
        let mut s = shader();
        s.speed = 2.5;
        let ctx = ctx_with_params(ShaderRuntimeParams::new());
        assert!((s.effective_speed(&ctx) - 2.5).abs() < 1e-6);
    }

    #[test]
    fn effective_speed_resolves_binding() {
        let mut s = shader();
        s.speed = 1.0;
        s.speed_binding = Some("sweep_speed".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("sweep_speed", ShaderRuntimeParamValue::Float(3.5));
        let ctx = ctx_with_params(params);
        assert!((s.effective_speed(&ctx) - 3.5).abs() < 1e-6);
    }

    #[test]
    fn effective_speed_clamps_below_lower_bound() {
        let mut s = shader();
        s.speed_binding = Some("sp".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("sp", ShaderRuntimeParamValue::Float(0.01));
        let ctx = ctx_with_params(params);
        assert!((s.effective_speed(&ctx) - 0.1).abs() < 1e-6);
    }

    #[test]
    fn effective_speed_clamps_above_upper_bound() {
        let mut s = shader();
        s.speed_binding = Some("sp".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("sp", ShaderRuntimeParamValue::Float(999.0));
        let ctx = ctx_with_params(params);
        assert!((s.effective_speed(&ctx) - 10.0).abs() < 1e-6);
    }

    #[test]
    fn effective_speed_missing_binding_falls_back_to_clamped_static() {
        let mut s = shader();
        s.speed = 50.0; // out-of-range static
        s.speed_binding = Some("missing".to_string());
        let ctx = ctx_with_params(ShaderRuntimeParams::new());
        assert!((s.effective_speed(&ctx) - 10.0).abs() < 1e-6);
    }

    #[test]
    fn style_at_reconnects_speed_to_produce_different_sweep_positions() {
        use crate::traits::StyleShader;
        let mut slow = shader();
        slow.speed = 0.5;
        slow.band_width = 10;
        slow.angle_deg = 0.0; // horizontal sweep so x drives the projection

        let mut fast = slow.clone();
        fast.speed = 4.0;

        // Same t, same cell — different speed must produce distinct
        // effective band positions. We probe style_at at a known cell
        // roughly one band-width from the origin at t = 0.2 so that the
        // slow sweep hasn't reached the cell yet but the fast one has.
        let params = ShaderRuntimeParams::new();
        let ctx = ShaderContext::new(5, 0, 50, 5, 0, 0, 0.2, None, Some(Arc::new(params)));

        let base = tui_vfx_types::Style::default();
        let slow_style = slow.style_at(&ctx, base);
        let fast_style = fast.style_at(&ctx, base);

        // At least one channel must differ once speed is reconnected.
        assert!(
            slow_style != fast_style,
            "reconnected speed should change sweep position at a fixed t, \
             but slow={slow_style:?} and fast={fast_style:?} are identical"
        );
    }
}

// <FILE>tui-vfx-style/src/models/cls_glisten_band_shader.rs</FILE> - <DESC>GlistenBand shader implementation</DESC>
// <VERS>END OF VERSION: 2.2.0</VERS>
