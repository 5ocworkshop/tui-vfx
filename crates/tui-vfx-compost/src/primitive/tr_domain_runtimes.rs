// <FILE>crates/tui-vfx-compost/src/primitive/tr_domain_runtimes.rs</FILE> - <DESC>Domain-specific primitive runtime traits</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>One universal EffectRuntime would be filter-shaped; Phase 0 maps runtime methods to EffectDomain semantics instead.</WCTX>
// <CLOG>0.1.0: INIT — add cell shader, frame filter, mask, coordinate sampler, content transform, and source runtime traits.</CLOG>

use super::{
    CellView, CoordinateSample, EffectPrimitive, EffectRuntimeContext, EffectRuntimeError,
    MaskVisibility, SourcePrimitive, SourceSurface,
};

/// Runtime behavior for `EffectDomain::CellShader` primitives.
pub trait CellShaderRuntime: EffectPrimitive {
    /// Shade one cell in place.
    fn shade_cell(
        inputs: &Self::Inputs,
        cell: &mut CellView<'_, Self>,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<(), EffectRuntimeError>
    where
        Self: Sized;
}

/// Runtime behavior for `EffectDomain::FrameFilter` primitives.
pub trait FrameFilterRuntime: EffectPrimitive {
    /// Filter one cell in place within the current frame/sample context.
    fn filter_cell(
        inputs: &Self::Inputs,
        cell: &mut CellView<'_, Self>,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<(), EffectRuntimeError>
    where
        Self: Sized;
}

/// Runtime behavior for `EffectDomain::Mask` primitives.
pub trait MaskRuntime: EffectPrimitive {
    /// Return this primitive's visibility contribution for one sampled cell.
    fn visibility(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<MaskVisibility, EffectRuntimeError>;
}

/// Runtime behavior for `EffectDomain::CoordinateSampler` primitives.
pub trait CoordinateSamplerRuntime: EffectPrimitive {
    /// Select the source coordinate to sample for one destination cell.
    fn sample_coordinate(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<CoordinateSample, EffectRuntimeError>;
}

/// Runtime behavior for `EffectDomain::ContentTransform` primitives.
pub trait ContentTransformRuntime: EffectPrimitive {
    /// Transform content channels for one cell.
    fn transform_cell(
        inputs: &Self::Inputs,
        cell: &mut CellView<'_, Self>,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<(), EffectRuntimeError>
    where
        Self: Sized;
}

/// Runtime behavior for source descriptors.
pub trait SourceRuntime: SourcePrimitive {
    /// Materialize this source into a semantic surface.
    fn materialize(
        inputs: &Self::Inputs,
        context: &EffectRuntimeContext<'_>,
    ) -> Result<SourceSurface, EffectRuntimeError>;
}

// <FILE>crates/tui-vfx-compost/src/primitive/tr_domain_runtimes.rs</FILE> - <DESC>Domain-specific primitive runtime traits</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
