// <FILE>tui-vfx-style/src/models/cls_border_sweep_shader.rs</FILE> - <DESC>Border sweep shader implementation</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Phase 0 P0.3 — add position_binding for runtime-parameter sweep position override</WCTX>
// <CLOG>Add position_binding: Option<String> so apps can drive the sweep position from a runtime parameter instead of loop_t</CLOG>

use crate::models::ColorConfig;
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct BorderSweepShader {
    #[config(default = 1.0)]
    pub speed: f32,
    #[config(default = 5)]
    pub length: u16,
    pub color: ColorConfig,
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
            position_binding: None,
        }
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
        // Unwrap perimeter: Top -> Right -> Bottom -> Left
        let perimeter = 2_u32 * (u32::from(width) + u32::from(height)) - 4; // Approx
        let pos = if y == 0 {
            u32::from(x) // Top edge
        } else if x == width - 1 {
            u32::from(width) + u32::from(y) // Right edge
        } else if y == height - 1 {
            u32::from(width) + u32::from(height) + (u32::from(width - 1 - x))
        } else {
            u32::from(width) + u32::from(height) + u32::from(width) + (u32::from(height - 1 - y))
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
            None => (t * perimeter as f32) % perimeter as f32,
        };
        let dist = (sweep_pos - pos)
            .abs()
            .min(perimeter as f32 - (sweep_pos - pos).abs());
        let mut style = base;
        if dist < self.length as f32 {
            style.fg = Color::from(self.color);
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
        ShaderContext::new(local_x, local_y, 10, 5, 0, 0, t, None, Some(Arc::new(params)))
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
}

// <FILE>tui-vfx-style/src/models/cls_border_sweep_shader.rs</FILE> - <DESC>Border sweep shader implementation</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
