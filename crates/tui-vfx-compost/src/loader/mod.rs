// <FILE>crates/tui-vfx-compost/src/loader/mod.rs</FILE> - <DESC>Native v3.1 recipe load boundary</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Loader is the single acceptance point for canonical v3.1 recipes before rendering.</WCTX>
// <CLOG>0.1.0: INIT — add loaded recipe and load error modules.</CLOG>

mod cls_load_error;
mod cls_loaded_recipe;

pub use cls_load_error::LoadError;
pub use cls_loaded_recipe::LoadedRecipe;

// <FILE>crates/tui-vfx-compost/src/loader/mod.rs</FILE> - <DESC>Native v3.1 recipe load boundary</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
