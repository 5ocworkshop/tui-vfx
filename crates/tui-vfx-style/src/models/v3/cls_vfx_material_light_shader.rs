// <FILE>tui-vfx-style/src/models/v3/cls_vfx_material_light_shader.rs</FILE> - <DESC>V3 material-light family shader surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a real grouped V3 material-light surface so the legacy Diffusion, ConcealedLight, and EdgeSheen variants become migration inputs instead of the only lasting conceptual model.</WCTX>
// <CLOG>Introduce VfxMaterialLightShader plus conversion helpers from the legacy material-light shader variants and SpatialShaderType.</CLOG>

//! V3 family surface for material-light shaders.
//!
//! This grouped type provides a forward-looking V3 home for the calm
//! shell/material-light effects that currently live as separate flat variants.

use crate::models::v3::enum_vfx_material_light_behavior::{
    VfxConcealedLightMode, VfxConcealedLightSource, VfxDiffusionMode, VfxDiffusionSource,
    VfxMaterialLightApplyTo, VfxMaterialLightBehavior,
};
use crate::models::{
    ConcealedLightApplyTo, ConcealedLightMode, ConcealedLightShader, ConcealedLightSource,
    DiffusionApplyTo, DiffusionMode, DiffusionShader, DiffusionSource, EdgeSheenApplyTo,
    EdgeSheenShader, SpatialShaderType,
};
use serde::{Deserialize, Serialize};

/// Canonical V3 family surface for material-light shaders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxMaterialLightShader {
    /// Behavior/configuration surface for the chosen material-light family member.
    pub behavior: VfxMaterialLightBehavior,
}

impl VfxMaterialLightShader {
    /// Convert a legacy flat `SpatialShaderType` variant into the V3
    /// material-light family when that shader belongs to this family.
    pub fn from_legacy_spatial_shader(shader: &SpatialShaderType) -> Option<Self> {
        match shader {
            SpatialShaderType::Diffusion(shader) => Some(Self::from(shader)),
            SpatialShaderType::ConcealedLight(shader) => Some(Self::from(shader)),
            SpatialShaderType::EdgeSheen(shader) => Some(Self::from(shader)),
            _ => None,
        }
    }
}

impl From<&DiffusionShader> for VfxMaterialLightShader {
    fn from(shader: &DiffusionShader) -> Self {
        Self {
            behavior: VfxMaterialLightBehavior::Diffusion {
                source: shader.source.into(),
                color: shader.color.clone(),
                radius: shader.radius,
                softness: shader.softness,
                edge_firmness: shader.edge_firmness,
                falloff: shader.falloff,
                intensity: shader.intensity,
                apply_to: shader.apply_to.into(),
                mode: shader.mode.into(),
                drift_speed: shader.drift_speed,
                drift_amount: shader.drift_amount,
            },
        }
    }
}

impl From<&ConcealedLightShader> for VfxMaterialLightShader {
    fn from(shader: &ConcealedLightShader) -> Self {
        Self {
            behavior: VfxMaterialLightBehavior::ConcealedLight {
                source: shader.source.into(),
                color: shader.color.clone(),
                spread: shader.spread,
                edge_width: shader.edge_width,
                falloff: shader.falloff,
                intensity: shader.intensity,
                apply_to: shader.apply_to.into(),
                mode: shader.mode.into(),
                pulse_speed: shader.pulse_speed,
                source_cutoff: shader.source_cutoff,
            },
        }
    }
}

impl From<&EdgeSheenShader> for VfxMaterialLightShader {
    fn from(shader: &EdgeSheenShader) -> Self {
        Self {
            behavior: VfxMaterialLightBehavior::EdgeSheen {
                color: shader.color.clone(),
                speed: shader.speed,
                band_width: shader.band_width,
                edge_width: shader.edge_width,
                intensity: shader.intensity,
                corner_boost: shader.corner_boost,
                apply_to: shader.apply_to.into(),
            },
        }
    }
}

impl From<DiffusionSource> for VfxDiffusionSource {
    fn from(value: DiffusionSource) -> Self {
        match value {
            DiffusionSource::Center => Self::Center,
            DiffusionSource::Top => Self::Top,
            DiffusionSource::Bottom => Self::Bottom,
            DiffusionSource::Left => Self::Left,
            DiffusionSource::Right => Self::Right,
            DiffusionSource::TopLeft => Self::TopLeft,
            DiffusionSource::TopRight => Self::TopRight,
            DiffusionSource::BottomLeft => Self::BottomLeft,
            DiffusionSource::BottomRight => Self::BottomRight,
        }
    }
}

impl From<DiffusionApplyTo> for VfxMaterialLightApplyTo {
    fn from(value: DiffusionApplyTo) -> Self {
        match value {
            DiffusionApplyTo::Foreground => Self::Foreground,
            DiffusionApplyTo::Background => Self::Background,
            DiffusionApplyTo::Both => Self::Both,
        }
    }
}

impl From<DiffusionMode> for VfxDiffusionMode {
    fn from(value: DiffusionMode) -> Self {
        match value {
            DiffusionMode::Static => Self::Static,
            DiffusionMode::WarmDrift => Self::WarmDrift,
            DiffusionMode::CoolDrift => Self::CoolDrift,
            DiffusionMode::Breath => Self::Breath,
        }
    }
}

impl From<ConcealedLightSource> for VfxConcealedLightSource {
    fn from(value: ConcealedLightSource) -> Self {
        match value {
            ConcealedLightSource::Top => Self::Top,
            ConcealedLightSource::Bottom => Self::Bottom,
            ConcealedLightSource::Left => Self::Left,
            ConcealedLightSource::Right => Self::Right,
        }
    }
}

impl From<ConcealedLightApplyTo> for VfxMaterialLightApplyTo {
    fn from(value: ConcealedLightApplyTo) -> Self {
        match value {
            ConcealedLightApplyTo::Foreground => Self::Foreground,
            ConcealedLightApplyTo::Background => Self::Background,
            ConcealedLightApplyTo::Both => Self::Both,
        }
    }
}

impl From<ConcealedLightMode> for VfxConcealedLightMode {
    fn from(value: ConcealedLightMode) -> Self {
        match value {
            ConcealedLightMode::Static => Self::Static,
            ConcealedLightMode::Pulse => Self::Pulse,
            ConcealedLightMode::Drift => Self::Drift,
        }
    }
}

impl From<EdgeSheenApplyTo> for VfxMaterialLightApplyTo {
    fn from(value: EdgeSheenApplyTo) -> Self {
        match value {
            EdgeSheenApplyTo::Foreground => Self::Foreground,
            EdgeSheenApplyTo::Background => Self::Background,
            EdgeSheenApplyTo::Both => Self::Both,
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/cls_vfx_material_light_shader.rs</FILE> - <DESC>V3 material-light family shader surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
