// <FILE>crates/tui-vfx-style/src/models/cls_focus_field_shader.rs</FILE> - <DESC>Focus-following field shader for point and pane emphasis</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Introduce a subtle attention-shaping primitive supporting both ellipse and pane-following rect modes</WCTX>
// <CLOG>Add FocusFieldShader with ellipse/rect shapes, runtime-bound geometry, low-amplitude pulse, and color-first emphasis suitable for non-obtrusive focus guidance</CLOG>

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
pub enum FocusFieldShape {
    #[default]
    Ellipse,
    Rect,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum FocusFieldApplyTo {
    Foreground,
    #[default]
    Background,
    Both,
}

/// A calm focus field that can either behave like a point/ellipse spotlight or
/// follow a pane/rect.
///
/// The effect is intended for subtle attention shaping: making the active pane,
/// modal, drawer, command palette, or focus hotspot feel more current without
/// turning into a theatrical spotlight. The field should mostly be *felt* via
/// color hierarchy rather than *seen* as a separate moving object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct FocusFieldShader {
    pub color: ColorConfig,
    #[serde(default)]
    pub shape: FocusFieldShape,

    #[serde(default)]
    pub center_x: u16,
    #[serde(default)]
    pub center_y: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub center_x_binding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub center_y_binding: Option<String>,

    #[serde(default = "default_radius_x")]
    pub radius_x: u16,
    #[serde(default = "default_radius_y")]
    pub radius_y: u16,

    #[serde(default)]
    pub rect_x: u16,
    #[serde(default)]
    pub rect_y: u16,
    #[serde(default = "default_rect_width")]
    pub rect_width: u16,
    #[serde(default = "default_rect_height")]
    pub rect_height: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rect_x_binding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rect_y_binding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rect_width_binding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rect_height_binding: Option<String>,

    #[serde(default = "default_feather")]
    pub feather: u8,
    #[serde(default)]
    pub falloff: FalloffType,
    #[serde(default = "default_intensity")]
    pub intensity: f32,
    #[serde(default)]
    pub apply_to: FocusFieldApplyTo,
    #[serde(default)]
    pub pulse_speed: f32,
}

fn default_radius_x() -> u16 {
    10
}

fn default_radius_y() -> u16 {
    4
}

fn default_rect_width() -> u16 {
    12
}

fn default_rect_height() -> u16 {
    5
}

fn default_feather() -> u8 {
    3
}

fn default_intensity() -> f32 {
    0.22
}

impl Default for FocusFieldShader {
    fn default() -> Self {
        Self {
            color: ColorConfig::White,
            shape: FocusFieldShape::Ellipse,
            center_x: 0,
            center_y: 0,
            center_x_binding: None,
            center_y_binding: None,
            radius_x: default_radius_x(),
            radius_y: default_radius_y(),
            rect_x: 0,
            rect_y: 0,
            rect_width: default_rect_width(),
            rect_height: default_rect_height(),
            rect_x_binding: None,
            rect_y_binding: None,
            rect_width_binding: None,
            rect_height_binding: None,
            feather: default_feather(),
            falloff: FalloffType::Quadratic,
            intensity: default_intensity(),
            apply_to: FocusFieldApplyTo::Background,
            pulse_speed: 0.0,
        }
    }
}

impl FocusFieldShader {
    pub fn runtime_binding_requests(&self) -> Vec<ShaderRuntimeBindingRequest> {
        let mut reqs = Vec::new();
        for (field, binding) in [
            ("center_x", self.center_x_binding.as_ref()),
            ("center_y", self.center_y_binding.as_ref()),
            ("rect_x", self.rect_x_binding.as_ref()),
            ("rect_y", self.rect_y_binding.as_ref()),
            ("rect_width", self.rect_width_binding.as_ref()),
            ("rect_height", self.rect_height_binding.as_ref()),
        ] {
            if let Some(binding) = binding {
                reqs.push(ShaderRuntimeBindingRequest {
                    field: field.to_string(),
                    binding: binding.clone(),
                    expected_type: "u16".to_string(),
                });
            }
        }
        reqs
    }

    pub fn runtime_binding_resolutions(
        &self,
        ctx: &ShaderContext,
    ) -> Vec<ShaderRuntimeBindingResolution> {
        let pairs = [
            ("center_x", self.center_x_binding.as_ref(), Some(self.center_x)),
            ("center_y", self.center_y_binding.as_ref(), Some(self.center_y)),
            ("rect_x", self.rect_x_binding.as_ref(), Some(self.rect_x)),
            ("rect_y", self.rect_y_binding.as_ref(), Some(self.rect_y)),
            ("rect_width", self.rect_width_binding.as_ref(), Some(self.rect_width)),
            ("rect_height", self.rect_height_binding.as_ref(), Some(self.rect_height)),
        ];

        pairs
            .into_iter()
            .filter_map(|(field, binding, fallback)| {
                binding.map(|binding| {
                    let supplied = ctx.runtime_param(binding);
                    let supplied_kind = supplied.map(|value| value.kind_name().to_string());
                    let supplied_value = supplied.map(serde_json::Value::from);
                    let coerced = matches!(supplied_kind.as_deref(), Some("float"));
                    let resolved = ctx.runtime_param_u16(binding);
                    ShaderRuntimeBindingResolution {
                        field: field.to_string(),
                        binding: binding.clone(),
                        expected_type: "u16".to_string(),
                        status: match (resolved, fallback) {
                            (Some(_), _) if coerced => ShaderRuntimeBindingStatus::Coerced,
                            (Some(_), _) => ShaderRuntimeBindingStatus::Resolved,
                            (None, Some(_)) => ShaderRuntimeBindingStatus::FallbackStatic,
                            (None, None) => ShaderRuntimeBindingStatus::Missing,
                        },
                        supplied_kind,
                        supplied_value,
                        effective_value: resolved.map(serde_json::Value::from),
                        fallback_value: fallback.map(serde_json::Value::from),
                    }
                })
            })
            .collect()
    }

    fn resolved_center_x(&self, ctx: &ShaderContext) -> u16 {
        self.center_x_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_u16(binding))
            .unwrap_or(self.center_x)
    }

    fn resolved_center_y(&self, ctx: &ShaderContext) -> u16 {
        self.center_y_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_u16(binding))
            .unwrap_or(self.center_y)
    }

    fn resolved_rect_x(&self, ctx: &ShaderContext) -> u16 {
        self.rect_x_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_u16(binding))
            .unwrap_or(self.rect_x)
    }

    fn resolved_rect_y(&self, ctx: &ShaderContext) -> u16 {
        self.rect_y_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_u16(binding))
            .unwrap_or(self.rect_y)
    }

    fn resolved_rect_width(&self, ctx: &ShaderContext) -> u16 {
        self.rect_width_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_u16(binding))
            .unwrap_or(self.rect_width)
            .max(1)
    }

    fn resolved_rect_height(&self, ctx: &ShaderContext) -> u16 {
        self.rect_height_binding
            .as_deref()
            .and_then(|binding| ctx.runtime_param_u16(binding))
            .unwrap_or(self.rect_height)
            .max(1)
    }

    fn pulse_factor(&self, ctx: &ShaderContext) -> f32 {
        if self.pulse_speed <= 0.0 {
            1.0
        } else {
            0.94 + 0.06 * (ctx.t as f32 * self.pulse_speed * std::f32::consts::TAU).sin()
        }
    }

    fn ellipse_factor(&self, ctx: &ShaderContext) -> f32 {
        let rx = self.radius_x.max(1) as f32;
        let ry = self.radius_y.max(1) as f32;
        let dx = ctx.local_x as f32 - self.resolved_center_x(ctx) as f32;
        let dy = ctx.local_y as f32 - self.resolved_center_y(ctx) as f32;
        let normalized = ((dx / rx).powi(2) + (dy / ry).powi(2)).sqrt();
        if normalized >= 1.0 {
            0.0
        } else {
            self.falloff.apply(normalized, 1.0)
        }
    }

    fn rect_factor(&self, ctx: &ShaderContext) -> f32 {
        let rx = self.resolved_rect_x(ctx) as f32;
        let ry = self.resolved_rect_y(ctx) as f32;
        let rw = self.resolved_rect_width(ctx) as f32;
        let rh = self.resolved_rect_height(ctx) as f32;
        let x = ctx.local_x as f32;
        let y = ctx.local_y as f32;

        let inside_x = x >= rx && x < rx + rw;
        let inside_y = y >= ry && y < ry + rh;
        if inside_x && inside_y {
            return 1.0;
        }

        let nearest_x = x.clamp(rx, rx + rw - 1.0);
        let nearest_y = y.clamp(ry, ry + rh - 1.0);
        let dx = x - nearest_x;
        let dy = y - nearest_y;
        let distance = (dx * dx + dy * dy).sqrt();
        let feather = self.feather.max(1) as f32;
        if distance >= feather {
            0.0
        } else {
            self.falloff.apply(distance, feather)
        }
    }

    fn blend_target(&self, base: Color, focus: Color, alpha: f32) -> Color {
        if base == Color::TRANSPARENT {
            focus
        } else {
            blend_colors(base, focus, alpha.clamp(0.0, 1.0), ColorSpace::Rgb)
        }
    }
}

impl StyleShader for FocusFieldShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.intensity <= 0.0 || ctx.width == 0 || ctx.height == 0 {
            return base;
        }

        let shape_factor = match self.shape {
            FocusFieldShape::Ellipse => self.ellipse_factor(ctx),
            FocusFieldShape::Rect => self.rect_factor(ctx),
        };
        if shape_factor <= 0.0 {
            return base;
        }

        let alpha = (shape_factor * self.intensity * self.pulse_factor(ctx)).clamp(0.0, 1.0);
        let focus_color: Color = self.color.into();
        let mut style = base;
        match self.apply_to {
            FocusFieldApplyTo::Foreground => {
                style.fg = self.blend_target(base.fg, focus_color, alpha * 0.7);
            }
            FocusFieldApplyTo::Background => {
                style.bg = self.blend_target(base.bg, focus_color, alpha);
            }
            FocusFieldApplyTo::Both => {
                style.fg = self.blend_target(base.fg, focus_color, alpha * 0.65);
                style.bg = self.blend_target(base.bg, focus_color, alpha);
            }
        }
        style
    }

    fn name(&self) -> &'static str {
        "FocusField"
    }
}

// <FILE>crates/tui-vfx-style/src/models/cls_focus_field_shader.rs</FILE> - <DESC>Focus-following field shader for point and pane emphasis</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
