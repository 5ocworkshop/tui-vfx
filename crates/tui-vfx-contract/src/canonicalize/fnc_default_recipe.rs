// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_default_recipe.rs</FILE> - <DESC>Apply structural defaults to fill the recipe-document required fields</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of canonicalize: fill version, metadata, lifecycle, graph, scenes, descriptorPacks defaults so the JSON deserializes as RecipeDocument.</WCTX>
// <CLOG>0.1.0: INIT — emit defaults for the bare-minimum baseline recipe shape.</CLOG>

use serde_json::{Map, Value, json};

use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};

/// Fill in defaults required for the JSON tree to deserialize as a canonical
/// [`RecipeDocument`]. Idempotent — already-populated fields are left alone.
///
/// [`RecipeDocument`]: crate::RecipeDocument
pub fn apply_recipe_defaults(recipe: &mut Value) -> Result<(), CanonicalizationError> {
    let obj = recipe.as_object_mut().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "object".into(),
            },
            "recipe root must be a JSON object",
        )
    })?;

    obj.entry("version")
        .or_insert_with(|| Value::String("3.1".into()));

    fill_metadata(obj);
    fill_lifecycle(obj);

    obj.entry("assets")
        .or_insert_with(|| Value::Object(Map::new()));
    obj.entry("sourceDescriptors")
        .or_insert_with(|| Value::Object(Map::new()));
    obj.entry("sources")
        .or_insert_with(|| Value::Object(Map::new()));
    obj.entry("graph").or_insert_with(default_graph);
    obj.entry("descriptorPacks")
        .or_insert_with(default_descriptor_packs);

    let scene_dimensions = scene_dimensions_for_default(obj);
    obj.entry("scenes")
        .or_insert_with(|| default_scenes(scene_dimensions));

    if !obj.contains_key("id") {
        return Err(CanonicalizationError::new(
            CanonicalizationErrorKind::MissingRequired { field: "id".into() },
            "recipe root must declare `id`",
        )
        .at(JsonPathSegment::field("id")));
    }
    Ok(())
}

fn fill_metadata(obj: &mut Map<String, Value>) {
    let title = obj.remove("title");
    let description = obj.remove("description");
    let authors = obj.remove("authors");
    let tags = obj.remove("tags");
    let expected_visual = obj.remove("expectedVisual");

    let metadata = obj
        .entry("metadata")
        .or_insert_with(|| Value::Object(Map::new()));
    if let Value::Object(meta) = metadata {
        if let Some(t) = title {
            meta.entry("title").or_insert(t);
        }
        if let Some(d) = description {
            meta.entry("description").or_insert(d);
        }
        if let Some(a) = authors {
            meta.entry("authors").or_insert(a);
        }
        if let Some(t) = tags {
            meta.entry("tags").or_insert(t);
        }
        if let Some(v) = expected_visual {
            meta.entry("expectedVisual").or_insert(v);
        }
        meta.entry("title").or_insert(Value::Null);
        meta.entry("description").or_insert(Value::Null);
    }
}

fn fill_lifecycle(obj: &mut Map<String, Value>) {
    obj.entry("lifecycle").or_insert_with(default_lifecycle);
}

fn default_lifecycle() -> Value {
    json!({
        "clock": { "clockMode": "monotonic", "period": null },
        "phases": [
            { "phase": "enter", "timing": { "kind": "fixed", "duration": { "kind": "milliseconds", "value": 1000 } } },
            { "phase": "dwell", "timing": { "kind": "dwell", "policy": { "kind": "fixed", "duration": { "kind": "milliseconds", "value": 5000 } } } },
            { "phase": "exit", "timing": { "kind": "fixed", "duration": { "kind": "milliseconds", "value": 1000 } } }
        ]
    })
}

fn default_graph() -> Value {
    json!({ "id": "mainGraph", "version": "3.1" })
}

fn default_descriptor_packs() -> Value {
    json!([{ "id": "v3.1.primitive" }])
}

/// Pull width / height for the default scene from `mainCard` if present.
fn scene_dimensions_for_default(obj: &Map<String, Value>) -> (i64, i64) {
    let inputs = obj
        .get("sources")
        .and_then(Value::as_object)
        .and_then(|s| s.get("mainCard"))
        .and_then(Value::as_object)
        .and_then(|c| c.get("inputs"))
        .and_then(Value::as_object);
    let pull = |key: &str| -> Option<i64> {
        inputs?
            .get(key)?
            .pointer("/value/value")
            .and_then(Value::as_i64)
    };
    let width = pull("width").unwrap_or(0);
    let height = pull("height").unwrap_or(0);
    (width, height)
}

fn default_scenes(dims: (i64, i64)) -> Value {
    let (width, height) = dims;
    json!([
        {
            "id": "mainScene",
            "width": width,
            "height": height,
            "elements": [
                {
                    "id": "mainElement",
                    "layer": "primary",
                    "zIndex": 0,
                    "placement": { "x": 0, "y": 0 },
                    "sourceInstance": "mainCard",
                    "clipPolicy": "clip",
                    "cellWritePolicy": "writeCell",
                    "roleWritePolicy": { "kind": "preserveDestination" }
                }
            ]
        }
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_baseline_defaults() {
        let mut recipe = json!({
            "id": "baseline",
            "title": "Baseline",
            "sources": {
                "mainCard": {
                    "source": "source.card",
                    "inputs": {
                        "width":  { "kind": "literal", "value": { "kind": "integer", "value": 35 } },
                        "height": { "kind": "literal", "value": { "kind": "integer", "value": 3 } }
                    },
                    "assets": {}
                }
            }
        });
        apply_recipe_defaults(&mut recipe).unwrap();
        assert_eq!(recipe["version"], "3.1");
        assert_eq!(recipe["metadata"]["title"], "Baseline");
        assert_eq!(recipe["graph"]["id"], "mainGraph");
        assert_eq!(recipe["scenes"][0]["width"], 35);
        assert_eq!(recipe["scenes"][0]["height"], 3);
        assert_eq!(recipe["descriptorPacks"][0]["id"], "v3.1.primitive");
    }

    #[test]
    fn missing_id_rejects() {
        let mut recipe = json!({});
        let err = apply_recipe_defaults(&mut recipe).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::MissingRequired { .. }
        ));
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_default_recipe.rs</FILE> - <DESC>Apply structural defaults to fill the recipe-document required fields</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
