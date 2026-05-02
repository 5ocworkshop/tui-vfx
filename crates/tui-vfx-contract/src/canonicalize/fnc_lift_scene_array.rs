// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_scene_array.rs</FILE> - <DESC>Lift the top-level `scene: [...]` multi-element shorthand into per-element sources + canonical scene elements</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2h of canonicalize: handle multi-element authoring, emitting one SourceSpec + RecipeSceneElement per author entry.</WCTX>
// <CLOG>0.1.0: INIT — basic per-element source + element placement; per-element transitions/animations are runtime-deferred.</CLOG>

use serde_json::{Map, Value, json};

use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
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

        // Per-element animation hooks (`enter`, `exit`, `effects`, `follow`)
        // describe meaningful author intent that the per-element graph-binding
        // canonicalize path does not yet implement. Refuse rather than emit a
        // partial scene that silently drops the animation semantics.
        for animation_key in ["enter", "exit", "dwell", "effects", "follow", "shadow"] {
            if entry.contains_key(animation_key) {
                return Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::UnsupportedShorthand {
                        detail: format!("scene[].{animation_key}"),
                    },
                    format!(
                        "scene[] entry carries `{animation_key}` per-element shorthand. The canonical RecipeSceneElement supports placement_motion / surface / graphBinding for these but the canonicalize pass does not yet emit them; the author intent would be silently lost. Add the per-element graph-binding lift before enabling this recipe."
                    ),
                )
                .at(JsonPathSegment::Index(index))
                .at(JsonPathSegment::field("scene")));
            }
        }

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

        elements.push(build_scene_element(&id, &source_id, entry));
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
    Ok(())
}

fn build_source_for_entry(
    id: &str,
    entry: &Map<String, Value>,
) -> Result<(String, Value), CanonicalizationError> {
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
        let style = match border {
            Value::String(s) => Some(s.clone()),
            Value::Object(obj) => obj
                .get("type")
                .and_then(Value::as_str)
                .map(str::to_string)
                .or_else(|| obj.contains_key("frame").then(|| "custom".into())),
            _ => None,
        };
        if let Some(style) = style {
            inputs.insert("borderStyle".into(), enum_envelope(style));
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

fn build_scene_element(id: &str, source_id: &str, entry: &Map<String, Value>) -> Value {
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

    if let Some(Value::String(anchor)) = entry.get("at")
        && let Some(elem_obj) = element.as_object_mut()
    {
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

    element
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
