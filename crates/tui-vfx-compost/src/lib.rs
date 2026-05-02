// <FILE>crates/tui-vfx-compost/src/lib.rs</FILE> - <DESC>v3.1-native compositor skeleton crate entry point</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Expose the clean v3.1-native loader/render API plus Rust-owned primitive registry substrate and domain-directory primitive ports; no legacy compositor DTOs or versioned source directory.</WCTX>
// <CLOG>0.6.0: MINOR — expose native sampler primitive ports from the existing samplers/ hierarchy.
// 0.5.0: MINOR — expose native mask primitive ports from the existing masks/ hierarchy.
// 0.4.0: MINOR — expose native filter primitive ports from the existing filters/ hierarchy.
// 0.3.0: MINOR — expose the Rust-owned primitive registry and domain runtime trait substrate.
// 0.2.0: MINOR — expose native loader, render, source, validation, and shader skeleton modules.
// 0.1.0: INIT — add empty crate entrypoint for RED skeleton tests.</CLOG>

//! Clean v3.1-native compositor staging crate.
//!
//! This crate intentionally starts small: canonical v3.1 recipes enter through
//! the loader, render through native responsibility modules, and do not lower
//! into legacy compositor DTOs.

pub mod filters;
mod loader;
pub mod masks;
pub mod primitive;
mod render;
mod runtime;
pub mod samplers;
mod shaders;
mod source;
mod validation;

pub use loader::{LoadError, LoadedRecipe};
pub use render::{Frame, RenderError, SampleContext, render_recipe, render_recipe_scene};

// <FILE>crates/tui-vfx-compost/src/lib.rs</FILE> - <DESC>v3.1-native compositor skeleton crate entry point</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
