// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_resolve_phases.rs</FILE> - <DESC>Normalize phase shorthand into a canonical activePhases array</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of canonicalize: accept "all", a single phase string, or a phase array; emit Vec<LifecyclePhase>.</WCTX>
// <CLOG>0.1.0: INIT — handle "all" expansion and single-string lift to canonical phases array.</CLOG>

use serde_json::Value;

use super::cls_canonicalization_error::{CanonicalizationError, CanonicalizationErrorKind};
use super::fnc_load_tables::canonicalization_rules;

/// Resolve author-side phase shorthand into a canonical `activePhases` array.
///
/// Accepts `"all"` (expands to `["enter", "dwell", "exit"]`), a single phase
/// name (lifts to a one-element array), or an explicit array of phase names.
pub fn resolve_phases(value: &Value) -> Result<Value, CanonicalizationError> {
    let rules = canonicalization_rules()?;
    match value {
        Value::String(s) if s == "all" => Ok(Value::Array(
            rules
                .phases
                .all
                .iter()
                .cloned()
                .map(Value::String)
                .collect(),
        )),
        Value::String(s) => {
            check_phase_name(s)?;
            Ok(Value::Array(vec![Value::String(s.clone())]))
        }
        Value::Array(arr) => {
            let mut out = Vec::with_capacity(arr.len());
            for entry in arr {
                let s = entry.as_str().ok_or_else(|| {
                    CanonicalizationError::new(
                        CanonicalizationErrorKind::UnknownPhase {
                            name: entry.to_string(),
                        },
                        format!("phase array entries must be strings, got {entry}"),
                    )
                })?;
                check_phase_name(s)?;
                out.push(Value::String(s.into()));
            }
            Ok(Value::Array(out))
        }
        _ => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "string or array".into(),
            },
            "phase must be a string or an array of strings",
        )),
    }
}

fn check_phase_name(name: &str) -> Result<(), CanonicalizationError> {
    match name {
        "enter" | "dwell" | "exit" => Ok(()),
        _ => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::UnknownPhase { name: name.into() },
            format!("unknown phase: {name}"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn all_expands_to_three_phases() {
        let out = resolve_phases(&json!("all")).unwrap();
        assert_eq!(out, json!(["enter", "dwell", "exit"]));
    }

    #[test]
    fn single_phase_string_lifts_to_one_element_array() {
        let out = resolve_phases(&json!("enter")).unwrap();
        assert_eq!(out, json!(["enter"]));
    }

    #[test]
    fn array_passes_through_after_validation() {
        let out = resolve_phases(&json!(["enter", "dwell"])).unwrap();
        assert_eq!(out, json!(["enter", "dwell"]));
    }

    #[test]
    fn unknown_phase_rejects() {
        let err = resolve_phases(&json!("invalid")).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::UnknownPhase { .. }
        ));
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_resolve_phases.rs</FILE> - <DESC>Normalize phase shorthand into a canonical activePhases array</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
