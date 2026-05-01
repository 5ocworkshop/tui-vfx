// <FILE>crates/tui-vfx-compost/src/render/cls_element_render_outcome.rs</FILE> - <DESC>Element render outcome with observability evidence</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Element render outcomes keep applied effects and diagnostics together until scene aggregation.</WCTX>
// <CLOG>0.1.0: INIT — add element outcome for render observability aggregation.</CLOG>

use crate::render::RenderDiagnostic;

#[derive(Clone, Debug, Default)]
pub(crate) struct ElementRenderOutcome {
    pub(crate) applied_effect_kinds: Vec<String>,
    pub(crate) diagnostics: Vec<RenderDiagnostic>,
}

impl ElementRenderOutcome {
    pub(crate) fn applied(applied_effect_kinds: Vec<String>) -> Self {
        Self {
            applied_effect_kinds,
            diagnostics: Vec::new(),
        }
    }

    pub(crate) fn skipped(message: impl Into<String>) -> Self {
        Self {
            applied_effect_kinds: Vec::new(),
            diagnostics: vec![RenderDiagnostic::new(message)],
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_element_render_outcome.rs</FILE> - <DESC>Element render outcome with observability evidence</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
