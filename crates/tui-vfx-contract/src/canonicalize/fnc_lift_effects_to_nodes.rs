// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_effects_to_nodes.rs</FILE> - <DESC>Walk effects[] and emit graph.nodes + graph.order</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of canonicalize: dispatch effects entries to per-axis alias tables and seed the canonical graph.</WCTX>
// <CLOG>0.1.0: INIT — build canonical graph nodes from filter/shader/sampler/style/mask effect entries plus a deterministic order.</CLOG>

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
use super::cls_recipe_intent::AliasUsage;
use super::fnc_apply_alias::apply_alias;
use super::fnc_load_tables::{AliasAxis, alias_table};

/// Lift the author-side `effects: [...]` array into canonical graph nodes.
///
/// Mutates `recipe.graph.nodes` and `recipe.graph.order`. Returns provenance
/// describing each alias usage so the top-level orchestrator can populate
/// `RecipeIntent.alias_usages`. The mask form is restricted to entries that
/// declare `phase: "dwell"` or a multi-phase array (Q3 effect form); other
/// `mask` entries belong to the transitions block and are left in place.
pub fn lift_effects_to_nodes(
    recipe: &mut Value,
) -> Result<BTreeMap<String, AliasUsage>, CanonicalizationError> {
    let recipe_obj = recipe.as_object_mut().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "object".into(),
            },
            "recipe root must be an object",
        )
    })?;

    let effects = match recipe_obj.remove("effects") {
        Some(Value::Array(arr)) => arr,
        Some(other) => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "array".into(),
                },
                format!("effects must be an array, got {other}"),
            )
            .at(JsonPathSegment::field("effects")));
        }
        None => return Ok(BTreeMap::new()),
    };

    let mut alias_usages = BTreeMap::new();
    let mut id_counter: BTreeMap<String, usize> = BTreeMap::new();
    let mut new_nodes: Vec<(String, Value)> = Vec::new();

    for (index, raw) in effects.into_iter().enumerate() {
        let entry_obj = raw.as_object().ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "object".into(),
                },
                "effects[] entry must be an object",
            )
            .at(JsonPathSegment::Index(index))
            .at(JsonPathSegment::field("effects"))
        })?;

        let (axis, axis_key, from) = detect_axis(entry_obj).map_err(|e| {
            e.at(JsonPathSegment::Index(index))
                .at(JsonPathSegment::field("effects"))
        })?;

        let table = alias_table(axis)?;
        let alias_entry = table.find(&from).ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::UnknownAlias {
                    axis: axis.as_str().into(),
                    from: from.clone(),
                },
                format!(
                    "no alias for {} '{}' in {}/aliases.json",
                    axis.as_str(),
                    from,
                    axis.as_str()
                ),
            )
            .at(JsonPathSegment::Index(index))
            .at(JsonPathSegment::field("effects"))
        })?;

        let node_id = match entry_obj.get("id").and_then(Value::as_str) {
            Some(explicit) => explicit.to_string(),
            None => allocate_node_id(&from, &mut id_counter),
        };

        let node_json = apply_alias(alias_entry, axis_key, entry_obj, node_id.clone())?;
        alias_usages.insert(
            node_id.clone(),
            AliasUsage {
                axis: axis.as_str().into(),
                from: from.clone(),
                canonical_effect: alias_entry.canonical_effect.clone(),
            },
        );
        new_nodes.push((node_id, node_json));
    }

    if new_nodes.is_empty() {
        return Ok(alias_usages);
    }

    let graph = recipe_obj
        .entry("graph")
        .or_insert_with(|| Value::Object(Map::new()));
    let graph_obj = graph.as_object_mut().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "object".into(),
            },
            "graph must be an object",
        )
        .at(JsonPathSegment::field("graph"))
    })?;
    graph_obj
        .entry("id")
        .or_insert_with(|| Value::String("mainGraph".into()));
    graph_obj
        .entry("version")
        .or_insert_with(|| Value::String("3.1".into()));

    let nodes = graph_obj
        .entry("nodes")
        .or_insert_with(|| Value::Object(Map::new()))
        .as_object_mut()
        .ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "object".into(),
                },
                "graph.nodes must be an object",
            )
            .at(JsonPathSegment::field("nodes"))
            .at(JsonPathSegment::field("graph"))
        })?;
    let mut node_ids = Vec::with_capacity(new_nodes.len());
    for (id, node) in new_nodes {
        nodes.insert(id.clone(), node);
        node_ids.push(id);
    }

    let order = graph_obj
        .entry("order")
        .or_insert_with(|| Value::Array(Vec::new()))
        .as_array_mut()
        .ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "array".into(),
                },
                "graph.order must be an array",
            )
            .at(JsonPathSegment::field("order"))
            .at(JsonPathSegment::field("graph"))
        })?;
    for id in node_ids {
        order.push(Value::String(id));
    }
    Ok(alias_usages)
}

fn detect_axis(
    entry: &Map<String, Value>,
) -> Result<(AliasAxis, &'static str, String), CanonicalizationError> {
    for (key, axis) in [
        ("filter", AliasAxis::Filter),
        ("shader", AliasAxis::Shader),
        ("sampler", AliasAxis::Sampler),
        ("style", AliasAxis::Style),
        ("mask", AliasAxis::Mask),
    ] {
        if let Some(Value::String(name)) = entry.get(key) {
            return Ok((axis, key, name.clone()));
        }
    }
    Err(CanonicalizationError::new(
        CanonicalizationErrorKind::UnexpectedJsonShape {
            expected: "object with one of filter/shader/sampler/style/mask key".into(),
        },
        "could not detect effect axis on entry",
    ))
}

fn allocate_node_id(base: &str, counter: &mut BTreeMap<String, usize>) -> String {
    let next = counter.entry(base.into()).or_insert(0);
    let id = format!("{base}{next}");
    *next += 1;
    id
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn filter_dim_two_entries_become_two_nodes_with_indexed_ids() {
        let mut recipe = json!({
            "id": "x",
            "effects": [
                { "filter": "dim", "factor": 0.3, "phase": ["enter", "dwell"] },
                { "filter": "dim", "factor": 0.5, "phase": "exit", "scope": { "channel": "foreground" } }
            ]
        });
        let usages = lift_effects_to_nodes(&mut recipe).unwrap();
        assert_eq!(usages.len(), 2);
        assert!(usages.contains_key("dim0"));
        assert!(usages.contains_key("dim1"));
        assert!(!recipe.as_object().unwrap().contains_key("effects"));

        let nodes = &recipe["graph"]["nodes"];
        assert_eq!(nodes["dim0"]["effect"], "filter.dim");
        assert_eq!(nodes["dim1"]["effect"], "filter.dim");
        assert_eq!(nodes["dim1"]["writeChannels"], json!(["foreground"]));
        assert_eq!(recipe["graph"]["order"], json!(["dim0", "dim1"]));
    }

    #[test]
    fn unknown_alias_rejects_with_axis_context() {
        let mut recipe = json!({
            "id": "x",
            "effects": [{ "filter": "no_such_thing" }]
        });
        let err = lift_effects_to_nodes(&mut recipe).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::UnknownAlias { ref axis, ref from }
                if axis == "filter" && from == "no_such_thing"
        ));
    }

    #[test]
    fn no_effects_field_is_a_noop() {
        let mut recipe = json!({ "id": "x" });
        let usages = lift_effects_to_nodes(&mut recipe).unwrap();
        assert!(usages.is_empty());
        assert!(!recipe.as_object().unwrap().contains_key("graph"));
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_effects_to_nodes.rs</FILE> - <DESC>Walk effects[] and emit graph.nodes + graph.order</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
