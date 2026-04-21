// <FILE>src/recipe_schema/validator/fnc_validate_scene_block.rs</FILE> - <DESC>Validate scene-block semantic rules</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — validator for duplicate layer ids, empty scenes, wrap-on-nontext, empty procedural source ids, and empty phase visibility sets.</WCTX>
// <CLOG>0.1.0: add validate_scene_block.</CLOG>

use std::collections::HashSet;

use crate::recipe_schema::{RaContentSource, RaLayerOverflow, RaLayerVisibility, RaSceneConfig};

use super::ValidationIssue;

pub fn validate_scene_block(scene: &RaSceneConfig) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    if scene.layers.is_empty() {
        issues.push(ValidationIssue::warning(
            "scene is empty; only default_role will be painted",
        ));
        return issues;
    }

    let mut seen = HashSet::new();
    for layer in &scene.layers {
        let id = layer.id.as_str().to_string();
        if !seen.insert(id.clone()) {
            issues.push(ValidationIssue::error(format!("duplicate layer id: {id}",)));
        }
        if layer.overflow == RaLayerOverflow::Wrap
            && !matches!(layer.source, RaContentSource::Text(_))
        {
            issues.push(ValidationIssue::error(format!(
                "wrap overflow is only valid for text layers ({id})",
            )));
        }
        if let RaContentSource::Procedural(source) = &layer.source {
            if source.source_id.trim().is_empty() {
                issues.push(ValidationIssue::error(format!(
                    "procedural source_id must not be empty ({id})",
                )));
            }
        }
        if let RaLayerVisibility::Phase(phases) = &layer.visibility {
            if phases.is_empty() {
                issues.push(ValidationIssue::warning(format!(
                    "layer {id} has empty phase visibility set",
                )));
            }
        }
    }

    issues
}

// <FILE>src/recipe_schema/validator/fnc_validate_scene_block.rs</FILE> - <DESC>validate_scene_block</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
