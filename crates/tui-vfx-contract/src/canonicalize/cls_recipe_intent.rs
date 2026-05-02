// <FILE>crates/tui-vfx-contract/src/canonicalize/cls_recipe_intent.rs</FILE> - <DESC>Recipe-level provenance metadata recorded by canonicalization</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of canonicalize: provenance struct attached as RecipeDocument.intent for diagnostics.</WCTX>
// <CLOG>0.1.0: INIT — add RecipeIntent and supporting usage types.</CLOG>

use std::collections::BTreeMap;

/// Provenance metadata recorded by canonicalization.
///
/// Parallels [`TransitionIntent`] at the recipe level. `None` for input
/// already in canonical form. Persisted alongside the canonical document
/// for diagnostics, theme tooling, and corpus analysis — the runtime
/// ignores it.
///
/// [`TransitionIntent`]: crate::TransitionIntent
#[derive(
    Clone, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecipeIntent {
    /// Templates resolved by the `extends:` chain, in order from most-derived to root.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extends_chain: Vec<ExtendsChainEntry>,
    /// Per-node alias usages keyed by the canonical node id.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub alias_usages: BTreeMap<String, AliasUsage>,
    /// Per-transition preset usages keyed by the canonical transition id.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub preset_usages: BTreeMap<String, PresetUsage>,
}

/// One step in a resolved `extends:` chain.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ExtendsChainEntry {
    /// Path of the template, as written in the parent's `extends:` field.
    pub path: String,
    /// Top-level keys contributed by this template after deep-merge.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub merged_keys: Vec<String>,
}

/// Record of one alias being applied during effect-axis canonicalization.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AliasUsage {
    /// Author-facing axis: `filter`, `shader`, `sampler`, `style`, `mask`.
    pub axis: String,
    /// Author-side spelling that matched.
    pub from: String,
    /// Canonical effect identifier emitted (e.g., `filter.dim`).
    pub canonical_effect: String,
}

/// Record of one preset being applied during transition-block canonicalization.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PresetUsage {
    /// Preset name authored.
    pub preset: String,
    /// Names of author-side params consumed by the expansion. Values are not
    /// captured here — they live in the canonical tracks the preset emitted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consumed_params: Vec<String>,
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/cls_recipe_intent.rs</FILE> - <DESC>Recipe-level provenance metadata recorded by canonicalization</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
