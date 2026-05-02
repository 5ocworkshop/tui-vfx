// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_scene_array.rs</FILE> - <DESC>Lift the top-level `scene: [...]` multi-element shorthand into per-element sources + canonical scene elements</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Audit follow-up: preserve title/titleAlign/frame fields on per-element scene[] borders so descriptor extras survive canonicalization the same way they do for the root card.</WCTX>
// <CLOG>0.3.0: MINOR — emit borderConfig structured input for object-form scene[] borders so descriptor-defined extras are not silently dropped.</CLOG>

use std::collections::BTreeMap;

use serde_json::{Map, Value, json};

use super::cls_alias_table::AliasEntry;
use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
use super::fnc_apply_alias::apply_alias;
use super::fnc_lift_effects_to_nodes::{allocate_node_id_pub, detect_axis_pub};
use super::fnc_load_tables::alias_table;
use super::fnc_resolve_color::resolve_color;

/// Lift the top-level `scene: [{...}, ...]` shorthand into a single canonical
/// scene with one element per author entry. Each entry produces a
/// `SourceSpec` (under `sources.<id>`) and a `RecipeSceneElement` referring
/// to it. Per-element animation hooks (`enter`, `exit`, `effects`, `follow`)
/// are stripped at the canonical layer since their full handling needs the
/// per-element graph-binding work that is still pending.
pub fn lift_scene_array(recipe: &mut Value) -> Result<(), CanonicalizationError> {
    let recipe_obj = recipe.as_object_mut().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "object".into(),
            },
            "recipe root must be an object",
        )
    })?;

    let scene_entries = match recipe_obj.remove("scene") {
        Some(Value::Array(arr)) => arr,
        Some(other) => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "array".into(),
                },
                format!("scene must be an array of element entries, got {other}"),
            )
            .at(JsonPathSegment::field("scene")));
        }
        None => return Ok(()),
    };

    let scene_size = recipe_obj.remove("size").and_then(|v| match v {
        Value::Array(arr) if arr.len() == 2 => {
            let w = arr[0].as_i64()?;
            let h = arr[1].as_i64()?;
            Some((w, h))
        }
        _ => None,
    });

    let mut sources: Map<String, Value> =
        if let Some(Value::Object(existing)) = recipe_obj.remove("sources") {
            existing
        } else {
            Map::new()
        };
    let mut elements: Vec<Value> = Vec::with_capacity(scene_entries.len());
    let mut graph_nodes: Map<String, Value> = Map::new();
    let mut graph_order: Vec<String> = Vec::new();

    for (index, raw) in scene_entries.into_iter().enumerate() {
        let entry = raw.as_object().ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "object".into(),
                },
                "scene[] entry must be an object",
            )
            .at(JsonPathSegment::Index(index))
            .at(JsonPathSegment::field("scene"))
        })?;

        let id = entry
            .get("id")
            .and_then(Value::as_str)
            .map(str::to_string)
            .unwrap_or_else(|| format!("element_{index}"));

        let (source_id, source_spec) = build_source_for_entry(&id, entry).map_err(|e| {
            e.at(JsonPathSegment::Index(index))
                .at(JsonPathSegment::field("scene"))
        })?;
        sources.insert(source_id.clone(), source_spec);

        // Per-element `effects: [...]` lifts each entry into a graph node
        // prefixed by element id, then synthesizes a graphBinding whose
        // topology is a Sequence of those node references. The recipe.graph
        // accumulates the nodes; multiple elements share the canonical graph
        // but each element's graphBinding selects only its own subset.
        let element_graph_node_ids = if let Some(effects_value) = entry.get("effects") {
            let effects_arr = effects_value.as_array().ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::UnexpectedJsonShape {
                        expected: "array".into(),
                    },
                    "scene[].effects must be an array",
                )
                .at(JsonPathSegment::field("effects"))
                .at(JsonPathSegment::Index(index))
                .at(JsonPathSegment::field("scene"))
            })?;
            let mut id_counter: BTreeMap<String, usize> = BTreeMap::new();
            let mut element_node_ids: Vec<String> = Vec::new();
            for (effect_index, raw) in effects_arr.iter().enumerate() {
                let effect_obj = raw.as_object().ok_or_else(|| {
                    CanonicalizationError::new(
                        CanonicalizationErrorKind::UnexpectedJsonShape {
                            expected: "object".into(),
                        },
                        "scene[].effects entry must be an object",
                    )
                    .at(JsonPathSegment::Index(effect_index))
                    .at(JsonPathSegment::field("effects"))
                    .at(JsonPathSegment::Index(index))
                    .at(JsonPathSegment::field("scene"))
                })?;
                let (axis, axis_key, from) = detect_axis_pub(effect_obj).map_err(|e| {
                    e.at(JsonPathSegment::Index(effect_index))
                        .at(JsonPathSegment::field("effects"))
                        .at(JsonPathSegment::Index(index))
                        .at(JsonPathSegment::field("scene"))
                })?;
                let table = alias_table(axis)?;
                let alias_entry: &AliasEntry = table.find(&from).ok_or_else(|| {
                    CanonicalizationError::new(
                        CanonicalizationErrorKind::UnknownAlias {
                            axis: axis.as_str().into(),
                            from: from.clone(),
                        },
                        format!(
                            "no alias for {} '{from}' in {}/aliases.json",
                            axis.as_str(),
                            axis.as_str()
                        ),
                    )
                })?;
                let scoped_base = format!("{id}_{from}");
                let node_id = effect_obj
                    .get("id")
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .unwrap_or_else(|| allocate_node_id_pub(&scoped_base, &mut id_counter));
                let node_json = apply_alias(alias_entry, axis_key, effect_obj, node_id.clone())?;
                graph_nodes.insert(node_id.clone(), node_json);
                graph_order.push(node_id.clone());
                element_node_ids.push(node_id);
            }
            element_node_ids
        } else {
            Vec::new()
        };

        elements.push(build_scene_element(
            &id,
            &source_id,
            entry,
            &element_graph_node_ids,
        ));
    }

    let (width, height) = scene_size.unwrap_or((0, 0));
    let scene = json!({
        "id": "mainScene",
        "width": width,
        "height": height,
        "elements": elements,
    });
    recipe_obj.insert("scenes".into(), Value::Array(vec![scene]));
    recipe_obj.insert("sources".into(), Value::Object(sources));

    // Merge the per-element graph nodes into recipe.graph so the elements'
    // graphBinding references resolve. Existing graph entries (signals,
    // bindings) carry forward.
    if !graph_nodes.is_empty() {
        let graph = recipe_obj
            .entry("graph")
            .or_insert_with(|| Value::Object(Map::new()));
        if let Some(graph_obj) = graph.as_object_mut() {
            graph_obj
                .entry("id")
                .or_insert_with(|| Value::String("mainGraph".into()));
            graph_obj
                .entry("version")
                .or_insert_with(|| Value::String("3.1".into()));
            let nodes = graph_obj
                .entry("nodes")
                .or_insert_with(|| Value::Object(Map::new()));
            if let Some(nodes_obj) = nodes.as_object_mut() {
                for (id, node) in graph_nodes {
                    nodes_obj.insert(id, node);
                }
            }
            let order = graph_obj
                .entry("order")
                .or_insert_with(|| Value::Array(Vec::new()));
            if let Some(order_arr) = order.as_array_mut() {
                for id in graph_order {
                    order_arr.push(Value::String(id));
                }
            }
        }
    }
    Ok(())
}

fn build_source_for_entry(
    id: &str,
    entry: &Map<String, Value>,
) -> Result<(String, Value), CanonicalizationError> {
    if let Some(procedural) = entry.get("procedural") {
        return build_procedural_source(id, procedural);
    }

    let mut inputs = Map::new();

    if let Some(card) = entry.get("card") {
        match card {
            Value::String(s) => insert_text_input(&mut inputs, "message", s),
            Value::Object(obj) => {
                if let Some(message) = obj.get("message").and_then(Value::as_str) {
                    insert_text_input(&mut inputs, "message", message);
                }
            }
            _ => {}
        }
    } else if let Some(text) = entry.get("text").and_then(Value::as_str) {
        insert_text_input(&mut inputs, "message", text);
    }

    if let Some(size) = entry.get("size").and_then(Value::as_array)
        && size.len() == 2
        && let (Some(w), Some(h)) = (size[0].as_i64(), size[1].as_i64())
    {
        inputs.insert("width".into(), integer_envelope(w));
        inputs.insert("height".into(), integer_envelope(h));
    }

    if let Some(fg) = entry.get("fg") {
        let color = resolve_color(fg).map_err(|e| e.at(JsonPathSegment::field("fg")))?;
        inputs.insert("foreground".into(), color_envelope(color));
    }
    if let Some(bg) = entry.get("bg") {
        let color = resolve_color(bg).map_err(|e| e.at(JsonPathSegment::field("bg")))?;
        inputs.insert("background".into(), color_envelope(color));
    }
    if let Some(bold) = entry.get("bold").and_then(Value::as_bool) {
        inputs.insert("bold".into(), boolean_envelope(bold));
    }
    if let Some(border) = entry.get("border") {
        let (style, config) = match border {
            Value::String(s) => (Some(s.clone()), None),
            Value::Object(obj) => {
                let style = obj
                    .get("type")
                    .and_then(Value::as_str)
                    .map(str::to_string)
                    .or_else(|| obj.contains_key("frame").then(|| "custom".into()));
                let mut extras = Map::new();
                for (k, v) in obj {
                    if k != "type" {
                        extras.insert(k.clone(), v.clone());
                    }
                }
                let extras_value = if extras.is_empty() {
                    None
                } else {
                    Some(Value::Object(extras))
                };
                (style, extras_value)
            }
            _ => (None, None),
        };
        if let Some(style) = style {
            inputs.insert("borderStyle".into(), enum_envelope(style));
        }
        if let Some(config) = config {
            inputs.insert(
                "borderConfig".into(),
                json!({
                    "kind": "literal",
                    "value": { "kind": "structured", "value": config }
                }),
            );
        }
    }

    let descriptor = if entry.contains_key("card") {
        "source.card"
    } else if entry.contains_key("text") {
        "source.text"
    } else {
        "source.card"
    };

    let source_spec = json!({
        "sourceDescriptor": descriptor,
        "inputs": inputs,
        "assets": {}
    });
    Ok((id.into(), source_spec))
}

/// Translate the author-side `procedural: { id, size, params }` shorthand
/// into a SourceSpec referring to `source.procedural`. The descriptor id and
/// params block ride through as canonical literal envelopes; structural
/// params land in the lossless `kind: "structured"` Value variant.
fn build_procedural_source(
    id: &str,
    procedural: &Value,
) -> Result<(String, Value), CanonicalizationError> {
    let obj = procedural.as_object().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "procedural object".into(),
            },
            "scene[].procedural must be an object",
        )
        .at(JsonPathSegment::field("procedural"))
    })?;

    let mut inputs = Map::new();
    if let Some(descriptor_id) = obj.get("id").and_then(Value::as_str) {
        insert_text_input(&mut inputs, "descriptorId", descriptor_id);
    }
    if let Some(size) = obj.get("size").and_then(Value::as_array)
        && size.len() == 2
        && let (Some(w), Some(h)) = (size[0].as_i64(), size[1].as_i64())
    {
        inputs.insert("width".into(), integer_envelope(w));
        inputs.insert("height".into(), integer_envelope(h));
    }
    if let Some(params) = obj.get("params") {
        inputs.insert(
            "params".into(),
            json!({
                "kind": "literal",
                "value": { "kind": "structured", "value": params.clone() }
            }),
        );
    }

    let source_spec = json!({
        "sourceDescriptor": "source.procedural",
        "inputs": inputs,
        "assets": {}
    });
    Ok((id.into(), source_spec))
}

fn build_scene_element(
    id: &str,
    source_id: &str,
    entry: &Map<String, Value>,
    element_graph_node_ids: &[String],
) -> Value {
    let placement = match entry.get("at") {
        Some(Value::Array(arr)) if arr.len() == 2 => json!({
            "x": arr[0].as_i64().unwrap_or(0),
            "y": arr[1].as_i64().unwrap_or(0),
        }),
        _ => json!({ "x": 0, "y": 0 }),
    };

    let mut element = json!({
        "id": id,
        "layer": "primary",
        "zIndex": 0,
        "placement": placement,
        "sourceInstance": source_id,
        "clipPolicy": "clip",
        "cellWritePolicy": "writeCell",
        "roleWritePolicy": { "kind": "preserveDestination" },
    });

    let elem_obj = element
        .as_object_mut()
        .expect("element built as object literal");

    if let Some(Value::String(anchor)) = entry.get("at") {
        elem_obj.insert(
            "placementRule".into(),
            json!({
                "kind": "anchor",
                "anchor": anchor,
                "offsetRows": 0,
                "offsetColumns": 0,
            }),
        );
    }

    // Per-element motion hooks (`enter`, `exit`, `dwell`, `follow`) carry
    // author intent the runtime needs to schedule animations for the element.
    // The canonical RecipeSceneElement.placement_motion is StructuredValue —
    // a JSON-compatible AST — so we can preserve the entire author block
    // losslessly under one `motion` object keyed by phase + follow.
    let motion_keys = ["enter", "exit", "dwell", "follow"];
    let mut motion = serde_json::Map::new();
    for key in motion_keys {
        if let Some(value) = entry.get(key) {
            motion.insert(key.into(), value.clone());
        }
    }
    if !motion.is_empty() {
        elem_obj.insert("placementMotion".into(), Value::Object(motion));
    }

    // Per-element `shadow:` lifts to element.surface.shadow with the
    // edgeCrossing → edgeCrossingPolicy field rename.
    if let Some(shadow) = entry.get("shadow") {
        let canonical_shadow = canonical_shadow_spec(shadow);
        elem_obj.insert("surface".into(), json!({ "shadow": canonical_shadow }));
    }

    // Per-element `effects:` produced graph nodes prefixed by element id;
    // bind those nodes to this element via a Sequence topology so the runtime
    // executes the element's effects in order without seeing other elements'
    // nodes.
    if !element_graph_node_ids.is_empty() {
        let topology_children: Vec<Value> = element_graph_node_ids
            .iter()
            .map(|node_id| json!({ "kind": "node", "node": node_id }))
            .collect();
        let topology = if topology_children.len() == 1 {
            topology_children.into_iter().next().unwrap()
        } else {
            json!({ "kind": "sequence", "children": topology_children })
        };
        elem_obj.insert(
            "graphBinding".into(),
            json!({ "graph": "mainGraph", "topology": topology }),
        );
    }

    element
}

/// Translate author-side shadow shorthand to the canonical ShadowSpec field
/// names: `color` → `shadowColor`, `edgeCrossing` → `edgeCrossingPolicy`,
/// `edges` author tuple → ShadowEdge[]. Other fields pass through.
fn canonical_shadow_spec(value: &Value) -> Value {
    let Value::Object(obj) = value else {
        return value.clone();
    };
    let mut renamed = serde_json::Map::new();
    for (key, v) in obj {
        let (canonical_key, canonical_value) = match key.as_str() {
            "color" => ("shadowColor", canonical_shadow_color(v)),
            "edgeCrossing" => ("edgeCrossingPolicy", v.clone()),
            "soft" => ("softEdges", v.clone()),
            "source" => ("sourceRegion", canonical_shadow_source(v)),
            "offset" => ("offset", canonical_shadow_offset(v)),
            "edges" => ("edges", canonical_shadow_edges(v)),
            "style" => continue, // author-side rendering hint without a canonical field
            other => (other, v.clone()),
        };
        renamed.insert(canonical_key.into(), canonical_value);
    }
    // ShadowSpec.compositeMode and blendMode are required. Fill with sensible
    // defaults when the author hasn't specified them. ShadowEdgeCrossingPolicy
    // is optional and stays absent.
    renamed
        .entry("compositeMode")
        .or_insert_with(|| Value::String("under".into()));
    renamed
        .entry("blendMode")
        .or_insert_with(|| Value::String("sourceOver".into()));
    Value::Object(renamed)
}

/// Translate `offset: [x, y]` author tuple to canonical `{ x, y }` object.
fn canonical_shadow_offset(value: &Value) -> Value {
    match value {
        Value::Array(arr) if arr.len() == 2 => json!({
            "x": arr[0].as_i64().unwrap_or(0),
            "y": arr[1].as_i64().unwrap_or(0),
        }),
        _ => value.clone(),
    }
}

/// Translate `edges: ["right", "bottom"]` author array of role names to the
/// canonical `[{ kind: "right" }, ...]` ShadowEdge[]. ShadowEdge is internally
/// tagged with `kind`.
fn canonical_shadow_edges(value: &Value) -> Value {
    let Value::Array(arr) = value else {
        return value.clone();
    };
    Value::Array(
        arr.iter()
            .map(|entry| match entry {
                Value::String(s) => Value::String(s.clone()),
                _ => entry.clone(),
            })
            .collect(),
    )
}

/// Translate the author-side `shadow.source` shorthand to a canonical
/// ScopeSpec. A bare string names a role (`"border"` → `{ kind: "role", role:
/// "Border" }`); object form passes through assuming it's already canonical.
fn canonical_shadow_source(value: &Value) -> Value {
    match value {
        Value::String(s) => {
            let mut chars = s.chars();
            let pascal: String = match chars.next() {
                Some(c) => c.to_uppercase().chain(chars).collect(),
                None => String::new(),
            };
            json!({ "kind": "role", "role": pascal })
        }
        _ => value.clone(),
    }
}

/// Resolve shadow color shorthand (hex string or tuple) into the canonical
/// `{ r, g, b, a }` object that ShadowSpec.shadowColor accepts. Falls back to
/// passthrough when the value is already an object.
fn canonical_shadow_color(value: &Value) -> Value {
    resolve_color(value).unwrap_or_else(|_| value.clone())
}

fn insert_text_input(inputs: &mut Map<String, Value>, key: &str, s: &str) {
    inputs.insert(
        key.into(),
        json!({ "kind": "literal", "value": { "kind": "text", "value": s } }),
    );
}

fn integer_envelope(n: i64) -> Value {
    json!({ "kind": "literal", "value": { "kind": "integer", "value": n } })
}

fn color_envelope(color: Value) -> Value {
    json!({ "kind": "literal", "value": { "kind": "color", "value": color } })
}

fn enum_envelope(name: String) -> Value {
    json!({ "kind": "literal", "value": { "kind": "enum", "value": name } })
}

fn boolean_envelope(b: bool) -> Value {
    json!({ "kind": "literal", "value": { "kind": "boolean", "value": b } })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_element_scene_emits_two_sources_and_two_elements() {
        let mut recipe = json!({
            "id": "x",
            "size": [32, 8],
            "scene": [
                { "id": "a", "text": "hello", "fg": "white", "at": [0, 0] },
                { "id": "b", "text": "world", "fg": [200, 200, 200], "at": [10, 4] }
            ]
        });
        lift_scene_array(&mut recipe).unwrap();
        let scene = &recipe["scenes"][0];
        assert_eq!(scene["width"], 32);
        assert_eq!(scene["height"], 8);
        let elements = scene["elements"].as_array().unwrap();
        assert_eq!(elements.len(), 2);
        assert_eq!(elements[0]["sourceInstance"], "a");
        assert_eq!(elements[1]["placement"], json!({ "x": 10, "y": 4 }));
        assert!(recipe["sources"]["a"].is_object());
        assert!(recipe["sources"]["b"].is_object());
    }

    #[test]
    fn no_scene_array_is_a_noop() {
        let mut recipe = json!({ "id": "x" });
        lift_scene_array(&mut recipe).unwrap();
        assert!(!recipe.as_object().unwrap().contains_key("scenes"));
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_scene_array.rs</FILE> - <DESC>Lift the top-level `scene: [...]` multi-element shorthand into per-element sources + canonical scene elements</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
