// <FILE>crates/tui-vfx-player/src/cls_player_render_ir.rs</FILE> - <DESC>Player-owned render IR report DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player render evidence work: introduce backend-seam-ready IR without compositor imports.</WCTX>
// <CLOG>0.1.0: INIT — add serializable player render IR for rows, styled cells, provenance, diagnostics, clock, and graph values.</CLOG>

use tui_vfx_contract::{LifecyclePhase, Value};

use crate::{PlayerError, PlayerStatus, PlayerWarning};

/// Player-owned render IR for one sampled recipe frame.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRenderIrReport {
    /// Stable render-IR report schema label.
    pub schema_version: &'static str,
    /// Canonical recipe id.
    pub recipe_id: String,
    /// Optional recipe file path when rendered from disk.
    pub path: Option<String>,
    /// Render status from the player pipeline.
    pub status: PlayerStatus,
    /// Requested lifecycle phase.
    pub phase: LifecyclePhase,
    /// Requested normalized phase progress.
    pub phase_t: f64,
    /// Optional requested loop progress.
    pub loop_t: Option<f64>,
    /// Frame width in terminal cells.
    pub width: usize,
    /// Frame height in terminal cells.
    pub height: usize,
    /// Deterministic hash shared with the existing frame report for compatibility checks.
    pub render_hash: u64,
    /// Number of non-space cells in rows.
    pub non_empty_cells: usize,
    /// Final compact text rows.
    pub rows: Vec<String>,
    /// Sparse styled-cell evidence emitted by player adapters.
    pub styled_cells: Vec<PlayerRenderCell>,
    /// Scene/source/layer provenance for placed scene elements.
    pub provenance: Vec<PlayerRenderProvenance>,
    /// Runtime layer visibility and skip decisions.
    pub layers: Vec<PlayerRenderLayer>,
    /// Graph value snapshot after graph execution.
    pub graph_values: Vec<PlayerRenderGraphValueSnapshot>,
    /// Hard player errors and unsupported adapter diagnostics.
    pub errors: Vec<PlayerError>,
    /// Non-fatal player warnings.
    pub warnings: Vec<PlayerWarning>,
}

/// One styled cell in the player render IR.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRenderCell {
    /// Cell x coordinate in terminal columns.
    pub x: usize,
    /// Cell y coordinate in terminal rows.
    pub y: usize,
    /// Rendered glyph.
    pub glyph: String,
    /// Foreground color label or serialized color.
    pub foreground: String,
    /// Background color label or serialized color.
    pub background: String,
    /// Text modifiers known for this cell.
    pub modifiers: Vec<String>,
    /// Optional semantic role.
    pub role: Option<String>,
}

/// Scene/source provenance for one placed recipe element.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRenderProvenance {
    /// Scene id that owns the element.
    pub scene_id: String,
    /// Element id inside the scene.
    pub element_id: String,
    /// Optional lightweight layer id.
    pub layer_id: Option<String>,
    /// Source instance id used by the element.
    pub source_id: Option<String>,
    /// Source descriptor id when the source instance exists.
    pub source_descriptor_id: Option<String>,
    /// Element-local x placement in scene coordinates.
    pub x: i32,
    /// Element-local y placement in scene coordinates.
    pub y: i32,
    /// Element z order.
    pub z_index: i32,
    /// Authored cell write policy.
    pub cell_write_policy: String,
    /// Whether this element was rendered into the final scene.
    pub rendered: bool,
    /// Machine-readable reason when the element was skipped.
    pub skip_reason: Option<String>,
}

/// Runtime render result for one scene element/layer entry.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRenderLayer {
    /// Scene id that owns the element.
    pub scene_id: String,
    /// Element id inside the scene.
    pub element_id: String,
    /// Optional lightweight layer id.
    pub layer_id: Option<String>,
    /// Whether the visibility predicate allowed rendering.
    pub visible: bool,
    /// Whether the player skipped render and placement for this element.
    pub skipped: bool,
    /// Machine-readable reason when skipped.
    pub skip_reason: Option<String>,
}

/// Graph value snapshot after player graph execution.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRenderGraphValueSnapshot {
    /// Graph value id.
    pub id: String,
    /// Serialized graph value.
    pub value: Value,
}

// <FILE>crates/tui-vfx-player/src/cls_player_render_ir.rs</FILE> - <DESC>Player-owned render IR report DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
