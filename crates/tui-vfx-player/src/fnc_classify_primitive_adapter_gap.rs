// <FILE>crates/tui-vfx-player/src/fnc_classify_primitive_adapter_gap.rs</FILE> - <DESC>Classify primitive adapter support outcomes</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Primitive adapter work: distinguish text-grid support from styled-cell blockers.</WCTX>
// <CLOG>0.1.0: INIT — classify represented primitive effect support outcomes.</CLOG>

use crate::{PlayerInventoryEffect, PlayerPrimitiveAdapterGapEntry};

/// Classify one inventory effect into a primitive adapter support outcome.
pub(crate) fn classify_primitive_adapter_gap(
    effect: &PlayerInventoryEffect,
) -> PlayerPrimitiveAdapterGapEntry {
    let (outcome, adapter_class, reason) = outcome_for_effect(effect);
    PlayerPrimitiveAdapterGapEntry {
        effect_id: effect.id.clone(),
        descriptor_covered: effect.descriptor_covered,
        represented_by_recipes: effect.represented_by_recipes,
        adapter_status: effect.adapter_status.clone(),
        outcome: outcome.to_string(),
        adapter_class: adapter_class.to_string(),
        recipe_paths: effect.recipe_paths.clone(),
        reason: reason.to_string(),
    }
}

fn outcome_for_effect(
    effect: &PlayerInventoryEffect,
) -> (&'static str, &'static str, &'static str) {
    if !effect.descriptor_covered {
        return (
            "blockedBySemanticDecision",
            "descriptor",
            "The effect id is represented by a recipe but absent from loaded descriptor packs.",
        );
    }
    match effect.adapter_status.as_str() {
        "visible" => (
            "rendered",
            "textGrid",
            "The player emits deterministic visible text-grid evidence.",
        ),
        "noop" => (
            "rendered",
            "textGrid",
            "The effect is intentionally non-mutating for text-grid evidence.",
        ),
        "unsupported" if requires_styled_cell_adapter(&effect.id) => (
            "blockedByStyledCellSubstrate",
            "styledCell",
            "The effect writes color, style, or role data that cannot be honestly represented while styleKnown is false.",
        ),
        "unsupported" => (
            "stillUnsupported",
            "unknown",
            "No honest player adapter is available yet.",
        ),
        _ => (
            "blockedBySemanticDecision",
            "semanticDecision",
            "Adapter semantics are not classified yet.",
        ),
    }
}

fn requires_styled_cell_adapter(effect_id: &str) -> bool {
    matches!(
        effect_id,
        "shader.borderSweep"
            | "shader.linearGradient"
            | "style.baseStyleOverride"
            | "style.colorFade"
    )
}

// <FILE>crates/tui-vfx-player/src/fnc_classify_primitive_adapter_gap.rs</FILE> - <DESC>Classify primitive adapter support outcomes</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
