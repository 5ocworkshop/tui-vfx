// <FILE>crates/tui-vfx-player/src/fnc_summarize_player_inventory.rs</FILE> - <DESC>Summarize inventory report rows</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: isolate inventory summary counting.</WCTX>
// <CLOG>0.1.0: INIT — split inventory summary logic from report DTO.</CLOG>

use std::collections::BTreeSet;

use crate::{
    PlayerInventoryEffect, PlayerInventoryRecipe, PlayerInventorySource, PlayerInventorySummary,
    PlayerStatus,
};

/// Summarize per-recipe and aggregate coverage rows into inventory counts.
pub(crate) fn summarize_inventory(
    recipes: &[PlayerInventoryRecipe],
    descriptor_effect_ids: &BTreeSet<String>,
    effects: &[PlayerInventoryEffect],
    sources: &[PlayerInventorySource],
) -> PlayerInventorySummary {
    let mut summary = PlayerInventorySummary {
        total_recipes: recipes.len(),
        descriptor_effect_ids: descriptor_effect_ids.len(),
        source_ids: sources.len(),
        ..PlayerInventorySummary::default()
    };
    count_recipe_statuses(&mut summary, recipes);
    summary.represented_effect_ids = represented_effect_count(effects);
    summary.unrepresented_effect_ids = unrepresented_effect_count(effects);
    summary.unsupported_effect_ids = unsupported_effect_count(effects);
    summary
}

fn count_recipe_statuses(summary: &mut PlayerInventorySummary, recipes: &[PlayerInventoryRecipe]) {
    for recipe in recipes {
        match recipe.status {
            PlayerStatus::Rendered => summary.rendered += 1,
            PlayerStatus::Unsupported => summary.unsupported += 1,
            PlayerStatus::Error => summary.errors += 1,
        }
    }
}

fn represented_effect_count(effects: &[PlayerInventoryEffect]) -> usize {
    effects
        .iter()
        .filter(|effect| effect.descriptor_covered && effect.represented_by_recipes)
        .count()
}

fn unrepresented_effect_count(effects: &[PlayerInventoryEffect]) -> usize {
    effects
        .iter()
        .filter(|effect| effect.descriptor_covered && !effect.represented_by_recipes)
        .count()
}

fn unsupported_effect_count(effects: &[PlayerInventoryEffect]) -> usize {
    effects
        .iter()
        .filter(|effect| effect.represented_by_recipes && effect.adapter_status == "unsupported")
        .count()
}

// <FILE>crates/tui-vfx-player/src/fnc_summarize_player_inventory.rs</FILE> - <DESC>Summarize inventory report rows</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
