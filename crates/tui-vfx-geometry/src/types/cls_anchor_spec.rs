// <FILE>tui-vfx-geometry/src/types/cls_anchor_spec.rs</FILE> - <DESC>Enhanced anchor with percentage offsets</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-31</VERS>
// <WCTX>V2.2 schema standardization - anchor enhancement</WCTX>
// <CLOG>New AnchorSpec wrapper supporting percentage offsets from anchor points</CLOG>

use super::Anchor;
use serde::{Deserialize, Serialize};

/// Enhanced anchor specification with optional percentage offsets.
///
/// Allows positioning elements at an anchor point with fine-grained percentage
/// adjustments along the horizontal and vertical axes from that anchor.
///
/// # Examples
///
/// Simple anchor (backward compatible):
/// ```json
/// "anchor": "bottom_left"
/// ```
///
/// Anchor with offsets:
/// ```json
/// {
///   "position": "bottom_left",
///   "offset_horizontal_percent": 10.0,
///   "offset_vertical_percent": 20.0
/// }
/// ```
///
/// This places the element 10% from the left edge and 20% from the bottom edge
/// of the frame, starting from the bottom_left anchor point.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(untagged)]
pub enum AnchorSpec {
    /// Simple anchor position (e.g., "bottom_right", "center")
    /// This is the backward-compatible format.
    Simple(Anchor),

    /// Anchor with percentage offsets from the anchor point.
    ///
    /// Offsets are expressed as percentages of the frame dimension:
    /// - `offset_horizontal_percent`: 0.0 = no offset, 100.0 = full frame width
    /// - `offset_vertical_percent`: 0.0 = no offset, 100.0 = full frame height
    ///
    /// Direction depends on anchor:
    /// - Horizontal: left anchors → positive is right, right anchors → positive is left
    /// - Vertical: top anchors → positive is down, bottom anchors → positive is up
    WithOffset {
        position: Anchor,
        #[serde(default)]
        #[config(
            help = "Horizontal offset as percentage of frame width",
            default = 0.0,
            min = 0.0,
            max = 100.0
        )]
        offset_horizontal_percent: f32,
        #[serde(default)]
        #[config(
            help = "Vertical offset as percentage of frame height",
            default = 0.0,
            min = 0.0,
            max = 100.0
        )]
        offset_vertical_percent: f32,
        #[serde(default)]
        #[config(help = "Horizontal offset in cells", default = 0)]
        offset_horizontal_cells: i16,
        #[serde(default)]
        #[config(help = "Vertical offset in cells", default = 0)]
        offset_vertical_cells: i16,
        #[serde(default)]
        #[config(help = "Horizontal offset in pixels", default = 0)]
        offset_horizontal_pixels: i32,
        #[serde(default)]
        #[config(help = "Vertical offset in pixels", default = 0)]
        offset_vertical_pixels: i32,
    },
}

impl AnchorSpec {
    /// Get the base anchor position
    pub fn position(&self) -> Anchor {
        match self {
            AnchorSpec::Simple(anchor) => *anchor,
            AnchorSpec::WithOffset { position, .. } => *position,
        }
    }

    /// Get horizontal offset percentage (0.0 if Simple)
    pub fn offset_horizontal_percent(&self) -> f32 {
        match self {
            AnchorSpec::Simple(_) => 0.0,
            AnchorSpec::WithOffset {
                offset_horizontal_percent,
                ..
            } => *offset_horizontal_percent,
        }
    }

    /// Get vertical offset percentage (0.0 if Simple)
    pub fn offset_vertical_percent(&self) -> f32 {
        match self {
            AnchorSpec::Simple(_) => 0.0,
            AnchorSpec::WithOffset {
                offset_vertical_percent,
                ..
            } => *offset_vertical_percent,
        }
    }

    /// Get horizontal cell offset (0 if Simple)
    pub fn offset_horizontal_cells(&self) -> i16 {
        match self {
            AnchorSpec::Simple(_) => 0,
            AnchorSpec::WithOffset {
                offset_horizontal_cells,
                ..
            } => *offset_horizontal_cells,
        }
    }

    /// Get vertical cell offset (0 if Simple)
    pub fn offset_vertical_cells(&self) -> i16 {
        match self {
            AnchorSpec::Simple(_) => 0,
            AnchorSpec::WithOffset {
                offset_vertical_cells,
                ..
            } => *offset_vertical_cells,
        }
    }

    /// Get horizontal pixel offset (0 if Simple)
    pub fn offset_horizontal_pixels(&self) -> i32 {
        match self {
            AnchorSpec::Simple(_) => 0,
            AnchorSpec::WithOffset {
                offset_horizontal_pixels,
                ..
            } => *offset_horizontal_pixels,
        }
    }

    /// Get vertical pixel offset (0 if Simple)
    pub fn offset_vertical_pixels(&self) -> i32 {
        match self {
            AnchorSpec::Simple(_) => 0,
            AnchorSpec::WithOffset {
                offset_vertical_pixels,
                ..
            } => *offset_vertical_pixels,
        }
    }

    /// Check if this anchor has any offsets
    pub fn has_offsets(&self) -> bool {
        matches!(self, AnchorSpec::WithOffset { .. })
    }
}

impl Default for AnchorSpec {
    fn default() -> Self {
        AnchorSpec::Simple(Anchor::default())
    }
}

impl From<Anchor> for AnchorSpec {
    fn from(anchor: Anchor) -> Self {
        AnchorSpec::Simple(anchor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_anchor_deserializes() {
        let json = r#""bottom_right""#;
        let spec: AnchorSpec = serde_json::from_str(json).unwrap();
        assert_eq!(spec.position(), Anchor::BottomRight);
        assert_eq!(spec.offset_horizontal_percent(), 0.0);
        assert_eq!(spec.offset_vertical_percent(), 0.0);
        assert!(!spec.has_offsets());
    }

    #[test]
    fn test_with_offset_deserializes() {
        let json = r#"{
            "position": "bottom_left",
            "offset_horizontal_percent": 10.0,
            "offset_vertical_percent": 20.0
        }"#;
        let spec: AnchorSpec = serde_json::from_str(json).unwrap();
        assert_eq!(spec.position(), Anchor::BottomLeft);
        assert_eq!(spec.offset_horizontal_percent(), 10.0);
        assert_eq!(spec.offset_vertical_percent(), 20.0);
        assert!(spec.has_offsets());
    }

    #[test]
    fn test_with_offset_default_values() {
        let json = r#"{ "position": "center" }"#;
        let spec: AnchorSpec = serde_json::from_str(json).unwrap();
        assert_eq!(spec.position(), Anchor::Center);
        assert_eq!(spec.offset_horizontal_percent(), 0.0);
        assert_eq!(spec.offset_vertical_percent(), 0.0);
    }

    #[test]
    fn test_simple_anchor_serializes() {
        let spec = AnchorSpec::Simple(Anchor::TopCenter);
        let json = serde_json::to_string(&spec).unwrap();
        assert_eq!(json, r#""top_center""#);
    }

    #[test]
    fn test_with_offset_serializes() {
        let spec = AnchorSpec::WithOffset {
            position: Anchor::BottomLeft,
            offset_horizontal_percent: 15.5,
            offset_vertical_percent: 25.5,
            offset_horizontal_cells: 0,
            offset_vertical_cells: 0,
            offset_horizontal_pixels: 0,
            offset_vertical_pixels: 0,
        };
        let json = serde_json::to_value(&spec).unwrap();
        assert_eq!(json["position"], "bottom_left");
        assert_eq!(json["offset_horizontal_percent"], 15.5);
        assert_eq!(json["offset_vertical_percent"], 25.5);
    }
}

// <FILE>tui-vfx-geometry/src/types/cls_anchor_spec.rs</FILE> - <DESC>Enhanced anchor with percentage offsets</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-31</VERS>
