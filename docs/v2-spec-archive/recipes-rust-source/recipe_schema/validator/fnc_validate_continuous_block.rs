// <FILE>src/recipe_schema/validator/fnc_validate_continuous_block.rs</FILE> - <DESC>Validate continuous-block semantic rules</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — validator for no-op continuous blocks and PhaseT-in-continuous warnings.</WCTX>
// <CLOG>0.1.0: add validate_continuous_block.</CLOG>

use tui_vfx_compositor::types::SamplerSpec;

use crate::recipe_schema::{RaClock, RaContinuousConfig};

use super::ValidationIssue;

pub fn validate_continuous_block(continuous: &RaContinuousConfig) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    let mask_empty = continuous.mask.enter.is_empty()
        && continuous.mask.dwell.is_empty()
        && continuous.mask.exit.is_empty();
    let sampler_empty = matches!(continuous.sampler.enter, SamplerSpec::None)
        && matches!(continuous.sampler.dwell, SamplerSpec::None)
        && matches!(continuous.sampler.exit, SamplerSpec::None);
    let filter_empty = continuous.filter.enter.is_empty()
        && continuous.filter.dwell.is_empty()
        && continuous.filter.exit.is_empty();

    if mask_empty && sampler_empty && filter_empty && continuous.styles.is_empty() {
        issues.push(ValidationIssue::warning(
            "continuous block is effectively no-op",
        ));
    }
    if continuous.clock == RaClock::PhaseT {
        issues.push(ValidationIssue::warning(
            "PhaseT inside continuous freezes at the last phase value; did you mean LoopT or AbsoluteT?",
        ));
    }

    issues
}

// <FILE>src/recipe_schema/validator/fnc_validate_continuous_block.rs</FILE> - <DESC>validate_continuous_block</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
