// <FILE>crates/tui-vfx-compost/src/primitive/cls_effect_runtime_kind.rs</FILE> - <DESC>Primitive registry runtime domain labels</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>The registry records which domain-specific runtime table owns each descriptor id.</WCTX>
// <CLOG>0.1.0: INIT — add effect runtime kind and EffectDomain mapping.</CLOG>

use tui_vfx_contract::EffectDomain;

/// Domain-specific runtime table for an effect primitive.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectRuntimeKind {
    /// Descriptor is carried for catalogs/codegen but has no runtime table yet.
    DescriptorOnly,
    /// `EffectDomain::CellShader` runtime.
    CellShader,
    /// `EffectDomain::FrameFilter` runtime.
    FrameFilter,
    /// `EffectDomain::CoordinateSampler` runtime.
    CoordinateSampler,
    /// `EffectDomain::Mask` runtime.
    Mask,
    /// `EffectDomain::ContentTransform` runtime.
    ContentTransform,
}

impl EffectRuntimeKind {
    /// Return the descriptor domain required for this runtime kind.
    pub fn required_domain(self) -> Option<EffectDomain> {
        match self {
            Self::DescriptorOnly => None,
            Self::CellShader => Some(EffectDomain::CellShader),
            Self::FrameFilter => Some(EffectDomain::FrameFilter),
            Self::CoordinateSampler => Some(EffectDomain::CoordinateSampler),
            Self::Mask => Some(EffectDomain::Mask),
            Self::ContentTransform => Some(EffectDomain::ContentTransform),
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_effect_runtime_kind.rs</FILE> - <DESC>Primitive registry runtime domain labels</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
