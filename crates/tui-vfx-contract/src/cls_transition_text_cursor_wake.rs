// <FILE>crates/tui-vfx-contract/src/cls_transition_text_cursor_wake.rs</FILE> - <DESC>Content typewriter cursor wake enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition recipe-oracle pass: cursor wake is explicit content-track vocabulary.</WCTX>
// <CLOG>0.1.0: INIT — add cursor wake vocabulary.</CLOG>

/// Cursor wake behavior for content reveal tracks.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum TransitionTextCursorWake {
    /// No cursor wake.
    Off,
    /// Leave a short fading trail behind the cursor.
    Trail,
    /// Blink the cursor while revealing content.
    Blink,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_text_cursor_wake.rs</FILE> - <DESC>Content typewriter cursor wake enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
