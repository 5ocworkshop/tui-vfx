// <FILE>crates/tui-vfx-player/src/cls_player_warning.rs</FILE> - <DESC>Structured player warning DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player reporting work: reserve a structured non-fatal diagnostic channel in frame reports.</WCTX>
// <CLOG>0.1.0: INIT — add stable code/path/message/hint warning shape.</CLOG>

/// Structured non-fatal player warning emitted in frame reports.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerWarning {
    /// Stable machine-facing diagnostic code.
    pub code: String,
    /// JSON-ish path associated with the warning.
    pub path: String,
    /// Human-readable warning summary.
    pub message: String,
    /// Actionable remediation hint when known.
    pub hint: Option<String>,
}

impl PlayerWarning {
    /// Build a player warning with an optional remediation hint.
    pub fn new(
        code: impl Into<String>,
        path: impl Into<String>,
        message: impl Into<String>,
        hint: Option<&str>,
    ) -> Self {
        Self {
            code: code.into(),
            path: path.into(),
            message: message.into(),
            hint: hint.map(str::to_string),
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_warning.rs</FILE> - <DESC>Structured player warning DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
