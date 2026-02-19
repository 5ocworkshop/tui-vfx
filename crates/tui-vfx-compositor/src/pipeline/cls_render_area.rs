// <FILE>tui-vfx-compositor/src/pipeline/cls_render_area.rs</FILE> - <DESC>Render area for pipeline target</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>OAI review - API ergonomics improvement</WCTX>
// <CLOG>Add RenderArea to reduce render_pipeline argument count</CLOG>

/// Defines a rectangular area for rendering operations.
///
/// Bundles width, height, and offset coordinates to reduce argument
/// count in render pipeline calls.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RenderArea {
    pub width: usize,
    pub height: usize,
    pub offset_x: usize,
    pub offset_y: usize,
}

impl RenderArea {
    /// Create a new render area with the given dimensions and offset.
    pub const fn new(width: usize, height: usize, offset_x: usize, offset_y: usize) -> Self {
        Self {
            width,
            height,
            offset_x,
            offset_y,
        }
    }

    /// Create a render area at the origin (0, 0) with the given dimensions.
    pub const fn from_size(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            offset_x: 0,
            offset_y: 0,
        }
    }
}

// <FILE>tui-vfx-compositor/src/pipeline/cls_render_area.rs</FILE> - <DESC>Render area for pipeline target</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
