// <FILE>crates/tui-vfx-player/src/fnc_build_primitive_field_instance.rs</FILE> - <DESC>Build primitive field coverage instance entries</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: build per-instance primitive field coverage rows.</WCTX>
// <CLOG>0.1.0: INIT — add primitive field coverage instance builder.</CLOG>

use std::collections::BTreeSet;

use crate::{
    PlayerPrimitiveFieldCoverageInstance, PrimitiveFieldDescriptorCoverage,
    fnc_classify_primitive_field_coverage::{
        classify_primitive_field_coverage, primitive_field_coverage_recommendation,
    },
    fnc_collect_handled_primitive_inputs::collect_handled_primitive_inputs,
};

/// Build one primitive field coverage instance entry.
pub(crate) fn build_primitive_field_instance(
    kind: &str,
    descriptor_id: &str,
    node_id: Option<&String>,
    source_instance_id: Option<&String>,
    used: BTreeSet<String>,
    descriptors: &PrimitiveFieldDescriptorCoverage,
) -> PlayerPrimitiveFieldCoverageInstance {
    let descriptor_inputs = descriptors
        .inputs
        .get(descriptor_id)
        .cloned()
        .unwrap_or_default();
    let adapter_handled = adapter_handled_inputs(descriptor_id, &used);
    let missing_descriptor_inputs = used
        .difference(&descriptor_inputs)
        .cloned()
        .collect::<Vec<_>>();
    let used_but_unhandled_inputs = used
        .difference(&adapter_handled)
        .cloned()
        .collect::<Vec<_>>();
    let declared_but_unused_inputs = descriptor_inputs
        .difference(&used)
        .cloned()
        .collect::<Vec<_>>();
    let classification = classify_primitive_field_coverage(
        &missing_descriptor_inputs,
        &used_but_unhandled_inputs,
        &declared_but_unused_inputs,
    );
    PlayerPrimitiveFieldCoverageInstance {
        kind: kind.to_string(),
        descriptor_id: descriptor_id.to_string(),
        node_id: node_id.cloned(),
        source_instance_id: source_instance_id.cloned(),
        domain: descriptors
            .domains
            .get(descriptor_id)
            .cloned()
            .unwrap_or_default(),
        used_inputs: used.into_iter().collect(),
        descriptor_inputs: descriptor_inputs.into_iter().collect(),
        adapter_handled_inputs: adapter_handled.into_iter().collect(),
        used_but_unhandled_inputs,
        declared_but_unused_inputs,
        missing_descriptor_inputs,
        recommendation: primitive_field_coverage_recommendation(classification).to_string(),
        classification: classification.to_string(),
    }
}

fn adapter_handled_inputs(descriptor_id: &str, used: &BTreeSet<String>) -> BTreeSet<String> {
    used.intersection(&collect_handled_primitive_inputs(descriptor_id, used))
        .cloned()
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_build_primitive_field_instance.rs</FILE> - <DESC>Build primitive field coverage instance entries</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
