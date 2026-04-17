// <FILE>crates/tui-vfx-style/src/models/cls_concealed_light_shader.rs</FILE> - <DESC>Concealed architectural light shader for hidden-source shell lighting</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Introduce a static-first architectural light primitive distinct from glow, sheen, and gradients</WCTX>
// <CLOG>Add ConcealedLightShader with source edge, concealed cutoff, lit band, inward falloff, and optional low-amplitude pulse/drift modes</CLOG>

use crate::models::{ColorConfig, ColorSpace, FalloffType};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ConcealedLightSource {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ConcealedLightApplyTo {
    Foreground,
    #[default]
    Background,
    Both,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ConcealedLightMode {
    #[default]
    Static,
    Pulse,
    Drift,
}

/// Hidden-source architectural light for thresholds, seams, and shell depth.
///
/// The effect is intentionally static-first and low-fatigue. It models a dark
/// lip at the source edge, a narrow lit band just inside that edge, and a
/// restrained inward falloff. The result should feel like built-in lighting,
/// not like a glow or sheen effect pasted on top.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct ConcealedLightShader {
    pub color: ColorConfig,
    #[serde(default)]
    pub source: ConcealedLightSource,
    #[serde(default = "default_spread")]
    pub spread: u8,
    #[serde(default = "default_edge_width")]
    pub edge_width: u8,
    #[serde(default)]
    pub falloff: FalloffType,
    #[serde(default = "default_intensity")]
    pub intensity: f32,
    #[serde(default)]
    pub apply_to: ConcealedLightApplyTo,
    #[serde(default)]
    pub mode: ConcealedLightMode,
    #[serde(default)]
    pub pulse_speed: f32,
    #[serde(default = "default_source_cutoff")]
    pub source_cutoff: f32,
}

fn default_spread() -> u8 {
    4
}

fn default_edge_width() -> u8 {
    1
}

fn default_intensity() -> f32 {
    0.18
}

fn default_source_cutoff() -> f32 {
    0.18
}

impl Default for ConcealedLightShader {
    fn default() -> Self {
        Self {
            color: ColorConfig::White,
            source: ConcealedLightSource::Top,
            spread: default_spread(),
            edge_width: default_edge_width(),
            falloff: FalloffType::Quadratic,
            intensity: default_intensity(),
            apply_to: ConcealedLightApplyTo::Background,
            mode: ConcealedLightMode::Static,
            pulse_speed: 0.0,
            source_cutoff: default_source_cutoff(),
        }
    }
}

impl ConcealedLightShader {
    fn inward_distance(&self, x: u16, y: u16, width: u16, height: u16) -> f32 {
        match self.source {
            ConcealedLightSource::Top => y as f32,
            ConcealedLightSource::Bottom => height.saturating_sub(y + 1) as f32,
            ConcealedLightSource::Left => x as f32,
            ConcealedLightSource::Right => width.saturating_sub(x + 1) as f32,
        }
    }

    fn along_axis_ratio(&self, x: u16, y: u16, width: u16, height: u16) -> f32 {
        match self.source {
            ConcealedLightSource::Top | ConcealedLightSource::Bottom => {
                if width <= 1 {
                    0.5
                } else {
                    x as f32 / (width - 1) as f32
                }
            }
            ConcealedLightSource::Left | ConcealedLightSource::Right => {
                if height <= 1 {
                    0.5
                } else {
                    y as f32 / (height - 1) as f32
                }
            }
        }
    }

    fn modulated_intensity(&self, ctx: &ShaderContext) -> f32 {
        match self.mode {
            ConcealedLightMode::Static => self.intensity,
            ConcealedLightMode::Pulse => {
                if self.pulse_speed <= 0.0 {
                    self.intensity
                } else {
                    let phase = (ctx.t as f32 * self.pulse_speed * std::f32::consts::TAU).sin();
                    self.intensity * (0.92 + 0.08 * phase)
                }
            }
            ConcealedLightMode::Drift => {
                if self.pulse_speed <= 0.0 {
                    self.intensity
                } else {
                    let axis =
                        self.along_axis_ratio(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
                    let phase = ((axis * 1.5) + ctx.t as f32 * self.pulse_speed).sin();
                    self.intensity * (0.94 + 0.06 * phase)
                }
            }
        }
    }

    fn blend_target(&self, base: Color, light: Color, alpha: f32) -> Color {
        if base == Color::TRANSPARENT {
            light
        } else {
            blend_colors(base, light, alpha.clamp(0.0, 1.0), ColorSpace::Rgb)
        }
    }
}

impl StyleShader for ConcealedLightShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.intensity <= 0.0 || self.spread == 0 || ctx.width == 0 || ctx.height == 0 {
            return base;
        }

        let inward = self.inward_distance(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        let spread = self.spread as f32;
        if inward >= spread {
            return base;
        }

        let cutoff = spread * self.source_cutoff.clamp(0.0, 0.9);
        if inward < cutoff {
            return base;
        }

        let visible_distance = inward - cutoff;
        let lit_band = self.edge_width.max(1) as f32;
        let radius_after_band = (spread - cutoff - lit_band).max(1.0);

        let band_or_falloff = if visible_distance <= lit_band {
            1.0
        } else {
            self.falloff
                .apply(visible_distance - lit_band, radius_after_band)
        };

        let alpha = (band_or_falloff * self.modulated_intensity(ctx)).clamp(0.0, 1.0);
        if alpha <= 0.0 {
            return base;
        }

        let light_color: Color = self.color.into();
        let mut style = base;
        match self.apply_to {
            ConcealedLightApplyTo::Foreground => {
                style.fg = self.blend_target(base.fg, light_color, alpha);
            }
            ConcealedLightApplyTo::Background => {
                style.bg = self.blend_target(base.bg, light_color, alpha);
            }
            ConcealedLightApplyTo::Both => {
                style.fg = self.blend_target(base.fg, light_color, alpha * 0.65);
                style.bg = self.blend_target(base.bg, light_color, alpha);
            }
        }

        style
    }

    fn name(&self) -> &'static str {
        "ConcealedLight"
    }
}

// <FILE>crates/tui-vfx-style/src/models/cls_concealed_light_shader.rs</FILE> - <DESC>Concealed architectural light shader for hidden-source shell lighting</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
