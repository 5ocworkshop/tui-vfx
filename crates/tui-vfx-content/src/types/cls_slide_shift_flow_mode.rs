// <FILE>tui-vfx-content/src/types/cls_slide_shift_flow_mode.rs</FILE> - <DESC>Flow behavior for SlideShift content effect</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>SlideShift content transformer</WCTX>
// <CLOG>Define flow mode for stay-shifted vs flow-back behavior</CLOG>

/// Controls whether SlideShift stays on the shifted row or flows back.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum SlideShiftFlowMode {
    /// Stay on the shifted row after crossing the barrier.
    #[default]
    StayShifted,
    /// Shift only while overlapping the barrier, then return to base row.
    FlowBack,
}

// <FILE>tui-vfx-content/src/types/cls_slide_shift_flow_mode.rs</FILE> - <DESC>Flow behavior for SlideShift content effect</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
