// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_value_envelope.rs</FILE> - <DESC>Lift author-side scalars into canonical { kind: "literal", value: { kind, value } } envelopes</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of canonicalize: bare values from author-side params lift into the canonical ValueSource shape.</WCTX>
// <CLOG>0.1.0: INIT — type-infer integer/number/boolean/text and accept named-envelope hints.</CLOG>

use serde_json::{Value, json};

use super::cls_canonicalization_error::{CanonicalizationError, CanonicalizationErrorKind};
use super::fnc_resolve_color::resolve_color;
use super::fnc_resolve_duration::resolve_duration;

/// Wrap an author-side bare scalar in the canonical `{ kind: "literal", value: { kind, value } }`
/// envelope. The `hint` selects how to interpret strings and arrays; pass [`EnvelopeHint::None`]
/// to type-infer from the JSON shape.
pub fn lift_value_envelope(
    value: &Value,
    hint: EnvelopeHint,
) -> Result<Value, CanonicalizationError> {
    if let Value::Object(obj) = value
        && obj.contains_key("kind")
    {
        return Ok(value.clone());
    }

    if let Value::String(s) = value
        && let Some(rest) = s.strip_prefix("$bind:")
    {
        return Ok(resolve_binding_string(rest));
    }

    // EnvelopeHint::None on a complex structural value (object without kind, or
    // an array that isn't a color tuple) passes through unwrapped. Shader
    // inputs like `paths`, `stops`, `nodes`, `pattern`, and `signal` are not
    // ValueSource positions — they're structural arguments the descriptor
    // deserializes directly.
    if matches!(hint, EnvelopeHint::None) {
        match value {
            Value::Object(_) => return Ok(value.clone()),
            Value::Array(arr) => {
                let is_color_tuple = (arr.len() == 3 || arr.len() == 4)
                    && arr
                        .iter()
                        .all(|v| v.is_u64() || v.is_i64() || v.is_number());
                if !is_color_tuple {
                    return Ok(value.clone());
                }
            }
            _ => {}
        }
    }

    let inner = match hint {
        EnvelopeHint::None => infer_literal(value)?,
        EnvelopeHint::Integer => match value.as_i64() {
            Some(n) => json!({ "kind": "integer", "value": n }),
            None => return mismatched(value, "integer"),
        },
        EnvelopeHint::Number => match value.as_f64() {
            Some(n) => json!({ "kind": "number", "value": n }),
            None => return mismatched(value, "number"),
        },
        EnvelopeHint::Boolean => match value.as_bool() {
            Some(b) => json!({ "kind": "boolean", "value": b }),
            None => return mismatched(value, "boolean"),
        },
        EnvelopeHint::Text => match value.as_str() {
            Some(s) => json!({ "kind": "text", "value": s }),
            None => return mismatched(value, "string"),
        },
        EnvelopeHint::EnumLiteral => match value.as_str() {
            Some(s) => json!({ "kind": "enum", "value": s }),
            None => return mismatched(value, "enum string"),
        },
        EnvelopeHint::Color => json!({ "kind": "color", "value": resolve_color(value)? }),
        EnvelopeHint::Duration => return resolve_duration(value),
    };
    Ok(json!({ "kind": "literal", "value": inner }))
}

fn resolve_binding_string(rest: &str) -> Value {
    let (id, fallback) = match rest.split_once('?') {
        Some((id, fb)) => (id.to_string(), Some(fb.to_string())),
        None => (rest.to_string(), None),
    };
    let mut obj = json!({ "kind": "signal", "id": id });
    if let Some(fb) = fallback {
        if let Ok(n) = fb.parse::<i64>() {
            obj["fallback"] = json!({ "kind": "integer", "value": n });
        } else if let Ok(n) = fb.parse::<f64>() {
            obj["fallback"] = json!({ "kind": "number", "value": n });
        } else {
            obj["fallback"] = json!({ "kind": "text", "value": fb });
        }
    }
    obj
}

fn infer_literal(value: &Value) -> Result<Value, CanonicalizationError> {
    match value {
        Value::Bool(b) => Ok(json!({ "kind": "boolean", "value": b })),
        Value::Number(n) => {
            if let Some(i) = n.as_i64()
                && !n.to_string().contains('.')
            {
                return Ok(json!({ "kind": "integer", "value": i }));
            }
            n.as_f64()
                .map(|f| json!({ "kind": "number", "value": f }))
                .ok_or_else(|| {
                    CanonicalizationError::new(
                        CanonicalizationErrorKind::EnvelopeLiftFailed,
                        "numeric value did not fit i64 or f64",
                    )
                })
        }
        Value::String(s) => Ok(json!({ "kind": "text", "value": s })),
        Value::Array(arr) if arr.len() == 3 || arr.len() == 4 => {
            // Heuristic: 3- or 4-element numeric array → color tuple.
            let all_numeric = arr
                .iter()
                .all(|v| v.is_u64() || v.is_i64() || v.is_number());
            if all_numeric {
                Ok(json!({ "kind": "color", "value": resolve_color(value)? }))
            } else {
                // Complex array (e.g., gradient stops `[[0, [r,g,b]], …]`,
                // wayfinding nodes `[[x, y], …]`) — pass through unwrapped.
                // The canonical input descriptor decides how to deserialize.
                Ok(value.clone())
            }
        }
        Value::Array(_) | Value::Object(_) => {
            // Structural input (pattern, paths, stops, signal expression).
            // Pass through unwrapped; canonical type-check at the end of
            // canonicalize will reject genuinely wrong shapes.
            Ok(value.clone())
        }
        Value::Null => Ok(json!({ "kind": "null" })),
    }
}

fn mismatched(value: &Value, expected: &str) -> Result<Value, CanonicalizationError> {
    Err(CanonicalizationError::new(
        CanonicalizationErrorKind::UnexpectedJsonShape {
            expected: expected.into(),
        },
        format!("expected {expected}, got {value}"),
    ))
}

/// Hint describing how an author-side scalar should be interpreted when wrapped
/// in a canonical literal envelope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnvelopeHint {
    None,
    Integer,
    Number,
    Boolean,
    Text,
    EnumLiteral,
    Color,
    Duration,
}

impl EnvelopeHint {
    /// Parse the alias-table envelope name (e.g., `"literal-integer"`) into a hint.
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "literal-integer" => Self::Integer,
            "literal-number" => Self::Number,
            "literal-boolean" => Self::Boolean,
            "literal-text" => Self::Text,
            "literal-enum" => Self::EnumLiteral,
            "literal-color" => Self::Color,
            "literal-duration" => Self::Duration,
            "value-source" | "passthrough" => Self::None,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_lift() {
        let out = lift_value_envelope(&json!(44), EnvelopeHint::None).unwrap();
        assert_eq!(
            out,
            json!({ "kind": "literal", "value": { "kind": "integer", "value": 44 } })
        );
    }

    #[test]
    fn number_lift_fractional() {
        let out = lift_value_envelope(&json!(0.3), EnvelopeHint::None).unwrap();
        assert_eq!(
            out,
            json!({ "kind": "literal", "value": { "kind": "number", "value": 0.3 } })
        );
    }

    #[test]
    fn boolean_lift() {
        let out = lift_value_envelope(&json!(true), EnvelopeHint::None).unwrap();
        assert_eq!(
            out,
            json!({ "kind": "literal", "value": { "kind": "boolean", "value": true } })
        );
    }

    #[test]
    fn text_lift() {
        let out = lift_value_envelope(&json!("hello"), EnvelopeHint::None).unwrap();
        assert_eq!(
            out,
            json!({ "kind": "literal", "value": { "kind": "text", "value": "hello" } })
        );
    }

    #[test]
    fn enum_hint_emits_enum_kind() {
        let out = lift_value_envelope(&json!("foreground"), EnvelopeHint::EnumLiteral).unwrap();
        assert_eq!(
            out,
            json!({ "kind": "literal", "value": { "kind": "enum", "value": "foreground" } })
        );
    }

    #[test]
    fn color_tuple_inferred() {
        let out = lift_value_envelope(&json!([255, 200, 50]), EnvelopeHint::None).unwrap();
        assert_eq!(
            out,
            json!({ "kind": "literal", "value": { "kind": "color", "value": { "r": 255, "g": 200, "b": 50, "a": 255 } } })
        );
    }

    #[test]
    fn binding_sigil_lifts_to_signal_value_source() {
        let out = lift_value_envelope(&json!("$bind:selectedRow"), EnvelopeHint::None).unwrap();
        assert_eq!(out, json!({ "kind": "signal", "id": "selectedRow" }));
    }

    #[test]
    fn binding_sigil_with_fallback_includes_fallback() {
        let out = lift_value_envelope(&json!("$bind:foo?42"), EnvelopeHint::None).unwrap();
        assert_eq!(
            out,
            json!({ "kind": "signal", "id": "foo", "fallback": { "kind": "integer", "value": 42 } })
        );
    }

    #[test]
    fn pre_canonical_value_passes_through() {
        let canonical = json!({ "kind": "literal", "value": { "kind": "integer", "value": 1 } });
        assert_eq!(
            lift_value_envelope(&canonical, EnvelopeHint::None).unwrap(),
            canonical
        );
    }

    #[test]
    fn duration_hint_emits_structural_duration_spec() {
        let out = lift_value_envelope(&json!("300ms"), EnvelopeHint::Duration).unwrap();
        assert_eq!(out, json!({ "kind": "milliseconds", "value": 300_u64 }));
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_value_envelope.rs</FILE> - <DESC>Lift author-side scalars into canonical { kind: "literal", value: { kind, value } } envelopes</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
