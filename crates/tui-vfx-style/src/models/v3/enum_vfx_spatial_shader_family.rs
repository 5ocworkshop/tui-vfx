// <FILE>tui-vfx-style/src/models/v3/enum_vfx_spatial_shader_family.rs</FILE> - <DESC>Central V3 representation for spatial shaders across primitive and composed layers</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Decision 2 migration slice — once all grouped family surfaces exist, expose a central style-family seam that encodes the primitive-vs-composed split and can provide stable family labels to runtime-facing inspection surfaces.</WCTX>
// <CLOG>0.3.0: add family_label() for runtime-facing seams.
// 0.2.0: promote the central V3 style-family enum to wrap explicit primitive and composed-primitive layers.</CLOG>

//! Canonical grouped V3 representation for spatial shaders across primitive
//! and composed layers.
//!
//! This enum is the lowering target for the legacy flat `SpatialShaderType`
//! catalog. It preserves the primitive/composed split while giving downstream
//! code one stable seam for docs, debug output, and runtime inspection.

use crate::models::v3::{VfxSpatialComposedPrimitive, VfxSpatialPrimitive};
use serde::{Deserialize, Serialize};

/// Top-level V3 representation for a spatial shader after family lowering.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "layer", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxSpatialShaderFamily {
    /// Primitive-layer spatial shader.
    Primitive(VfxSpatialPrimitive),
    /// Composed-primitive spatial shader.
    ComposedPrimitive(VfxSpatialComposedPrimitive),
}

impl VfxSpatialShaderFamily {
    /// Stable family label for inspection/debug surfaces.
    pub fn family_label(&self) -> &'static str {
        match self {
            Self::Primitive(primitive) => primitive.family_label(),
            Self::ComposedPrimitive(composed) => composed.family_label(),
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_spatial_shader_family.rs</FILE> - <DESC>Central V3 representation for spatial shaders across primitive and composed layers</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
