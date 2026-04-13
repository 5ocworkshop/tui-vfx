// <FILE>tui-vfx-style/src/models/cls_trace_path_shader.rs</FILE> - <DESC>TracePath shader implementation</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Introduce authored routed traces as the deterministic foundation for later auto-routing</WCTX>
// <CLOG>Add TracePath spatial shader with explicit orthogonal polylines, per-route delay, and turn emphasis</CLOG>

use crate::models::{
    ColorConfig,
    cls_trace_common::{
        TraceApplyTo, TracePolyline, blend_trace_target, polyline_total_length,
        project_onto_polyline,
    },
};
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Authored trace route shader.
///
/// `TracePath` follows explicit waypoints instead of inferring a lane field.
/// That makes it suitable for blueprint schematics, PCB routes, and any case
/// where the motion should follow an intentional path silhouette.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct TracePathShader {
    /// Trace pulse color.
    pub color: ColorConfig,

    /// Travel speed multiplier for the pulse head.
    #[serde(default = "default_speed")]
    pub speed: f32,

    /// Length of the illuminated pulse tail in cells.
    #[serde(default = "default_tail_length")]
    pub tail_length: f32,

    /// Maximum route thickness in cells.
    #[serde(default = "default_thickness")]
    pub thickness: u16,

    /// Blend strength.
    #[serde(default = "default_intensity")]
    pub intensity: f32,

    /// Extra emphasis when the pulse passes a turn/junction.
    #[serde(default = "default_junction_boost")]
    pub junction_boost: f32,

    /// Which channel(s) to affect.
    #[serde(default)]
    pub apply_to: TraceApplyTo,

    /// One or more authored routes.
    pub paths: Vec<TracePolyline>,
}

fn default_speed() -> f32 {
    1.0
}

fn default_tail_length() -> f32 {
    7.0
}

fn default_thickness() -> u16 {
    1
}

fn default_intensity() -> f32 {
    0.85
}

fn default_junction_boost() -> f32 {
    0.2
}

impl Default for TracePathShader {
    fn default() -> Self {
        Self {
            color: ColorConfig::Cyan,
            speed: default_speed(),
            tail_length: default_tail_length(),
            thickness: default_thickness(),
            intensity: default_intensity(),
            junction_boost: default_junction_boost(),
            apply_to: TraceApplyTo::Foreground,
            paths: vec![TracePolyline {
                points: vec![
                    crate::models::cls_trace_common::TracePoint { x: 0, y: 0 },
                    crate::models::cls_trace_common::TracePoint { x: 4, y: 0 },
                    crate::models::cls_trace_common::TracePoint { x: 4, y: 3 },
                ],
                delay: 0.0,
            }],
        }
    }
}

impl TracePathShader {
    fn apply_channel(&self, base: Style, trace_color: Color, alpha: f32) -> Style {
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
}

impl StyleShader for TracePathShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.paths.is_empty()
            || self.intensity <= 0.0
            || self.tail_length <= 0.0
            || self.thickness == 0
        {
            return base;
        }

        let trace_color: Color = self.color.into();
        let cell_x = ctx.local_x as f32;
        let cell_y = ctx.local_y as f32;
        let mut best_alpha = 0.0_f32;

        for path in &self.paths {
            let Some(projection) = project_onto_polyline(cell_x, cell_y, path) else {
                continue;
            };

            if projection.distance > self.thickness as f32 {
                continue;
            }

            let total_length = polyline_total_length(path);
            if total_length <= 0.0 {
                continue;
            }

            let effective_t = (ctx.t as f32 * self.speed - path.delay).max(0.0);
            let head = effective_t.fract() * (total_length + self.tail_length);
            let behind_head = head - projection.progress;

            if behind_head < 0.0 || behind_head > self.tail_length {
                continue;
            }

            let path_factor = 1.0 - (behind_head / self.tail_length);
            let thickness_factor = 1.0 - (projection.distance / self.thickness as f32);
            let junction_factor = if projection.at_turn {
                1.0 + self.junction_boost
            } else {
                1.0
            };

            let alpha =
                (path_factor * thickness_factor * self.intensity * junction_factor).clamp(0.0, 1.0);
            best_alpha = best_alpha.max(alpha);
        }

        if best_alpha <= 0.0 {
            base
        } else {
            self.apply_channel(base, trace_color, best_alpha)
        }
    }

    fn name(&self) -> &'static str {
        "TracePath"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::cls_trace_common::{TracePoint, TracePolyline};
    use crate::models::test_support::{make_ctx_at, make_style};

    fn demo_path() -> TracePolyline {
        TracePolyline {
            points: vec![
                TracePoint { x: 0, y: 0 },
                TracePoint { x: 4, y: 0 },
                TracePoint { x: 4, y: 3 },
            ],
            delay: 0.0,
        }
    }

    #[test]
    fn default_values_are_valid() {
        let shader = TracePathShader::default();
        assert_eq!(shader.speed, 1.0);
        assert_eq!(shader.tail_length, 7.0);
        assert_eq!(shader.thickness, 1);
        assert_eq!(shader.intensity, 0.85);
        assert_eq!(shader.junction_boost, 0.2);
        assert!(!shader.paths.is_empty());
    }

    #[test]
    fn off_path_cells_are_untouched() {
        let shader = TracePathShader {
            paths: vec![demo_path()],
            ..Default::default()
        };
        let ctx = make_ctx_at(8, 8, 12, 12, 0.5);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_eq!(result, base);
    }

    #[test]
    fn authored_path_cells_light_up() {
        let shader = TracePathShader {
            color: ColorConfig::Rgb {
                r: 80,
                g: 255,
                b: 200,
            },
            speed: 1.0,
            tail_length: 6.0,
            thickness: 1,
            intensity: 1.0,
            junction_boost: 0.0,
            apply_to: TraceApplyTo::Foreground,
            paths: vec![demo_path()],
        };
        let ctx = make_ctx_at(4, 0, 12, 12, 0.35);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_ne!(result.fg, base.fg);
    }

    #[test]
    fn serde_roundtrip_preserves_values() {
        let shader = TracePathShader {
            color: ColorConfig::Green,
            speed: 1.5,
            tail_length: 9.0,
            thickness: 2,
            intensity: 0.75,
            junction_boost: 0.35,
            apply_to: TraceApplyTo::Both,
            paths: vec![demo_path()],
        };
        let json = serde_json::to_string(&shader).unwrap();
        let parsed: TracePathShader = serde_json::from_str(&json).unwrap();
        assert_eq!(shader, parsed);
    }
}

// <FILE>tui-vfx-style/src/models/cls_trace_path_shader.rs</FILE> - <DESC>TracePath shader implementation</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
