// <FILE>crates/tui-vfx-compost/src/render/mod.rs</FILE> - <DESC>Native v3.1 render entrypoints and types</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Render orchestration consumes LoadedRecipe directly without versioned folders or legacy lowerers.</WCTX>
// <CLOG>0.1.0: INIT — add frame, context, error, and render entrypoint modules.</CLOG>

mod cls_frame;
mod cls_render_error;
mod cls_sample_context;
mod col_collect_graph_step_nodes;
mod fnc_render_recipe;

pub use cls_frame::Frame;
pub use cls_render_error::RenderError;
pub use cls_sample_context::SampleContext;
pub use fnc_render_recipe::render_recipe;

pub(crate) use col_collect_graph_step_nodes::collect_graph_step_nodes;

// <FILE>crates/tui-vfx-compost/src/render/mod.rs</FILE> - <DESC>Native v3.1 render entrypoints and types</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
