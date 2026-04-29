// <FILE>crates/tui-vfx-player/src/fnc_summarize_primitive_field_coverage.rs</FILE> - <DESC>Summarize primitive field coverage entries</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player evidence tooling: aggregate primitive field coverage counters.</WCTX>
// <CLOG>0.1.0: INIT — summarize primitive field coverage counts.</CLOG>

use crate::{PlayerPrimitiveFieldCoverageRecipe, PlayerPrimitiveFieldCoverageSummary};

/// Summarize primitive field coverage recipes.
pub(crate) fn summarize_primitive_field_coverage(
    recipes: &[PlayerPrimitiveFieldCoverageRecipe],
) -> PlayerPrimitiveFieldCoverageSummary {
    let mut summary = PlayerPrimitiveFieldCoverageSummary {
        total_recipes: recipes.len(),
        ..PlayerPrimitiveFieldCoverageSummary::default()
    };
    for recipe in recipes {
        for instance in &recipe.primitive_instances {
            summary.total_primitive_instances += 1;
            summary.used_input_fields += instance.used_inputs.len();
            summary.handled_input_fields += instance.adapter_handled_inputs.len();
            summary.used_but_unhandled_input_fields += instance.used_but_unhandled_inputs.len();
            summary.declared_but_unused_input_fields += instance.declared_but_unused_inputs.len();
            summary.missing_descriptor_input_fields += instance.missing_descriptor_inputs.len();
            if instance.classification == "schemaDecisionNeeded" {
                summary.schema_decision_needed_fields += instance.used_inputs.len();
            }
        }
    }
    summary
}

// <FILE>crates/tui-vfx-player/src/fnc_summarize_primitive_field_coverage.rs</FILE> - <DESC>Summarize primitive field coverage entries</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
