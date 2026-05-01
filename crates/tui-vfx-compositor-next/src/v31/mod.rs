// <FILE>crates/tui-vfx-compositor-next/src/v31/mod.rs</FILE> - <DESC>Direct canonical v3.1 recipe rendering entrypoints</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>v3.1 compositor-next direct lane — small OFPF modules validate RecipeDocument once, then render supported primitives without transition-seam code growth.</WCTX>
// <CLOG>0.3.0: MINOR — replace load/render hubs with OFPF-shaped v3.1 validation and rendering modules.</CLOG>

mod cls_loaded_v31_recipe;
mod cls_v31_load_error;
mod rendering;
mod validation;

pub use cls_loaded_v31_recipe::LoadedV31Recipe;
pub use cls_v31_load_error::V31LoadError;
pub use rendering::{V31Frame, V31RenderError, V31SampleContext, render_v31_recipe};

// <FILE>crates/tui-vfx-compositor-next/src/v31/mod.rs</FILE> - <DESC>Direct canonical v3.1 recipe rendering entrypoints</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
