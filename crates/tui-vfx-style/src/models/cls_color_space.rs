// <FILE>tui-vfx-style/src/models/cls_color_space.rs</FILE> - <DESC>Enum for color interpolation modes</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-24</VERS>
// <WCTX>Cleanup - remove PascalCase aliases</WCTX>
// <CLOG>BREAKING: Removed Rgb/Hsl PascalCase aliases, standardized to lowercase rgb/hsl</CLOG>

use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ColorSpace {
    #[default]
    Rgb,
    Hsl,
}

// <FILE>tui-vfx-style/src/models/cls_color_space.rs</FILE> - <DESC>Enum for color interpolation modes</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-24</VERS>
