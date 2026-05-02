// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_top_level_extras.rs</FILE> - <DESC>Lift top-level shorthand fields (at, loop) the canonical recipe doesn't accept directly</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2d of canonicalize: extract top-level `at` placement and `loop` clock-period shorthands so the canonical recipe deserializes cleanly.</WCTX>
// <CLOG>0.1.0: INIT — strip and apply at/loop, error clearly on the scene[]/extends shapes that need Phase 2e/2f.</CLOG>

use serde_json::{Value, json};

use super::cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
use super::fnc_resolve_duration::resolve_duration;

/// Extracted top-level shorthand applied later by other passes.
#[derive(Debug, Clone, Default)]
pub struct TopLevelExtras {
    /// Top-level `at: "<anchor>"` placement (e.g., `"topRight"`). Applied to
    /// the default scene element by `apply_recipe_defaults`.
    pub anchor: Option<String>,
}

/// Strip the top-level shorthand fields the canonical recipe doesn't
/// accept directly:
///
/// * `at: "<anchor>"` is stashed for later application as the default scene
///   element's `placementRule`.
/// * `loop: "<duration>"` rewrites `lifecycle.clock` to `{ clockMode:
///   "looping", period: <DurationSpec> }`.
/// * `scene: [...]` and `extends: "..."` cause clear errors today; full
///   handling lands in later phases.
pub fn lift_top_level_extras(recipe: &mut Value) -> Result<TopLevelExtras, CanonicalizationError> {
    let recipe_obj = recipe.as_object_mut().ok_or_else(|| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::UnexpectedJsonShape {
                expected: "object".into(),
            },
            "recipe root must be an object",
        )
    })?;

    // `extends:` is consumed by `fnc_resolve_extends::resolve_extends` before
    // this pass runs (via canonicalize_recipe_with_templates). If it survives
    // to here it indicates a caller used the template-less entry without
    // pre-resolving; surface as ExtendsTargetNotFound for clarity.
    if recipe_obj.contains_key("extends") {
        let path = recipe_obj
            .get("extends")
            .and_then(Value::as_str)
            .unwrap_or("(non-string)")
            .to_string();
        return Err(CanonicalizationError::new(
            CanonicalizationErrorKind::ExtendsTargetNotFound { path: path.clone() },
            format!(
                "recipe declares `extends: \"{path}\"` but canonicalize_recipe was called without a template map. Use canonicalize_recipe_with_templates(value, templates) to resolve the chain."
            ),
        )
        .at(JsonPathSegment::field("extends")));
    }

    let mut extras = TopLevelExtras::default();
    if let Some(at) = recipe_obj.remove("at") {
        match at {
            Value::String(s) => extras.anchor = Some(canonical_anchor_name(&s)),
            other => {
                return Err(CanonicalizationError::new(
                    CanonicalizationErrorKind::UnexpectedJsonShape {
                        expected: "string anchor name".into(),
                    },
                    format!("top-level `at` must be a string, got {other}"),
                )
                .at(JsonPathSegment::field("at")));
            }
        }
    }

    if let Some(loop_value) = recipe_obj.remove("loop") {
        let period =
            resolve_duration(&loop_value).map_err(|e| e.at(JsonPathSegment::field("loop")))?;
        let lifecycle = recipe_obj.entry("lifecycle").or_insert_with(|| json!({}));
        let lifecycle_obj = lifecycle.as_object_mut().ok_or_else(|| {
            CanonicalizationError::new(
                CanonicalizationErrorKind::UnexpectedJsonShape {
                    expected: "object".into(),
                },
                "lifecycle must be an object",
            )
            .at(JsonPathSegment::field("lifecycle"))
        })?;
        lifecycle_obj.insert(
            "clock".into(),
            json!({ "clockMode": "looping", "period": period }),
        );
    }

    Ok(extras)
}

/// Map author-side anchor synonyms (`middleLeft`, `middleRight`, `topMiddle`)
/// to the canonical [`SceneAnchor`] variants (`centerLeft`, `centerRight`,
/// `topCenter`, …). Names that already match the canonical form pass through.
fn canonical_anchor_name(author: &str) -> String {
    match author {
        "middleLeft" | "leftCenter" | "left" => "centerLeft",
        "middleRight" | "rightCenter" | "right" => "centerRight",
        "middleCenter" | "middle" => "center",
        "top" | "topMiddle" => "topCenter",
        "bottom" | "bottomMiddle" => "bottomCenter",
        other => other,
    }
    .into()
}

/// Apply the stashed `at:` anchor to the default scene element by adding a
/// `placementRule` of kind `anchor`. Called from `apply_recipe_defaults`
/// after the default scene has been emitted.
pub fn apply_anchor_to_default_element(
    scenes: &mut [Value],
    anchor: &str,
) -> Result<(), CanonicalizationError> {
    if let Some(scene) = scenes.first_mut()
        && let Some(scene_obj) = scene.as_object_mut()
        && let Some(elements) = scene_obj.get_mut("elements")
        && let Some(elements_arr) = elements.as_array_mut()
        && let Some(first) = elements_arr.first_mut()
        && let Some(elem) = first.as_object_mut()
    {
        elem.insert(
            "placementRule".into(),
            json!({
                "kind": "anchor",
                "anchor": anchor,
                "offsetRows": 0,
                "offsetColumns": 0,
            }),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_topright_extracts_to_extras() {
        let mut recipe = json!({ "id": "x", "at": "topRight" });
        let extras = lift_top_level_extras(&mut recipe).unwrap();
        assert_eq!(extras.anchor.as_deref(), Some("topRight"));
        assert!(!recipe.as_object().unwrap().contains_key("at"));
    }

    #[test]
    fn loop_shorthand_rewrites_lifecycle_clock_to_looping() {
        let mut recipe = json!({ "id": "x", "loop": "2.2s" });
        lift_top_level_extras(&mut recipe).unwrap();
        let clock = &recipe["lifecycle"]["clock"];
        assert_eq!(clock["clockMode"], "looping");
        assert_eq!(clock["period"], json!({ "kind": "seconds", "value": 2.2 }));
    }

    #[test]
    fn extends_without_templates_surfaces_extends_target_not_found() {
        let mut recipe = json!({ "id": "x", "extends": "themes/foo.json" });
        let err = lift_top_level_extras(&mut recipe).unwrap_err();
        assert!(matches!(
            err.kind,
            CanonicalizationErrorKind::ExtendsTargetNotFound { .. }
        ));
    }

    #[test]
    fn no_extras_is_a_noop() {
        let mut recipe = json!({ "id": "x" });
        let extras = lift_top_level_extras(&mut recipe).unwrap();
        assert!(extras.anchor.is_none());
    }
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_lift_top_level_extras.rs</FILE> - <DESC>Lift top-level shorthand fields (at, loop) the canonical recipe doesn't accept directly</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
