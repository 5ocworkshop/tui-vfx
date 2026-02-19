// <FILE>tui-vfx-geometry/src/types/cls_origin.rs</FILE> - <DESC>Origin enum for positioning</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>Schema V2.2 standardization</WCTX>
// <CLOG>Changed from PascalCase to snake_case serialization for consistency</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_types::Rect;

/// Origin point for scale and anchor operations.
///
/// Determines which point of a rectangle stays fixed during scaling
/// or serves as the anchor point for positioning.
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
pub enum Origin {
    /// Center of the rectangle (default)
    #[default]
    Center,
    /// Top-left corner
    TopLeft,
    /// Top-center edge
    TopCenter,
    /// Top-right corner
    TopRight,
    /// Middle-left edge
    MiddleLeft,
    /// Middle-center (same as Center)
    MiddleCenter,
    /// Middle-right edge
    MiddleRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-center edge
    BottomCenter,
    /// Bottom-right corner
    BottomRight,
}

impl Origin {
    /// Returns the anchor point (x, y) for this origin within the given rectangle.
    pub fn anchor_point(&self, rect: Rect) -> (u16, u16) {
        let center_x = rect.x + rect.width / 2;
        let center_y = rect.y + rect.height / 2;
        let right = rect.x + rect.width;
        let bottom = rect.y + rect.height;

        match self {
            Origin::Center | Origin::MiddleCenter => (center_x, center_y),
            Origin::TopLeft => (rect.x, rect.y),
            Origin::TopCenter => (center_x, rect.y),
            Origin::TopRight => (right, rect.y),
            Origin::MiddleLeft => (rect.x, center_y),
            Origin::MiddleRight => (right, center_y),
            Origin::BottomLeft => (rect.x, bottom),
            Origin::BottomCenter => (center_x, bottom),
            Origin::BottomRight => (right, bottom),
        }
    }

    /// Returns the offset factors (fx, fy) for positioning a rect relative to its anchor.
    ///
    /// Returns values in [0.0, 1.0] where:
    /// - (0.0, 0.0) means the anchor is at the top-left
    /// - (0.5, 0.5) means the anchor is at the center
    /// - (1.0, 1.0) means the anchor is at the bottom-right
    pub fn offset_factors(&self) -> (f32, f32) {
        match self {
            Origin::TopLeft => (0.0, 0.0),
            Origin::TopCenter => (0.5, 0.0),
            Origin::TopRight => (1.0, 0.0),
            Origin::MiddleLeft => (0.0, 0.5),
            Origin::Center | Origin::MiddleCenter => (0.5, 0.5),
            Origin::MiddleRight => (1.0, 0.5),
            Origin::BottomLeft => (0.0, 1.0),
            Origin::BottomCenter => (0.5, 1.0),
            Origin::BottomRight => (1.0, 1.0),
        }
    }
}

// <FILE>tui-vfx-geometry/src/types/cls_origin.rs</FILE> - <DESC>Origin enum for positioning</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T22:00:00Z</VERS>
