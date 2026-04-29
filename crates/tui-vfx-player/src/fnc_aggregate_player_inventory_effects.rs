// <FILE>crates/tui-vfx-player/src/fnc_aggregate_player_inventory_effects.rs</FILE> - <DESC>Aggregate per-effect inventory coverage</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: isolate effect aggregation from report DTO.</WCTX>
// <CLOG>0.1.0: INIT — split per-effect recipe path aggregation.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    PlayerInventoryEffect, PlayerInventoryRecipe,
    fnc_player_inventory_adapter_status::effect_adapter_status,
};

/// Aggregate per-recipe inventory entries into per-effect coverage rows.
pub(crate) fn aggregate_effects(
    recipes: &[PlayerInventoryRecipe],
    descriptor_effect_ids: &BTreeSet<String>,
) -> Vec<PlayerInventoryEffect> {
    let recipe_paths = effect_recipe_paths(recipes);
    descriptor_effect_ids
        .iter()
        .chain(recipe_paths.keys())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|id| effect_row(id, descriptor_effect_ids, &recipe_paths))
        .collect()
}

fn effect_row(
    id: String,
    descriptor_effect_ids: &BTreeSet<String>,
    recipe_paths: &BTreeMap<String, Vec<String>>,
) -> PlayerInventoryEffect {
    let paths = recipe_paths.get(&id).cloned().unwrap_or_default();
    let descriptor_covered = descriptor_effect_ids.contains(&id);
    PlayerInventoryEffect {
        adapter_status: effect_adapter_status(&id, descriptor_covered).to_string(),
        represented_by_recipes: !paths.is_empty(),
        descriptor_covered,
        id,
        recipe_paths: paths,
    }
}

fn effect_recipe_paths(recipes: &[PlayerInventoryRecipe]) -> BTreeMap<String, Vec<String>> {
    let mut paths = BTreeMap::<String, Vec<String>>::new();
    for recipe in recipes {
        for effect_id in &recipe.effect_ids {
            paths
                .entry(effect_id.clone())
                .or_default()
                .push(recipe.path.clone());
        }
    }
    paths
}

// <FILE>crates/tui-vfx-player/src/fnc_aggregate_player_inventory_effects.rs</FILE> - <DESC>Aggregate per-effect inventory coverage</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
