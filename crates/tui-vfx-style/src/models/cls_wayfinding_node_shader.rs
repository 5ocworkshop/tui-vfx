//! Wayfinding-node shader — likely an earned named factory/composition.
//!
//! ## V3 family note
//!
//! `WayfindingNodeShader` is currently a direct flat variant in the live style
//! surface, but the V3 capability catalog and style-model restructure inventory
//! classify it as a likely earned named factory/composition rather than a
//! forever-flat primitive leaf.
//!
// <FILE>crates/tui-vfx-style/src/models/cls_wayfinding_node_shader.rs</FILE> - <DESC>Calm node-based wayfinding shader for breadcrumbs, steps, and junction emphasis</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Introduce a practical application-oriented node guidance primitive distinct from routed signal traces</WCTX>
// <CLOG>Add WayfindingNodeShader with explicit node list, current-index binding, trail strengths, and optional low-amplitude pulse for current node</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{
    ShaderContext, ShaderRuntimeBindingRequest, ShaderRuntimeBindingResolution,
    ShaderRuntimeBindingStatus, StyleShader,
};
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct WayfindingNode {
    pub x: u16,
    pub y: u16,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum WayfindingNodeApplyTo {
    Foreground,
    #[default]
    Background,
    Both,
}

/// Calm node/junction emphasis for breadcrumbs, progress steps, and route hints.
///
/// Unlike `TracePath` and `TracePropagation`, this shader is not about animated
/// signal flow. It is about clear current-position and prior-step emphasis with
/// a low-noise application UX vocabulary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct WayfindingNodeShader {
    pub color: ColorConfig,
    pub nodes: Vec<WayfindingNode>,
    #[serde(default = "default_radius")]
    pub radius: u8,
    #[serde(default = "default_intensity")]
    pub intensity: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_index: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_index_binding: Option<String>,
    #[serde(default = "default_previous_strength")]
    pub previous_strength: f32,
    #[serde(default = "default_future_strength")]
    pub future_strength: f32,
    #[serde(default)]
    pub pulse_speed: f32,
    #[serde(default)]
    pub apply_to: WayfindingNodeApplyTo,
}

fn default_radius() -> u8 {
    2
}

fn default_intensity() -> f32 {
    0.22
}

fn default_previous_strength() -> f32 {
    0.45
}

fn default_future_strength() -> f32 {
    0.0
}

impl Default for WayfindingNodeShader {
    fn default() -> Self {
        Self {
            color: ColorConfig::White,
            nodes: Vec::new(),
            radius: default_radius(),
            intensity: default_intensity(),
            current_index: None,
            current_index_binding: None,
            previous_strength: default_previous_strength(),
            future_strength: default_future_strength(),
            pulse_speed: 0.0,
            apply_to: WayfindingNodeApplyTo::Background,
        }
    }
}

impl WayfindingNodeShader {
    pub fn runtime_binding_requests(&self) -> Vec<ShaderRuntimeBindingRequest> {
        self.current_index_binding
            .as_ref()
            .map(|binding| {
                vec![ShaderRuntimeBindingRequest {
                    field: "current_index".to_string(),
                    binding: binding.clone(),
                    expected_type: "u16".to_string(),
                }]
            })
            .unwrap_or_default()
    }

    pub fn runtime_binding_resolutions(
        &self,
        ctx: &ShaderContext,
    ) -> Vec<ShaderRuntimeBindingResolution> {
        self.current_index_binding
            .as_ref()
            .map(|binding| {
                let supplied = ctx.runtime_param(binding);
                let supplied_kind = supplied.map(|value| value.kind_name().to_string());
                let supplied_value = supplied.map(serde_json::Value::from);
                let coerced = matches!(supplied_kind.as_deref(), Some("float"));
                let resolved = ctx.runtime_param_u16(binding);
                ShaderRuntimeBindingResolution {
                    field: "current_index".to_string(),
                    binding: binding.clone(),
                    expected_type: "u16".to_string(),
                    status: match (resolved, self.current_index) {
                        (Some(_), _) if coerced => ShaderRuntimeBindingStatus::Coerced,
                        (Some(_), _) => ShaderRuntimeBindingStatus::Resolved,
                        (None, Some(_)) => ShaderRuntimeBindingStatus::FallbackStatic,
                        (None, None) => ShaderRuntimeBindingStatus::Missing,
                    },
                    supplied_kind,
                    supplied_value,
                    effective_value: resolved.map(serde_json::Value::from),
                    fallback_value: self.current_index.map(serde_json::Value::from),
                }
            })
            .map(|value| vec![value])
            .unwrap_or_default()
    }

    fn resolved_current_index(&self, ctx: &ShaderContext) -> Option<usize> {
        self.current_index_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_u16(binding))
            .or(self.current_index)
            .map(|value| value as usize)
    }

    fn node_strength(&self, index: usize, current: Option<usize>, ctx: &ShaderContext) -> f32 {
        match current {
            None => 1.0,
            Some(current_idx) if index == current_idx => {
                if self.pulse_speed <= 0.0 {
                    1.0
                } else {
                    0.94 + 0.06 * (ctx.t as f32 * self.pulse_speed * std::f32::consts::TAU).sin()
                }
            }
            Some(current_idx) if index < current_idx => self.previous_strength.clamp(0.0, 1.0),
            Some(_) => self.future_strength.clamp(0.0, 1.0),
        }
    }

    fn blend_target(&self, base: Color, node: Color, alpha: f32) -> Color {
        if base == Color::TRANSPARENT {
            node
        } else {
            blend_colors(base, node, alpha.clamp(0.0, 1.0), ColorSpace::Rgb)
        }
    }
}

impl StyleShader for WayfindingNodeShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.nodes.is_empty() || self.radius == 0 || self.intensity <= 0.0 {
            return base;
        }

        let current = self.resolved_current_index(ctx);
        let radius = self.radius as f32;
        let mut best_alpha = 0.0_f32;
        for (index, node) in self.nodes.iter().enumerate() {
            let dx = ctx.local_x as f32 - node.x as f32;
            let dy = ctx.local_y as f32 - node.y as f32;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance > radius {
                continue;
            }
            let distance_factor = 1.0 - (distance / radius);
            let strength = self.node_strength(index, current, ctx);
            best_alpha = best_alpha.max(distance_factor * strength * self.intensity);
        }

        if best_alpha <= 0.0 {
            return base;
        }

        let node_color: Color = self.color.into();
        let mut style = base;
        match self.apply_to {
            WayfindingNodeApplyTo::Foreground => {
                style.fg = self.blend_target(base.fg, node_color, best_alpha);
            }
            WayfindingNodeApplyTo::Background => {
                style.bg = self.blend_target(base.bg, node_color, best_alpha);
            }
            WayfindingNodeApplyTo::Both => {
                style.fg = self.blend_target(base.fg, node_color, best_alpha * 0.65);
                style.bg = self.blend_target(base.bg, node_color, best_alpha);
            }
        }

        style
    }

    fn name(&self) -> &'static str {
        "WayfindingNode"
    }
}

// <FILE>crates/tui-vfx-style/src/models/cls_wayfinding_node_shader.rs</FILE> - <DESC>Calm node-based wayfinding shader for breadcrumbs, steps, and junction emphasis</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
