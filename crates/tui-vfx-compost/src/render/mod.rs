// <FILE>crates/tui-vfx-compost/src/render/mod.rs</FILE> - <DESC>Native v3.1 render entrypoints and types</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Render orchestration is split by recipe, scene, element, and clip responsibilities.</WCTX>
// <CLOG>0.2.0: MINOR — add scene and element helpers for Phase 1 substrate composition.
// 0.1.0: INIT — add frame, context, error, and render entrypoint modules.</CLOG>

mod cls_frame;
mod cls_render_error;
mod cls_sample_context;
mod col_collect_graph_step_nodes;
mod col_scene_elements_in_paint_order;
mod fnc_clip_element_bounds;
mod fnc_render_recipe;
mod fnc_render_scene;
mod fnc_render_scene_element;

pub use cls_frame::Frame;
pub use cls_render_error::RenderError;
pub use cls_sample_context::SampleContext;
pub use fnc_render_recipe::render_recipe;

pub(crate) use col_collect_graph_step_nodes::collect_graph_step_nodes;
pub(crate) use col_scene_elements_in_paint_order::scene_elements_in_paint_order;
pub(crate) use fnc_clip_element_bounds::{ElementClipBounds, clip_element_bounds};
pub(crate) use fnc_render_scene::render_scene;
pub(crate) use fnc_render_scene_element::render_scene_element;

// <FILE>crates/tui-vfx-compost/src/render/mod.rs</FILE> - <DESC>Native v3.1 render entrypoints and types</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
