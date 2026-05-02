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
    spec.insert(
        "intent".into(),
        json!({ "kind": "preset", "preset": preset }),
    );
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
    Ok(json!({
        "kind": "opacity.fade",
        "subject": "to",
        "from": 0.0,
        "to": 1.0,
    }))
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
    if let Some(seed) = author.get("seed").and_then(Value::as_i64) {
        consumed.push("seed".into());
        track["seed"] = json!(seed);
    }
    if let Some(chunk) = author.get("chunkSize").and_then(Value::as_i64) {
        consumed.push("chunkSize".into());
        track["chunkSize"] = json!(chunk);
    }
    Ok(track)
}

fn build_blinds_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    let orientation =
        consume_string(author, "orientation", consumed).unwrap_or_else(|| "horizontal".into());
    let mut track = json!({
        "kind": "visibility.blinds",
        "subject": "to",
        "orientation": orientation,
    });
    if let Some(count) = author.get("count").and_then(Value::as_i64) {
        consumed.push("count".into());
        track["count"] = json!(count);
    }
    Ok(track)
}

fn build_stippled_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    let mut track = json!({
        "kind": "visibility.stippled",
        "subject": "to",
    });
    if let Some(pattern) = consume_string(author, "pattern", consumed) {
        track["pattern"] = Value::String(pattern);
    }
    if let Some(density) = author.get("density").and_then(Value::as_f64) {
        consumed.push("density".into());
        track["density"] = json!(density);
    }
    if let Some(seed) = author.get("seed").and_then(Value::as_i64) {
        consumed.push("seed".into());
        track["seed"] = json!(seed);
    }
    Ok(track)
}

fn build_braille_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    let mut track = json!({
        "kind": "visibility.braille",
        "subject": "to",
    });
    if let Some(order) = consume_string(author, "subcellOrder", consumed) {
        track["subcellOrder"] = Value::String(order);
    }
    if let Some(seed) = author.get("seed").and_then(Value::as_i64) {
        consumed.push("seed".into());
        track["seed"] = json!(seed);
    }
    Ok(track)
}

fn build_relation_track(
    kind: &str,
    _author: &Map<String, Value>,
    _consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    Ok(json!({ "kind": kind, "subject": "shared" }))
}

fn build_push_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    let direction =
        consume_string(author, "direction", consumed).unwrap_or_else(|| "leftToRight".into());
    Ok(json!({
        "kind": "relation.push",
        "subject": "shared",
        "direction": direction,
    }))
}

fn build_morph_track(
    author: &Map<String, Value>,
    consumed: &mut Vec<String>,
) -> Result<Value, CanonicalizationError> {
    let match_kind = consume_string(author, "match", consumed).unwrap_or_else(|| "glyph".into());
    Ok(json!({
        "kind": "relation.morph",
        "subject": "shared",
        "match": match_kind,
    }))
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
        let resolved = match easing {
            Value::String(s) => json!({ "kind": "named", "value": s }),
            Value::Object(_) => easing.clone(),
            _ => {
                return Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::UnexpectedJsonShape {
                        expected: "string or object".into(),
                    },
                    "easing must be a named string or object",
                )
                .at(JsonPathSegment::field("easing")));
            }
        };
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
