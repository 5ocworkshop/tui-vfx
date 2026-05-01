// <FILE>crates/tui-vfx-contract/src/cls_transition_edge.rs</FILE> - <DESC>Transition visibility edge DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition tracks: describe hard and soft visibility boundaries.</WCTX>
// <CLOG>0.1.0: INIT — add visibility edge union.</CLOG>

/// Edge treatment for visibility transition tracks.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum TransitionEdge {
    /// Hard threshold edge.
    Hard,
    /// Soft edge feathered by whole terminal cells.
    Soft {
        /// Feather width in cells.
        feather_cells: u16,
    },
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_edge.rs</FILE> - <DESC>Transition visibility edge DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
