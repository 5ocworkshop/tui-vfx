// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_lifecycle.rs</FILE> - <DESC>Lift the lifecycle: { enter, dwell, exit } shorthand into canonical clock + phases</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2d of canonicalize: turn the per-phase duration shorthand into the canonical phases array.</WCTX>
// <CLOG>0.1.0: INIT — accept string per-phase durations and object form with route/easing extras (route deferred).</CLOG>

use serde_json::{Value, json};

use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
use super::fnc_resolve_duration::resolve_duration;

/// If the recipe's `lifecycle` block is in shorthand form
/// (`{ enter: "180ms", dwell: "5s", exit: "120ms" }`), lift it to the
/// canonical `{ clock, phases }` shape. Already-canonical input
/// (`{ phases: [...] }` or `{ clock: {...}, phases: [...] }`) passes through.
pub fn lift_lifecycle(recipe: &mut Value) -> Result<(), CanonicalizationError> {
    let recipe_obj = recipe.as_object_mut().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "object".into(),
            },
            "recipe root must be an object",
        )
    })?;

    let lifecycle = match recipe_obj.get_mut("lifecycle") {
        Some(value) => value,
        None => return Ok(()),
    };
    let obj = match lifecycle.as_object_mut() {
        Some(o) => o,
        None => return Ok(()),
    };

    if obj.contains_key("phases") {
        return Ok(());
    }

    let mut phases = Vec::new();
    for phase_key in ["enter", "dwell", "exit"] {
        let phase = match obj.remove(phase_key) {
            Some(value) => build_phase(phase_key, &value).map_err(|e| {
                e.at(JsonPathSegment::field(phase_key))
                    .at(JsonPathSegment::field("lifecycle"))
            })?,
            None => default_phase(phase_key),
        };
        phases.push(phase);
    }

    obj.entry("clock")
        .or_insert_with(|| json!({ "clockMode": "monotonic", "period": null }));
    obj.insert("phases".into(), Value::Array(phases));
    Ok(())
}

fn default_phase(name: &str) -> Value {
    let duration_ms = match name {
        "dwell" => 5000,
        _ => 1000,
    };
    let timing = if name == "dwell" {
        json!({
            "kind": "dwell",
            "policy": { "kind": "fixed", "duration": { "kind": "milliseconds", "value": duration_ms } }
        })
    } else {
        json!({
            "kind": "fixed",
            "duration": { "kind": "milliseconds", "value": duration_ms }
        })
    };
    json!({ "phase": name, "timing": timing })
}

fn build_phase(name: &str, value: &Value) -> Result<Value, CanonicalizationError> {
    let duration = match value {
        Value::String(_) => resolve_duration(value)?,
        Value::Object(obj) => match obj.get("duration") {
            Some(d) => resolve_duration(d)?,
            None => match obj.get("fallback") {
                // `dwell: { until: <trigger>, fallback: "<duration>" }` shorthand:
                // the canonical TriggerSpec dwell-until policy needs full
                // condition/latch/reset/action wiring that descriptor catalogs
                // own. As a near-term canonicalization, emit fixed dwell with
                // the fallback duration so the recipe is canonicalize-clean;
                // runtime tooling that knows the binding can promote it later.
                Some(fb) => resolve_duration(fb)?,
                None => {
                    return Err(CanonicalizationError::new(
                        CanonicalizationErrorKind::MissingRequired {
                            field: "duration".into(),
                        },
                        format!(
                            "lifecycle.{name} object must declare `duration` (or `fallback` for trigger-based dwell)"
                        ),
                    ));
                }
            },
        },
        _ => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "duration string or object".into(),
                },
                format!("lifecycle.{name} must be a duration string or object"),
            ));
        }
    };

    let timing = if name == "dwell" {
        json!({
            "kind": "dwell",
            "policy": { "kind": "fixed", "duration": duration }
        })
    } else {
        json!({ "kind": "fixed", "duration": duration })
    };
    Ok(json!({ "phase": name, "timing": timing }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shorthand_durations_lift_to_canonical_phases() {
        let mut recipe = json!({
            "id": "x",
            "lifecycle": { "enter": "180ms", "dwell": "5s", "exit": "120ms" }
        });
        lift_lifecycle(&mut recipe).unwrap();
        let lifecycle = &recipe["lifecycle"];
        assert_eq!(lifecycle["clock"]["clockMode"], "monotonic");
        let phases = lifecycle["phases"].as_array().unwrap();
        assert_eq!(phases.len(), 3);
        assert_eq!(phases[0]["phase"], "enter");
        assert_eq!(
            phases[0]["timing"],
            json!({
                "kind": "fixed",
                "duration": { "kind": "milliseconds", "value": 180 }
            })
        );
        assert_eq!(
            phases[1]["timing"],
            json!({
                "kind": "dwell",
                "policy": { "kind": "fixed", "duration": { "kind": "seconds", "value": 5.0 } }
            })
        );
    }

    #[test]
    fn already_canonical_lifecycle_passes_through() {
        let mut recipe = json!({
            "id": "x",
            "lifecycle": {
                "clock": { "clockMode": "monotonic", "period": null },
                "phases": [{ "phase": "enter", "timing": { "kind": "fixed", "duration": { "kind": "milliseconds", "value": 100 } } }]
            }
        });
        let original = recipe.clone();
        lift_lifecycle(&mut recipe).unwrap();
        assert_eq!(recipe, original);
    }

    #[test]
    fn missing_lifecycle_is_a_noop() {
        let mut recipe = json!({ "id": "x" });
        lift_lifecycle(&mut recipe).unwrap();
        assert!(!recipe.as_object().unwrap().contains_key("lifecycle"));
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_lifecycle.rs</FILE> - <DESC>Lift the lifecycle: { enter, dwell, exit } shorthand into canonical clock + phases</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
