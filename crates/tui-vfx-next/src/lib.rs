// <FILE>crates/tui-vfx-next/src/lib.rs</FILE> - <DESC>Clean-room v3.1 surface and pipeline contract spike</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>New kernel Phase D1: expose scene/element/layer composition contract types.</WCTX>
// <CLOG>0.5.0: MINOR — expose Phase D1 scene composition DTOs and outcome.
// 0.4.1: PATCH — wire fnc_annotate_stage_diagnostics after OFPF helper extraction.</CLOG>

//! Clean-room v3.1 surface and scene contract spike.
//!
//! This crate proves the Phase A/B/C semantic surface, sampled-source, pipeline,
//! and Phase D1 scene composition rules
//! without depending on the legacy compositor, style, content, or shadow
//! implementation crates. It is intentionally small: a surface, scope
//! evaluation, coordinate sampling, write policy, diagnostics, and two tiny
//! effects that make role preservation and explicit role writes testable.

pub mod cls_apply_outcome;
pub mod cls_cell_channel;
pub mod cls_cell_write;
pub mod cls_cell_write_policy;
pub mod cls_clip_policy;
pub mod cls_coordinate_space;
pub mod cls_diagnostic_level;
pub mod cls_dim_effect;
pub mod cls_effect_descriptor;
pub mod cls_effect_domain;
pub mod cls_element_id;
pub mod cls_element_placement;
pub mod cls_explicit_role_write_effect;
pub mod cls_identity_sampler;
pub mod cls_layer_id;
pub mod cls_pipeline_outcome;
pub mod cls_pipeline_sampler;
pub mod cls_pipeline_stage;
pub mod cls_role_space;
pub mod cls_role_write_policy;
pub mod cls_scene;
pub mod cls_scene_element;
pub mod cls_scene_outcome;
pub mod cls_scope_eval_input;
pub mod cls_scope_spec;
pub mod cls_shift_sampler;
pub mod cls_surface;
pub mod cls_surface_diagnostic;
pub mod cls_surface_diagnostic_code;
pub mod cls_surface_engine;
pub mod cls_surface_metadata;
pub mod cls_surface_pipeline;
pub mod fnc_annotate_stage_diagnostic;
pub mod fnc_annotate_stage_diagnostics;
pub mod fnc_apply_from_source_with_sampler;
pub mod fnc_rewrite_glyph_cell;
pub mod fnc_scope_coordinate;
pub mod tr_coordinate_sampler;

pub use cls_apply_outcome::ApplyOutcome;
pub use cls_cell_channel::CellChannel;
pub use cls_cell_write::CellWrite;
pub use cls_cell_write_policy::CellWritePolicy;
pub use cls_clip_policy::ClipPolicy;
pub use cls_coordinate_space::CoordinateSpace;
pub use cls_diagnostic_level::DiagnosticLevel;
pub use cls_dim_effect::DimEffect;
pub use cls_effect_descriptor::EffectDescriptor;
pub use cls_effect_domain::EffectDomain;
pub use cls_element_id::ElementId;
pub use cls_element_placement::ElementPlacement;
pub use cls_explicit_role_write_effect::ExplicitRoleWriteEffect;
pub use cls_identity_sampler::IdentitySampler;
pub use cls_layer_id::LayerId;
pub use cls_pipeline_outcome::PipelineOutcome;
pub use cls_pipeline_sampler::PipelineSampler;
pub use cls_pipeline_stage::PipelineStage;
pub use cls_role_space::RoleSpace;
pub use cls_role_write_policy::RoleWritePolicy;
pub use cls_scene::Scene;
pub use cls_scene_element::SceneElement;
pub use cls_scene_outcome::SceneOutcome;
pub use cls_scope_eval_input::ScopeEvalInput;
pub use cls_scope_spec::ScopeSpec;
pub use cls_shift_sampler::ShiftSampler;
pub use cls_surface::Surface;
pub use cls_surface_diagnostic::SurfaceDiagnostic;
pub use cls_surface_diagnostic_code::SurfaceDiagnosticCode;
pub use cls_surface_engine::SurfaceEngine;
pub use cls_surface_metadata::SurfaceMetadata;
pub use cls_surface_pipeline::SurfacePipeline;
pub use tr_coordinate_sampler::CoordinateSampler;

// <FILE>crates/tui-vfx-next/src/lib.rs</FILE> - <DESC>Clean-room v3.1 surface and pipeline contract spike</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
