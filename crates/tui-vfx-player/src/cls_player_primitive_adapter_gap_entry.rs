// <FILE>crates/tui-vfx-player/src/cls_player_primitive_adapter_gap_entry.rs</FILE> - <DESC>Primitive adapter gap entry DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: expose per-effect support outcomes.</WCTX>
// <CLOG>0.1.0: INIT — add stable adapter gap row shape.</CLOG>

/// Per-effect adapter support classification for represented primitive effects.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPrimitiveAdapterGapEntry {
    /// Effect descriptor id.
    pub effect_id: String,
    /// Whether the id is supplied by a loaded descriptor pack.
    pub descriptor_covered: bool,
    /// Whether any inventoried recipe references the id.
    pub represented_by_recipes: bool,
    /// Current inventory adapter status.
    pub adapter_status: String,
    /// Support outcome for this effect.
    pub outcome: String,
    /// Runtime substrate class needed for honest support.
    pub adapter_class: String,
    /// Recipe paths that reference the effect.
    pub recipe_paths: Vec<String>,
    /// Human-readable classification rationale.
    pub reason: String,
}

// <FILE>crates/tui-vfx-player/src/cls_player_primitive_adapter_gap_entry.rs</FILE> - <DESC>Primitive adapter gap entry DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
