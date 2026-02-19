// <FILE>tui-vfx-geometry/src/types/slide_direction.rs</FILE>
// <DESC>Slide directions (including Default resolution)</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>Schema V2.2 standardization</WCTX>
// <CLOG>Added snake_case serialization for consistency</CLOG>

use serde::{Deserialize, Serialize};

/// Direction from which a rectangle slides in/out.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SlideDirection {
    /// Auto-select direction based on anchor point (default).
    #[default]
    Default,
    FromTop,
    FromBottom,
    FromLeft,
    FromRight,
    FromTopLeft,
    FromTopRight,
    FromBottomLeft,
    FromBottomRight,
}

// <FILE>tui-vfx-geometry/src/types/slide_direction.rs</FILE>
// <DESC>Slide directions (including Default resolution)</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
