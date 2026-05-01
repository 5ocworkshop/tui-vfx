// <FILE>crates/tui-vfx-compost/src/render/mod.rs</FILE> - <DESC>Native v3.1 render entrypoints and types</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Render orchestration is split by recipe, scene, element, clip, effect-stack, and timing responsibilities.</WCTX>
// <CLOG>0.6.0: MINOR — add native frame diagnostics and trace event observability.
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
mod fnc_build_effect_stack;
mod fnc_clip_element_bounds;
mod fnc_is_node_active;
mod fnc_merge_element_surface;
mod fnc_merge_parallel_surfaces;
mod fnc_render_recipe;
mod fnc_render_scene;
mod fnc_render_scene_element;
mod fnc_resolve_node_phase;
mod orc_render_observability;

pub use cls_frame::Frame;
pub use cls_render_diagnostic::RenderDiagnostic;
pub use cls_render_error::RenderError;
pub use cls_render_trace_event::RenderTraceEvent;
pub use cls_sample_context::SampleContext;
pub use fnc_render_recipe::render_recipe;

pub(crate) use cls_cell_write_decision::CellWriteDecision;
pub(crate) use cls_effect_stack::EffectStack;
pub(crate) use cls_effect_stage::{EffectFamily, EffectStage};
pub(crate) use cls_element_render_outcome::ElementRenderOutcome;
pub(crate) use cls_render_timing::RenderTiming;
pub(crate) use col_collect_graph_step_nodes::collect_graph_step_nodes;
pub(crate) use col_scene_elements_in_paint_order::scene_elements_in_paint_order;
pub(crate) use fnc_apply_cell_write_policy::apply_cell_write_policy;
pub(crate) use fnc_apply_effect_stack::apply_effect_stack;
pub(crate) use fnc_apply_role_write_policy::apply_role_write_policy;
pub(crate) use fnc_build_effect_stack::build_effect_stack;
pub(crate) use fnc_clip_element_bounds::{ElementClipBounds, clip_element_bounds};
pub(crate) use fnc_is_node_active::is_node_active;
pub(crate) use fnc_merge_element_surface::merge_element_surface;
pub(crate) use fnc_merge_parallel_surfaces::has_parallel_surface_merge;
pub(crate) use fnc_render_scene::render_scene;
pub(crate) use fnc_render_scene_element::render_scene_element;
pub(crate) use fnc_resolve_node_phase::resolve_node_phase;
pub(crate) use orc_render_observability::trace_applied_effects;

// <FILE>crates/tui-vfx-compost/src/render/mod.rs</FILE> - <DESC>Native v3.1 render entrypoints and types</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
