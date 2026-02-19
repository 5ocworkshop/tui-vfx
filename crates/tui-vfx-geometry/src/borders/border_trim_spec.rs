// <FILE>tui-vfx-geometry/src/borders/border_trim_spec.rs</FILE>
// <DESC>Border trimming spec (what vanishes vs what stays)</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Legacy effects parity: vanishing-edge border behavior</WCTX>
// <CLOG>Extracted spec types from borders module</CLOG>

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderSegment {
    Keep,
    Blank,
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderTrimSpec {
    // Edges
    pub top: BorderSegment,
    pub right: BorderSegment,
    pub bottom: BorderSegment,
    pub left: BorderSegment,
    // Corners
    pub top_left: BorderSegment,
    pub top_right: BorderSegment,
    pub bottom_left: BorderSegment,
    pub bottom_right: BorderSegment,
}

impl BorderTrimSpec {
    pub const fn none() -> Self {
        Self {
            top: BorderSegment::Keep,
            right: BorderSegment::Keep,
            bottom: BorderSegment::Keep,
            left: BorderSegment::Keep,
            top_left: BorderSegment::Keep,
            top_right: BorderSegment::Keep,
            bottom_left: BorderSegment::Keep,
            bottom_right: BorderSegment::Keep,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClippedEdges {
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

// <FILE>tui-vfx-geometry/src/borders/border_trim_spec.rs</FILE>
// <DESC>Border trimming spec (what vanishes vs what stays)</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-17T00:00:00Z</VERS>
