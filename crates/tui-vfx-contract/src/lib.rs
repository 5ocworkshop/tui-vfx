// <FILE>crates/tui-vfx-contract/src/lib.rs</FILE> - <DESC>Stable v3.1 contract DTO exports</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase F2: expose declarative value source and binding contract vocabulary.</WCTX>
// <CLOG>0.4.0: MINOR — export ValueSource, ParameterSpec, SignalSpec, BindingSpec, and schema roots.
// 0.3.0: MINOR — export Value, ValueSpec, EffectInputSpec, and input schema roots.
// 0.2.0: MINOR — export EffectDescriptor and descriptor capability DTOs/schema root.
// 0.1.0: INIT — expose stable surface, scene, scope, write, sampler, diagnostic, and schema-root contract vocabulary.</CLOG>

//! Stable v3.1 contract DTOs for tui-vfx.
//!
//! This crate owns public contract vocabulary that is useful without the
//! clean-room proof engine. It carries Serde/Schemars-ready DTOs, rustdoc-backed
//! schema descriptions, and no dependency on the proof incubator or legacy
//! compositor/style/content/shadow crates.

pub mod cls_apply_outcome;
pub mod cls_binding_mode;
pub mod cls_binding_spec;
pub mod cls_binding_target;
pub mod cls_cell_access;
pub mod cls_cell_channel;
pub mod cls_cell_write;
pub mod cls_cell_write_policy;
pub mod cls_clip_policy;
pub mod cls_coordinate_space;
pub mod cls_descriptor_validation_error;
pub mod cls_diagnostic_level;
pub mod cls_effect_completion;
pub mod cls_effect_descriptor;
pub mod cls_effect_domain;
pub mod cls_effect_id;
pub mod cls_effect_input_id;
pub mod cls_effect_input_spec;
pub mod cls_effect_lifecycle;
pub mod cls_element_id;
pub mod cls_element_placement;
pub mod cls_layer_id;
pub mod cls_numeric_range;
pub mod cls_parameter_id;
pub mod cls_parameter_spec;
pub mod cls_role_space;
pub mod cls_role_write_policy;
pub mod cls_role_write_policy_kind;
pub mod cls_runtime_mutability;
pub mod cls_scene;
pub mod cls_scene_element;
pub mod cls_scene_outcome;
pub mod cls_scope_eval_input;
pub mod cls_scope_kind;
pub mod cls_scope_spec;
pub mod cls_scope_support;
pub mod cls_shift_sampler;
pub mod cls_signal_id;
pub mod cls_signal_spec;
pub mod cls_surface;
pub mod cls_surface_diagnostic;
pub mod cls_surface_diagnostic_code;
pub mod cls_surface_metadata;
pub mod cls_value;
pub mod cls_value_kind;
pub mod cls_value_source;
pub mod cls_value_spec;
pub mod cls_write_support;
pub mod fnc_scope_coordinate;
pub mod tr_coordinate_sampler;

pub use cls_apply_outcome::ApplyOutcome;
pub use cls_binding_mode::BindingMode;
pub use cls_binding_spec::BindingSpec;
pub use cls_binding_target::BindingTarget;
pub use cls_cell_access::CellAccess;
pub use cls_cell_channel::CellChannel;
pub use cls_cell_write::CellWrite;
pub use cls_cell_write_policy::CellWritePolicy;
pub use cls_clip_policy::ClipPolicy;
pub use cls_coordinate_space::CoordinateSpace;
pub use cls_descriptor_validation_error::DescriptorValidationError;
pub use cls_diagnostic_level::DiagnosticLevel;
pub use cls_effect_completion::EffectCompletion;
pub use cls_effect_descriptor::EffectDescriptor;
pub use cls_effect_domain::EffectDomain;
pub use cls_effect_id::EffectId;
pub use cls_effect_input_id::EffectInputId;
pub use cls_effect_input_spec::EffectInputSpec;
pub use cls_effect_lifecycle::EffectLifecycle;
pub use cls_element_id::ElementId;
pub use cls_element_placement::ElementPlacement;
pub use cls_layer_id::LayerId;
pub use cls_numeric_range::NumericRange;
pub use cls_parameter_id::ParameterId;
pub use cls_parameter_spec::ParameterSpec;
pub use cls_role_space::RoleSpace;
pub use cls_role_write_policy::RoleWritePolicy;
pub use cls_role_write_policy_kind::RoleWritePolicyKind;
pub use cls_runtime_mutability::RuntimeMutability;
pub use cls_scene::Scene;
pub use cls_scene_element::SceneElement;
pub use cls_scene_outcome::SceneOutcome;
pub use cls_scope_eval_input::ScopeEvalInput;
pub use cls_scope_kind::ScopeKind;
pub use cls_scope_spec::ScopeSpec;
pub use cls_scope_support::ScopeSupport;
pub use cls_shift_sampler::ShiftSampler;
pub use cls_signal_id::SignalId;
pub use cls_signal_spec::SignalSpec;
pub use cls_surface::Surface;
pub use cls_surface_diagnostic::SurfaceDiagnostic;
pub use cls_surface_diagnostic_code::SurfaceDiagnosticCode;
pub use cls_surface_metadata::SurfaceMetadata;
pub use cls_value::Value;
pub use cls_value_kind::ValueKind;
pub use cls_value_source::ValueSource;
pub use cls_value_spec::ValueSpec;
pub use cls_write_support::WriteSupport;
pub use tr_coordinate_sampler::CoordinateSampler;

/// Checked stable contract schema roots.
///
/// These roots generate fixtures under `schemas/v3.1/contract/`.
pub mod schema_roots {
    pub use crate::{
        BindingSpec, CellWrite, EffectDescriptor, EffectInputSpec, ParameterSpec, Scene,
        SceneElement, SceneOutcome, ScopeSpec, SignalSpec, Surface, SurfaceDiagnostic, Value,
        ValueSource,
    };
}

// <FILE>crates/tui-vfx-contract/src/lib.rs</FILE> - <DESC>Stable v3.1 contract DTO exports</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
