//! Diffusion shader — likely an earned named factory/composition.
//!
//! ## V3 family note
//!
//! `DiffusionShader` is currently a direct flat variant in the live style
//! surface, but the V3 capability catalog and style-model restructure inventory
//! classify it as a likely earned named factory/composition rather than a
//! forever-flat primitive leaf.
//!
// <FILE>crates/tui-vfx-style/src/models/cls_diffusion_shader.rs</FILE> - <DESC>Soft diffusion shader for textile, paper, and frosted material light</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Introduce a static-first material-light diffusion primitive distinct from glow, sparkle, and flicker</WCTX>
// <CLOG>Add DiffusionShader with source geometry, softness, edge discipline, and optional low-amplitude breathing/drift modes</CLOG>

use crate::models::{ColorConfig, ColorSpace, FalloffType};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::fnc_blend_colors::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum DiffusionSource {
    #[default]
    Center,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum DiffusionApplyTo {
    Foreground,
    #[default]
    Background,
    Both,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum DiffusionMode {
    #[default]
    Static,
    WarmDrift,
    CoolDrift,
    Breath,
}

/// Soft material-light diffusion for paper, textile, and frosted surfaces.
///
/// The shader favors calm source-to-surface diffusion over edge halos. It is
/// best used on shell-owned or background-heavy regions, where it can add
/// supportive warmth and atmosphere without competing with dense text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct DiffusionShader {
    #[serde(default)]
    pub source: DiffusionSource,
    pub color: ColorConfig,
    #[serde(default = "default_radius")]
    pub radius: u8,
    #[serde(default = "default_softness")]
    pub softness: f32,
    #[serde(default = "default_edge_firmness")]
    pub edge_firmness: f32,
    #[serde(default)]
    pub falloff: FalloffType,
    #[serde(default = "default_intensity")]
    pub intensity: f32,
    #[serde(default)]
    pub apply_to: DiffusionApplyTo,
    #[serde(default)]
    pub mode: DiffusionMode,
    #[serde(default)]
    pub drift_speed: f32,
    #[serde(default = "default_drift_amount")]
    pub drift_amount: f32,
}

fn default_radius() -> u8 {
    6
}

fn default_softness() -> f32 {
    0.55
}

fn default_edge_firmness() -> f32 {
    0.2
}

fn default_intensity() -> f32 {
    0.2
}

fn default_drift_amount() -> f32 {
    0.06
}

impl Default for DiffusionShader {
    fn default() -> Self {
        Self {
            source: DiffusionSource::Center,
            color: ColorConfig::White,
            radius: default_radius(),
            softness: default_softness(),
            edge_firmness: default_edge_firmness(),
            falloff: FalloffType::Quadratic,
            intensity: default_intensity(),
            apply_to: DiffusionApplyTo::Background,
            mode: DiffusionMode::Static,
            drift_speed: 0.0,
            drift_amount: default_drift_amount(),
        }
    }
}

impl DiffusionShader {
    fn source_distance(&self, x: u16, y: u16, width: u16, height: u16) -> f32 {
        let max_x = width.saturating_sub(1) as f32;
        let max_y = height.saturating_sub(1) as f32;
        let x = x as f32;
        let y = y as f32;
        match self.source {
            DiffusionSource::Center => {
                let cx = max_x / 2.0;
                let cy = max_y / 2.0;
                ((x - cx).powi(2) + (y - cy).powi(2)).sqrt()
            }
            DiffusionSource::Top => y,
            DiffusionSource::Bottom => max_y - y,
            DiffusionSource::Left => x,
            DiffusionSource::Right => max_x - x,
            DiffusionSource::TopLeft => (x.powi(2) + y.powi(2)).sqrt(),
            DiffusionSource::TopRight => ((max_x - x).powi(2) + y.powi(2)).sqrt(),
            DiffusionSource::BottomLeft => (x.powi(2) + (max_y - y).powi(2)).sqrt(),
            DiffusionSource::BottomRight => ((max_x - x).powi(2) + (max_y - y).powi(2)).sqrt(),
        }
    }

    fn edge_distance_for_frame(&self, x: u16, y: u16, width: u16, height: u16) -> f32 {
        let max_x = width.saturating_sub(1) as f32;
        let max_y = height.saturating_sub(1) as f32;
        let x = x as f32;
        let y = y as f32;

        let top = y;
        let bottom = max_y - y;
        let left = x;
        let right = max_x - x;

        match self.source {
            DiffusionSource::Center => top.min(bottom).min(left).min(right),
            DiffusionSource::Top => bottom.min(left).min(right),
            DiffusionSource::Bottom => top.min(left).min(right),
            DiffusionSource::Left => top.min(bottom).min(right),
            DiffusionSource::Right => top.min(bottom).min(left),
            DiffusionSource::TopLeft => bottom.min(right),
            DiffusionSource::TopRight => bottom.min(left),
            DiffusionSource::BottomLeft => top.min(right),
            DiffusionSource::BottomRight => top.min(left),
        }
    }

    fn modulated_intensity(&self, ctx: &ShaderContext) -> f32 {
        if self.drift_speed <= 0.0 || self.drift_amount <= 0.0 {
            return self.intensity;
        }

        let phase = ctx.t as f32 * self.drift_speed * std::f32::consts::TAU;
        let modulation = match self.mode {
            DiffusionMode::Static => 1.0,
            DiffusionMode::Breath => 1.0 + self.drift_amount * phase.sin(),
            DiffusionMode::WarmDrift => 1.0 + self.drift_amount * (phase.sin() * 0.6),
            DiffusionMode::CoolDrift => 1.0 + self.drift_amount * (phase.cos() * 0.6),
        };
        self.intensity * modulation
    }

    fn blend_target(&self, base: Color, light: Color, alpha: f32) -> Color {
        if base == Color::TRANSPARENT {
            light
        } else {
            blend_colors(base, light, alpha.clamp(0.0, 1.0), ColorSpace::Rgb)
        }
    }
}

impl StyleShader for DiffusionShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.intensity <= 0.0 || self.radius == 0 || ctx.width == 0 || ctx.height == 0 {
            return base;
        }

        let radius = self.radius as f32;
        let distance = self.source_distance(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        if distance >= radius {
            return base;
        }

        let base_factor = self.falloff.apply(distance, radius);
        let softened =
            base_factor + (base_factor.sqrt() - base_factor) * self.softness.clamp(0.0, 1.0);

        let frame_distance =
            self.edge_distance_for_frame(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        let frame_window = (radius * 0.75).max(1.0);
        let frame_factor = 1.0
            - self.edge_firmness.clamp(0.0, 1.0)
                * (1.0 - (frame_distance / frame_window).clamp(0.0, 1.0));

        let alpha = (softened * frame_factor * self.modulated_intensity(ctx)).clamp(0.0, 1.0);
        if alpha <= 0.0 {
            return base;
        }

        let light_color: Color = self.color.into();
        let mut style = base;
        match self.apply_to {
            DiffusionApplyTo::Foreground => {
                style.fg = self.blend_target(base.fg, light_color, alpha * 0.7);
            }
            DiffusionApplyTo::Background => {
                style.bg = self.blend_target(base.bg, light_color, alpha);
            }
            DiffusionApplyTo::Both => {
                style.fg = self.blend_target(base.fg, light_color, alpha * 0.65);
                style.bg = self.blend_target(base.bg, light_color, alpha);
            }
        }

        style
    }

    fn name(&self) -> &'static str {
        "Diffusion"
    }
}

// <FILE>crates/tui-vfx-style/src/models/cls_diffusion_shader.rs</FILE> - <DESC>Soft diffusion shader for textile, paper, and frosted material light</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
