// <FILE>crates/tui-vfx-player/src/cls_player_error.rs</FILE> - <DESC>Structured player diagnostic error DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K0: report unsupported adapters and hard player errors.</WCTX>
// <CLOG>0.1.0: INIT — add stable code/path/message/hint/details diagnostic shape.</CLOG>

/// Structured player error emitted in frame reports.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerError {
    /// Stable machine-facing diagnostic code.
    pub code: String,
    /// JSON-ish path or command path associated with the diagnostic.
    pub path: String,
    /// Human-readable diagnostic summary.
    pub message: String,
    /// Actionable remediation hint when known.
    pub hint: Option<String>,
    /// Optional structured details for future harnesses.
    pub details: serde_json::Value,
}

impl PlayerError {
    /// Build a player error with optional structured details.
    pub fn new(
        code: impl Into<String>,
        path: impl Into<String>,
        message: impl Into<String>,
        hint: Option<&str>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            code: code.into(),
            path: path.into(),
            message: message.into(),
            hint: hint.map(str::to_string),
            details,
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_error.rs</FILE> - <DESC>Structured player diagnostic error DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
