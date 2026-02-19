// <FILE>tui-vfx-content/src/types/cls_slide_shift_line_mode.rs</FILE> - <DESC>Line handling for SlideShift content effect</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>SlideShift content transformer</WCTX>
// <CLOG>Define line mode for block vs first-line indentation</CLOG>

/// Controls how horizontal shifts apply across multi-line text.
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
pub enum SlideShiftLineMode {
    /// Apply horizontal shift to every line (block movement).
    #[default]
    Block,
    /// Apply horizontal shift only to the first line.
    FirstLineOnly,
}

// <FILE>tui-vfx-content/src/types/cls_slide_shift_line_mode.rs</FILE> - <DESC>Line handling for SlideShift content effect</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
