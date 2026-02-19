// <FILE>tui-vfx-content/src/types/cls_mirror_axis.rs</FILE> - <DESC>MirrorAxis enum for text mirroring direction</DESC>
// <VERS>VERSION: 1.0.1 - 2025-12-18T18:50:00Z</VERS>
// <WCTX>Adding Mirror content effect for rotation illusion</WCTX>
// <CLOG>Added ConfigSchema derive</CLOG>

/// Axis for text mirroring transformation.
///
/// - `Horizontal`: Reverses character order ("HELLO" → "OLLEH")
/// - `Vertical`: Reverses line order (top line becomes bottom)
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum MirrorAxis {
    /// Reverse character order within each line.
    #[default]
    Horizontal,
    /// Reverse line order (first line becomes last).
    Vertical,
}

// <FILE>tui-vfx-content/src/types/cls_mirror_axis.rs</FILE> - <DESC>MirrorAxis enum for text mirroring direction</DESC>
// <VERS>END OF VERSION: 1.0.1 - 2025-12-18T18:50:00Z</VERS>
