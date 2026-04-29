// <FILE>crates/tui-vfx-player/src/fnc_collect_legacy_mask_payloads.rs</FILE> - <DESC>Collect legacy mask payload evidence</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>K2.9 migration mapping: derive mask descriptors and inputs from legacy recipe JSON.</WCTX>
// <CLOG>0.1.0: INIT — add focused legacy mask payload evidence helpers.</CLOG>

use std::collections::BTreeSet;

use serde_json::Value;

/// Collect all legacy `kind: mask` payload objects from a recipe JSON value.
pub(crate) fn collect_legacy_mask_payloads(value: &Value) -> Vec<&serde_json::Map<String, Value>> {
    let mut payloads = Vec::new();
    collect_mask_payloads_from_value(value, &mut payloads);
    payloads
}

/// Build required descriptor ids from collected legacy mask payloads.
pub(crate) fn required_legacy_mask_descriptors(
    payloads: &[&serde_json::Map<String, Value>],
) -> Vec<String> {
    sorted_unique(
        payloads
            .iter()
            .filter_map(|payload| payload.get("type").and_then(Value::as_str))
            .map(|mask_type| format!("mask.{mask_type}")),
    )
}

/// Build required input field names from collected legacy mask payloads.
pub(crate) fn required_legacy_mask_inputs(
    payloads: &[&serde_json::Map<String, Value>],
) -> Vec<String> {
    sorted_unique(payloads.iter().flat_map(|payload| {
        payload
            .keys()
            .filter(|key| key.as_str() != "type")
            .map(|key| canonical_input_field(key))
    }))
}

/// Build a compact human-readable mask evidence summary.
pub(crate) fn legacy_mask_evidence_for(payloads: &[&serde_json::Map<String, Value>]) -> String {
    if payloads.is_empty() {
        return "no mask payloads found".to_string();
    }
    let labels = payloads
        .iter()
        .filter_map(|payload| payload.get("type").and_then(Value::as_str))
        .collect::<Vec<_>>();
    format!("legacy mask payload types: {}", labels.join(", "))
}

fn collect_mask_payloads_from_value<'a>(
    value: &'a Value,
    payloads: &mut Vec<&'a serde_json::Map<String, Value>>,
) {
    match value {
        Value::Object(object) => {
            if object.get("kind").and_then(Value::as_str) == Some("mask")
                && let Some(Value::Object(payload)) = object.get("payload")
            {
                payloads.push(payload);
            }
            for child in object.values() {
                collect_mask_payloads_from_value(child, payloads);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_mask_payloads_from_value(item, payloads);
            }
        }
        _ => {}
    }
}

fn canonical_input_field(field: &str) -> String {
    match field {
        "soft_edge" => "softEdge".to_string(),
        value => value.to_string(),
    }
}

fn sorted_unique(values: impl IntoIterator<Item = String>) -> Vec<String> {
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_collect_legacy_mask_payloads.rs</FILE> - <DESC>Collect legacy mask payload evidence</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
