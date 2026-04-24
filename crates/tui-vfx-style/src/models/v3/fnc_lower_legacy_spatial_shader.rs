// <FILE>tui-vfx-style/src/models/v3/fnc_lower_legacy_spatial_shader.rs</FILE> - <DESC>Lower the legacy flat spatial shader surface into the primitive/composed V3 layers</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Decision 2 migration slice — centralize lowering from SpatialShaderType into the grouped V3 family representation and make the primitive/composed split explicit for downstream runtime wiring.</WCTX>
// <CLOG>0.3.0: lower RadialSpiral into the V3 motion-field primitive family.
// Promote lower_legacy_spatial_shader to return the top-level VfxSpatialShaderFamily with explicit primitive/composed variants.</CLOG>

//! Central lowering helper from the legacy flat spatial shader surface into the
//! grouped V3 primitive/composed family representation.

use crate::models::{
    SpatialShaderType,
    v3::{VfxSpatialComposedPrimitive, VfxSpatialPrimitive, VfxSpatialShaderFamily},
};

impl VfxSpatialShaderFamily {
    /// Lower a legacy flat `SpatialShaderType` into the grouped V3 family form.
    pub fn from_legacy_spatial_shader(shader: &SpatialShaderType) -> Self {
        lower_legacy_spatial_shader(shader)
    }
}

/// Lower a legacy flat `SpatialShaderType` into the grouped V3 family form.
pub fn lower_legacy_spatial_shader(shader: &SpatialShaderType) -> VfxSpatialShaderFamily {
    match shader {
        SpatialShaderType::LinearGradient(shader) => {
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::GradientReveal(shader.into()))
        }
        SpatialShaderType::BarberPole(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::StripeMotion(shader.into()),
        ),
        SpatialShaderType::Radar(shader) => {
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(shader.into()))
        }
        SpatialShaderType::Orbit(shader) => {
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(shader.into()))
        }
        SpatialShaderType::BorderSweep(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::TravelingBand(shader.into()),
        ),
        SpatialShaderType::Highlighter(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::ProgressEmphasis(shader.into()),
        ),
        SpatialShaderType::Reflect(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::TravelingBand(shader.into()),
        ),
        SpatialShaderType::GlistenBand(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::TravelingBand(shader.into()),
        ),
        SpatialShaderType::GlitchLines(shader) => {
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::EdgeDistortion(shader.into()))
        }
        SpatialShaderType::NeonFlicker(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::StochasticTexture(shader.into()),
        ),
        SpatialShaderType::PulseWave(shader) => {
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(shader.into()))
        }
        SpatialShaderType::RadialSpiral(shader) => {
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::MotionField(shader.into()))
        }
        SpatialShaderType::TracePropagation(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::TravelingBand(shader.into()),
        ),
        SpatialShaderType::TracePath(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::TravelingBand(shader.into()),
        ),
        SpatialShaderType::FocusedRowGradient(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::GuidanceCue(shader.into()),
        ),
        SpatialShaderType::RevealWipe(shader) => {
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::GradientReveal(shader.into()))
        }
        SpatialShaderType::StochasticSparkle(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::StochasticTexture(shader.into()),
        ),
        SpatialShaderType::AmbientOcclusion(shader) => {
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::SurfaceDepth(shader.into()))
        }
        SpatialShaderType::Bevel(shader) => {
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::SurfaceDepth(shader.into()))
        }
        SpatialShaderType::Glow(shader) => {
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::SurfaceDepth(shader.into()))
        }
        SpatialShaderType::EdgeSheen(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::MaterialLight(shader.into()),
        ),
        SpatialShaderType::ConcealedLight(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::MaterialLight(shader.into()),
        ),
        SpatialShaderType::Diffusion(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::MaterialLight(shader.into()),
        ),
        SpatialShaderType::FocusField(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::GuidanceCue(shader.into()),
        ),
        SpatialShaderType::AffordanceWake(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::GuidanceCue(shader.into()),
        ),
        SpatialShaderType::WayfindingNode(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::GuidanceCue(shader.into()),
        ),
        SpatialShaderType::SubCellShake(shader) => {
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::EdgeDistortion(shader.into()))
        }
        SpatialShaderType::ChromaticEdge(shader) => {
            VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::EdgeDistortion(shader.into()))
        }
        SpatialShaderType::Cursor(shader) => VfxSpatialShaderFamily::ComposedPrimitive(
            VfxSpatialComposedPrimitive::Cursor(shader.into()),
        ),
    }
}

// <FILE>tui-vfx-style/src/models/v3/fnc_lower_legacy_spatial_shader.rs</FILE> - <DESC>Lower the legacy flat spatial shader surface into the primitive/composed V3 layers</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
