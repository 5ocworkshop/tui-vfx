// <FILE>crates/tui-vfx-player/src/cls_player_render_backend_output.rs</FILE> - <DESC>Player-owned render backend output DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Render backend seam: keep deterministic text and styled-cell outputs inside player core.</WCTX>
// <CLOG>0.1.0: INIT — add backend output and diagnostic DTOs consumed from PlayerRenderIrReport.</CLOG>

use crate::PlayerRenderCell;

/// Deterministic output from a player-owned render backend.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRenderBackendOutput {
    /// Stable backend output report schema label.
    pub schema_version: &'static str,
    /// Backend implementation label.
    pub backend: &'static str,
    /// Text rows emitted by the backend.
    pub rows: Vec<String>,
    /// Sparse styled-cell evidence emitted by the backend.
    pub styled_cells: Vec<PlayerRenderCell>,
    /// Backend-owned non-fatal diagnostics.
    pub diagnostics: Vec<PlayerRenderBackendDiagnostic>,
}

/// Player-owned render backend diagnostic.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRenderBackendDiagnostic {
    /// Stable machine-facing diagnostic code.
    pub code: String,
    /// JSON-ish path associated with the diagnostic.
    pub path: String,
    /// Human-readable diagnostic summary.
    pub message: String,
}

impl PlayerRenderBackendOutput {
    /// Build a backend output with the shared backend report schema label.
    pub fn new(
        backend: &'static str,
        rows: Vec<String>,
        styled_cells: Vec<PlayerRenderCell>,
        diagnostics: Vec<PlayerRenderBackendDiagnostic>,
    ) -> Self {
        Self {
            schema_version: "v3.1.player.renderBackend.1",
            backend,
            rows,
            styled_cells,
            diagnostics,
        }
    }
}

// <FILE>crates/tui-vfx-player/src/cls_player_render_backend_output.rs</FILE> - <DESC>Player-owned render backend output DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
