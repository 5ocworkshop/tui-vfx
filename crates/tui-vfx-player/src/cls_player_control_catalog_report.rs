// <FILE>crates/tui-vfx-player/src/cls_player_control_catalog_report.rs</FILE> - <DESC>Player control catalog report DTOs</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Implementation readiness/control catalog: expose descriptor-derived studio controls.</WCTX>
// <CLOG>0.1.0: INIT — add machine-readable player control catalog DTOs.</CLOG>

/// Machine-readable catalog of controls a studio can build from player-owned evidence.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerControlCatalogReport {
    /// Stable report schema label.
    pub schema_version: &'static str,
    /// Descriptor packs loaded for this invocation.
    pub descriptor_packs: Vec<crate::DescriptorPackReport>,
    /// Optional recipe path used for recipe-aware control usage.
    pub recipe: Option<String>,
    /// Aggregate counts for quick smoke checks.
    pub summary: PlayerControlCatalogSummary,
    /// Descriptor-derived controls.
    pub controls: Vec<PlayerControlCatalogControl>,
    /// Non-fatal report warnings.
    pub warnings: Vec<String>,
}

/// Aggregate control-catalog counts.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerControlCatalogSummary {
    /// Total controls emitted.
    pub controls: usize,
    /// Controls tied to source descriptor inputs.
    pub source_controls: usize,
    /// Controls tied to effect descriptor inputs.
    pub effect_controls: usize,
    /// Controls that are used by the optional recipe.
    pub recipe_used_controls: usize,
}

/// One studio-facing control declaration.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerControlCatalogControl {
    /// Stable catalog-local control id.
    pub id: String,
    /// Human-facing label.
    pub label: String,
    /// Origin kind for the control.
    pub source_kind: &'static str,
    /// Descriptor id that owns this input.
    pub descriptor_id: String,
    /// Optional recipe-local node id in recipe-aware mode.
    pub node_id: Option<String>,
    /// Descriptor-local input name.
    pub input_name: String,
    /// Canonical value kind.
    pub value_kind: String,
    /// Recommended studio control kind.
    pub control_kind: &'static str,
    /// Numeric range when declared.
    pub range: Option<tui_vfx_contract::NumericRange>,
    /// Closed allowed values for enum controls.
    pub allowed_values: Vec<String>,
    /// Optional unit label.
    pub unit: Option<String>,
    /// Optional semantic hint.
    pub semantic: Option<String>,
    /// Descriptor runtime mutability label.
    pub runtime_mutability: tui_vfx_contract::RuntimeMutability,
    /// Whether the input supports binding affordances.
    pub bindable: bool,
    /// Whether the input is optional.
    pub optional: bool,
    /// Descriptor default value, if any.
    pub default_value: Option<serde_json::Value>,
    /// Recipe-authored current value, if this is recipe-aware and supplied.
    pub current_value: Option<serde_json::Value>,
    /// Recipe nodes or source instances that use this control.
    pub used_by: Vec<String>,
    /// Human-facing documentation.
    pub documentation: Option<String>,
}

// <FILE>crates/tui-vfx-player/src/cls_player_control_catalog_report.rs</FILE> - <DESC>Player control catalog report DTOs</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
