// <FILE>tui-vfx-style/src/models/cls_trace_path_shader.rs</FILE> - <DESC>TracePath shader implementation</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Introduce authored routed traces as the deterministic foundation for later auto-routing</WCTX>
// <CLOG>Add TracePath spatial shader with explicit orthogonal polylines, per-route delay, and turn emphasis</CLOG>

use crate::models::{
    ColorConfig,
    cls_trace_common::{
        TraceApplyTo, TracePolyline, blend_trace_target, project_onto_polyline,
        weighted_polyline_total_length, weighted_progress_for_projection,
    },
};
use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// How the illuminated tail should behave along an authored route.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum TraceTailMode {
    /// Tail spans the full authored path continuously.
    #[default]
    Path,
    /// Tail is constrained to the active segment, with only a small corner carry-over.
    Segment,
}

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

    /// Perceptual weight applied to vertical travel.
    ///
    /// Values above 1.0 make vertical segments consume more route time,
    /// which helps tall terminal cells read more smoothly during turns.
    #[serde(default = "default_vertical_weight")]
    pub vertical_weight: f32,

    /// Maximum route thickness in cells.
    ///
    /// `1` means a single-cell centerline. Larger values expand outward from
    /// the route centerline.
    #[serde(default = "default_thickness")]
    pub thickness: u16,

    /// Blend strength.
    #[serde(default = "default_intensity")]
    pub intensity: f32,

    /// Extra emphasis when the pulse passes a turn/junction.
    #[serde(default = "default_junction_boost")]
    pub junction_boost: f32,

    /// Additional local brightness at the turn cell itself.
    #[serde(default = "default_junction_glow")]
    pub junction_glow: f32,

    /// Tail behavior along the route.
    #[serde(default)]
    pub tail_mode: TraceTailMode,

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

fn default_vertical_weight() -> f32 {
    1.0
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

fn default_junction_glow() -> f32 {
    0.15
}

impl Default for TracePathShader {
    fn default() -> Self {
        Self {
            color: ColorConfig::Cyan,
            speed: default_speed(),
            tail_length: default_tail_length(),
            vertical_weight: default_vertical_weight(),
            thickness: default_thickness(),
            intensity: default_intensity(),
            junction_boost: default_junction_boost(),
            junction_glow: default_junction_glow(),
            tail_mode: TraceTailMode::Path,
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
    fn max_distance_for_thickness(&self) -> f32 {
        self.thickness.saturating_sub(1) as f32
    }

    fn tail_intensity_profile(&self, behind_head: f32) -> f32 {
        let normalized = (behind_head / self.tail_length).clamp(0.0, 1.0);
        let strong_zone = 0.35_f32;

        if normalized <= strong_zone {
            1.0
        } else {
            let fade_t = (normalized - strong_zone) / (1.0 - strong_zone);
            1.0 - fade_t.powf(1.6)
        }
    }

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

    fn head_progress(&self, total_length: f32, path: &TracePolyline, t: f32) -> f32 {
        let effective_t = (t * self.speed - path.delay).max(0.0);
        effective_t.fract() * (total_length + self.tail_length)
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

            if projection.distance > self.max_distance_for_thickness() {
                continue;
            }

            let weighted_total_length = weighted_polyline_total_length(path, self.vertical_weight);
            if weighted_total_length <= 0.0 {
                continue;
            }

            let head = self.head_progress(weighted_total_length, path, ctx.t as f32);
            let (weighted_progress, weighted_segment_progress, weighted_segment_length) =
                weighted_progress_for_projection(path, &projection, self.vertical_weight);
            let path_factor = match self.tail_mode {
                TraceTailMode::Path => {
                    let behind_head = head - weighted_progress;
                    if behind_head < 0.0 || behind_head > self.tail_length {
                        continue;
                    }
                    self.tail_intensity_profile(behind_head)
                }
                TraceTailMode::Segment => {
                    let traversed_before_segment = weighted_progress - weighted_segment_progress;
                    let segment_head = head - traversed_before_segment;
                    if segment_head < 0.0
                        || segment_head > weighted_segment_length + self.tail_length
                    {
                        continue;
                    }

                    let behind_segment_head = segment_head - weighted_segment_progress;
                    let in_current_tail =
                        behind_segment_head >= 0.0 && behind_segment_head <= self.tail_length;

                    if in_current_tail {
                        self.tail_intensity_profile(behind_segment_head)
                    } else if projection.at_turn {
                        let overshoot = projection.segment_progress - segment_head;
                        if overshoot >= 0.0 && overshoot <= self.junction_glow.max(0.001) {
                            1.0 - (overshoot / self.junction_glow.max(0.001))
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
            };
            let thickness_span = self.max_distance_for_thickness().max(0.0001);
            let thickness_factor = if self.thickness <= 1 {
                1.0
            } else {
                1.0 - (projection.distance / thickness_span)
            };
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
        assert_eq!(shader.vertical_weight, 1.0);
        assert_eq!(shader.thickness, 1);
        assert_eq!(shader.intensity, 0.85);
        assert_eq!(shader.junction_boost, 0.2);
        assert_eq!(shader.junction_glow, 0.15);
        assert_eq!(shader.tail_mode, TraceTailMode::Path);
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
            vertical_weight: 1.0,
            thickness: 1,
            intensity: 1.0,
            junction_boost: 0.0,
            junction_glow: 0.15,
            tail_mode: TraceTailMode::Path,
            apply_to: TraceApplyTo::Foreground,
            paths: vec![demo_path()],
        };
        let ctx = make_ctx_at(4, 0, 12, 12, 0.35);
        let base = make_style();
        let result = shader.style_at(&ctx, base);
        assert_ne!(result.fg, base.fg);
    }

    #[test]
    fn thickness_one_keeps_single_cell_centerline() {
        let shader = TracePathShader {
            thickness: 1,
            paths: vec![demo_path()],
            ..Default::default()
        };
        let base = make_style();

        let on_path = shader.style_at(&make_ctx_at(4, 0, 12, 12, 0.35), base);
        let off_path_adjacent = shader.style_at(&make_ctx_at(4, 1, 12, 12, 0.35), base);

        assert_ne!(on_path, base);
        assert_eq!(off_path_adjacent, base);
    }

    #[test]
    fn serde_roundtrip_preserves_values() {
        let shader = TracePathShader {
            color: ColorConfig::Green,
            speed: 1.5,
            tail_length: 9.0,
            vertical_weight: 1.8,
            thickness: 2,
            intensity: 0.75,
            junction_boost: 0.35,
            junction_glow: 0.2,
            tail_mode: TraceTailMode::Segment,
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
