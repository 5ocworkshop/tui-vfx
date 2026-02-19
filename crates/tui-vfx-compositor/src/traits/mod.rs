// <FILE>tui-vfx-compositor/src/traits/mod.rs</FILE>
// <DESC>Traits module</DESC>
// <VERS>VERSION: 1.5.0</VERS>
// <WCTX>Centralize on pipeline API as the only public API method</WCTX>
// <CLOG>Made filter/mask/sampler traits pub(crate), kept pipeline_inspector public</CLOG>

pub(crate) mod filter;
pub(crate) mod mask;
pub mod pipeline_inspector;
pub(crate) mod sampler;
pub use tui_vfx_core::ConfigSchema;

// <FILE>tui-vfx-compositor/src/traits/mod.rs</FILE>
// <DESC>Traits module</DESC>
// <VERS>END OF VERSION: 1.5.0</VERS>
