// <FILE>crates/tui-vfx-contract/src/cls_transition_intent.rs</FILE> - <DESC>Preserved transition author intent metadata</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition schema: keep author shorthand intent separate from executable tracks.</WCTX>
// <CLOG>0.1.0: INIT — add preset and alias transition intent metadata.</CLOG>

use crate::TransitionPreset;

/// Preset intent preserved after author shorthand canonicalization.
///
/// The compositor executes `TransitionTrack` values directly; intent is retained
/// for diagnostics, documentation, theme tooling, profiling labels, and corpus
/// analysis. It is not a runtime bridge or legacy execution DTO.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum TransitionIntent {
    /// Author selected a canonical transition preset.
    Preset {
        /// Canonical preset name selected by the authoring shorthand.
        preset: TransitionPreset,
    },
    /// Author selected an alias that canonicalized to a preset.
    Alias {
        /// Author-facing alias spelling.
        alias: String,
        /// Canonical preset represented by the alias.
        canonical_preset: TransitionPreset,
    },
}

// <FILE>crates/tui-vfx-contract/src/cls_transition_intent.rs</FILE> - <DESC>Preserved transition author intent metadata</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
