// <FILE>tui-vfx-style/src/models/cls_trace_propagation_shader.rs</FILE> - <DESC>TracePropagation shader implementation</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Introduce orthogonal signal-flow visualization for schematic and PCB-inspired themes</WCTX>
// <CLOG>Add TracePropagation spatial shader with grid lanes, configurable origin, and traveling pulse tail</CLOG>

use crate::models::{
    ColorConfig,
    cls_trace_common::{
        TraceApplyTo, TraceOrigin, blend_trace_target, max_distance_from_origin, origin_point,
    },
};
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Orthogonal propagation shader for schematic and circuit-like flows.
///
/// The shader treats every Nth row and column as a trace lane, then sends a
/// pulse outward from a configurable origin using Manhattan distance. The
/// result feels routed and explicit rather than radial or decorative.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct TracePropagationShader {
    /// Trace pulse color.
    pub color: ColorConfig,

    /// Propagation speed multiplier.
    #[serde(default = "default_speed")]
    pub speed: f32,

    /// Distance in cells between trace lanes.
    #[serde(default = "default_grid_spacing")]
    pub grid_spacing: u16,

    /// Thickness of a trace lane in cells.
    #[serde(default = "default_line_width")]
    pub line_width: u16,

    /// Length of the pulse tail in cells.
    #[serde(default = "default_tail_length")]
    pub tail_length: f32,

    /// Blend strength.
    #[serde(default = "default_intensity")]
    pub intensity: f32,

    /// Starting point for the propagation.
    #[serde(default)]
    pub origin: TraceOrigin,

    /// Which channel(s) to affect.
    #[serde(default)]
    pub apply_to: TraceApplyTo,
}

fn default_speed() -> f32 {
    1.0
}

fn default_grid_spacing() -> u16 {
    6
}

fn default_line_width() -> u16 {
    1
}

fn default_tail_length() -> f32 {
    5.0
}

fn default_intensity() -> f32 {
    0.8
}

impl Default for TracePropagationShader {
    fn default() -> Self {
        Self {
            color: ColorConfig::Cyan,
            speed: default_speed(),
            grid_spacing: default_grid_spacing(),
            line_width: default_line_width(),
            tail_length: default_tail_length(),
            intensity: default_intensity(),
            origin: TraceOrigin::TopLeft,
            apply_to: TraceApplyTo::Foreground,
        }
    }
}

impl TracePropagationShader {
    fn axis_distance(coord: u16, spacing: u16) -> u16 {
        if spacing == 0 {
            return 0;
        }
        let remainder = coord % spacing;
        remainder.min(spacing.saturating_sub(remainder))
    }

    fn is_on_trace_lane(&self, x: u16, y: u16) -> (bool, bool) {
        if self.grid_spacing == 0 {
            return (true, true);
        }

        let threshold = self.line_width.max(1).saturating_sub(1);
        let vertical = Self::axis_distance(x, self.grid_spacing) <= threshold;
        let horizontal = Self::axis_distance(y, self.grid_spacing) <= threshold;
        (vertical, horizontal)
    }
}

impl StyleShader for TracePropagationShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.intensity <= 0.0 || self.tail_length <= 0.0 || ctx.width == 0 || ctx.height == 0 {
            return base;
        }

        let (vertical, horizontal) = self.is_on_trace_lane(ctx.local_x, ctx.local_y);
        if !vertical && !horizontal {
            return base;
        }

        let (origin_x, origin_y) = origin_point(self.origin, ctx.width, ctx.height);
        let distance =
            (ctx.local_x as f32 - origin_x).abs() + (ctx.local_y as f32 - origin_y).abs();
        let max_distance = max_distance_from_origin(origin_x, origin_y, ctx.width, ctx.height);
        let sweep_range = max_distance + self.tail_length;
        let head = (ctx.t as f32 * self.speed).fract() * sweep_range;
        let behind_head = head - distance;

        if behind_head < 0.0 || behind_head > self.tail_length {
            return base;
        }

        let band_factor = 1.0 - (behind_head / self.tail_length);
        let intersection_factor = if vertical && horizontal { 1.15 } else { 1.0 };
        let alpha = (band_factor * self.intensity * intersection_factor).clamp(0.0, 1.0);
        let trace_color: Color = self.color.into();

        let mut style = base;
        match self.apply_to {
            TraceApplyTo::Foreground => {
                style.fg = blend_trace_target(base.fg, trace_color, alpha);
            }
            TraceApplyTo::Background => {
                style.bg = blend_trace_target(base.bg, trace_color, alpha * 0.8);
            }
            TraceApplyTo::Both => {
                style.fg = blend_trace_target(base.fg, trace_color, alpha);
                style.bg = blend_trace_target(base.bg, trace_color, alpha * 0.65);
            }
        }

        style
    }

    fn name(&self) -> &'static str {
        "TracePropagation"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::test_support::{make_ctx_at, make_style};

    #[test]
    fn default_values_are_valid() {
        let shader = TracePropagationShader::default();
        assert_eq!(shader.speed, 1.0);
        assert_eq!(shader.grid_spacing, 6);
        assert_eq!(shader.line_width, 1);
        assert_eq!(shader.tail_length, 5.0);
        assert_eq!(shader.intensity, 0.8);
        assert_eq!(shader.origin, TraceOrigin::TopLeft);
    }

    #[test]
    fn non_lane_cells_are_untouched() {
        let shader = TracePropagationShader::default();
        let ctx = make_ctx_at(2, 2, 12, 12, 0.5);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result, base);
    }

    #[test]
    fn lane_cells_receive_trace_when_wave_passes() {
        let shader = TracePropagationShader {
            color: ColorConfig::Rgb {
                r: 80,
                g: 255,
                b: 200,
            },
            speed: 1.0,
            grid_spacing: 4,
            line_width: 1,
            tail_length: 5.0,
            intensity: 1.0,
            origin: TraceOrigin::TopLeft,
            apply_to: TraceApplyTo::Foreground,
        };
        let ctx = make_ctx_at(4, 0, 12, 12, 0.2);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_ne!(result.fg, base.fg);
    }

    #[test]
    fn serde_roundtrip_preserves_values() {
        let shader = TracePropagationShader {
            color: ColorConfig::Green,
            speed: 1.5,
            grid_spacing: 5,
            line_width: 2,
            tail_length: 7.0,
            intensity: 0.75,
            origin: TraceOrigin::Center,
            apply_to: TraceApplyTo::Both,
        };
        let json = serde_json::to_string(&shader).unwrap();
        let parsed: TracePropagationShader = serde_json::from_str(&json).unwrap();
        assert_eq!(shader, parsed);
    }
}

// <FILE>tui-vfx-style/src/models/cls_trace_propagation_shader.rs</FILE> - <DESC>TracePropagation shader implementation</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
