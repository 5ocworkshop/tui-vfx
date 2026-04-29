// <FILE>crates/tui-vfx-player/src/fnc_scan_primitive_field_recipe.rs</FILE> - <DESC>Scan one recipe for primitive input field coverage</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: scan canonical recipe JSON for primitive field coverage.</WCTX>
// <CLOG>0.1.0: INIT — collect source/effect primitive field coverage from one recipe.</CLOG>

use std::{collections::BTreeSet, path::Path};

use crate::{
    PlayerPrimitiveFieldCoverageInstance, PlayerPrimitiveFieldCoverageRecipe,
    PrimitiveFieldDescriptorCoverage,
    fnc_build_primitive_field_instance::build_primitive_field_instance,
};

/// Scan one recipe JSON file for primitive field coverage.
pub(crate) fn scan_primitive_field_recipe(
    path: &Path,
    descriptors: &PrimitiveFieldDescriptorCoverage,
) -> Result<PlayerPrimitiveFieldCoverageRecipe, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("read recipe `{}` failed: {error}", path.display()))?;
    let json: serde_json::Value = serde_json::from_str(&text)
        .map_err(|error| format!("parse recipe `{}` failed: {error}", path.display()))?;
    let mut primitive_instances = Vec::new();
    collect_source_instances(&mut primitive_instances, &json, descriptors);
    collect_effect_instances(&mut primitive_instances, &json, descriptors);
    Ok(PlayerPrimitiveFieldCoverageRecipe {
        recipe_path: path.display().to_string(),
        status: "scanned".to_string(),
        primitive_instances,
        errors: Vec::new(),
        warnings: Vec::new(),
    })
}

fn collect_source_instances(
    out: &mut Vec<PlayerPrimitiveFieldCoverageInstance>,
    json: &serde_json::Value,
    descriptors: &PrimitiveFieldDescriptorCoverage,
) {
    let Some(sources) = json.get("sources").and_then(serde_json::Value::as_object) else {
        return;
    };
    for (instance_id, source) in sources {
        let descriptor_id = source
            .get("source")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        out.push(build_primitive_field_instance(
            "source",
            descriptor_id,
            None,
            Some(instance_id),
            input_keys(source),
            descriptors,
        ));
    }
}

fn collect_effect_instances(
    out: &mut Vec<PlayerPrimitiveFieldCoverageInstance>,
    json: &serde_json::Value,
    descriptors: &PrimitiveFieldDescriptorCoverage,
) {
    let Some(nodes) = json
        .get("graph")
        .and_then(|graph| graph.get("nodes"))
        .and_then(serde_json::Value::as_object)
    else {
        return;
    };
    for (instance_id, node) in nodes {
        let descriptor_id = node
            .get("effect")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        out.push(build_primitive_field_instance(
            "effect",
            descriptor_id,
            Some(instance_id),
            None,
            input_keys(node),
            descriptors,
        ));
    }
}

fn input_keys(value: &serde_json::Value) -> BTreeSet<String> {
    value
        .get("inputs")
        .and_then(serde_json::Value::as_object)
        .map(|inputs| inputs.keys().cloned().collect())
        .unwrap_or_default()
}

// <FILE>crates/tui-vfx-player/src/fnc_scan_primitive_field_recipe.rs</FILE> - <DESC>Scan one recipe for primitive input field coverage</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
