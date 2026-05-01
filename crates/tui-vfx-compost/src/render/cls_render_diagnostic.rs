// <FILE>crates/tui-vfx-compost/src/render/cls_render_diagnostic.rs</FILE> - <DESC>Native render diagnostic emitted with frames</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Render diagnostics record skipped-work explanations without external inspector DTOs.</WCTX>
// <CLOG>0.1.0: INIT — add frame diagnostic type.</CLOG>

/// Human-readable diagnostic attached to a rendered frame.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderDiagnostic {
    /// Diagnostic message.
    pub message: String,
}

impl RenderDiagnostic {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_render_diagnostic.rs</FILE> - <DESC>Native render diagnostic emitted with frames</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
