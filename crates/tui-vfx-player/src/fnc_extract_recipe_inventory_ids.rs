// <FILE>crates/tui-vfx-player/src/fnc_extract_recipe_inventory_ids.rs</FILE> - <DESC>Extract recipe ids for inventory rows</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase K2.1 de-slop: keep recipe inventory loading OFPF-sized.</WCTX>
// <CLOG>0.1.0: INIT — split source/effect/unsupported id extraction helpers.</CLOG>

use std::collections::BTreeSet;

use tui_vfx_contract::RecipeDocument;

use crate::{PlayerError, fnc_collect_unsupported_effect_ids::collect_unsupported_effect_ids};

/// Extract source ids referenced by source instances in a recipe.
pub(crate) fn recipe_source_ids(recipe: &RecipeDocument) -> BTreeSet<String> {
    recipe
        .sources
        .values()
        .map(|source| source.source.as_str().to_string())
        .collect()
}

/// Extract effect ids referenced by graph nodes in a recipe.
pub(crate) fn recipe_effect_ids(recipe: &RecipeDocument) -> BTreeSet<String> {
    recipe
        .graph
        .nodes
        .values()
        .map(|node| node.effect.as_str().to_string())
        .collect()
}

/// Extract distinct unsupported effect ids from K0 render diagnostics.
pub(crate) fn unsupported_effect_ids(errors: &[PlayerError]) -> Vec<String> {
    collect_unsupported_effect_ids(errors)
}

/// Return sorted values also present in a descriptor set.
pub(crate) fn intersection(
    values: &BTreeSet<String>,
    descriptors: &BTreeSet<String>,
) -> Vec<String> {
    values
        .intersection(descriptors)
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Return sorted values missing from a descriptor set.
pub(crate) fn difference(values: &BTreeSet<String>, descriptors: &BTreeSet<String>) -> Vec<String> {
    values
        .difference(descriptors)
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

// <FILE>crates/tui-vfx-player/src/fnc_extract_recipe_inventory_ids.rs</FILE> - <DESC>Extract recipe ids for inventory rows</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
