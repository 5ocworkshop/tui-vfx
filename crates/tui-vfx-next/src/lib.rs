// <FILE>crates/tui-vfx-next/src/lib.rs</FILE> - <DESC>Clean-room v3.1 surface and pipeline contract spike</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>New kernel Phase D3: expose logical contract/proof/schema-root module boundaries.</WCTX>
// <CLOG>0.6.0: MINOR — add logical contract, proof, and schema-root re-export modules without changing runtime behavior.
// 0.5.0: MINOR — expose Phase D1 scene composition DTOs and outcome.
// 0.4.1: PATCH — wire fnc_annotate_stage_diagnostics after OFPF helper extraction.</CLOG>

//! Clean-room v3.1 surface and scene contract spike.
//!
//! This crate proves the Phase A/B/C semantic surface, sampled-source, pipeline,
//! and Phase D1 scene composition rules
//! without depending on the legacy compositor, style, content, or shadow
//! implementation crates. Phase D3 adds a logical boundary: [`contract`] is the
//! stable vocabulary future descriptors should reuse, [`proof`] is the small
//! execution harness used to prove behavior, and [`schema_roots`] lists checked
//! schema fixtures that must remain current.

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

/// Stable v3.1 contract vocabulary for future descriptors and canonical recipes.
///
/// These exports describe public semantic concepts: surfaces, scenes, scopes,
/// writes, roles, samplers, diagnostics, ids, and operation outcomes. The module
/// is a logical boundary; the crate intentionally remains physically unsplit in
/// Phase D3.
pub mod contract {
    pub use crate::{
        ApplyOutcome, CellChannel, CellWrite, CellWritePolicy, ClipPolicy, CoordinateSampler,
        CoordinateSpace, DiagnosticLevel, EffectDomain, ElementId, ElementPlacement, LayerId,
        PipelineOutcome, PipelineSampler, RoleSpace, RoleWritePolicy, Scene, SceneElement,
        SceneOutcome, ScopeSpec, ShiftSampler, Surface, SurfaceDiagnostic, SurfaceDiagnosticCode,
        SurfaceMetadata,
    };
}

/// Clean-room proof harness and toy implementation pieces.
///
/// These exports are public so tests and downstream experiments can prove the
/// contract, but future descriptor or recipe schemas must not copy their toy
/// effect/stage shapes as the production descriptor model.
pub mod proof {
    pub use crate::{
        CoordinateSampler, DimEffect, EffectDescriptor, ExplicitRoleWriteEffect, IdentitySampler,
        PipelineStage, SurfaceEngine, SurfacePipeline,
    };
}

/// Checked schema roots for the current clean-room public artifacts.
///
/// A type in this module is intentionally represented by a checked JSON Schema
/// fixture under `schemas/v3.1/next/`. Some roots, especially the Phase C toy
/// pipeline, are contract-visible proof artifacts rather than the future recipe
/// or descriptor model.
pub mod schema_roots {
    pub use crate::{
        CellWrite, PipelineSampler, Scene, SceneElement, SceneOutcome, ScopeSpec, Surface,
        SurfaceDiagnostic, SurfacePipeline,
    };
}

// <FILE>crates/tui-vfx-next/src/lib.rs</FILE> - <DESC>Clean-room v3.1 surface and pipeline contract spike</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
