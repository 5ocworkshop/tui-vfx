// <FILE>crates/tui-vfx-shadow/src/types/mod.rs</FILE> - <DESC>Type definitions for shadow rendering</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New crate for theme-aware shadow rendering with multiple styles</WCTX>
// <CLOG>Initial creation - re-export ShadowStyle, ShadowConfig, ShadowEdges</CLOG>

//! Type definitions for shadow rendering.

mod shadow_config;
mod shadow_edge;
mod shadow_style;

pub use shadow_config::ShadowConfig;
pub use shadow_edge::ShadowEdges;
pub use shadow_style::ShadowStyle;

// <FILE>crates/tui-vfx-shadow/src/types/mod.rs</FILE> - <DESC>Type definitions for shadow rendering</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
