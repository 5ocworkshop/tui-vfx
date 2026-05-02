// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_resolve_easing.rs</FILE> - <DESC>Translate author-side easing names into the canonical NamedEasing wire form</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2d of canonicalize: corpus uses CSS-style 'quadOut'/'cubicInOut'; contract uses 'outQuad'/'inOutCubic'.</WCTX>
// <CLOG>0.1.0: INIT — map author easing names to NamedEasing variants and emit { kind: 'named', value }.</CLOG>

use serde_json::{Value, json};

use super::cls_canonicalization_error::{CanonicalizationError, CanonicalizationErrorKind};

/// Convert an author-side easing name or object into the canonical
/// `EasingSpec` shape (`{ kind: "named", value: "<NamedEasing>" }` or a
/// passthrough object that already carries `kind`).
pub fn resolve_easing(value: &Value) -> Result<Value, CanonicalizationError> {
    match value {
        Value::String(s) => {
            let canonical = canonical_easing_name(s)?;
            Ok(json!({ "kind": "named", "value": canonical }))
        }
        Value::Object(obj) if obj.contains_key("kind") => Ok(value.clone()),
        Value::Object(_) => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "easing string or { kind, value } object".into(),
            },
            "easing object must include `kind` (e.g., named, cubicBezier)",
        )),
        _ => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "string or object".into(),
            },
            "easing must be a string or object",
        )),
    }
}

/// Map an author-side easing name (CSS-style `quadOut`, `cubicInOut`) to the
/// canonical `NamedEasing` wire form (`outQuad`, `inOutCubic`). Names that
/// already match the canonical form pass through.
fn canonical_easing_name(author: &str) -> Result<&'static str, CanonicalizationError> {
    Ok(match author {
        // pass-through canonical names
        "linear" => "linear",
        "inQuad" => "inQuad",
        "outQuad" => "outQuad",
        "inOutQuad" => "inOutQuad",
        "inCubic" => "inCubic",
        "outCubic" => "outCubic",
        "inOutCubic" => "inOutCubic",
        "inSine" => "inSine",
        "outSine" => "outSine",
        "inOutSine" => "inOutSine",
        "inBack" => "inBack",
        "outBack" => "outBack",
        "inOutBack" => "inOutBack",

        // CSS-style author shorthand (X-In/Out → InX, OutX, InOutX)
        "quadIn" => "inQuad",
        "quadOut" => "outQuad",
        "quadInOut" => "inOutQuad",
        "cubicIn" => "inCubic",
        "cubicOut" => "outCubic",
        "cubicInOut" => "inOutCubic",
        "sineIn" => "inSine",
        "sineOut" => "outSine",
        "sineInOut" => "inOutSine",
        "backIn" => "inBack",
        "backOut" => "outBack",
        "backInOut" => "inOutBack",

        other => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "named easing".into(),
                },
                format!("unknown easing name: {other}"),
            ));
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_style_names_translate_to_named_easing() {
        assert_eq!(
            resolve_easing(&json!("quadOut")).unwrap(),
            json!({ "kind": "named", "value": "outQuad" })
        );
        assert_eq!(
            resolve_easing(&json!("cubicInOut")).unwrap(),
            json!({ "kind": "named", "value": "inOutCubic" })
        );
    }

    #[test]
    fn canonical_names_pass_through() {
        assert_eq!(
            resolve_easing(&json!("inOutCubic")).unwrap(),
            json!({ "kind": "named", "value": "inOutCubic" })
        );
    }

    #[test]
    fn unknown_easing_rejects() {
        let err = resolve_easing(&json!("magicalSpring")).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::UnexpectedJsonShape { .. }
        ));
    }

    #[test]
    fn object_with_kind_passes_through() {
        let canonical =
            json!({ "kind": "cubicBezier", "x1": 0.0, "y1": 0.0, "x2": 1.0, "y2": 1.0 });
        assert_eq!(resolve_easing(&canonical).unwrap(), canonical);
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_resolve_easing.rs</FILE> - <DESC>Translate author-side easing names into the canonical NamedEasing wire form</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
