// <FILE>crates/tui-vfx-shadow/src/types/mod.rs</FILE> - <DESC>Type definitions for shadow rendering</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase 0 dramatic color-shadow rollout: re-export compositing mode and grade config</WCTX>
// <CLOG>Add ShadowCompositeMode and ShadowGradeConfig re-exports</CLOG>

//! Type definitions for shadow rendering.

mod shadow_composite_mode;
mod shadow_config;
mod shadow_edge;
mod shadow_grade_config;
mod shadow_style;

pub use shadow_composite_mode::ShadowCompositeMode;
pub use shadow_config::ShadowConfig;
pub use shadow_edge::ShadowEdges;
pub use shadow_grade_config::ShadowGradeConfig;
pub use shadow_style::ShadowStyle;

// <FILE>crates/tui-vfx-shadow/src/types/mod.rs</FILE> - <DESC>Type definitions for shadow rendering</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
