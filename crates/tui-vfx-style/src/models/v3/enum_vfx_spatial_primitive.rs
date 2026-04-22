// <FILE>tui-vfx-style/src/models/v3/enum_vfx_spatial_primitive.rs</FILE> - <DESC>Primitive-layer V3 spatial shader representation</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Decision 2 migration slice — make the primitive-vs-composed split explicit once all grouped family surfaces exist and expose a stable primitive-family label for downstream runtime wiring and inspection.</WCTX>
// <CLOG>0.2.0: add family_label() for inspection/runtime seams while keeping the grouped primitive surface stable.
// 0.1.0: define the VfxSpatialPrimitive enum over grouped primitive-family V3 shader surfaces.</CLOG>

//! Primitive-layer V3 spatial shader representation.
//!
//! This enum is the primitive half of the central V3 style-family seam. It is
//! intentionally narrower than the old flat shader catalog: only families the
//! plan treats as true primitives belong here.

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

impl VfxSpatialPrimitive {
    /// Stable primitive-family label for inspection/debug surfaces.
    pub fn family_label(&self) -> &'static str {
        match self {
            Self::SurfaceDepth(_) => "surface_depth",
            Self::MotionField(_) => "motion_field",
            Self::EdgeDistortion(_) => "edge_distortion",
            Self::GradientReveal(_) => "gradient_reveal",
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_spatial_primitive.rs</FILE> - <DESC>Primitive-layer V3 spatial shader representation</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
