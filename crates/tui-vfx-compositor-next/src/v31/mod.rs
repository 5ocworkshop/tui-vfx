// <FILE>crates/tui-vfx-compositor-next/src/v31/mod.rs</FILE> - <DESC>Direct canonical v3.1 recipe rendering entrypoints</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>v3.1 compositor-next direct lane — validate RecipeDocument once, then render supported primitives without transition-seam code growth.</WCTX>
// <CLOG>0.2.0: MINOR — split load validation and render implementation into OFPF-sized modules.
// 0.1.0: INIT — add direct shader.linearGradient RecipeDocument rendering.</CLOG>

mod load;
mod render;

pub use load::{LoadedV31Recipe, V31LoadError};
pub use render::{V31Frame, V31RenderError, V31SampleContext, render_v31_recipe};

// <FILE>crates/tui-vfx-compositor-next/src/v31/mod.rs</FILE> - <DESC>Direct canonical v3.1 recipe rendering entrypoints</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
