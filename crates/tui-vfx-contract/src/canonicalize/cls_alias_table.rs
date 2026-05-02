// <FILE>crates/tui-vfx-contract/src/canonicalize/cls_alias_table.rs</FILE> - <DESC>Typed shape of schemas/v3.1/authoring/<axis>/aliases.json</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of canonicalize: deserialize per-axis alias tables for runtime lookup.</WCTX>
// <CLOG>0.1.0: INIT — add AliasTable, AliasEntry, ParamMapping types matching meta/alias-table.schema.json.</CLOG>

use std::collections::BTreeMap;

/// One per-axis alias table loaded from `schemas/v3.1/authoring/<axis>/aliases.json`.
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AliasTable {
    /// Optional `$schema` reference for editor tooling; unused at runtime.
    #[serde(rename = "$schema", default)]
    pub schema_ref: Option<String>,
    pub version: String,
    pub axis: String,
    #[serde(default)]
    pub description: Option<String>,
    pub aliases: Vec<AliasEntry>,
}

impl AliasTable {
    /// Find an alias entry by its author-side spelling.
    pub fn find(&self, from: &str) -> Option<&AliasEntry> {
        self.aliases.iter().find(|entry| entry.from == from)
    }
}

/// One row in an alias table.
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AliasEntry {
    pub from: String,
    pub canonical_effect: String,
    #[serde(default)]
    pub param_mapping: BTreeMap<String, ParamMapping>,
    #[serde(default)]
    pub default_params: serde_json::Value,
    #[serde(default)]
    pub scope_rules: Option<ScopeRules>,
    /// When true, author-side `applyTo` binds to a per-effect input rather than
    /// lifting to `NodeSpec.writeChannels`.
    #[serde(default)]
    pub apply_to_is_input: bool,
    #[serde(default)]
    pub deprecated: Option<String>,
    #[serde(default)]
    pub since: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

/// Optional per-alias scope override.
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScopeRules {
    /// Canonical scope to use when the author omits `scope` entirely.
    #[serde(default)]
    pub default: Option<serde_json::Value>,
}

/// Per-parameter envelope mapping inside an alias entry.
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParamMapping {
    pub to: String,
    #[serde(default)]
    pub envelope: Option<String>,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/cls_alias_table.rs</FILE> - <DESC>Typed shape of schemas/v3.1/authoring/<axis>/aliases.json</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
