// <FILE>crates/tui-vfx-compost/src/render/cls_render_trace_event.rs</FILE> - <DESC>Native render trace event emitted with frames</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Render trace events record scene/element/stage identity, lifecycle status, skip reason, and cell-count evidence through closed native vocabulary.</WCTX>
// <CLOG>0.3.0: MINOR — centralize trace stage, status, and skip vocabulary in native enums.
// 0.2.0: MINOR — add canonical stage kind, status, skip reason, and scope cell counts.
// 0.1.0: INIT — add frame trace event type.</CLOG>

/// Native closed vocabulary for render trace stage kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RenderStageKind {
    /// Scene element lifecycle or placement stage.
    Element,
    /// Content effect family stage.
    Content,
    /// Style effect family stage.
    Style,
    /// Shader effect family stage.
    Shader,
    /// Filter effect family stage.
    Filter,
    /// Mask effect family stage.
    Mask,
    /// Sampler effect family stage.
    Sampler,
    /// Unknown effect family stage.
    Unknown,
    /// Surface shadow render stage.
    Shadow,
    /// Native parallel graph merge stage.
    Parallel,
}

impl RenderStageKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Element => "element",
            Self::Content => "content",
            Self::Style => "style",
            Self::Shader => "shader",
            Self::Filter => "filter",
            Self::Mask => "mask",
            Self::Sampler => "sampler",
            Self::Unknown => "unknown",
            Self::Shadow => "shadow",
            Self::Parallel => "parallel",
        }
    }
}

/// Native closed vocabulary for render trace stage status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RenderStageStatus {
    /// Stage made at least one cell-level contribution.
    Finished,
    /// Stage did not execute for the recorded reason.
    Skipped,
}

impl RenderStageStatus {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Finished => "finished",
            Self::Skipped => "skipped",
        }
    }
}

/// Native closed vocabulary for skipped render stage reasons.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum RenderSkipReason {
    /// Element visibility prevented rendering.
    Visibility,
    /// Element was fully clipped by scene bounds.
    FullyClipped,
    /// Hide overflow policy skipped an element that would leave scene bounds.
    OverflowHide,
    /// Node lifecycle phase did not match the current sample.
    InactiveLifecycle,
    /// Node scope matched no cells in the rendered area.
    ScopeMatchedZeroCells,
}

impl RenderSkipReason {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Visibility => "visibility",
            Self::FullyClipped => "fullyClipped",
            Self::OverflowHide => "overflowHide",
            Self::InactiveLifecycle => "inactiveLifecycle",
            Self::ScopeMatchedZeroCells => "scopeMatchedZeroCells",
        }
    }
}

/// Structured trace evidence for one render stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderTraceEvent {
    /// Scene id that owned the rendered element.
    pub scene_id: String,
    /// Element id that owned the rendered stage.
    pub element_id: String,
    /// Authored stage index within the element stack, or synthetic stage order.
    pub stage_index: usize,
    /// Canonical stage kind such as `shader`, `shadow`, `parallel`, or `element`.
    pub stage_kind: String,
    /// Effect or synthetic substrate identifier applied at the stage.
    pub effect: String,
    /// Lifecycle status, currently `finished` or `skipped`.
    pub status: String,
    /// Canonical skip reason when status is `skipped`.
    pub skip_reason: Option<String>,
    /// Cells matched by the stage scope or substrate area.
    pub cells_matched: u32,
    /// Cells skipped by the stage scope.
    pub cells_skipped: u32,
}

impl RenderTraceEvent {
    pub(crate) fn finished(
        scene_id: impl Into<String>,
        element_id: impl Into<String>,
        stage_index: usize,
        stage_kind: RenderStageKind,
        effect: impl Into<String>,
        cells_matched: u32,
        cells_skipped: u32,
    ) -> Self {
        Self {
            scene_id: scene_id.into(),
            element_id: element_id.into(),
            stage_index,
            stage_kind: stage_kind.as_str().to_string(),
            effect: effect.into(),
            status: RenderStageStatus::Finished.as_str().to_string(),
            skip_reason: None,
            cells_matched,
            cells_skipped,
        }
    }

    pub(crate) fn skipped(
        scene_id: impl Into<String>,
        element_id: impl Into<String>,
        stage_index: usize,
        stage_kind: RenderStageKind,
        effect: impl Into<String>,
        reason: RenderSkipReason,
        cells_matched: u32,
        cells_skipped: u32,
    ) -> Self {
        Self {
            scene_id: scene_id.into(),
            element_id: element_id.into(),
            stage_index,
            stage_kind: stage_kind.as_str().to_string(),
            effect: effect.into(),
            status: RenderStageStatus::Skipped.as_str().to_string(),
            skip_reason: Some(reason.as_str().to_string()),
            cells_matched,
            cells_skipped,
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/render/cls_render_trace_event.rs</FILE> - <DESC>Native render trace event emitted with frames</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
