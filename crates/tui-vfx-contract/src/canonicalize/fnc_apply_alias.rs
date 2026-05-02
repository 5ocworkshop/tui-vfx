// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_apply_alias.rs</FILE> - <DESC>Materialize one author-side effect entry into a canonical NodeSpec JSON value</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of canonicalize: walk an effects[] entry, apply the alias paramMapping, lift channel/applyTo to writeChannels.</WCTX>
// <CLOG>0.1.0: INIT — emit a NodeSpec-shape JSON object from a single alias entry plus author params.</CLOG>

use serde_json::{Map, Value, json};

use super::cls_alias_table::AliasEntry;
use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
use super::fnc_lift_value_envelope::{EnvelopeHint, lift_value_envelope};
use super::fnc_resolve_phases::resolve_phases;
use super::fnc_resolve_scope::resolve_scope;

/// NodeSpec-level keys that route to fields on the node rather than into `inputs`.
const NODE_LEVEL_KEYS: &[&str] = &[
    "filter",
    "shader",
    "sampler",
    "style",
    "mask",
    "id",
    "phase",
    "scope",
    "applyTo",
    "writeChannels",
    "cellWritePolicy",
    "roleWritePolicy",
];

/// Build a canonical NodeSpec JSON value from one author-side effects entry
/// plus the matching alias entry from the table.
///
/// The `axis_key` argument is the authoring-axis property (`"filter"`,
/// `"shader"`, `"sampler"`, `"style"`, `"mask"`) so the function can skip it
/// while iterating author keys.
pub fn apply_alias(
    entry: &AliasEntry,
    axis_key: &str,
    author: &Map<String, Value>,
    node_id: String,
) -> Result<Value, CanonicalizationError> {
    let mut inputs = Map::new();
    let mut active_phases: Option<Value> = None;
    let mut scope: Option<Value> = None;
    let mut write_channels: Vec<String> = Vec::new();

    if let Value::Object(defaults) = &entry.default_params {
        for (key, value) in defaults {
            inputs.insert(key.clone(), value.clone());
        }
    }

    for (key, value) in author {
        if key == axis_key {
            continue;
        }
        match key.as_str() {
            "id" => continue,
            "phase" => {
                let resolved =
                    resolve_phases(value).map_err(|e| e.at(JsonPathSegment::field("phase")))?;
                active_phases = Some(resolved);
            }
            "scope" => {
                let resolved =
                    resolve_scope(value).map_err(|e| e.at(JsonPathSegment::field("scope")))?;
                if let Some(s) = resolved.scope {
                    scope = Some(s);
                }
                write_channels.extend(resolved.write_channels);
            }
            "applyTo" => {
                if entry.apply_to_is_input {
                    let envelope_hint = entry
                        .param_mapping
                        .get("applyTo")
                        .and_then(|m| m.envelope.as_deref())
                        .and_then(EnvelopeHint::from_name)
                        .unwrap_or(EnvelopeHint::EnumLiteral);
                    let mapped = entry
                        .param_mapping
                        .get("applyTo")
                        .map(|m| m.to.clone())
                        .unwrap_or_else(|| "applyTo".into());
                    let lifted = lift_value_envelope(value, envelope_hint)
                        .map_err(|e| e.at(JsonPathSegment::field("applyTo")))?;
                    inputs.insert(mapped, lifted);
                } else {
                    apply_to_lift(value, &mut write_channels)?;
                }
            }
            "writeChannels" => {
                let arr = value.as_array().ok_or_else(|| {
                    CanonicalizationError::new(
                        CanonicalizationErrorKind::UnexpectedJsonShape {
                            expected: "array of channel names".into(),
                        },
                        "writeChannels must be an array",
                    )
                    .at(JsonPathSegment::field("writeChannels"))
                })?;
                for entry in arr {
                    let s = entry.as_str().ok_or_else(|| {
                        CanonicalizationError::new(
                            CanonicalizationErrorKind::UnexpectedJsonShape {
                                expected: "string".into(),
                            },
                            "writeChannels entries must be strings",
                        )
                    })?;
                    write_channels.push(s.into());
                }
            }
            _ if NODE_LEVEL_KEYS.contains(&key.as_str()) => continue,
            _ => {
                let mapping = entry.param_mapping.get(key);
                let target_key = mapping.map(|m| m.to.clone()).unwrap_or_else(|| key.clone());
                let envelope_hint = mapping
                    .and_then(|m| m.envelope.as_deref())
                    .and_then(EnvelopeHint::from_name)
                    .unwrap_or(EnvelopeHint::None);
                let lifted = lift_value_envelope(value, envelope_hint)
                    .map_err(|e| e.at(JsonPathSegment::field(key.clone())))?;
                inputs.insert(target_key, lifted);
            }
        }
    }

    if scope.is_none()
        && let Some(rules) = &entry.scope_rules
        && let Some(default_scope) = &rules.default
    {
        scope = Some(default_scope.clone());
    }

    let mut node = Map::new();
    node.insert("id".into(), Value::String(node_id));
    node.insert(
        "effect".into(),
        Value::String(entry.canonical_effect.clone()),
    );
    node.insert("inputs".into(), Value::Object(inputs));
    node.insert("outputs".into(), Value::Object(Map::new()));
    if let Some(phases) = active_phases {
        node.insert("activePhases".into(), phases);
    } else {
        node.insert("activePhases".into(), json!([]));
    }
    if let Some(s) = scope {
        node.insert("scope".into(), s);
    }
    if !write_channels.is_empty() {
        let dedup = dedup_preserve_order(write_channels);
        node.insert(
            "writeChannels".into(),
            Value::Array(dedup.into_iter().map(Value::String).collect()),
        );
    }
    Ok(Value::Object(node))
}

fn apply_to_lift(
    value: &Value,
    write_channels: &mut Vec<String>,
) -> Result<(), CanonicalizationError> {
    let s = value.as_str().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "string".into(),
            },
            "applyTo lift expects a channel name string",
        )
        .at(JsonPathSegment::field("applyTo"))
    })?;
    match s {
        "both" => Ok(()),
        "foreground" | "background" => {
            write_channels.push(s.into());
            Ok(())
        }
        _ => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "foreground / background / both".into(),
            },
            format!("unknown applyTo channel: {s}"),
        )
        .at(JsonPathSegment::field("applyTo"))),
    }
}

fn dedup_preserve_order(items: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::BTreeSet::new();
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        if seen.insert(item.clone()) {
            out.push(item);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonicalize::cls_alias_table::AliasEntry;

    fn dim_entry() -> AliasEntry {
        serde_json::from_value(json!({
            "from": "dim",
            "canonicalEffect": "filter.dim",
            "paramMapping": {
                "factor": { "to": "factor", "envelope": "literal-number", "default": 0.5 },
                "phase":  { "to": "activePhases", "envelope": "phases-list" },
                "scope":  { "to": "scope", "envelope": "scope-spec" }
            }
        }))
        .unwrap()
    }

    fn focused_row_entry() -> AliasEntry {
        serde_json::from_value(json!({
            "from": "focused_row_gradient",
            "canonicalEffect": "shader.focused_row_gradient",
            "applyToIsInput": true,
            "paramMapping": {
                "applyTo": { "to": "applyTo", "envelope": "literal-enum", "default": "both" }
            }
        }))
        .unwrap()
    }

    #[test]
    fn dim_with_scope_channel_lifts_to_write_channels() {
        let entry = dim_entry();
        let author: Map<String, Value> = serde_json::from_value(json!({
            "filter": "dim",
            "factor": 0.5,
            "phase": "exit",
            "scope": { "channel": "foreground" }
        }))
        .unwrap();
        let node = apply_alias(&entry, "filter", &author, "dim0".into()).unwrap();
        assert_eq!(node["effect"], "filter.dim");
        assert_eq!(
            node["inputs"]["factor"],
            json!({ "kind": "literal", "value": { "kind": "number", "value": 0.5 } })
        );
        assert_eq!(node["activePhases"], json!(["exit"]));
        assert_eq!(node["writeChannels"], json!(["foreground"]));
        assert!(
            node.get("scope").is_none(),
            "scope should be omitted when only channel was authored"
        );
    }

    #[test]
    fn dim_with_phase_array_handles_array() {
        let entry = dim_entry();
        let author: Map<String, Value> = serde_json::from_value(json!({
            "filter": "dim",
            "factor": 0.3,
            "phase": ["enter", "dwell"]
        }))
        .unwrap();
        let node = apply_alias(&entry, "filter", &author, "dim0".into()).unwrap();
        assert_eq!(node["activePhases"], json!(["enter", "dwell"]));
    }

    #[test]
    fn focused_row_gradient_keeps_apply_to_as_input() {
        let entry = focused_row_entry();
        let author: Map<String, Value> = serde_json::from_value(json!({
            "shader": "focused_row_gradient",
            "applyTo": "foreground"
        }))
        .unwrap();
        let node = apply_alias(&entry, "shader", &author, "rg".into()).unwrap();
        assert_eq!(
            node["inputs"]["applyTo"],
            json!({ "kind": "literal", "value": { "kind": "enum", "value": "foreground" } })
        );
        assert!(
            node.get("writeChannels").is_none(),
            "applyToIsInput should suppress the write_channels lift"
        );
    }

    #[test]
    fn unmapped_param_passes_through_with_inferred_envelope() {
        let entry = dim_entry();
        let author: Map<String, Value> = serde_json::from_value(json!({
            "filter": "dim",
            "factor": 0.3,
            "novelKnob": 7
        }))
        .unwrap();
        let node = apply_alias(&entry, "filter", &author, "dim0".into()).unwrap();
        assert_eq!(
            node["inputs"]["novelKnob"],
            json!({ "kind": "literal", "value": { "kind": "integer", "value": 7 } })
        );
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_apply_alias.rs</FILE> - <DESC>Materialize one author-side effect entry into a canonical NodeSpec JSON value</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
