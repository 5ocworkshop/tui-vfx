// <FILE>crates/tui-vfx-compost/src/lib.rs</FILE> - <DESC>v3.1-native compositor skeleton crate entry point</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Expose only the clean v3.1-native loader/render API; no legacy compositor DTOs or versioned source directory.</WCTX>
// <CLOG>0.2.0: MINOR — expose native loader, render, source, validation, and shader skeleton modules.
// 0.1.0: INIT — add empty crate entrypoint for RED skeleton tests.</CLOG>

//! Clean v3.1-native compositor staging crate.
//!
//! This crate intentionally starts small: canonical v3.1 recipes enter through
//! the loader, render through native responsibility modules, and do not lower
//! into legacy compositor DTOs.

mod loader;
mod render;
mod runtime;
mod shaders;
mod source;
mod validation;

pub use loader::{LoadError, LoadedRecipe};
pub use render::{Frame, RenderError, SampleContext, render_recipe, render_recipe_scene};

// <FILE>crates/tui-vfx-compost/src/lib.rs</FILE> - <DESC>v3.1-native compositor skeleton crate entry point</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
