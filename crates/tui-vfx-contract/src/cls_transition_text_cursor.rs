// <FILE>crates/tui-vfx-contract/src/cls_transition_text_cursor.rs</FILE> - <DESC>Content typewriter cursor DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition recipe-oracle pass: typewriter is a content transition track.</WCTX>
// <CLOG>0.1.0: INIT — add optional cursor payload for typewriter transitions.</CLOG>

use crate::TransitionTextCursorWake;

/// Optional cursor behavior for `content.typewriter` tracks.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransitionTextCursor {
    /// Cursor glyph rendered at the reveal frontier.
    pub character: String,
    /// Cursor wake/trail behavior.
    pub wake: TransitionTextCursorWake,
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_text_cursor.rs</FILE> - <DESC>Content typewriter cursor DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
