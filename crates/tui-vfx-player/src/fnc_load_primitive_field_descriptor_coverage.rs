// <FILE>crates/tui-vfx-player/src/fnc_load_primitive_field_descriptor_coverage.rs</FILE> - <DESC>Load descriptor input coverage data</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: extract descriptor input/domain metadata for field coverage.</WCTX>
// <CLOG>0.1.0: INIT — load primitive field descriptor coverage from descriptor packs.</CLOG>

use crate::{DescriptorPackReport, PrimitiveFieldDescriptorCoverage};

/// Load descriptor input/domain coverage data from descriptor pack reports.
pub(crate) fn load_primitive_field_descriptor_coverage(
    packs: &[DescriptorPackReport],
) -> Result<PrimitiveFieldDescriptorCoverage, String> {
    let mut data = PrimitiveFieldDescriptorCoverage::default();
    for pack in packs {
        let text = std::fs::read_to_string(&pack.path)
            .map_err(|error| format!("read descriptor pack `{}` failed: {error}", pack.path))?;
        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|error| format!("parse descriptor pack `{}` failed: {error}", pack.path))?;
        collect_descriptor_section(&mut data, &json, "sourceDescriptors");
        collect_descriptor_section(&mut data, &json, "effects");
    }
    Ok(data)
}

fn collect_descriptor_section(
    data: &mut PrimitiveFieldDescriptorCoverage,
    json: &serde_json::Value,
    section: &str,
) {
    let Some(entries) = json.get(section).and_then(serde_json::Value::as_object) else {
        return;
    };
    for (id, descriptor) in entries {
        let inputs = descriptor
            .get("inputs")
            .and_then(serde_json::Value::as_object)
            .map(|inputs| inputs.keys().cloned().collect())
            .unwrap_or_default();
        let domain = descriptor
            .get("domain")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string);
        data.inputs.insert(id.clone(), inputs);
        data.domains.insert(id.clone(), domain);
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_load_primitive_field_descriptor_coverage.rs</FILE> - <DESC>Load descriptor input coverage data</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
