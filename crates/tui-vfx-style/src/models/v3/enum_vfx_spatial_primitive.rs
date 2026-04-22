// <FILE>tui-vfx-style/src/models/v3/enum_vfx_spatial_primitive.rs</FILE> - <DESC>Primitive-layer V3 spatial shader representation</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — make the primitive-vs-composed split explicit once all grouped family surfaces exist.</WCTX>
// <CLOG>Define the VfxSpatialPrimitive enum over grouped primitive-family V3 shader surfaces.</CLOG>

//! Primitive-layer V3 spatial shader representation.

use crate::models::v3::{
    VfxEdgeDistortionShader, VfxGradientRevealShader, VfxMotionFieldShader, VfxSurfaceDepthShader,
};
use serde::{Deserialize, Serialize};

/// Primitive-layer V3 representation for spatial shaders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "primitive", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxSpatialPrimitive {
    /// Surface/depth primitive family.
    SurfaceDepth(VfxSurfaceDepthShader),
    /// Motion-field primitive family.
    MotionField(VfxMotionFieldShader),
    /// Edge-distortion primitive family.
    EdgeDistortion(VfxEdgeDistortionShader),
    /// Gradient/reveal primitive family.
    GradientReveal(VfxGradientRevealShader),
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_spatial_primitive.rs</FILE> - <DESC>Primitive-layer V3 spatial shader representation</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
