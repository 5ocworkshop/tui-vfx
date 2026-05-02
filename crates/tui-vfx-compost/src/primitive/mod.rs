// <FILE>crates/tui-vfx-compost/src/primitive/mod.rs</FILE> - <DESC>Rust-owned v3.1 primitive registry and runtime trait surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0 of Rust-SSOT primitive migration: expose descriptor traits, domain-specific runtime traits, and a registry skeleton without importing legacy compositor crates.</WCTX>
// <CLOG>0.1.0: INIT — add public primitive substrate traits, runtime context wrappers, cell access guard, source/runtime surfaces, and registry exports.</CLOG>

mod cls_cell_view;
mod cls_coordinate_sample;
mod cls_effect_registry;
mod cls_effect_runtime_context;
mod cls_effect_runtime_error;
mod cls_effect_runtime_kind;
mod cls_input_wrappers;
mod cls_mask_visibility;
mod cls_no_outputs;
mod cls_primitive_registry_error;
mod cls_source_surface;
mod tr_domain_runtimes;
mod tr_effect_primitive;
mod tr_primitive_inputs;
mod tr_primitive_outputs;
mod tr_source_primitive;

pub use cls_cell_view::CellView;
pub use cls_coordinate_sample::CoordinateSample;
pub use cls_effect_registry::EffectRegistry;
pub use cls_effect_runtime_context::EffectRuntimeContext;
pub use cls_effect_runtime_error::EffectRuntimeError;
pub use cls_effect_runtime_kind::EffectRuntimeKind;
pub use cls_input_wrappers::{Bindable, Literal};
pub use cls_mask_visibility::MaskVisibility;
pub use cls_no_outputs::NoOutputs;
pub use cls_primitive_registry_error::PrimitiveRegistryError;
pub use cls_source_surface::SourceSurface;
pub use tr_domain_runtimes::{
    CellShaderRuntime, ContentTransformRuntime, CoordinateSamplerRuntime, FrameFilterRuntime,
    MaskRuntime, SourceRuntime,
};
pub use tr_effect_primitive::EffectPrimitive;
pub use tr_primitive_inputs::{NoInputs, PrimitiveInputs};
pub use tr_primitive_outputs::PrimitiveOutputs;
pub use tr_source_primitive::SourcePrimitive;

// <FILE>crates/tui-vfx-compost/src/primitive/mod.rs</FILE> - <DESC>Rust-owned v3.1 primitive registry and runtime trait surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
