// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_resolve_color.rs</FILE> - <DESC>Lift author-side color shorthand into canonical { r, g, b, a } form</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of canonicalize: turn "#fff" / "white" / [r,g,b] into { r, g, b, a } objects.</WCTX>
// <CLOG>0.1.0: INIT — accept hex strings, named colors, and tuple forms.</CLOG>

use serde_json::{Value, json};

use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
use super::fnc_load_tables::canonicalization_rules;

/// Resolve any of the accepted author-side color shorthands into the canonical
/// `{ r, g, b, a }` color object.
pub fn resolve_color(value: &Value) -> Result<Value, CanonicalizationError> {
    match value {
        Value::String(s) => {
            if let Some(stripped) = s.strip_prefix('#') {
                return parse_hex(stripped);
            }
            let rules = canonicalization_rules()?;
            if let Some(rgb) = rules.named_colors.get(s) {
                return Ok(tuple_to_color(rgb));
            }
            Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnknownNamedColor { name: s.clone() },
                format!("unknown named color: {s}"),
            ))
        }
        Value::Array(arr) => {
            let parts: Vec<u8> = arr
                .iter()
                .enumerate()
                .map(|(idx, v)| {
                    v.as_u64()
                        .filter(|n| *n <= 255)
                        .map(|n| n as u8)
                        .ok_or_else(|| {
                            CanonicalizationError::new(
                                CanonicalizationErrorKind::MalformedColor,
                                "color tuple element must be an integer in 0..=255",
                            )
                            .at(JsonPathSegment::Index(idx))
                        })
                })
                .collect::<Result<_, _>>()?;
            if parts.len() != 3 && parts.len() != 4 {
                return Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::MalformedColor,
                    format!(
                        "color tuple must have 3 or 4 elements (got {})",
                        parts.len()
                    ),
                ));
            }
            Ok(tuple_to_color(&parts))
        }
        Value::Object(_) => Ok(value.clone()),
        _ => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::MalformedColor,
            "color must be a hex string, named color, tuple, or canonical object",
        )),
    }
}

fn tuple_to_color(rgb: &[u8]) -> Value {
    let r = rgb.first().copied().unwrap_or(0);
    let g = rgb.get(1).copied().unwrap_or(0);
    let b = rgb.get(2).copied().unwrap_or(0);
    let a = rgb.get(3).copied().unwrap_or(255);
    json!({ "r": r, "g": g, "b": b, "a": a })
}

fn parse_hex(hex: &str) -> Result<Value, CanonicalizationError> {
    let bytes: Vec<u8> = match hex.len() {
        6 | 8 => hex_pairs(hex)?,
        3 => {
            let expanded: String = hex.chars().flat_map(|c| [c, c]).collect();
            hex_pairs(&expanded)?
        }
        _ => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::MalformedColor,
                format!("hex color #{hex} must have 3, 6, or 8 hex digits"),
            ));
        }
    };
    Ok(tuple_to_color(&bytes))
}

fn hex_pairs(hex: &str) -> Result<Vec<u8>, CanonicalizationError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16).map_err(|_| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::MalformedColor,
                    format!("hex color contains non-hex digits: {hex}"),
                )
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_six_digit() {
        let out = resolve_color(&json!("#ffffff")).unwrap();
        assert_eq!(out, json!({ "r": 255, "g": 255, "b": 255, "a": 255 }));
    }

    #[test]
    fn hex_eight_digit_alpha() {
        let out = resolve_color(&json!("#00000080")).unwrap();
        assert_eq!(out, json!({ "r": 0, "g": 0, "b": 0, "a": 128 }));
    }

    #[test]
    fn hex_three_digit_shorthand() {
        let out = resolve_color(&json!("#fff")).unwrap();
        assert_eq!(out, json!({ "r": 255, "g": 255, "b": 255, "a": 255 }));
    }

    #[test]
    fn named_color() {
        let out = resolve_color(&json!("white")).unwrap();
        assert_eq!(out["r"], 229);
        assert_eq!(out["a"], 255);
    }

    #[test]
    fn tuple_rgb() {
        let out = resolve_color(&json!([60, 60, 80])).unwrap();
        assert_eq!(out, json!({ "r": 60, "g": 60, "b": 80, "a": 255 }));
    }

    #[test]
    fn tuple_rgba() {
        let out = resolve_color(&json!([1, 2, 3, 4])).unwrap();
        assert_eq!(out, json!({ "r": 1, "g": 2, "b": 3, "a": 4 }));
    }

    #[test]
    fn unknown_named_color_rejects() {
        let err = resolve_color(&json!("magentish")).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::UnknownNamedColor { .. }
        ));
    }

    #[test]
    fn malformed_hex_rejects() {
        let err = resolve_color(&json!("#xyz")).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::MalformedColor
        ));
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_resolve_color.rs</FILE> - <DESC>Lift author-side color shorthand into canonical { r, g, b, a } form</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
