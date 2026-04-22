// <FILE>tui-vfx-compositor/src/pipeline/cls_shader_layer_spec.rs</FILE>
// <DESC>Shader layer spec for pipeline bindings</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Spec-based pipeline API for bindings</WCTX>
// <CLOG>Initial ShaderLayerSpec with SpatialShaderType and StyleRegion</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_style::models::{
    SpatialShaderType, StyleRegion, VfxSpatialShaderFamily, lower_legacy_spatial_shader,
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
// <VERS>END OF VERSION: 0.1.0</VERS>

impl ShaderLayerSpec {
    /// Returns the grouped V3 family form of this shader layer's spatial shader.
    pub fn v3_shader_family(&self) -> VfxSpatialShaderFamily {
        lower_legacy_spatial_shader(&self.shader)
    }
}
