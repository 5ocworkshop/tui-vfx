// <FILE>crates/tui-vfx-compost/src/render/cls_render_trace_event.rs</FILE> - <DESC>Native render trace event emitted with frames</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Render trace events record scene/element/stage/effect identity for debugging.</WCTX>
// <CLOG>0.1.0: INIT — add frame trace event type.</CLOG>

/// Structured trace evidence for one applied effect stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderTraceEvent {
    /// Scene id that owned the rendered element.
    pub scene_id: String,
    /// Element id that owned the rendered effect stage.
    pub element_id: String,
    /// Authored stage index within the element stack.
    pub stage_index: usize,
    /// Effect id applied at the stage.
    pub effect: String,
}

impl RenderTraceEvent {
    pub(crate) fn effect_stage(
        scene_id: impl Into<String>,
        element_id: impl Into<String>,
        stage_index: usize,
        effect: impl Into<String>,
    ) -> Self {
        Self {
            scene_id: scene_id.into(),
            element_id: element_id.into(),
            stage_index,
            effect: effect.into(),
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_render_trace_event.rs</FILE> - <DESC>Native render trace event emitted with frames</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
