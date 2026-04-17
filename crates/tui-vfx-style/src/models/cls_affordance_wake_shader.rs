// <FILE>crates/tui-vfx-style/src/models/cls_affordance_wake_shader.rs</FILE> - <DESC>Latent affordance wake shader for dormant-to-active edge, corner, and rail cues</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Introduce a low-noise reveal-on-need primitive for secondary affordances</WCTX>
// <CLOG>Add AffordanceWakeShader with progress binding, latent baseline intensity, and zone-targeted wake behavior</CLOG>

use crate::models::{ColorConfig, ColorSpace, FalloffType};
use crate::traits::{
    ShaderContext, ShaderRuntimeBindingRequest, ShaderRuntimeBindingResolution,
    ShaderRuntimeBindingStatus, StyleShader,
};
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AffordanceWakeZone {
    #[default]
    AllEdges,
    Corners,
    LeftRail,
    RightRail,
    TopRail,
    BottomRail,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum AffordanceWakeApplyTo {
    Foreground,
    #[default]
    Background,
    Both,
}

/// Subtle secondary-affordance wake for edges, corners, and rails.
///
/// The shader is designed for dormant-to-active transitions: a low or absent
/// baseline at rest, then a quiet resolve when focus, hover, or another
/// contextual progress signal says the affordance should become more visible.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct AffordanceWakeShader {
    pub color: ColorConfig,
    #[serde(default)]
    pub zone: AffordanceWakeZone,
    #[serde(default = "default_radius")]
    pub radius: u8,
    #[serde(default)]
    pub falloff: FalloffType,
    #[serde(default = "default_progress")]
    pub progress: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_binding: Option<String>,
    #[serde(default = "default_rest_intensity")]
    pub rest_intensity: f32,
    #[serde(default = "default_peak_intensity")]
    pub peak_intensity: f32,
    #[serde(default)]
    pub apply_to: AffordanceWakeApplyTo,
}

fn default_radius() -> u8 {
    2
}

fn default_progress() -> f32 {
    0.0
}

fn default_rest_intensity() -> f32 {
    0.0
}

fn default_peak_intensity() -> f32 {
    0.25
}

impl Default for AffordanceWakeShader {
    fn default() -> Self {
        Self {
            color: ColorConfig::White,
            zone: AffordanceWakeZone::AllEdges,
            radius: default_radius(),
            falloff: FalloffType::Quadratic,
            progress: default_progress(),
            progress_binding: None,
            rest_intensity: default_rest_intensity(),
            peak_intensity: default_peak_intensity(),
            apply_to: AffordanceWakeApplyTo::Background,
        }
    }
}

impl AffordanceWakeShader {
    pub fn runtime_binding_requests(&self) -> Vec<ShaderRuntimeBindingRequest> {
        self.progress_binding
            .as_ref()
            .map(|binding| {
                vec![ShaderRuntimeBindingRequest {
                    field: "progress".to_string(),
                    binding: binding.clone(),
                    expected_type: "f32".to_string(),
                }]
            })
            .unwrap_or_default()
    }

    pub fn runtime_binding_resolutions(
        &self,
        ctx: &ShaderContext,
    ) -> Vec<ShaderRuntimeBindingResolution> {
        self.progress_binding
            .as_ref()
            .map(|binding| {
                let supplied = ctx.runtime_param(binding);
                let supplied_kind = supplied.map(|value| value.kind_name().to_string());
                let supplied_value = supplied.map(serde_json::Value::from);
                let coerced = matches!(supplied_kind.as_deref(), Some("integer"));
                let resolved = ctx
                    .runtime_param_f32(binding)
                    .map(|value| value.clamp(0.0, 1.0));
                ShaderRuntimeBindingResolution {
                    field: "progress".to_string(),
                    binding: binding.clone(),
                    expected_type: "f32".to_string(),
                    status: match resolved {
                        Some(_) if coerced => ShaderRuntimeBindingStatus::Coerced,
                        Some(_) => ShaderRuntimeBindingStatus::Resolved,
                        None => ShaderRuntimeBindingStatus::FallbackStatic,
                    },
                    supplied_kind,
                    supplied_value,
                    effective_value: resolved.map(serde_json::Value::from),
                    fallback_value: Some(serde_json::Value::from(self.progress.clamp(0.0, 1.0))),
                }
            })
            .map(|value| vec![value])
            .unwrap_or_default()
    }

    fn resolved_progress(&self, ctx: &ShaderContext) -> f32 {
        self.progress_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_f32(binding))
            .unwrap_or(self.progress)
            .clamp(0.0, 1.0)
    }

    fn zone_distance(&self, x: u16, y: u16, width: u16, height: u16) -> f32 {
        let max_x = width.saturating_sub(1) as f32;
        let max_y = height.saturating_sub(1) as f32;
        let x = x as f32;
        let y = y as f32;
        match self.zone {
            AffordanceWakeZone::AllEdges => {
                let top = y;
                let bottom = max_y - y;
                let left = x;
                let right = max_x - x;
                top.min(bottom).min(left).min(right)
            }
            AffordanceWakeZone::Corners => {
                let corners = [(0.0, 0.0), (max_x, 0.0), (0.0, max_y), (max_x, max_y)];
                corners
                    .iter()
                    .map(|(cx, cy)| ((x - cx).powi(2) + (y - cy).powi(2)).sqrt())
                    .fold(f32::INFINITY, f32::min)
            }
            AffordanceWakeZone::LeftRail => x,
            AffordanceWakeZone::RightRail => max_x - x,
            AffordanceWakeZone::TopRail => y,
            AffordanceWakeZone::BottomRail => max_y - y,
        }
    }

    fn blend_target(&self, base: Color, wake: Color, alpha: f32) -> Color {
        if base == Color::TRANSPARENT {
            wake
        } else {
            blend_colors(base, wake, alpha.clamp(0.0, 1.0), ColorSpace::Rgb)
        }
    }
}

impl StyleShader for AffordanceWakeShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.radius == 0 || self.peak_intensity <= 0.0 {
            return base;
        }

        let distance = self.zone_distance(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        let radius = self.radius as f32;
        if distance >= radius {
            return base;
        }

        let wake_factor = self.falloff.apply(distance, radius);
        let progress = self.resolved_progress(ctx);
        let intensity =
            self.rest_intensity + (self.peak_intensity - self.rest_intensity) * progress;
        let alpha = (wake_factor * intensity).clamp(0.0, 1.0);
        if alpha <= 0.0 {
            return base;
        }

        let wake_color: Color = self.color.into();
        let mut style = base;
        match self.apply_to {
            AffordanceWakeApplyTo::Foreground => {
                style.fg = self.blend_target(base.fg, wake_color, alpha);
            }
            AffordanceWakeApplyTo::Background => {
                style.bg = self.blend_target(base.bg, wake_color, alpha);
            }
            AffordanceWakeApplyTo::Both => {
                style.fg = self.blend_target(base.fg, wake_color, alpha * 0.65);
                style.bg = self.blend_target(base.bg, wake_color, alpha);
            }
        }
        style
    }

    fn name(&self) -> &'static str {
        "AffordanceWake"
    }
}

// <FILE>crates/tui-vfx-style/src/models/cls_affordance_wake_shader.rs</FILE> - <DESC>Latent affordance wake shader for dormant-to-active edge, corner, and rail cues</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
