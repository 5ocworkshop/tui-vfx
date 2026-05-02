// <FILE>crates/tui-vfx-contract/src/canonicalize/cls_expansion_table.rs</FILE> - <DESC>Typed shape of schemas/v3.1/authoring/<axis>/expansion.json</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of canonicalize: deserialize per-axis preset expansion tables for runtime lookup.</WCTX>
// <CLOG>0.1.0: INIT — add ExpansionTable, PresetEntry, ParamSpec types matching meta/expansion-table.schema.json.</CLOG>

/// Preset expansion table loaded from `schemas/v3.1/authoring/<axis>/expansion.json`.
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExpansionTable {
    #[serde(rename = "$schema", default)]
    pub schema_ref: Option<String>,
    pub version: String,
    pub axis: String,
    #[serde(default)]
    pub description: Option<String>,
    pub presets: Vec<PresetEntry>,
}

impl ExpansionTable {
    /// Find a preset entry by name.
    pub fn find(&self, preset: &str) -> Option<&PresetEntry> {
        self.presets.iter().find(|entry| entry.preset == preset)
    }
}

/// One preset row.
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PresetEntry {
    pub preset: String,
    pub kind: PresetKind,
    #[serde(default)]
    pub params: Vec<ParamSpec>,
    #[serde(default)]
    pub tracks: Vec<serde_json::Value>,
    #[serde(default)]
    pub nodes: Vec<serde_json::Value>,
    #[serde(default)]
    pub subjects: Option<serde_json::Value>,
    #[serde(default)]
    pub interruption: Option<String>,
    #[serde(default)]
    pub deprecated: Option<String>,
    #[serde(default)]
    pub since: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum PresetKind {
    Transition,
    EffectStack,
}

/// Author-side parameter specification.
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParamSpec {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(default)]
    pub values: Vec<String>,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
    #[serde(default)]
    pub applies_to: Vec<String>,
    #[serde(default)]
    pub description: Option<String>,
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/cls_expansion_table.rs</FILE> - <DESC>Typed shape of schemas/v3.1/authoring/<axis>/expansion.json</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
