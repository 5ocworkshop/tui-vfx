// <FILE>crates/tui-vfx-compost/src/render/mod.rs</FILE> - <DESC>Native v3.1 render entrypoints and types</DESC>
// <VERS>VERSION: 0.8.0</VERS>
// <WCTX>Render orchestration is split by recipe, scene, element, clip, effect-stack, and timing responsibilities.</WCTX>
// <CLOG>0.8.0: MINOR — expose execution-stage trace aggregation helpers.
// 0.7.0: MINOR — expose native observability helpers for stage trace lifecycle.
// 0.6.1: PATCH — expose shader phase timing through the renamed render helper.
// 0.6.0: MINOR — add native frame diagnostics and trace event observability.
// 0.5.0: MINOR — add cell, role, and parallel-merge policy seams for native surface writes.
// 0.4.0: MINOR — add native timing substrate.
// 0.3.0: MINOR — add native effect stack substrate.
// 0.2.0: MINOR — add scene and element helpers for substrate composition.
// 0.1.0: INIT — add frame, context, error, and render entrypoint modules.</CLOG>

mod cls_cell_write_decision;
mod cls_effect_stack;
mod cls_effect_stage;
mod cls_element_render_outcome;
mod cls_frame;
mod cls_render_diagnostic;
mod cls_render_error;
mod cls_render_timing;
mod cls_render_trace_event;
mod cls_sample_context;
mod col_collect_graph_step_nodes;
mod col_scene_elements_in_paint_order;
mod fnc_apply_cell_write_policy;
mod fnc_apply_effect_stack;
mod fnc_apply_role_write_policy;
mod fnc_blend_shadow_cell;
mod fnc_blend_underlying_shadow_cell;
mod fnc_build_effect_stack;
mod fnc_build_shadow_config;
mod fnc_clip_element_bounds;
mod fnc_element_bounds_fully_visible;
mod fnc_evaluate_value_predicate;
mod fnc_execute_effect_graph;
mod fnc_is_node_active;
mod fnc_is_scene_element_visible;
mod fnc_merge_element_surface;
mod fnc_merge_parallel_surfaces;
mod fnc_publish_node_outputs;
mod fnc_render_clipped_scene_element;
mod fnc_render_element_shadow;
mod fnc_render_hidden_overflow_scene_element;
mod fnc_render_recipe;
mod fnc_render_scene;
mod fnc_render_scene_element;
mod fnc_render_wrapped_scene_element;
mod fnc_resolve_element_graph_sample;
mod fnc_resolve_shader_phase_t;
mod fnc_shadow_cast_rect;
mod fnc_wrap_element_cell_bounds;
mod orc_render_observability;

pub use cls_frame::Frame;
pub use cls_render_diagnostic::RenderDiagnostic;
pub use cls_render_error::RenderError;
pub use cls_render_trace_event::{RenderSkipReason, RenderStageKind, RenderTraceEvent};
pub use cls_sample_context::SampleContext;
pub use fnc_render_recipe::{render_recipe, render_recipe_scene};

pub(crate) use cls_cell_write_decision::CellWriteDecision;
pub(crate) use cls_effect_stack::EffectStack;
pub(crate) use cls_effect_stage::{EffectFamily, EffectStage};
pub(crate) use cls_element_render_outcome::ElementRenderOutcome;
pub(crate) use cls_render_timing::RenderTiming;
pub(crate) use col_collect_graph_step_nodes::collect_graph_step_nodes;
pub(crate) use col_scene_elements_in_paint_order::scene_elements_in_paint_order;
pub(crate) use fnc_apply_cell_write_policy::apply_cell_write_policy;
pub(crate) use fnc_apply_effect_stack::{ScopeCoordinateMode, apply_effect_stack};
pub(crate) use fnc_apply_role_write_policy::apply_role_write_policy;
pub(crate) use fnc_blend_shadow_cell::blend_shadow_cell;
pub(crate) use fnc_blend_underlying_shadow_cell::{
    blend_shadow_color, blend_underlying_shadow_cell,
};
pub(crate) use fnc_build_effect_stack::build_effect_stack;
pub(crate) use fnc_build_shadow_config::build_shadow_config;
pub(crate) use fnc_clip_element_bounds::{ElementClipBounds, clip_element_bounds};
pub(crate) use fnc_element_bounds_fully_visible::element_bounds_fully_visible;
pub(crate) use fnc_evaluate_value_predicate::evaluate_value_predicate;
pub(crate) use fnc_execute_effect_graph::execute_effect_graph;
pub(crate) use fnc_is_node_active::is_node_active;
pub(crate) use fnc_is_scene_element_visible::is_scene_element_visible;
pub(crate) use fnc_merge_element_surface::merge_element_surface;
pub(crate) use fnc_merge_parallel_surfaces::{
    ParallelMergeConflict, explicit_node_write_mask, parallel_merge_conflict,
};
pub(crate) use fnc_publish_node_outputs::publish_node_outputs;
pub(crate) use fnc_render_clipped_scene_element::{
    render_clipped_scene_element, source_fits_scene,
};
pub(crate) use fnc_render_element_shadow::render_element_shadow;
pub(crate) use fnc_render_hidden_overflow_scene_element::render_hidden_overflow_scene_element;
pub(crate) use fnc_render_scene::render_scene;
pub(crate) use fnc_render_scene_element::render_scene_element;
pub(crate) use fnc_render_wrapped_scene_element::render_wrapped_scene_element;
pub(crate) use fnc_resolve_element_graph_sample::resolve_element_graph_sample;
pub(crate) use fnc_resolve_shader_phase_t::resolve_shader_phase_t;
pub(crate) use fnc_shadow_cast_rect::{shadow_cast_rect, shadow_edge_progress};
pub(crate) use fnc_wrap_element_cell_bounds::wrap_element_cell_bounds;
pub(crate) use orc_render_observability::{
    CellStageTrace, RenderStageAccumulator, ScopeDestination, parallel_cell_trace,
    scope_eval_input, trace_element_skipped,
};

// <FILE>crates/tui-vfx-compost/src/render/mod.rs</FILE> - <DESC>Native v3.1 render entrypoints and types</DESC>
// <VERS>END OF VERSION: 0.8.0</VERS>
