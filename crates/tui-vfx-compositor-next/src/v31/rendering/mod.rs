// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/mod.rs</FILE> - <DESC>Direct v3.1 rendering modules</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Mirror compositor source layout with small render entrypoint, source, topology, and shader modules.</WCTX>
// <CLOG>0.1.0: INIT — add OFPF-shaped direct rendering module tree.</CLOG>

mod cls_v31_frame;
mod cls_v31_render_error;
mod cls_v31_sample_context;
mod col_collect_graph_step_nodes;
mod fnc_render_v31_recipe;
mod orc_composition_spec_for_element;
mod shaders;
mod source;

pub use cls_v31_frame::V31Frame;
pub use cls_v31_render_error::V31RenderError;
pub use cls_v31_sample_context::V31SampleContext;
pub(crate) use col_collect_graph_step_nodes::collect_graph_step_nodes;
pub use fnc_render_v31_recipe::render_v31_recipe;
pub(crate) use orc_composition_spec_for_element::composition_spec_for_element;

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/mod.rs</FILE> - <DESC>Direct v3.1 rendering modules</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
