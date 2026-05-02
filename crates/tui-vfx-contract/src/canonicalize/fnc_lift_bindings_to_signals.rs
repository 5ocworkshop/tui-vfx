// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_bindings_to_signals.rs</FILE> - <DESC>Lift the author-side bindings block into graph.signals</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2c of canonicalize: turn bindings shorthand (type/range/default/loopback) into canonical SignalSpec values.</WCTX>
// <CLOG>0.1.0: INIT — handle number/integer/boolean/text bindings, range tuples, defaults, and ramp loopbacks.</CLOG>

use serde_json::{Map, Value, json};

use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
use super::fnc_resolve_duration::resolve_duration;

/// Lift the top-level `bindings: { … }` block into canonical
/// `graph.signals: { id: SignalSpec }`.
///
/// Author-side schema:
/// ```json
/// "bindings": {
///   "selectedRow": { "type": "number", "range": [0, 9], "default": 4 },
///   "ready":       { "type": "boolean", "default": false }
/// }
/// ```
pub fn lift_bindings_to_signals(recipe: &mut Value) -> Result<(), CanonicalizationError> {
    let recipe_obj = recipe.as_object_mut().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "object".into(),
            },
            "recipe root must be an object",
        )
    })?;

    let raw = match recipe_obj.remove("bindings") {
        Some(Value::Object(map)) => map,
        Some(other) => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "object".into(),
                },
                format!("bindings must be an object, got {other}"),
            )
            .at(JsonPathSegment::field("bindings")));
        }
        None => return Ok(()),
    };

    if raw.is_empty() {
        return Ok(());
    }

    let mut signals = Map::new();
    for (id, entry) in raw {
        let signal = build_signal(&id, entry).map_err(|e| {
            e.at(JsonPathSegment::field(id.clone()))
                .at(JsonPathSegment::field("bindings"))
        })?;
        signals.insert(id, signal);
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
    graph_obj.insert("signals".into(), Value::Object(signals));
    Ok(())
}

fn build_signal(id: &str, entry: Value) -> Result<Value, CanonicalizationError> {
    let entry_obj = match entry {
        Value::Object(map) => map,
        other => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "object".into(),
                },
                format!("binding entry must be an object, got {other}"),
            ));
        }
    };

    let kind = entry_obj
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::MissingRequired {
                    field: "type".into(),
                },
                "binding entry must declare `type`",
            )
        })?;
    let canonical_kind = canonical_value_kind(kind)?;

    let default = match entry_obj.get("default") {
        Some(raw) => Some(
            typed_value(canonical_kind, raw)
                .map_err(|e| e.at(JsonPathSegment::field("default")))?,
        ),
        None => None,
    };

    let range = entry_obj
        .get("range")
        .map(typed_range)
        .transpose()
        .map_err(|e| e.at(JsonPathSegment::field("range")))?;

    let mut value_spec = Map::new();
    value_spec.insert("kind".into(), Value::String(canonical_kind.into()));
    value_spec.insert("default".into(), default.unwrap_or(Value::Null));
    value_spec.insert("range".into(), range.unwrap_or(Value::Null));
    if let Some(allowed) = entry_obj.get("allowedValues") {
        value_spec.insert("allowedValues".into(), allowed.clone());
    }
    value_spec.insert(
        "unit".into(),
        entry_obj.get("unit").cloned().unwrap_or(Value::Null),
    );
    value_spec.insert(
        "semantic".into(),
        entry_obj.get("semantic").cloned().unwrap_or(Value::Null),
    );

    let preview_loopback = match entry_obj.get("loopback") {
        Some(raw) => {
            Some(build_loopback(raw).map_err(|e| e.at(JsonPathSegment::field("loopback")))?)
        }
        None => None,
    };

    let mut signal = Map::new();
    signal.insert("id".into(), Value::String(id.into()));
    signal.insert(
        "displayName".into(),
        entry_obj.get("displayName").cloned().unwrap_or(Value::Null),
    );
    signal.insert(
        "description".into(),
        entry_obj.get("description").cloned().unwrap_or(Value::Null),
    );
    signal.insert("value".into(), Value::Object(value_spec));
    if let Some(loopback) = preview_loopback {
        signal.insert("previewLoopback".into(), loopback);
    }
    signal.insert(
        "required".into(),
        entry_obj
            .get("required")
            .and_then(Value::as_bool)
            .map(Value::Bool)
            .unwrap_or(Value::Bool(false)),
    );
    Ok(Value::Object(signal))
}

fn canonical_value_kind(author_kind: &str) -> Result<&'static str, CanonicalizationError> {
    Ok(match author_kind {
        "boolean" | "bool" => "boolean",
        "integer" | "int" | "i32" | "i64" | "u16" | "u32" | "u64" | "usize" => "integer",
        "number" | "float" | "f32" | "f64" => "number",
        "text" | "string" | "str" => "text",
        "color" => "color",
        "duration" => "duration",
        other => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "binding type (boolean/integer/number/text/color/duration)".into(),
                },
                format!("unknown binding type: {other}"),
            ));
        }
    })
}

fn typed_value(kind: &str, raw: &Value) -> Result<Value, CanonicalizationError> {
    Ok(match kind {
        "boolean" => match raw.as_bool() {
            Some(b) => json!({ "kind": "boolean", "value": b }),
            None => return mismatch(raw, "boolean"),
        },
        "integer" => match raw.as_i64() {
            Some(n) => json!({ "kind": "integer", "value": n }),
            None => return mismatch(raw, "integer"),
        },
        "number" => match raw.as_f64() {
            Some(n) => json!({ "kind": "number", "value": n }),
            None => return mismatch(raw, "number"),
        },
        "text" => match raw.as_str() {
            Some(s) => json!({ "kind": "text", "value": s }),
            None => return mismatch(raw, "text"),
        },
        "color" => {
            json!({ "kind": "color", "value": super::fnc_resolve_color::resolve_color(raw)? })
        }
        "duration" => json!({ "kind": "duration", "value": resolve_duration(raw)? }),
        _ => unreachable!("kind already validated"),
    })
}

fn typed_range(raw: &Value) -> Result<Value, CanonicalizationError> {
    match raw {
        Value::Array(arr) if arr.len() == 2 => {
            let min = arr[0].as_f64().ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::UnexpectedJsonShape {
                        expected: "[min, max] numeric tuple".into(),
                    },
                    "range tuple element must be numeric",
                )
            })?;
            let max = arr[1].as_f64().ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::UnexpectedJsonShape {
                        expected: "[min, max] numeric tuple".into(),
                    },
                    "range tuple element must be numeric",
                )
            })?;
            Ok(json!({ "min": min, "max": max }))
        }
        Value::Object(_) => Ok(raw.clone()),
        _ => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "[min, max] tuple or {min,max} object".into(),
            },
            "range must be a 2-element tuple or { min, max } object",
        )),
    }
}

fn build_loopback(raw: &Value) -> Result<Value, CanonicalizationError> {
    match raw {
        Value::Object(obj) if obj.get("type").and_then(Value::as_str) == Some("ramp") => {
            let start = obj.get("start").and_then(Value::as_f64).ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::MissingRequired {
                        field: "start".into(),
                    },
                    "ramp loopback requires `start`",
                )
            })?;
            let end = obj.get("end").and_then(Value::as_f64).ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::MissingRequired {
                        field: "end".into(),
                    },
                    "ramp loopback requires `end`",
                )
            })?;
            let duration = obj.get("duration").ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::MissingRequired {
                        field: "duration".into(),
                    },
                    "ramp loopback requires `duration`",
                )
            })?;
            let resolved_duration = resolve_duration(duration)?;
            let repeat = obj.get("repeat").and_then(Value::as_bool).unwrap_or(true);
            Ok(json!({
                "kind": "numericRamp",
                "start": start,
                "end": end,
                "duration": resolved_duration,
                "repeat": repeat,
            }))
        }
        Value::Number(n) => {
            Ok(json!({ "kind": "numericStatic", "value": n.as_f64().unwrap_or(0.0) }))
        }
        Value::Object(obj) => {
            // Already-canonical PreviewLoopbackSpec passes through; otherwise
            // wrap the author-side `type:`-tagged signal expression
            // (perlin/keyframes/spatial_noise/adsr/clamp/add/multiply) as the
            // lossless `signalExpression` escape hatch the contract provides
            // for non-canonical authoring vocabularies.
            if obj.get("kind").and_then(Value::as_str).is_some_and(|k| {
                k.starts_with("numeric") || k == "literal" || k == "signalExpression"
            }) {
                Ok(raw.clone())
            } else {
                Ok(json!({
                    "kind": "signalExpression",
                    "expression": raw.clone(),
                    "fallback": Value::Null,
                }))
            }
        }
        _ => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "loopback object or numeric literal".into(),
            },
            "loopback must be an object or numeric literal",
        )),
    }
}

fn mismatch(raw: &Value, expected: &str) -> Result<Value, CanonicalizationError> {
    Err(CanonicalizationError::new(
        CanonicalizationErrorKind::UnexpectedJsonShape {
            expected: expected.into(),
        },
        format!("expected {expected}, got {raw}"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_with_range_and_default_lifts_to_signal_spec() {
        let mut recipe = json!({
            "id": "x",
            "bindings": {
                "selectedRow": { "type": "number", "range": [0, 9], "default": 4 }
            }
        });
        lift_bindings_to_signals(&mut recipe).unwrap();
        let signal = &recipe["graph"]["signals"]["selectedRow"];
        assert_eq!(signal["id"], "selectedRow");
        assert_eq!(signal["value"]["kind"], "number");
        assert_eq!(
            signal["value"]["default"],
            json!({ "kind": "number", "value": 4.0 })
        );
        assert_eq!(signal["value"]["range"], json!({ "min": 0.0, "max": 9.0 }));
        assert_eq!(signal["required"], false);
    }

    #[test]
    fn boolean_default_lifts_correctly() {
        let mut recipe = json!({
            "id": "x",
            "bindings": { "ready": { "type": "boolean", "default": true } }
        });
        lift_bindings_to_signals(&mut recipe).unwrap();
        assert_eq!(
            recipe["graph"]["signals"]["ready"]["value"]["default"],
            json!({ "kind": "boolean", "value": true })
        );
    }

    #[test]
    fn ramp_loopback_lifts_to_numeric_ramp() {
        let mut recipe = json!({
            "id": "x",
            "bindings": {
                "active": {
                    "type": "number",
                    "range": [0, 4],
                    "loopback": { "type": "ramp", "start": 0, "end": 4, "duration": "4s" }
                }
            }
        });
        lift_bindings_to_signals(&mut recipe).unwrap();
        let loopback = &recipe["graph"]["signals"]["active"]["previewLoopback"];
        assert_eq!(loopback["kind"], "numericRamp");
        assert_eq!(loopback["start"], 0.0);
        assert_eq!(loopback["end"], 4.0);
        assert_eq!(
            loopback["duration"],
            json!({ "kind": "seconds", "value": 4 })
        );
        assert_eq!(loopback["repeat"], true);
    }

    #[test]
    fn unknown_type_rejects() {
        let mut recipe = json!({
            "id": "x",
            "bindings": { "foo": { "type": "wat" } }
        });
        let err = lift_bindings_to_signals(&mut recipe).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::UnexpectedJsonShape { .. }
        ));
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_bindings_to_signals.rs</FILE> - <DESC>Lift the author-side bindings block into graph.signals</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
