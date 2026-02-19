// <FILE>tui-vfx-geometry/src/types/anchor.rs</FILE>
// <DESC>Anchor positions for screen-relative placement</DESC>
// <VERS>VERSION: 2.0.0 - 2025-12-31</VERS>
// <WCTX>V2.2 schema standardization - snake_case serialization</WCTX>
// <CLOG>BREAKING: Added snake_case serde serialization for all variants</CLOG>

use serde::{Deserialize, Serialize};

/// Screen-relative anchor for placing a rectangle within a frame.
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
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    /// Center position - dead center of the screen
    Center,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    /// Default anchor position. Notifications expand from bottom-right.
    #[default]
    BottomRight,
    /// Absolute screen coordinates (x, y)
    Absolute(u16, u16),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_deserializes_correctly() {
        // Test that "center" (snake_case) in JSON deserializes to Center
        let json = r#""center""#;
        let anchor: Anchor = serde_json::from_str(json).unwrap();
        assert_eq!(anchor, Anchor::Center);
    }

    #[test]
    fn test_center_serializes_correctly() {
        // Test that Center serializes to "center" (snake_case) in JSON
        let anchor = Anchor::Center;
        let json = serde_json::to_string(&anchor).unwrap();
        assert_eq!(json, r#""center""#);
    }
}

// <FILE>tui-vfx-geometry/src/types/anchor.rs</FILE>
// <DESC>Anchor positions for screen-relative placement</DESC>
// <VERS>END OF VERSION: 2.0.0 - 2025-12-31</VERS>
