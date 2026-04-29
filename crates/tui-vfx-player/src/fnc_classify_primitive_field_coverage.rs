// <FILE>crates/tui-vfx-player/src/fnc_classify_primitive_field_coverage.rs</FILE> - <DESC>Classify primitive field coverage entries</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: keep primitive field coverage classification vocabulary stable.</WCTX>
// <CLOG>0.1.0: INIT — add primitive field coverage classification helpers.</CLOG>

/// Classify one primitive instance from its field-coverage gaps.
pub(crate) fn classify_primitive_field_coverage(
    missing_descriptor_inputs: &[String],
    used_but_unhandled_inputs: &[String],
    declared_but_unused_inputs: &[String],
) -> &'static str {
    if !missing_descriptor_inputs.is_empty() {
        "missingDescriptorInput"
    } else if !used_but_unhandled_inputs.is_empty() {
        "usedButUnhandled"
    } else if !declared_but_unused_inputs.is_empty() {
        "declaredButUnused"
    } else {
        "usedAndHandled"
    }
}

/// Recommend the next action for a primitive field coverage classification.
pub(crate) fn primitive_field_coverage_recommendation(classification: &str) -> &'static str {
    match classification {
        "missingDescriptorInput" => "addDescriptorInput",
        "usedButUnhandled" => "addPlayerAdapter",
        "declaredButUnused" => "addMigrationRule",
        _ => "none",
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_classify_primitive_field_coverage.rs</FILE> - <DESC>Classify primitive field coverage entries</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
