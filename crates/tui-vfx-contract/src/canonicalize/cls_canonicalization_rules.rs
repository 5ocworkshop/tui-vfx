// <FILE>crates/tui-vfx-contract/src/canonicalize/cls_canonicalization_rules.rs</FILE> - <DESC>Typed shape of schemas/v3.1/authoring/common/canonicalization-rules.json</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of canonicalize: deserialize the universal lookup tables (named colors, scope shapes, sigils).</WCTX>
// <CLOG>0.1.0: INIT — add CanonicalizationRules type matching meta/canonicalization-rules.schema.json.</CLOG>

use std::collections::BTreeMap;

/// Universal canonicalization rules table loaded from
/// `schemas/v3.1/authoring/common/canonicalization-rules.json`.
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CanonicalizationRules {
    #[serde(rename = "$schema", default)]
    pub schema_ref: Option<String>,
    pub version: String,
    /// Named-color set (camelCase keys → `[r, g, b]` or `[r, g, b, a]` tuples).
    pub named_colors: NamedColors,
    /// Phase aliases — only `all` matters today.
    pub phases: PhasesTable,
    /// Author-side scope shorthand shapes (documentation; canonicalize hardcodes the dispatch).
    pub scope_shape_map: Vec<ScopeShapeMapEntry>,
    /// Structural lifts that move information out of an author-facing position into a sibling NodeSpec field.
    #[serde(default)]
    pub node_field_lifts: Vec<NodeFieldLift>,
    /// Shadow-block author-to-canonical field renames.
    #[serde(default)]
    pub shadow_field_renames: Vec<ShadowFieldRename>,
    pub envelopes: Vec<EnvelopeEntry>,
    pub duration_string_forms: Vec<DurationStringForm>,
    pub sigils: Vec<SigilEntry>,
}

pub type NamedColors = BTreeMap<String, Vec<u8>>;

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PhasesTable {
    pub all: Vec<String>,
    pub single: Vec<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScopeShapeMapEntry {
    pub author_shape: String,
    pub canonical_kind: String,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub blocked_on: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NodeFieldLift {
    pub author_shape: String,
    pub canonical_field: String,
    #[serde(default)]
    pub value_rule: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShadowFieldRename {
    pub author_field: String,
    pub canonical_field: String,
    #[serde(default)]
    pub value_renames: BTreeMap<String, String>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EnvelopeEntry {
    pub author_form: String,
    pub canonical_kind: String,
    #[serde(default)]
    pub examples: Vec<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DurationStringForm {
    pub suffix: String,
    pub unit: String,
    #[serde(default)]
    pub example: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SigilEntry {
    pub sigil: String,
    pub expansion: String,
    #[serde(default)]
    pub notes: Option<String>,
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/cls_canonicalization_rules.rs</FILE> - <DESC>Typed shape of schemas/v3.1/authoring/common/canonicalization-rules.json</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
