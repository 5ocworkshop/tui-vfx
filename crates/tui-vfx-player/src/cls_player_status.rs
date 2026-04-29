// <FILE>crates/tui-vfx-player/src/cls_player_status.rs</FILE> - <DESC>Player frame status enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player reporting work: classify rendered, unsupported, and error frame reports.</WCTX>
// <CLOG>0.1.0: INIT — add stable serialized player status vocabulary.</CLOG>

/// Stable status for one player frame report.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PlayerStatus {
    /// The supported adapter subset rendered the frame.
    Rendered,
    /// The recipe is valid but needs an adapter that the player does not implement.
    Unsupported,
    /// The recipe could not be loaded, deserialized, or contract-validated.
    Error,
}

// <FILE>crates/tui-vfx-player/src/cls_player_status.rs</FILE> - <DESC>Player frame status enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
