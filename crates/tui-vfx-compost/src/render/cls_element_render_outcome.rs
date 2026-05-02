// <FILE>crates/tui-vfx-compost/src/render/cls_element_render_outcome.rs</FILE> - <DESC>Element render outcome with observability evidence</DESC>
// <VERS>VERSION: 0.3.1</VERS>
// <WCTX>Element render outcomes keep applied effects, diagnostics, and trace events together until scene aggregation.</WCTX>
// <CLOG>0.3.1: PATCH — remove pre-observability constructors after scope-aware trace aggregation.
// 0.3.0: MINOR — carry native trace events with element render outcomes.
// 0.2.0: MINOR — allow applied element outcomes to carry diagnostics.
// 0.1.0: INIT — add element outcome for render observability aggregation.</CLOG>

use crate::render::{RenderDiagnostic, RenderTraceEvent};

#[derive(Clone, Debug, Default)]
pub(crate) struct ElementRenderOutcome {
    pub(crate) applied_effect_kinds: Vec<String>,
    pub(crate) diagnostics: Vec<RenderDiagnostic>,
    pub(crate) trace_events: Vec<RenderTraceEvent>,
}

impl ElementRenderOutcome {
    pub(crate) fn applied_with_diagnostics_and_trace(
        applied_effect_kinds: Vec<String>,
        diagnostics: Vec<RenderDiagnostic>,
        trace_events: Vec<RenderTraceEvent>,
    ) -> Self {
        Self {
            applied_effect_kinds,
            diagnostics,
            trace_events,
        }
    }

    pub(crate) fn skipped_with_trace(
        message: impl Into<String>,
        trace_event: RenderTraceEvent,
    ) -> Self {
        Self {
            applied_effect_kinds: Vec::new(),
            diagnostics: vec![RenderDiagnostic::new(message)],
            trace_events: vec![trace_event],
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_element_render_outcome.rs</FILE> - <DESC>Element render outcome with observability evidence</DESC>
// <VERS>END OF VERSION: 0.3.1</VERS>
