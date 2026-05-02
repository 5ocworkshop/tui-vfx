// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_transitions.rs</FILE> - <DESC>Lift the author-side transitions block into RecipeDocument.transitions keyed by phase+preset</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase B of canonicalize completion: emit the canonical Compose preset intent for compose-form transitions instead of aliasing to the first sub-preset.</WCTX>
// <CLOG>0.2.0: MINOR — emit TransitionPreset::Compose for compose-form transitions; record sub-preset names in PresetUsage rather than the intent label.</CLOG>

use std::collections::BTreeMap;

use serde_json::{Map, Value};

use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
use super::cls_recipe_intent::PresetUsage;
use super::fnc_apply_preset::apply_preset;

/// Lift the author-side `transitions: { enter, dwell, exit }` block into
/// canonical `RecipeDocument.transitions` entries.
///
/// Each phase-keyed entry that names a `preset` becomes one canonical
/// `TransitionSpec` with id `<phase><Preset>` (e.g., `enterIris`). Returns
/// the per-id `PresetUsage` provenance for the top-level orchestrator to
/// merge into `RecipeIntent.preset_usages`.
pub fn lift_transitions(
    recipe: &mut Value,
) -> Result<BTreeMap<String, PresetUsage>, CanonicalizationError> {
    let recipe_obj = recipe.as_object_mut().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "object".into(),
            },
            "recipe root must be an object",
        )
    })?;

    let raw = match recipe_obj.remove("transitions") {
        Some(Value::Object(map)) => map,
        Some(other) => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "object".into(),
                },
                format!("transitions block must be an object, got {other}"),
            )
            .at(JsonPathSegment::field("transitions")));
        }
        None => return Ok(BTreeMap::new()),
    };

    let mut canonical_transitions = Map::new();
    let mut preset_usages = BTreeMap::new();

    for (phase_key, value) in raw {
        if !matches!(phase_key.as_str(), "enter" | "dwell" | "exit") {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::UnknownPhase {
                    name: phase_key.clone(),
                },
                format!("transitions block keys must be enter/dwell/exit; got {phase_key}"),
            )
            .at(JsonPathSegment::field(phase_key.clone()))
            .at(JsonPathSegment::field("transitions")));
        }

        let entry_obj = match value {
            Value::Object(obj) => obj,
            Value::String(name) => {
                let mut shorthand = Map::new();
                shorthand.insert("preset".into(), Value::String(name));
                shorthand
            }
            other => {
                return Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::UnexpectedJsonShape {
                        expected: "object or string".into(),
                    },
                    format!("transition phase entry must be an object or preset name, got {other}"),
                )
                .at(JsonPathSegment::field(phase_key))
                .at(JsonPathSegment::field("transitions")));
            }
        };

        // Multi-track form: { compose: "all", tracks: [{preset: ..., ...}, ...] }
        if entry_obj.contains_key("tracks") && entry_obj.contains_key("compose") {
            let transition_id = format!("{phase_key}Compose");
            let (spec, usage) =
                apply_compose_transition(&phase_key, &entry_obj, transition_id.clone()).map_err(
                    |e| {
                        e.at(JsonPathSegment::field(phase_key.clone()))
                            .at(JsonPathSegment::field("transitions"))
                    },
                )?;
            canonical_transitions.insert(transition_id.clone(), spec);
            preset_usages.insert(transition_id, usage);
            continue;
        }

        let preset = entry_obj
            .get("preset")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::MissingRequired {
                        field: "preset".into(),
                    },
                    "transition entry must declare `preset`",
                )
                .at(JsonPathSegment::field(phase_key.clone()))
                .at(JsonPathSegment::field("transitions"))
            })?
            .to_string();

        let transition_id = format!("{phase_key}{}", capitalize_first(&preset));
        let (spec, usage) = apply_preset(&preset, &phase_key, &entry_obj, transition_id.clone())
            .map_err(|e| {
                e.at(JsonPathSegment::field(phase_key.clone()))
                    .at(JsonPathSegment::field("transitions"))
            })?;
        canonical_transitions.insert(transition_id.clone(), spec);
        preset_usages.insert(transition_id, usage);
    }

    if !canonical_transitions.is_empty() {
        recipe_obj.insert("transitions".into(), Value::Object(canonical_transitions));
    }
    Ok(preset_usages)
}

/// Build a single canonical TransitionSpec from a multi-track compose-form
/// transition entry. Iterates `tracks: [{preset: ..., ...}, ...]`, runs each
/// sub-entry through `apply_preset`, and merges the resulting tracks under one
/// transition. Records the compose mode in `PresetUsage.consumed_params`.
fn apply_compose_transition(
    phase_key: &str,
    entry: &Map<String, Value>,
    transition_id: String,
) -> Result<(Value, super::cls_recipe_intent::PresetUsage), CanonicalizationError> {
    let tracks_raw = entry
        .get("tracks")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "tracks: [{preset, ...}, ...]".into(),
                },
                "compose transition must declare `tracks` array",
            )
        })?;

    let mut all_tracks = Vec::new();
    let mut consumed: Vec<String> = vec!["compose".into(), "tracks".into()];
    let mut sub_presets: Vec<String> = Vec::new();

    for sub in tracks_raw {
        let sub_obj = sub.as_object().ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "object".into(),
                },
                "compose tracks entry must be an object with `preset`",
            )
        })?;
        let sub_preset = sub_obj
            .get("preset")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                CanonicalizationError::new(
                    CanonicalizationErrorKind::MissingRequired {
                        field: "preset".into(),
                    },
                    "compose tracks entry must declare `preset`",
                )
            })?;
        sub_presets.push(sub_preset.into());
        let sub_id = format!("{transition_id}_{sub_preset}");
        let (sub_spec, _) = apply_preset(sub_preset, phase_key, sub_obj, sub_id)?;
        if let Some(track) = sub_spec.pointer("/tracks/0") {
            all_tracks.push(track.clone());
        }
    }

    // Build the merged TransitionSpec borrowing timing from the parent entry.
    // Author-side compose blocks describe a single coordinated transition;
    // intent is recorded as the canonical Compose preset, and the sub-preset
    // names ride along in PresetUsage.consumed_params for provenance.
    use serde_json::json;
    let mut spec = Map::new();
    spec.insert("id".into(), Value::String(transition_id.clone()));
    spec.insert(
        "intent".into(),
        json!({ "kind": "preset", "preset": "compose" }),
    );
    spec.insert(
        "subjects".into(),
        json!({ "from": { "kind": "empty" }, "to": { "kind": "empty" } }),
    );
    spec.insert(
        "timing".into(),
        super::fnc_apply_preset::build_compose_timing(entry, &mut consumed)?,
    );
    spec.insert("activePhases".into(), json!([phase_key]));
    spec.insert("tracks".into(), Value::Array(all_tracks));
    spec.insert(
        "interruption".into(),
        Value::String("snapToEndThenStartNext".into()),
    );
    spec.insert("reducedMotion".into(), json!({ "policy": "instant" }));

    Ok((
        Value::Object(spec),
        super::cls_recipe_intent::PresetUsage {
            preset: format!("compose({})", sub_presets.join(",")),
            consumed_params: consumed,
        },
    ))
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn iris_enter_and_exit_become_two_transitions() {
        let mut recipe = json!({
            "id": "x",
            "transitions": {
                "enter": { "preset": "iris", "shape": "circle", "softEdge": true, "duration": "2s" },
                "exit":  { "preset": "iris", "shape": "diamond", "softEdge": true, "duration": "1s" }
            }
        });
        let usages = lift_transitions(&mut recipe).unwrap();
        assert_eq!(usages.len(), 2);
        assert!(usages.contains_key("enterIris"));
        assert!(usages.contains_key("exitIris"));

        let transitions = recipe["transitions"].as_object().unwrap();
        let enter = transitions.get("enterIris").unwrap();
        assert_eq!(
            enter["intent"],
            json!({ "kind": "preset", "preset": "iris" })
        );
        assert_eq!(enter["activePhases"], json!(["enter"]));
        assert_eq!(enter["tracks"][0]["shape"], "circle");

        let exit = transitions.get("exitIris").unwrap();
        assert_eq!(exit["tracks"][0]["shape"], "diamond");
    }

    #[test]
    fn unknown_phase_key_rejects() {
        let mut recipe = json!({ "id": "x", "transitions": { "weird": { "preset": "iris" } } });
        let err = lift_transitions(&mut recipe).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::UnknownPhase { .. }
        ));
    }

    #[test]
    fn no_transitions_block_is_a_noop() {
        let mut recipe = json!({ "id": "x" });
        let usages = lift_transitions(&mut recipe).unwrap();
        assert!(usages.is_empty());
        assert!(!recipe.as_object().unwrap().contains_key("transitions"));
    }

    #[test]
    fn string_form_lifts_to_preset_with_defaults() {
        let mut recipe = json!({ "id": "x", "transitions": { "enter": "fade" } });
        let usages = lift_transitions(&mut recipe).unwrap();
        assert!(usages.contains_key("enterFade"));
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_transitions.rs</FILE> - <DESC>Lift the author-side transitions block into RecipeDocument.transitions keyed by phase+preset</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
