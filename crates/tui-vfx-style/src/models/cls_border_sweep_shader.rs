//! Border-focused traveling-band / sweep shader.
//!
//! ## V3 family note
//!
//! `BorderSweepShader` belongs to the broader traveling-band / sweep family
//! identified in the V3 capability catalog and style-model restructure
//! inventory. It is currently a direct flat-variant implementation, but it is a
//! likely candidate for future convergence with sibling sweep-style shaders such
//! as `Reflect`, `GlistenBand`, `TracePropagation`, and `TracePath`.
//!
// <FILE>tui-vfx-style/src/models/cls_border_sweep_shader.rs</FILE> - <DESC>Border sweep shader implementation</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Fix sticky wrap/corner artifacts in border sweeps. The old perimeter unwrap used an approximate position formula that could assign left-edge cells positions beyond the perimeter, making them light at the wrong time and creating visible end-of-loop stickiness.</WCTX>
// <CLOG>Add optional head/tail colors so V3 traveling-band head_tail lowering can execute without changing solid-color border sweep behavior.</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct BorderSweepShader {
    #[config(default = 1.0)]
    pub speed: f32,
    #[config(default = 5)]
    pub length: u16,
    /// Solid-color fallback used when `head` / `tail` are absent.
    pub color: ColorConfig,
    /// Optional leading-edge color for V3 `head_tail` traveling-band lowering.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head: Option<ColorConfig>,
    /// Optional trailing-tail color for V3 `head_tail` traveling-band lowering.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tail: Option<ColorConfig>,
    /// Optional runtime parameter key used to override the sweep position
    /// at render time. When present, the compositor looks up this key in
    /// `ShaderRuntimeParams` and uses the resolved value (a normalized
    /// 0.0-1.0 ratio along the perimeter) instead of the time-driven sweep
    /// position computed from `ctx.t`. Missing bindings fall back to the
    /// time-based default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_binding: Option<String>,
}

impl Default for BorderSweepShader {
    fn default() -> Self {
        Self {
            speed: 1.0,
            length: 5,
            color: ColorConfig::White,
            head: None,
            tail: None,
            position_binding: None,
        }
    }
}

impl BorderSweepShader {
    fn has_head_tail_policy(&self) -> bool {
        self.head.is_some() || self.tail.is_some()
    }

    fn head_color(&self) -> &ColorConfig {
        self.head.as_ref().unwrap_or(&self.color)
    }

    fn tail_color(&self) -> &ColorConfig {
        self.tail.as_ref().unwrap_or(&self.color)
    }

    fn band_color(&self, intensity: f32) -> Color {
        blend_colors(
            Color::from(*self.tail_color()),
            Color::from(*self.head_color()),
            intensity.clamp(0.0, 1.0),
            ColorSpace::Rgb,
        )
    }
}

impl StyleShader for BorderSweepShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let (x, y, width, height) = (ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        if width < 2 || height < 2 {
            return base;
        }

        let t = ctx.t as f32;

        // Only affects border cells
        if x > 0 && x < width - 1 && y > 0 && y < height - 1 {
            return base;
        }
        // Unwrap each unique border cell clockwise:
        // top row (including both top corners), right edge (excluding top
        // corner, including bottom-right), bottom row (excluding right
        // corner, including bottom-left), then left edge (excluding both
        // corners). This keeps every border cell in [0, perimeter) exactly
        // once, avoiding sticky wrap artifacts near the end of the loop.
        let perimeter = 2_u32 * (u32::from(width) + u32::from(height)) - 4;
        let pos = if y == 0 {
            u32::from(x)
        } else if x == width - 1 {
            u32::from(width) + u32::from(y - 1)
        } else if y == height - 1 {
            u32::from(width) + u32::from(height - 1) + u32::from(width - 2 - x)
        } else {
            u32::from(width)
                + u32::from(height - 1)
                + u32::from(width - 1)
                + u32::from(height - 2 - y)
        } as f32;
        // Resolve sweep position: runtime binding takes precedence over
        // the time-based default. A binding value of 0.0-1.0 maps linearly
        // onto [0, perimeter); values outside that range wrap via rem_euclid
        // so apps can feed raw progress counters without clamping.
        let bound_ratio = self
            .position_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_f32(binding));
        let sweep_pos = match bound_ratio {
            Some(ratio) => (ratio * perimeter as f32).rem_euclid(perimeter as f32),
            None => (t * self.speed * perimeter as f32).rem_euclid(perimeter as f32),
        };
        let mut style = base;
        if self.has_head_tail_policy() {
            let behind_head = (sweep_pos - pos).rem_euclid(perimeter as f32);
            if behind_head < self.length as f32 {
                let intensity = 1.0 - (behind_head / self.length.max(1) as f32);
                style.fg = self.band_color(intensity);
            }
        } else {
            let dist = (sweep_pos - pos)
                .abs()
                .min(perimeter as f32 - (sweep_pos - pos).abs());
            if dist < self.length as f32 {
                style.fg = Color::from(self.color);
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
    use tui_vfx_types::Color;

    fn ctx_at(local_x: u16, local_y: u16, t: f64, params: ShaderRuntimeParams) -> ShaderContext {
        ShaderContext::new(
            local_x,
            local_y,
            10,
            5,
            0,
            0,
            t,
            None,
            Some(Arc::new(params)),
        )
    }

    #[test]
    fn position_binding_overrides_time_based_sweep() {
        // A binding value of 0.0 forces the sweep position to the very start
        // of the perimeter (top-left corner at (0, 0)) regardless of ctx.t.
        // Pinning the sweep lets us assert that the top-left cell is lit
        // even though ctx.t = 0.9 would normally put the sweep far along.
        let shader = BorderSweepShader {
            speed: 1.0,
            length: 2,
            color: ColorConfig::Red,
            head: None,
            tail: None,
            position_binding: Some("sweep".to_string()),
        };
        let mut params = ShaderRuntimeParams::new();
        params.insert("sweep", ShaderRuntimeParamValue::Float(0.0));
        let ctx = ctx_at(0, 0, 0.9, params);

        let base = tui_vfx_types::Style::default();
        let styled = shader.style_at(&ctx, base);
        assert_eq!(styled.fg, Color::from(ColorConfig::Red));
    }

    #[test]
    fn position_binding_missing_falls_back_to_time_driven_sweep() {
        // With the binding key absent from runtime_params the sweep should
        // behave identically to the pre-P0.3 time-driven computation, so
        // bound-but-unresolved and unbound shaders produce the same output.
        let with_binding = BorderSweepShader {
            speed: 1.0,
            length: 2,
            color: ColorConfig::Red,
            head: None,
            tail: None,
            position_binding: Some("missing".to_string()),
        };
        let without_binding = BorderSweepShader {
            position_binding: None,
            ..with_binding.clone()
        };
        let ctx = ctx_at(0, 0, 0.0, ShaderRuntimeParams::new());
        let base = tui_vfx_types::Style::default();
        assert_eq!(
            with_binding.style_at(&ctx, base),
            without_binding.style_at(&ctx, base),
        );
    }

    #[test]
    fn speed_scales_time_driven_sweep_position() {
        let slow = BorderSweepShader {
            speed: 0.5,
            length: 1,
            color: ColorConfig::Red,
            head: None,
            tail: None,
            position_binding: None,
        };
        let normal = BorderSweepShader {
            speed: 1.0,
            ..slow.clone()
        };
        let ctx = ctx_at(6, 0, 0.5, ShaderRuntimeParams::new());
        let base = tui_vfx_types::Style::default();

        assert_ne!(
            slow.style_at(&ctx, base),
            normal.style_at(&ctx, base),
            "speed must scale the time-driven border sweep position"
        );
        assert_eq!(slow.style_at(&ctx, base).fg, Color::from(ColorConfig::Red));
        assert_ne!(
            normal.style_at(&ctx, base).fg,
            Color::from(ColorConfig::Red)
        );
    }

    #[test]
    fn left_edge_does_not_stick_at_loop_start() {
        let shader = BorderSweepShader {
            speed: 1.0,
            length: 1,
            color: ColorConfig::Red,
            head: None,
            tail: None,
            position_binding: None,
        };
        let ctx = ctx_at(0, 1, 0.0, ShaderRuntimeParams::new());
        let base = tui_vfx_types::Style::default();

        assert_ne!(
            shader.style_at(&ctx, base).fg,
            Color::from(ColorConfig::Red),
            "left edge must not light at t=0 unless the sweep actually reaches the end of the perimeter"
        );
    }

    #[test]
    fn head_tail_policy_colors_the_clockwise_head_and_tail_losslessly() {
        let shader = BorderSweepShader {
            speed: 1.0,
            length: 3,
            color: ColorConfig::Red,
            head: Some(ColorConfig::White),
            tail: Some(ColorConfig::Black),
            position_binding: Some("sweep".to_string()),
        };
        let mut params = ShaderRuntimeParams::new();
        params.insert("sweep", ShaderRuntimeParamValue::Float(2.0 / 26.0));
        let base = tui_vfx_types::Style::default();

        let head = shader.style_at(&ctx_at(2, 0, 0.0, params.clone()), base);
        let tail = shader.style_at(&ctx_at(1, 0, 0.0, params), base);

        assert_eq!(head.fg, Color::WHITE);
        assert_eq!(tail.fg, Color::rgb(169, 169, 169));
    }
}

// <FILE>tui-vfx-style/src/models/cls_border_sweep_shader.rs</FILE> - <DESC>Border sweep shader implementation</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
