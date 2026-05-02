// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_card_to_source.rs</FILE> - <DESC>Lift author-side card block into a canonical source instance</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of canonicalize: lift the top-level card: { ... } shorthand into sources.mainCard with literal envelopes.</WCTX>
// <CLOG>0.1.0: INIT — handle message, size tuple, fg, bg, border (string + object), bold; emit literal envelopes.</CLOG>

use serde_json::{Map, Value, json};

use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
use super::fnc_resolve_color::resolve_color;

/// Promote any author-side `card: { ... }` block on the recipe root into the
/// canonical `sources.mainCard` instance referencing `source.card`.
///
/// Idempotent: when the input already declares a `sources` object, this
/// function returns without changes.
pub fn lift_card_to_source(recipe: &mut Value) -> Result<(), CanonicalizationError> {
    let recipe_obj = match recipe.as_object_mut() {
        Some(obj) => obj,
        None => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "object".into(),
                },
                "recipe root must be a JSON object",
            ));
        }
    };

    if recipe_obj.contains_key("sources") {
        return Ok(());
    }

    let card = match recipe_obj.remove("card") {
        Some(Value::Object(map)) => map,
        Some(other) => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "object".into(),
                },
                format!("card must be an object, got {}", shape_label(&other)),
            )
            .at(JsonPathSegment::field("card")));
        }
        None => return Ok(()),
    };

    let inputs = build_card_inputs(card)?;
    let main_card = json!({
        "sourceDescriptor": "source.card",
        "inputs": inputs,
        "assets": {}
    });
    recipe_obj.insert(
        "sources".into(),
        Value::Object({
            let mut sources = Map::new();
            sources.insert("mainCard".into(), main_card);
            sources
        }),
    );
    Ok(())
}

fn build_card_inputs(card: Map<String, Value>) -> Result<Value, CanonicalizationError> {
    let mut inputs = Map::new();

    if let Some(message) = card.get("message") {
        inputs.insert("message".into(), text_envelope(message)?);
    }

    if let Some(size) = card.get("size") {
        let arr = size.as_array().ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "[width, height]".into(),
                },
                "card.size must be a [width, height] tuple",
            )
            .at(JsonPathSegment::field("card"))
            .at(JsonPathSegment::field("size"))
        })?;
        let width = arr.first().and_then(Value::as_i64).ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "integer".into(),
                },
                "card.size[0] (width) must be an integer",
            )
        })?;
        let height = arr.get(1).and_then(Value::as_i64).ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "integer".into(),
                },
                "card.size[1] (height) must be an integer",
            )
        })?;
        inputs.insert("width".into(), integer_envelope(width));
        inputs.insert("height".into(), integer_envelope(height));
    }

    if let Some(fg) = card.get("fg") {
        let color = resolve_color(fg).map_err(|e| {
            e.at(JsonPathSegment::field("fg"))
                .at(JsonPathSegment::field("card"))
        })?;
        inputs.insert("foreground".into(), color_envelope(color));
    }

    if let Some(bg) = card.get("bg") {
        let color = resolve_color(bg).map_err(|e| {
            e.at(JsonPathSegment::field("bg"))
                .at(JsonPathSegment::field("card"))
        })?;
        inputs.insert("background".into(), color_envelope(color));
    }

    if let Some(border) = card.get("border") {
        let style = border_style_value(border)?;
        inputs.insert("borderStyle".into(), enum_envelope(style));
    }

    if let Some(bold) = card.get("bold").and_then(Value::as_bool) {
        inputs.insert("bold".into(), boolean_envelope(bold));
    }

    Ok(Value::Object(inputs))
}

fn border_style_value(border: &Value) -> Result<String, CanonicalizationError> {
    match border {
        Value::String(s) => Ok(s.clone()),
        Value::Object(obj) => {
            if let Some(t) = obj.get("type").and_then(Value::as_str) {
                Ok(t.to_string())
            } else if obj.contains_key("frame") {
                Ok("custom".into())
            } else {
                Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::UnexpectedJsonShape {
                        expected: "object with `type` or `frame` field".into(),
                    },
                    "card.border object must include `type` or `frame`",
                )
                .at(JsonPathSegment::field("border"))
                .at(JsonPathSegment::field("card")))
            }
        }
        _ => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "string or object".into(),
            },
            "card.border must be a string or { type, ... } object",
        )
        .at(JsonPathSegment::field("border"))
        .at(JsonPathSegment::field("card"))),
    }
}

fn shape_label(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn text_envelope(value: &Value) -> Result<Value, CanonicalizationError> {
    let s = value.as_str().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "string".into(),
            },
            "card.message must be a string",
        )
    })?;
    Ok(json!({ "kind": "literal", "value": { "kind": "text", "value": s } }))
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
    fn lifts_baseline_card_block() {
        let mut recipe = json!({
            "id": "baseline",
            "card": {
                "message": "BASELINE TEST",
                "size": [35, 3],
                "fg": "#ffffff",
                "bg": "#282828",
                "border": "rounded"
            }
        });
        lift_card_to_source(&mut recipe).unwrap();
        let sources = &recipe["sources"]["mainCard"];
        assert_eq!(sources["sourceDescriptor"], "source.card");
        assert_eq!(
            sources["inputs"]["message"],
            json!({ "kind": "literal", "value": { "kind": "text", "value": "BASELINE TEST" } })
        );
        assert_eq!(
            sources["inputs"]["foreground"]["value"]["value"],
            json!({ "r": 255, "g": 255, "b": 255, "a": 255 })
        );
        assert_eq!(
            sources["inputs"]["borderStyle"]["value"],
            json!({ "kind": "enum", "value": "rounded" })
        );
        assert!(!recipe.as_object().unwrap().contains_key("card"));
    }

    #[test]
    fn idempotent_when_sources_present() {
        let mut recipe = json!({
            "sources": { "preExisting": {} },
            "card": { "message": "ignored" }
        });
        lift_card_to_source(&mut recipe).unwrap();
        assert!(
            recipe["sources"]
                .as_object()
                .unwrap()
                .contains_key("preExisting")
        );
        assert!(recipe.as_object().unwrap().contains_key("card"));
    }

    #[test]
    fn no_op_without_card() {
        let mut recipe = json!({ "id": "empty" });
        lift_card_to_source(&mut recipe).unwrap();
        assert!(!recipe.as_object().unwrap().contains_key("sources"));
    }

    #[test]
    fn border_object_form_extracts_type() {
        let mut recipe = json!({
            "card": {
                "message": "x",
                "size": [3, 1],
                "border": { "type": "rounded", "title": "hi" }
            }
        });
        lift_card_to_source(&mut recipe).unwrap();
        assert_eq!(
            recipe["sources"]["mainCard"]["inputs"]["borderStyle"]["value"],
            json!({ "kind": "enum", "value": "rounded" })
        );
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_card_to_source.rs</FILE> - <DESC>Lift author-side card block into a canonical source instance</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
