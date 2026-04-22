// <FILE>tui-vfx-style/src/models/v3/enum_vfx_progress_emphasis_behavior.rs</FILE> - <DESC>V3 progress/emphasis family behavior surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — start moving the progress/emphasis family into a parallel V3 surface while the legacy Highlighter shader remains available for current playback and cutover paths.</WCTX>
// <CLOG>Define the V3 progress/emphasis enums by lifting the legacy Highlighter policy surface into Vfx-prefixed family-local types.</CLOG>

//! V3 behavior surface for progress/emphasis shaders.
//!
//! The first concrete member of this family comes from the legacy
//! `HighlighterShader`, but the grouped V3 surface is intentionally named for
//! the broader progress/emphasis role rather than the legacy effect label.

use crate::models::ColorConfig;
use serde::{Deserialize, Serialize};

/// Which channel(s) the emphasis ink affects.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxProgressEmphasisApplyTo {
    /// Paint the emphasis color on the background.
    #[default]
    Background,
    /// Tint the foreground only.
    Foreground,
    /// Affect both foreground and background.
    Both,
}

/// Foreground handling when the emphasis primarily paints the background.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum VfxProgressEmphasisTextContrast {
    /// Force a black foreground for strong contrast.
    #[default]
    Black,
    /// Preserve the incoming foreground.
    Preserve,
    /// Use an explicit foreground color.
    Explicit {
        /// Explicit contrast color.
        color: ColorConfig,
    },
}

/// Coverage shape of the emphasis motion.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxProgressEmphasisMode {
    /// Everything behind the head stays emphasized.
    #[default]
    Fill,
    /// Only a moving band is emphasized.
    Band,
}

/// Direction the emphasis motion travels.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxProgressEmphasisDirection {
    /// Left-to-right.
    #[default]
    Forward,
    /// Right-to-left.
    Reverse,
    /// Top-to-bottom.
    TopDown,
    /// Bottom-to-top.
    BottomUp,
    /// Center to edges.
    CenterOut,
    /// Edges to center.
    EdgesIn,
}

/// Row-selection policy for progress/emphasis motion.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum VfxProgressEmphasisRowMask {
    /// Affect every row.
    #[default]
    AllRows,
    /// Affect only the first row.
    FirstRow,
    /// Affect only the last row.
    LastRow,
    /// Affect the top and bottom rows only.
    TopAndBottom,
    /// Affect an inclusive row range.
    Range {
        /// First row in the range.
        start: u16,
        /// Last row in the range.
        end: u16,
    },
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_progress_emphasis_behavior.rs</FILE> - <DESC>V3 progress/emphasis family behavior surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
