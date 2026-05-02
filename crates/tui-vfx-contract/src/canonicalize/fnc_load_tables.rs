// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_load_tables.rs</FILE> - <DESC>One-time embedded loader for alias/expansion/canonicalization-rules JSON tables</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of canonicalize: bake schemas/v3.1/authoring/* into the binary via include_str! and parse on first use.</WCTX>
// <CLOG>0.1.0: INIT — load canonicalization rules and per-axis alias/expansion tables once.</CLOG>

use std::sync::OnceLock;

use super::cls_alias_table::AliasTable;
use super::cls_canonicalization_error::{CanonicalizationError, CanonicalizationErrorKind};
use super::cls_canonicalization_rules::CanonicalizationRules;
use super::cls_expansion_table::ExpansionTable;

const RULES_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/common/canonicalization-rules.json"
));

const TRANSITION_ALIASES_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/transition/aliases.json"
));
const TRANSITION_EXPANSION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/transition/expansion.json"
));

const FILTER_ALIASES_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/filter/aliases.json"
));

const SHADER_ALIASES_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/shader/aliases.json"
));

const SAMPLER_ALIASES_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/sampler/aliases.json"
));

const STYLE_ALIASES_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/style/aliases.json"
));

const MASK_ALIASES_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/mask/aliases.json"
));

const CONTENT_ALIASES_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../schemas/v3.1/authoring/content/aliases.json"
));

static RULES: OnceLock<CanonicalizationRules> = OnceLock::new();
static TRANSITION_ALIASES: OnceLock<AliasTable> = OnceLock::new();
static TRANSITION_EXPANSION: OnceLock<ExpansionTable> = OnceLock::new();
static FILTER_ALIASES: OnceLock<AliasTable> = OnceLock::new();
static SHADER_ALIASES: OnceLock<AliasTable> = OnceLock::new();
static SAMPLER_ALIASES: OnceLock<AliasTable> = OnceLock::new();
static STYLE_ALIASES: OnceLock<AliasTable> = OnceLock::new();
static MASK_ALIASES: OnceLock<AliasTable> = OnceLock::new();
static CONTENT_ALIASES: OnceLock<AliasTable> = OnceLock::new();

/// Universal canonicalization rules table.
pub fn canonicalization_rules() -> Result<&'static CanonicalizationRules, CanonicalizationError> {
    if let Some(rules) = RULES.get() {
        return Ok(rules);
    }
    let parsed: CanonicalizationRules = serde_json::from_str(RULES_JSON).map_err(|e| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::MalformedTable {
                table: "common/canonicalization-rules.json".into(),
                detail: e.to_string(),
            },
            e.to_string(),
        )
    })?;
    Ok(RULES.get_or_init(|| parsed))
}

/// Per-axis alias table for the named axis.
pub fn alias_table(axis: AliasAxis) -> Result<&'static AliasTable, CanonicalizationError> {
    let (cell, json, name) = match axis {
        AliasAxis::Transition => (
            &TRANSITION_ALIASES,
            TRANSITION_ALIASES_JSON,
            "transition/aliases.json",
        ),
        AliasAxis::Filter => (&FILTER_ALIASES, FILTER_ALIASES_JSON, "filter/aliases.json"),
        AliasAxis::Shader => (&SHADER_ALIASES, SHADER_ALIASES_JSON, "shader/aliases.json"),
        AliasAxis::Sampler => (
            &SAMPLER_ALIASES,
            SAMPLER_ALIASES_JSON,
            "sampler/aliases.json",
        ),
        AliasAxis::Style => (&STYLE_ALIASES, STYLE_ALIASES_JSON, "style/aliases.json"),
        AliasAxis::Mask => (&MASK_ALIASES, MASK_ALIASES_JSON, "mask/aliases.json"),
        AliasAxis::Content => (
            &CONTENT_ALIASES,
            CONTENT_ALIASES_JSON,
            "content/aliases.json",
        ),
    };
    if let Some(table) = cell.get() {
        return Ok(table);
    }
    let parsed: AliasTable = serde_json::from_str(json).map_err(|e| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::MalformedTable {
                table: name.into(),
                detail: e.to_string(),
            },
            e.to_string(),
        )
    })?;
    Ok(cell.get_or_init(|| parsed))
}

/// Transition expansion table.
pub fn transition_expansion() -> Result<&'static ExpansionTable, CanonicalizationError> {
    if let Some(table) = TRANSITION_EXPANSION.get() {
        return Ok(table);
    }
    let parsed: ExpansionTable = serde_json::from_str(TRANSITION_EXPANSION_JSON).map_err(|e| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::MalformedTable {
                table: "transition/expansion.json".into(),
                detail: e.to_string(),
            },
            e.to_string(),
        )
    })?;
    Ok(TRANSITION_EXPANSION.get_or_init(|| parsed))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AliasAxis {
    Transition,
    Filter,
    Shader,
    Sampler,
    Style,
    Mask,
    Content,
}

impl AliasAxis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Transition => "transition",
            Self::Filter => "filter",
            Self::Shader => "shader",
            Self::Sampler => "sampler",
            Self::Style => "style",
            Self::Mask => "mask",
            Self::Content => "content",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_table_loads() {
        let rules = canonicalization_rules().expect("rules parse");
        assert!(rules.named_colors.contains_key("white"));
        assert!(rules.named_colors.contains_key("transparent"));
        assert_eq!(rules.phases.all, vec!["enter", "dwell", "exit"]);
    }

    #[test]
    fn alias_tables_load_all_axes() {
        for axis in [
            AliasAxis::Transition,
            AliasAxis::Filter,
            AliasAxis::Shader,
            AliasAxis::Sampler,
            AliasAxis::Style,
            AliasAxis::Mask,
            AliasAxis::Content,
        ] {
            let table = alias_table(axis).expect("alias table parses");
            assert_eq!(table.axis, axis.as_str());
        }
    }

    #[test]
    fn transition_expansion_includes_corpus_witnessed_presets() {
        let table = transition_expansion().expect("expansion parses");
        for preset in [
            "fade",
            "iris",
            "wipe",
            "dissolve",
            "blinds",
            "stippled",
            "braille",
            "crossfade",
            "push",
            "morph",
        ] {
            assert!(
                table.find(preset).is_some(),
                "transition expansion missing preset: {preset}"
            );
        }
    }

    #[test]
    fn focused_row_gradient_has_apply_to_is_input_flag() {
        let shader = alias_table(AliasAxis::Shader).expect("shader aliases parse");
        let entry = shader
            .find("focused_row_gradient")
            .expect("focused_row_gradient alias present");
        assert!(
            entry.apply_to_is_input,
            "focused_row_gradient must declare applyToIsInput per OQ3"
        );
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_load_tables.rs</FILE> - <DESC>One-time embedded loader for alias/expansion/canonicalization-rules JSON tables</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
