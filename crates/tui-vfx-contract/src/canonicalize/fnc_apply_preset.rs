// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_apply_preset.rs</FILE> - <DESC>Materialize an author-side transition preset into a canonical TransitionSpec JSON value</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2b of canonicalize: expand named presets into TransitionSpec values, recording TransitionIntent::Preset provenance.</WCTX>
// <CLOG>0.1.0: INIT — handle the visibility iris/wipe/dissolve/blinds/fade family plus relation crossfade/push/morph and visibility stippled/braille via the per-preset materialize switch.</CLOG>

use serde_json::{Map, Value, json};

use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
use super::cls_recipe_intent::PresetUsage;
use super::fnc_resolve_duration::resolve_duration;
use super::fnc_resolve_easing::resolve_easing;

/// Build a canonical [`TransitionSpec`]-shape JSON value from one author-side
/// preset entry.
///
/// `phase_key` is the author phase key (`"enter"`, `"dwell"`, or `"exit"`)
/// that the transition runs in. The function returns the materialized
/// JSON plus a `PresetUsage` provenance record.
pub fn apply_preset(
    preset: &str,
    phase_key: &str,
    author: &Map<String, Value>,
    transition_id: String,
) -> Result<(Value, PresetUsage), CanonicalizationError> {
    let mut consumed_params: Vec<String> = Vec::new();
    let track = build_track_for_preset(preset, author, &mut consumed_params)?;

    let mut spec = Map::new();
    spec.insert("id".into(), Value::String(transition_id));
    spec.insert("intent".into(), build_intent(preset));
    spec.insert(
        "subjects".into(),
        json!({ "from": { "kind": "empty" }, "to": { "kind": "empty" } }),
    );
    spec.insert("timing".into(), build_timing(author, &mut consumed_params)?);
    spec.insert("activePhases".into(), json!([phase_key]));
    spec.insert("tracks".into(), json!([track]));
    spec.insert(
        "interruption".into(),
        Value::String("snapToEndThenStartNext".into()),
    );
    spec.insert("reducedMotion".into(), json!({ "policy": "instant" }));

    Ok((
        Value::Object(spec),
        PresetUsage {
            preset: preset.into(),
            consumed_params,
        },
    ))
}

/// Build the [`TransitionIntent`] JSON for a preset name. Names in the closed
/// `TransitionPreset` enum become `Preset { preset }`; corpus-witnessed
/// non-canonical names (`radial`, `cellular`) become `Alias { alias,
/// canonicalPreset }` referencing the closest canonical preset they expand to.
fn build_intent(preset: &str) -> Value {
    match preset {
        "radial" => json!({
            "kind": "alias",
            "alias": "radial",
            "canonicalPreset": "iris"
        }),
        "cellular" => json!({
            "kind": "alias",
            "alias": "cellular",
            "canonicalPreset": "dissolve"
        }),
        canonical => json!({ "kind": "preset", "preset": canonical }),
    }
}

fn build_track_for_preset(
    preset: &str,
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    match preset {
        "iris" => build_iris_track(author, consumed),
        "fade" => build_fade_track(author, consumed),
        "wipe" => build_wipe_track(author, consumed),
        "dissolve" => build_dissolve_track(author, consumed),
        "blinds" => build_blinds_track(author, consumed),
        "stippled" => build_stippled_track(author, consumed),
        "braille" => build_braille_track(author, consumed),
        "crossfade" => build_relation_track("relation.crossfade", author, consumed),
        "push" => build_push_track(author, consumed),
        "morph" => build_morph_track(author, consumed),
        // `radial` is an iris-from-anchor variant; emit as visibility.iris
        // with circle shape so the canonical pipeline can execute it.
        "radial" => build_iris_track(author, consumed),
        // `cellular` reads as a per-cell stipple reveal in the corpus; emit
        // as visibility.dissolve so it round-trips structurally.
        "cellular" => build_dissolve_track(author, consumed),
        _ => Err(CanonicalizationError::new(
            CanonicalizationErrorKind::UnknownPreset {
                axis: "transition".into(),
                preset: preset.into(),
            },
            format!("unknown transition preset: {preset}"),
        )),
    }
}

fn build_iris_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    let shape = consume_string(author, "shape", consumed).unwrap_or_else(|| "circle".into());
    let mut track = json!({
        "kind": "visibility.iris",
        "subject": "to",
        "shape": shape,
    });
    if let Some(soft) = author.get("softEdge").and_then(Value::as_bool) {
        consumed.push("softEdge".into());
        let edge = if soft {
            json!({ "kind": "soft", "featherCells": 1 })
        } else {
            json!({ "kind": "hard" })
        };
        track["edge"] = edge;
    }
    Ok(track)
}

fn build_fade_track(
    _author: &Map<String, Value>,
    _consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    // The canonical opacity.fade track defaults from=0 / to=1 when the
    // optional sources are omitted. Emitting just `subject` keeps the
    // canonical document smaller and avoids a ValueSource wrap on each side.
    Ok(json!({ "kind": "opacity.fade", "subject": "to" }))
}

fn build_wipe_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    let direction =
        consume_string(author, "direction", consumed).unwrap_or_else(|| "leftToRight".into());
    let mut track = json!({
        "kind": "visibility.wipe",
        "subject": "to",
        "revealDirection": direction,
    });
    if let Some(soft) = author.get("softEdge").and_then(Value::as_bool) {
        consumed.push("softEdge".into());
        if soft {
            track["edge"] = json!({ "kind": "soft", "featherCells": 1 });
        }
    }
    Ok(track)
}

fn build_dissolve_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    let mut track = json!({
        "kind": "visibility.dissolve",
        "subject": "to",
    });
    if let Some(seed) = author.get("seed") {
        consumed.push("seed".into());
        track["seed"] = wrap_value_source(seed)?;
    }
    // chunkSize is author-side metadata; the canonical visibility.dissolve
    // track does not carry it. Record consumption so PresetUsage reflects the
    // author-side intent without polluting the track.
    if author.contains_key("chunkSize") {
        consumed.push("chunkSize".into());
    }
    Ok(track)
}

fn build_blinds_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    let orientation =
        consume_string(author, "orientation", consumed).unwrap_or_else(|| "horizontal".into());
    let count_value = author.get("count").cloned().unwrap_or_else(|| json!(4));
    if author.contains_key("count") {
        consumed.push("count".into());
    }
    Ok(json!({
        "kind": "visibility.blinds",
        "subject": "to",
        "orientation": orientation,
        "count": wrap_value_source(&count_value)?,
    }))
}

fn build_stippled_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    // The canonical visibility.stippled track only accepts `subject` and
    // `transitionProgress`. Record pattern/density/seed as consumed for
    // PresetUsage but do not emit them onto the track.
    for author_key in ["pattern", "density", "seed"] {
        if author.contains_key(author_key) {
            consumed.push(author_key.into());
        }
    }
    Ok(json!({ "kind": "visibility.stippled", "subject": "to" }))
}

fn build_braille_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    for author_key in ["subcellOrder", "seed"] {
        if author.contains_key(author_key) {
            consumed.push(author_key.into());
        }
    }
    Ok(json!({ "kind": "visibility.braille", "subject": "to" }))
}

fn build_relation_track(
    kind: &str,
    _author: &Map<String, Value>,
    _consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    Ok(json!({ "kind": kind }))
}

fn build_push_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    let direction =
        consume_string(author, "direction", consumed).unwrap_or_else(|| "leftToRight".into());
    Ok(json!({
        "kind": "relation.push",
        "travelDirection": travel_direction_from_reveal(&direction),
    }))
}

fn build_morph_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    if author.contains_key("match") {
        consumed.push("match".into());
    }
    Ok(json!({ "kind": "relation.morph" }))
}

/// Wrap an author-side raw value into a canonical ValueSource literal envelope
/// when it isn't already one.
fn wrap_value_source(value: &Value) -> Result<Value, CanonicalizationError> {
    if let Value::Object(obj) = value
        && obj.contains_key("kind")
    {
        return Ok(value.clone());
    }
    let inner = match value {
        Value::Bool(b) => json!({ "kind": "boolean", "value": b }),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                json!({ "kind": "integer", "value": i })
            } else {
                json!({ "kind": "number", "value": n.as_f64().unwrap_or(0.0) })
            }
        }
        Value::String(s) => json!({ "kind": "text", "value": s }),
        _ => {
            return Err(CanonicalizationError::new(
                CanonicalizationErrorKind::EnvelopeLiftFailed,
                format!("cannot wrap value {value} as ValueSource literal"),
            ));
        }
    };
    Ok(json!({ "kind": "literal", "value": inner }))
}

/// Map an author-side reveal-style direction (`leftToRight`, `rightToLeft`,
/// `topToBottom`, `bottomToTop`) to the canonical TransitionTravelDirection
/// (`left`, `right`, `up`, `down`). The canonical naming describes which way
/// the surface travels; the corpus uses CSS reveal-direction naming.
fn travel_direction_from_reveal(reveal: &str) -> &'static str {
    match reveal {
        "leftToRight" | "right" => "right",
        "rightToLeft" | "left" => "left",
        "topToBottom" | "down" => "down",
        "bottomToTop" | "up" => "up",
        _ => "right",
    }
}

fn build_timing(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    let mut timing = Map::new();
    if let Some(duration) = author.get("duration") {
        consumed.push("duration".into());
        let resolved =
            resolve_duration(duration).map_err(|e| e.at(JsonPathSegment::field("duration")))?;
        timing.insert("duration".into(), resolved);
    }
    if let Some(easing) = author.get("easing") {
        consumed.push("easing".into());
        let resolved =
            resolve_easing(easing).map_err(|e| e.at(JsonPathSegment::field("easing")))?;
        timing.insert("easing".into(), resolved);
    }
    Ok(Value::Object(timing))
}

fn consume_string(
    author: &Map<String, Value>,
    key: &str,
    consumed: &mut Vec<String>,
) -> Option<String> {
    author.get(key).and_then(Value::as_str).map(|s| {
        consumed.push(key.into());
        s.to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iris_preset_builds_visibility_iris_track_with_soft_edge() {
        let author: Map<String, Value> = serde_json::from_value(json!({
            "preset": "iris", "shape": "circle", "softEdge": true, "duration": "2s"
        }))
        .unwrap();
        let (spec, usage) = apply_preset("iris", "enter", &author, "enterIris".into()).unwrap();
        assert_eq!(
            spec["intent"],
            json!({ "kind": "preset", "preset": "iris" })
        );
        assert_eq!(spec["activePhases"], json!(["enter"]));
        assert_eq!(spec["tracks"][0]["kind"], "visibility.iris");
        assert_eq!(spec["tracks"][0]["shape"], "circle");
        assert_eq!(
            spec["tracks"][0]["edge"],
            json!({ "kind": "soft", "featherCells": 1 })
        );
        assert_eq!(
            spec["timing"]["duration"],
            json!({ "kind": "seconds", "value": 2 })
        );
        assert!(usage.consumed_params.contains(&"shape".into()));
        assert!(usage.consumed_params.contains(&"softEdge".into()));
        assert!(usage.consumed_params.contains(&"duration".into()));
    }

    #[test]
    fn fade_preset_emits_opacity_fade_track() {
        let author: Map<String, Value> = serde_json::from_value(json!({})).unwrap();
        let (spec, _) = apply_preset("fade", "enter", &author, "enterFade".into()).unwrap();
        assert_eq!(spec["tracks"][0]["kind"], "opacity.fade");
    }

    #[test]
    fn unknown_preset_rejects() {
        let author: Map<String, Value> = serde_json::from_value(json!({})).unwrap();
        let err = apply_preset("no_such_preset", "enter", &author, "x".into()).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::UnknownPreset { .. }
        ));
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_apply_preset.rs</FILE> - <DESC>Materialize an author-side transition preset into a canonical TransitionSpec JSON value</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
