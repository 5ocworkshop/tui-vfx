// <FILE>tui-vfx-style/src/traits/mod.rs</FILE> - <DESC>Traits module definition</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Turn 6 Implementation</WCTX>
// <CLOG>Removed local ConfigSchema; re-export from tui_motion</CLOG>

pub mod cls_shader_context;
pub mod tr_style_interpolator;
pub mod tr_style_shader;
pub use cls_shader_context::ShaderContext;
pub use tr_style_interpolator::StyleInterpolator;
pub use tr_style_shader::StyleShader;
pub use tui_vfx_core::ConfigSchema;

// <FILE>tui-vfx-style/src/traits/mod.rs</FILE> - <DESC>Traits module definition</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
