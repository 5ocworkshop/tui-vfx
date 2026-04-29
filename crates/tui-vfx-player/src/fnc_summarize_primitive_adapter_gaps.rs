// <FILE>crates/tui-vfx-player/src/fnc_summarize_primitive_adapter_gaps.rs</FILE> - <DESC>Summarize primitive adapter gap entries</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: count support outcomes.</WCTX>
// <CLOG>0.1.0: INIT — add gap report summary reducer.</CLOG>

use crate::{PlayerPrimitiveAdapterGapEntry, PlayerPrimitiveAdapterGapSummary};

/// Summarize primitive adapter gap outcomes.
pub(crate) fn summarize_primitive_adapter_gaps(
    effects: &[PlayerPrimitiveAdapterGapEntry],
) -> PlayerPrimitiveAdapterGapSummary {
    let mut summary = PlayerPrimitiveAdapterGapSummary {
        total_effects: effects.len(),
        ..PlayerPrimitiveAdapterGapSummary::default()
    };
    for effect in effects {
        count_effect(&mut summary, effect);
    }
    summary
}

fn count_effect(
    summary: &mut PlayerPrimitiveAdapterGapSummary,
    effect: &PlayerPrimitiveAdapterGapEntry,
) {
    match effect.outcome.as_str() {
        "rendered" => summary.rendered += 1,
        "stillUnsupported" => summary.still_unsupported += 1,
        "blockedByStyledCellSubstrate" => summary.blocked_by_styled_cell_substrate += 1,
        "blockedBySemanticDecision" => summary.blocked_by_semantic_decision += 1,
        _ => {}
    }
    if !effect.descriptor_covered {
        summary.missing_descriptor += 1;
    }
}

// <FILE>crates/tui-vfx-player/src/fnc_summarize_primitive_adapter_gaps.rs</FILE> - <DESC>Summarize primitive adapter gap entries</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
