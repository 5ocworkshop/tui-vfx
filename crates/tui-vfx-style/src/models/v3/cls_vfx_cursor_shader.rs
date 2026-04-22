// <FILE>tui-vfx-style/src/models/v3/cls_vfx_cursor_shader.rs</FILE> - <DESC>V3 cursor family shader surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a real grouped V3 surface for the cursor shader so the per-frame cursor path has an explicit V3 home instead of remaining a permanent flat special case.</WCTX>
// <CLOG>Introduce VfxCursorShader plus conversion helpers from the legacy cursor shader and SpatialShaderType.</CLOG>

//! V3 family surface for cursor shaders.
//!
//! This grouped type provides a forward-looking V3 home for the cursor shader's
//! per-frame primary/trail payload while the legacy cursor path stays intact.

use crate::models::v3::enum_vfx_cursor_behavior::{
    VfxCursorMode, VfxCursorPrimary, VfxCursorTrail,
};
use crate::models::{
    ColorConfig, CursorShader, CursorShaderMode, CursorShaderPrimary, CursorShaderTrail,
    SpatialShaderType,
};
use serde::{Deserialize, Serialize};

/// Canonical V3 family surface for cursor shaders.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(default)]
pub struct VfxCursorShader {
    /// Wake/trail mode.
    pub mode: VfxCursorMode,
    /// Tint color for tint/ghost modes.
    pub tint: ColorConfig,
    /// Optional primary-cell op.
    pub primary: Option<VfxCursorPrimary>,
    /// Trail-cell ops.
    pub trail: Vec<VfxCursorTrail>,
}

impl VfxCursorShader {
    /// Convert a legacy flat `SpatialShaderType` variant into the V3 cursor family.
    pub fn from_legacy_spatial_shader(shader: &SpatialShaderType) -> Option<Self> {
        match shader {
            SpatialShaderType::Cursor(shader) => Some(Self::from(shader)),
            _ => None,
        }
    }
}

impl From<&CursorShader> for VfxCursorShader {
    fn from(shader: &CursorShader) -> Self {
        Self {
            mode: shader.mode.into(),
            tint: shader.tint.clone(),
            primary: shader.primary.as_ref().map(Into::into),
            trail: shader.trail.iter().map(Into::into).collect(),
        }
    }
}

impl From<CursorShaderMode> for VfxCursorMode {
    fn from(value: CursorShaderMode) -> Self {
        match value {
            CursorShaderMode::Off => Self::Off,
            CursorShaderMode::Tint => Self::Tint,
            CursorShaderMode::Ghost => Self::Ghost,
        }
    }
}

impl From<&CursorShaderPrimary> for VfxCursorPrimary {
    fn from(value: &CursorShaderPrimary) -> Self {
        Self {
            position: value.position,
            alpha: value.alpha,
        }
    }
}

impl From<&CursorShaderTrail> for VfxCursorTrail {
    fn from(value: &CursorShaderTrail) -> Self {
        Self {
            position: value.position,
            alpha: value.alpha,
            glyph: value.glyph.clone(),
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/cls_vfx_cursor_shader.rs</FILE> - <DESC>V3 cursor family shader surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
