// <FILE>crates/tui-vfx-compost/src/render/cls_frame.rs</FILE> - <DESC>Native rendered frame type</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Frame carries rendered output plus native diagnostics and trace evidence.</WCTX>
// <CLOG>0.2.0: MINOR — add frame diagnostics and trace events for render observability.
// 0.1.0: INIT — add rendered frame type.</CLOG>

use tui_vfx_types::SemanticScene;

use crate::render::{RenderDiagnostic, RenderTraceEvent};

/// Rendered frame from a load-validated v3.1 recipe.
#[derive(Clone, Debug)]
pub struct Frame {
    /// Recipe id used for diagnostics and test assertions.
    pub recipe_id: String,
    /// Frame width in cells.
    pub width: usize,
    /// Frame height in cells.
    pub height: usize,
    /// Rendered cell/role surface.
    pub grid: SemanticScene,
    /// Effects actually applied during rendering.
    pub applied_effect_kinds: Vec<String>,
    /// Non-fatal diagnostics explaining skipped or deferred render work.
    pub diagnostics: Vec<RenderDiagnostic>,
    /// Structured trace events for applied render stages.
    pub trace_events: Vec<RenderTraceEvent>,
}

// <FILE>crates/tui-vfx-compost/src/render/cls_frame.rs</FILE> - <DESC>Native rendered frame type</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
