// <FILE>tui-vfx-compositor/src/pipeline/cls_shader_layer_spec.rs</FILE>
// <DESC>Shader layer spec for pipeline bindings</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Spec-based pipeline API for bindings</WCTX>
// <CLOG>0.2.0: add a fallible constructor from grouped V3 spatial shader families so runtime seams can execute grouped family values through the legacy shader surface during cutover.
// Initial ShaderLayerSpec with SpatialShaderType and StyleRegion</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_style::models::{
    SpatialShaderType, StyleRegion, TryLowerV3SpatialShaderError, VfxSpatialShaderFamily,
    lower_legacy_spatial_shader, try_lower_v3_spatial_shader_family,
};

/// Serializable shader layer specification for pipeline bindings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct ShaderLayerSpec {
    /// Shader implementation and parameters.
    pub shader: SpatialShaderType,
    /// Region constraint for this shader (default: All).
    #[serde(default)]
    pub region: StyleRegion,
}

// <FILE>tui-vfx-compositor/src/pipeline/cls_shader_layer_spec.rs</FILE>
// <DESC>Shader layer spec for pipeline bindings</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>

impl ShaderLayerSpec {

    /// Build a shader-layer spec directly from a grouped V3 spatial family by
    /// lowering it back into the current executable legacy shader surface.
    pub fn try_from_v3_shader_family(
        family: &VfxSpatialShaderFamily,
        region: StyleRegion,
    ) -> Result<Self, TryLowerV3SpatialShaderError> {
        Ok(Self {
            shader: try_lower_v3_spatial_shader_family(family)?,
            region,
        })
    }

    /// Returns the grouped V3 family form of this shader layer's spatial shader.
    pub fn v3_shader_family(&self) -> VfxSpatialShaderFamily {
        lower_legacy_spatial_shader(&self.shader)
    }
}
