// <FILE>crates/tui-vfx-player/src/fnc_aggregate_player_inventory_sources.rs</FILE> - <DESC>Aggregate per-source inventory coverage</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: isolate source aggregation from report DTO.</WCTX>
// <CLOG>0.1.0: INIT — split per-source recipe path aggregation.</CLOG>

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    PlayerInventoryRecipe, PlayerInventorySource,
    fnc_player_inventory_adapter_status::source_adapter_status,
};

/// Aggregate per-recipe inventory entries into per-source coverage rows.
pub(crate) fn aggregate_sources(
    recipes: &[PlayerInventoryRecipe],
    descriptor_source_ids: &BTreeSet<String>,
) -> Vec<PlayerInventorySource> {
    let recipe_paths = source_recipe_paths(recipes);
    descriptor_source_ids
        .iter()
        .chain(recipe_paths.keys())
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|id| source_row(id, descriptor_source_ids, &recipe_paths))
        .collect()
}

fn source_row(
    id: String,
    descriptor_source_ids: &BTreeSet<String>,
    recipe_paths: &BTreeMap<String, Vec<String>>,
) -> PlayerInventorySource {
    let paths = recipe_paths.get(&id).cloned().unwrap_or_default();
    let descriptor_covered = descriptor_source_ids.contains(&id);
    PlayerInventorySource {
        adapter_status: source_adapter_status(&id, descriptor_covered).to_string(),
        represented_by_recipes: !paths.is_empty(),
        descriptor_covered,
        id,
        recipe_paths: paths,
    }
}

fn source_recipe_paths(recipes: &[PlayerInventoryRecipe]) -> BTreeMap<String, Vec<String>> {
    let mut paths = BTreeMap::<String, Vec<String>>::new();
    for recipe in recipes {
        for source_id in &recipe.source_ids {
            paths
                .entry(source_id.clone())
                .or_default()
                .push(recipe.path.clone());
        }
    }
    paths
}

// <FILE>crates/tui-vfx-player/src/fnc_aggregate_player_inventory_sources.rs</FILE> - <DESC>Aggregate per-source inventory coverage</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
