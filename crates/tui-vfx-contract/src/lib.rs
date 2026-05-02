// <FILE>crates/tui-vfx-contract/src/lib.rs</FILE> - <DESC>Stable v3.1 contract DTO exports</DESC>
// <VERS>VERSION: 0.15.0</VERS>
// <WCTX>Authoring shorthand canonicalize: expose the canonicalize module and recipe-intent provenance type.</WCTX>
// <CLOG>0.15.0: MINOR — expose the canonicalize module providing canonicalize_recipe and the RecipeIntent provenance type.
// 0.14.0: MINOR — export signal expressions, typed shadows, style color sources, and visibility geometry.
// 0.13.0: MINOR — export transition recipe-oracle track support types.
// 0.12.0: MINOR — export ScrollFactor scene-element metadata and schema root.
// 0.11.0: MINOR — export descriptor pack, pack ref, catalog, and catalog-aware validation.
// 0.10.0: MINOR — export recipe lifecycle, clock, duration, phase, and trigger schema roots.
// 0.9.0: MINOR — export canonical recipe document, recipe scene, source instance, and metadata schema roots.
// 0.8.0: MINOR — export source descriptor, source spec, asset, and source output schema roots.
// 0.7.0: MINOR — export graph value, effect output, node output, and value merge schema roots.
// 0.6.0: MINOR — export GraphStep, ParallelMergePolicy, and topology schema root.
// 0.5.0: MINOR — export GraphSpec, NodeSpec, GraphId, NodeId, and schema roots.
// 0.4.0: MINOR — export ValueSource, ParameterSpec, SignalSpec, BindingSpec, and schema roots.
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
pub mod cls_asset_format;
pub mod cls_asset_id;
pub mod cls_asset_kind;
pub mod cls_asset_locator;
pub mod cls_asset_ref;
pub mod cls_asset_requirement;
pub mod cls_asset_spec;
pub mod cls_binding_mode;
pub mod cls_binding_spec;
pub mod cls_binding_target;
pub mod cls_cell_access;
pub mod cls_cell_channel;
pub mod cls_cell_write;
pub mod cls_cell_write_policy;
pub mod cls_clip_policy;
pub mod cls_clock_mode;
pub mod cls_clock_spec;
pub mod cls_clock_value_source;
pub mod cls_coordinate_space;
pub mod cls_descriptor_catalog;
pub mod cls_descriptor_pack;
pub mod cls_descriptor_pack_id;
pub mod cls_descriptor_pack_ref;
pub mod cls_descriptor_validation_error;
pub mod cls_diagnostic_level;
pub mod cls_duration_spec;
pub mod cls_dwell_policy;
pub mod cls_easing_spec;
pub mod cls_effect_completion;
pub mod cls_effect_descriptor;
pub mod cls_effect_domain;
pub mod cls_effect_id;
pub mod cls_effect_input_id;
pub mod cls_effect_input_spec;
pub mod cls_effect_lifecycle;
pub mod cls_effect_output_id;
pub mod cls_effect_output_spec;
pub mod cls_element_id;
pub mod cls_element_placement;
pub mod cls_gradient_spec;
pub mod cls_graph_id;
pub mod cls_graph_spec;
pub mod cls_graph_step;
pub mod cls_graph_value_id;
pub mod cls_graph_value_kind;
pub mod cls_graph_value_merge_policy;
pub mod cls_graph_value_shape;
pub mod cls_layer_id;
pub mod cls_lifecycle_phase;
pub mod cls_lifecycle_spec;
pub mod cls_named_easing;
pub mod cls_node_id;
pub mod cls_node_output_source;
pub mod cls_node_output_spec;
pub mod cls_node_spec;
pub mod cls_numeric_range;
pub mod cls_parallel_merge_policy;
pub mod cls_parameter_id;
pub mod cls_parameter_spec;
pub mod cls_phase_spec;
pub mod cls_phase_timing;
pub mod cls_preview_loopback_spec;
pub mod cls_recipe_document;
pub mod cls_recipe_element_graph_binding;
pub mod cls_recipe_element_graph_timing;
pub mod cls_recipe_id;
pub mod cls_recipe_metadata;
pub mod cls_recipe_scene;
pub mod cls_recipe_scene_element;
pub mod cls_reduced_motion_kind;
pub mod cls_reduced_motion_policy;
pub mod cls_role_space;
pub mod cls_role_write_policy;
pub mod cls_role_write_policy_kind;
pub mod cls_runtime_mutability;
pub mod cls_scene;
pub mod cls_scene_anchor;
pub mod cls_scene_element;
pub mod cls_scene_element_overflow_policy;
pub mod cls_scene_element_placement_rule;
pub mod cls_scene_element_surface;
pub mod cls_scene_element_visibility;
pub mod cls_scene_id;
pub mod cls_scene_outcome;
pub mod cls_scope_eval_input;
pub mod cls_scope_kind;
pub mod cls_scope_spec;
pub mod cls_scope_support;
pub mod cls_scroll_factor;
pub mod cls_shadow_spec;
pub mod cls_shift_sampler;
pub mod cls_signal_expression_spec;
pub mod cls_signal_id;
pub mod cls_signal_spec;
pub mod cls_source_descriptor;
pub mod cls_source_id;
pub mod cls_source_input_id;
pub mod cls_source_input_spec;
pub mod cls_source_instance_id;
pub mod cls_source_kind;
pub mod cls_source_lifecycle;
pub mod cls_source_output_size;
pub mod cls_source_output_spec;
pub mod cls_source_role_policy;
pub mod cls_source_spec;
pub mod cls_structured_value;
pub mod cls_style_color_source;
pub mod cls_surface;
pub mod cls_surface_diagnostic;
pub mod cls_surface_diagnostic_code;
pub mod cls_surface_metadata;
pub mod cls_transition_blinds_orientation;
pub mod cls_transition_cascade_order;
pub mod cls_transition_edge;
pub mod cls_transition_focal;
pub mod cls_transition_id;
pub mod cls_transition_intent;
pub mod cls_transition_interruption;
pub mod cls_transition_materialize_pattern;
pub mod cls_transition_motion_path;
pub mod cls_transition_motion_sampling;
pub mod cls_transition_preset;
pub mod cls_transition_reveal_direction;
pub mod cls_transition_spec;
pub mod cls_transition_subject_ref;
pub mod cls_transition_subjects;
pub mod cls_transition_text_cursor;
pub mod cls_transition_text_cursor_wake;
pub mod cls_transition_timing;
pub mod cls_transition_track;
pub mod cls_transition_track_subject;
pub mod cls_transition_travel_direction;
pub mod cls_transition_variant;
pub mod cls_transition_variant_condition;
pub mod cls_transition_visibility_geometry;
pub mod cls_trigger_action;
pub mod cls_trigger_condition;
pub mod cls_trigger_latch_policy;
pub mod cls_trigger_reset_boundary;
pub mod cls_trigger_spec;
pub mod cls_value;
pub mod cls_value_kind;
pub mod cls_value_predicate;
pub mod cls_value_source;
pub mod cls_value_spec;
pub mod cls_visibility_iris_shape;
pub mod cls_write_support;
pub mod fnc_collect_graph_value_kinds;
pub mod fnc_scope_coordinate;
pub mod fnc_validate_recipe_with_catalog;
pub mod orc_validate_graph_spec;
pub mod orc_validate_recipe_document;
pub mod orc_validate_source_spec;
pub mod tr_coordinate_sampler;

pub mod canonicalize;

pub use cls_apply_outcome::ApplyOutcome;
pub use cls_asset_format::AssetFormat;
pub use cls_asset_id::AssetId;
pub use cls_asset_kind::AssetKind;
pub use cls_asset_locator::AssetLocator;
pub use cls_asset_ref::AssetRef;
pub use cls_asset_requirement::AssetRequirement;
pub use cls_asset_spec::AssetSpec;
pub use cls_binding_mode::BindingMode;
pub use cls_binding_spec::BindingSpec;
pub use cls_binding_target::BindingTarget;
pub use cls_cell_access::CellAccess;
pub use cls_cell_channel::CellChannel;
pub use cls_cell_write::CellWrite;
pub use cls_cell_write_policy::CellWritePolicy;
pub use cls_clip_policy::ClipPolicy;
pub use cls_clock_mode::ClockMode;
pub use cls_clock_spec::ClockSpec;
pub use cls_clock_value_source::ClockValueSource;
pub use cls_coordinate_space::CoordinateSpace;
pub use cls_descriptor_catalog::DescriptorCatalog;
pub use cls_descriptor_pack::DescriptorPack;
pub use cls_descriptor_pack_id::DescriptorPackId;
pub use cls_descriptor_pack_ref::DescriptorPackRef;
pub use cls_descriptor_validation_error::DescriptorValidationError;
pub use cls_diagnostic_level::DiagnosticLevel;
pub use cls_duration_spec::DurationSpec;
pub use cls_dwell_policy::DwellPolicy;
pub use cls_easing_spec::EasingSpec;
pub use cls_effect_completion::EffectCompletion;
pub use cls_effect_descriptor::EffectDescriptor;
pub use cls_effect_domain::EffectDomain;
pub use cls_effect_id::EffectId;
pub use cls_effect_input_id::EffectInputId;
pub use cls_effect_input_spec::EffectInputSpec;
pub use cls_effect_lifecycle::EffectLifecycle;
pub use cls_effect_output_id::EffectOutputId;
pub use cls_effect_output_spec::EffectOutputSpec;
pub use cls_element_id::ElementId;
pub use cls_element_placement::ElementPlacement;
pub use cls_gradient_spec::{GradientSpec, GradientStop};
pub use cls_graph_id::GraphId;
pub use cls_graph_spec::GraphSpec;
pub use cls_graph_step::GraphStep;
pub use cls_graph_value_id::GraphValueId;
pub use cls_graph_value_kind::GraphValueKind;
pub use cls_graph_value_merge_policy::GraphValueMergePolicy;
pub use cls_graph_value_shape::GraphValueShape;
pub use cls_layer_id::LayerId;
pub use cls_lifecycle_phase::LifecyclePhase;
pub use cls_lifecycle_spec::LifecycleSpec;
pub use cls_named_easing::NamedEasing;
pub use cls_node_id::NodeId;
pub use cls_node_output_source::NodeOutputSource;
pub use cls_node_output_spec::NodeOutputSpec;
pub use cls_node_spec::NodeSpec;
pub use cls_numeric_range::NumericRange;
pub use cls_parallel_merge_policy::ParallelMergePolicy;
pub use cls_parameter_id::ParameterId;
pub use cls_parameter_spec::ParameterSpec;
pub use cls_phase_spec::PhaseSpec;
pub use cls_phase_timing::PhaseTiming;
pub use cls_preview_loopback_spec::PreviewLoopbackSpec;
pub use cls_recipe_document::RecipeDocument;
pub use cls_recipe_element_graph_binding::RecipeElementGraphBinding;
pub use cls_recipe_element_graph_timing::RecipeElementGraphTiming;
pub use cls_recipe_id::RecipeId;
pub use cls_recipe_metadata::RecipeMetadata;
pub use cls_recipe_scene::RecipeScene;
pub use cls_recipe_scene_element::RecipeSceneElement;
pub use cls_reduced_motion_kind::ReducedMotionKind;
pub use cls_reduced_motion_policy::ReducedMotionPolicy;
pub use cls_role_space::RoleSpace;
pub use cls_role_write_policy::RoleWritePolicy;
pub use cls_role_write_policy_kind::RoleWritePolicyKind;
pub use cls_runtime_mutability::RuntimeMutability;
pub use cls_scene::Scene;
pub use cls_scene_anchor::SceneAnchor;
pub use cls_scene_element::SceneElement;
pub use cls_scene_element_overflow_policy::SceneElementOverflowPolicy;
pub use cls_scene_element_placement_rule::SceneElementPlacementRule;
pub use cls_scene_element_surface::SceneElementSurface;
pub use cls_scene_element_visibility::SceneElementVisibility;
pub use cls_scene_id::SceneId;
pub use cls_scene_outcome::SceneOutcome;
pub use cls_scope_eval_input::ScopeEvalInput;
pub use cls_scope_kind::ScopeKind;
pub use cls_scope_spec::ScopeSpec;
pub use cls_scope_support::ScopeSupport;
pub use cls_scroll_factor::ScrollFactor;
pub use cls_shadow_spec::{
    ShadowBlendMode, ShadowCompositeMode, ShadowEdge, ShadowEdgeCrossingPolicy, ShadowFalloff,
    ShadowGlyphMaterial, ShadowInset, ShadowOffset, ShadowOutset, ShadowSpec,
};
pub use cls_shift_sampler::ShiftSampler;
pub use cls_signal_expression_spec::SignalExpressionSpec;
pub use cls_signal_id::SignalId;
pub use cls_signal_spec::SignalSpec;
pub use cls_source_descriptor::SourceDescriptor;
pub use cls_source_id::SourceId;
pub use cls_source_input_id::SourceInputId;
pub use cls_source_input_spec::SourceInputSpec;
pub use cls_source_instance_id::SourceInstanceId;
pub use cls_source_kind::SourceKind;
pub use cls_source_lifecycle::SourceLifecycle;
pub use cls_source_output_size::SourceOutputSize;
pub use cls_source_output_spec::SourceOutputSpec;
pub use cls_source_role_policy::SourceRolePolicy;
pub use cls_source_spec::SourceSpec;
pub use cls_structured_value::StructuredValue;
pub use cls_style_color_source::StyleColorSource;
pub use cls_surface::Surface;
pub use cls_surface_diagnostic::SurfaceDiagnostic;
pub use cls_surface_diagnostic_code::SurfaceDiagnosticCode;
pub use cls_surface_metadata::SurfaceMetadata;
pub use cls_transition_blinds_orientation::TransitionBlindsOrientation;
pub use cls_transition_cascade_order::TransitionCascadeOrder;
pub use cls_transition_edge::TransitionEdge;
pub use cls_transition_focal::TransitionFocal;
pub use cls_transition_id::TransitionId;
pub use cls_transition_intent::TransitionIntent;
pub use cls_transition_interruption::TransitionInterruption;
pub use cls_transition_materialize_pattern::TransitionMaterializePattern;
pub use cls_transition_motion_path::TransitionMotionPath;
pub use cls_transition_motion_sampling::TransitionMotionSampling;
pub use cls_transition_preset::TransitionPreset;
pub use cls_transition_reveal_direction::TransitionRevealDirection;
pub use cls_transition_spec::TransitionSpec;
pub use cls_transition_subject_ref::TransitionSubjectRef;
pub use cls_transition_subjects::TransitionSubjects;
pub use cls_transition_text_cursor::TransitionTextCursor;
pub use cls_transition_text_cursor_wake::TransitionTextCursorWake;
pub use cls_transition_timing::TransitionTiming;
pub use cls_transition_track::TransitionTrack;
pub use cls_transition_track_subject::TransitionTrackSubject;
pub use cls_transition_travel_direction::TransitionTravelDirection;
pub use cls_transition_variant::TransitionVariant;
pub use cls_transition_variant_condition::TransitionVariantCondition;
pub use cls_transition_visibility_geometry::{
    TransitionCornerArcMode, TransitionDistanceMetric, TransitionVisibilityGeometry,
};
pub use cls_trigger_action::TriggerAction;
pub use cls_trigger_condition::TriggerCondition;
pub use cls_trigger_latch_policy::TriggerLatchPolicy;
pub use cls_trigger_reset_boundary::TriggerResetBoundary;
pub use cls_trigger_spec::TriggerSpec;
pub use cls_value::Value;
pub use cls_value_kind::ValueKind;
pub use cls_value_predicate::ValuePredicate;
pub use cls_value_source::{GraphValueKinds, ValueSource};
pub use cls_value_spec::ValueSpec;
pub use cls_visibility_iris_shape::VisibilityIrisShape;
pub use cls_write_support::WriteSupport;
pub use tr_coordinate_sampler::CoordinateSampler;

/// Checked stable contract schema roots.
///
/// These roots generate fixtures under `schemas/v3.1/contract/`.
pub mod schema_roots {
    pub use crate::{
        AssetRef, AssetRequirement, AssetSpec, BindingSpec, CellWrite, ClockSpec, ClockValueSource,
        DurationSpec, DwellPolicy, EasingSpec, EffectDescriptor, EffectInputSpec, EffectOutputSpec,
        GradientSpec, GraphSpec, GraphStep, GraphValueId, GraphValueKind, GraphValueMergePolicy,
        GraphValueShape, LifecycleSpec, NamedEasing, NodeOutputSource, NodeOutputSpec, NodeSpec,
        ParameterSpec, PhaseSpec, PreviewLoopbackSpec, RecipeDocument, RecipeElementGraphBinding,
        RecipeMetadata, RecipeScene, RecipeSceneElement, ReducedMotionPolicy, Scene, SceneAnchor,
        SceneElement, SceneElementOverflowPolicy, SceneElementPlacementRule, SceneElementSurface,
        SceneElementVisibility, SceneOutcome, ScopeSpec, ScrollFactor, ShadowSpec,
        SignalExpressionSpec, SignalSpec, SourceDescriptor, SourceInputSpec, SourceInstanceId,
        SourceOutputSpec, SourceSpec, StructuredValue, StyleColorSource, Surface,
        SurfaceDiagnostic, TransitionSpec, TransitionVisibilityGeometry, TriggerSpec, Value,
        ValuePredicate, ValueSource,
    };
    pub use crate::{DescriptorCatalog, DescriptorPack, DescriptorPackId, DescriptorPackRef};
}

// <FILE>crates/tui-vfx-contract/src/lib.rs</FILE> - <DESC>Stable v3.1 contract DTO exports</DESC>
// <VERS>END OF VERSION: 0.15.0</VERS>
