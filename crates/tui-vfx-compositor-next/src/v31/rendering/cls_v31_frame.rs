// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/cls_v31_frame.rs</FILE> - <DESC>Direct v3.1 rendered frame type</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Keep returned frame data separate from render orchestration.</WCTX>
// <CLOG>0.1.0: INIT — extract V31Frame.</CLOG>

use tui_vfx_types::SemanticScene;

/// Rendered frame from the direct v3.1 compositor-next path.
#[derive(Clone, Debug)]
pub struct V31Frame {
    /// Canonical recipe id rendered into this frame.
    pub recipe_id: String,
    /// Frame width in cells.
    pub width: usize,
    /// Frame height in cells.
    pub height: usize,
    /// Semantic scene produced by compositor-next.
    pub grid: SemanticScene,
    /// Non-fatal direct-render diagnostics.
    pub diagnostics: Vec<String>,
    /// Effect descriptor ids applied by this direct v3.1 render.
    pub applied_effect_kinds: Vec<String>,
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/rendering/cls_v31_frame.rs</FILE> - <DESC>Direct v3.1 rendered frame type</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
