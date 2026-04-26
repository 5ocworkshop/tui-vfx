// <FILE>tui-vfx-content/src/traits/mod.rs</FILE> - <DESC>Traits module</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: register and re-export TransformContext alongside TextTransformer.</WCTX>
// <CLOG>1.2.0: register cls_transform_context module and re-export TransformContext.</CLOG>

pub mod cls_transform_context;
pub mod text_transformer;
pub use cls_transform_context::TransformContext;
pub use text_transformer::TextTransformer;
pub use tui_vfx_core::ConfigSchema;

// <FILE>tui-vfx-content/src/traits/mod.rs</FILE> - <DESC>Traits module</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
